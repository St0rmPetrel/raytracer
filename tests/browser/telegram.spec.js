import { test, expect } from '@playwright/test';
import { fileURLToPath } from 'node:url';

const hostPage = fileURLToPath(new URL('./telegram-host.html', import.meta.url));
const sdkMock = fileURLToPath(new URL('./telegram-sdk-mock.js', import.meta.url));

async function settleLayout(frame) {
  await frame.evaluate(
    () => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))),
  );
}

for (const fallback of [false, true]) {
  test(`Telegram layout: ${fallback ? 'page button fallback' : 'native button'}`, async ({
    page,
  }, testInfo) => {
    await page.setViewportSize({ width: 390, height: 844 });
    const errors = [];
    page.on('pageerror', (error) => errors.push(error.message));
    // Host chrome is outside the app iframe, just as it is outside a real webview.
    await page.route('**/__telegram_test_host?*', (route) =>
      route.fulfill({ path: hostPage, contentType: 'text/html' }),
    );
    await page.route('https://telegram.org/**', (route) =>
      route.fulfill({ path: sdkMock, contentType: 'text/javascript' }),
    );
    await page.goto(`/__telegram_test_host?fallback=${fallback}`);
    const app = page.frameLocator('#app');
    // Get the actual Frame for scrolling and animation-frame synchronization.
    const appFrame = page.frames().find((candidate) => candidate.parentFrame());
    await expect(app.locator('#status')).toHaveText(/^Превью ·/);
    await expect(app.locator('#size')).toHaveValue('1024');
    await expect(app.locator('#quality')).toHaveValue('16');
    await app.locator('#live-preview').uncheck();
    if (fallback) {
      await expect(app.locator('body')).not.toHaveClass(/telegram-native/);
    } else {
      await expect(app.locator('body')).toHaveClass(/telegram-native/);
    }
    await expect(page.locator('#native-render')).toBeVisible({
      visible: !fallback,
    });
    await expect(app.locator('#render')).toBeVisible({ visible: fallback });
    await expect(app.locator('#cancel')).toBeHidden();

    for (const id of ['size', 'quality']) {
      await expect(app.locator(`#render-settings #${id}`)).toHaveCount(1);
      const scrollsWithSettings = await app.locator(`#${id}`).evaluate((input) => {
        for (let node = input; node && node !== document.body; node = node.parentElement) {
          if (['fixed', 'sticky'].includes(getComputedStyle(node).position)) return false;
        }
        return true;
      });
      expect(scrollsWithSettings).toBe(true);
    }
    await appFrame.evaluate(() =>
      window.scrollTo(0, document.querySelector('.preview-panel').offsetTop),
    );
    await settleLayout(appFrame);
    const space = await app
      .locator('.preview-panel')
      .evaluate((preview) => innerHeight - preview.getBoundingClientRect().bottom);
    expect(space, 'Space for scrollable settings below the preview').toBeGreaterThan(250);
    await testInfo.attach('telegram-layout', {
      body: await page.screenshot(),
      contentType: 'image/png',
    });
    await appFrame.evaluate(() =>
      window.scrollTo(
        0,
        document.querySelector('#render-settings').offsetTop -
          document.querySelector('.preview-panel').getBoundingClientRect().height -
          16,
      ),
    );
    await settleLayout(appFrame);
    await testInfo.attach('telegram-settings', {
      body: await page.screenshot(),
      contentType: 'image/png',
    });

    const renderButton = fallback ? app.locator('#render') : page.locator('#native-render');
    const cancelButton = fallback ? app.locator('#cancel') : page.locator('#native-render');
    await app.locator('#size').selectOption('512');
    await renderButton.click();
    await expect(app.locator('body')).toHaveClass(/rendering/);
    await expect(app.locator('#render')).toBeHidden();
    await expect(app.locator('#cancel')).toBeVisible({ visible: fallback });
    await expect(cancelButton).toBeEnabled();
    await expect(cancelButton).toHaveText('Отменить');
    await cancelButton.click();
    await expect(app.locator('#status')).toHaveText(/^Отменено/);

    if (!fallback) {
      await app.locator('#open-preview').click();
      await expect(app.locator('#viewer-actions #render')).toBeHidden();
      await expect(app.locator('#viewer-actions #cancel')).toBeHidden();
      await renderButton.click();
      await expect(app.locator('#viewer-footer #status')).toHaveText(/^Уточняем/);
      await cancelButton.click();
      await expect(app.locator('#status')).toHaveText(/^Отменено/);
      await app.locator('#close-preview').click();
      await expect(app.locator('#image-viewer')).toBeHidden();
      await app.locator('#size').selectOption('128');
      await renderButton.click();
      await expect(app.locator('#status')).toHaveText(/^Готово ·/);
      await expect(renderButton).toHaveText('Перерисовать');
      await expect(app.locator('#render')).toBeHidden();
    }
    expect(errors).toEqual([]);
  });
}
