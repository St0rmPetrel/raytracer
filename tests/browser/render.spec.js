import { test, expect, render, pixels } from './fixtures.js';
import { presets } from '../../web/scene-model.js';

test('maximum quality defaults and preview viewer preserve the same canvas', async ({ page }) => {
  await expect(page.locator('#size')).toHaveValue('1024');
  await expect(page.locator('#quality')).toHaveValue('16');
  const initial = await pixels(page);
  expect(initial.nonBlack).toBe(true);
  expect(initial.width).toBe(128);
  await page.locator('#open-preview').click();
  await expect(page.locator('#image-viewer')).toBeVisible();
  await expect(page.locator('#viewer-frame canvas')).toHaveCount(1);
  expect((await pixels(page)).encoded).toBe(initial.encoded);
  expect(await page.evaluate(() => document.body.style.overflow)).toBe('hidden');
  await page.keyboard.press('Escape');
  await expect(page.locator('#image-viewer')).not.toBeVisible();
  await expect(page.locator('#open-preview canvas')).toHaveCount(1);
  expect(await page.evaluate(() => document.body.style.overflow)).toBe('');
});

for (const [preset, { label }] of Object.entries(presets)) {
  test(`preset: ${label}`, async ({ page }) => {
    await page.locator('#preset').selectOption(preset);
    await render(page, { size: '256', quality: '4' });
    const image = await pixels(page);
    expect(image.width).toBe(256);
    expect(image.nonBlack).toBe(true);
  });
}

test('sample counts 1, 4, 16 and a 512px render complete', async ({ page }) => {
  for (const quality of ['1', '4', '16']) {
    await test.step(`${quality} samples`, async () => {
      await render(page, { quality });
      await expect(page.locator('#status')).toContainText(`${quality} луч`);
    });
  }
  await render(page, { size: '512' });
  expect((await pixels(page)).width).toBe(512);
});

test('cancel in the expanded viewer preserves the last frame', async ({ page }) => {
  await page.locator('#render').click();
  await page.locator('#open-preview').click();
  await expect(page.locator('#viewer-actions #cancel')).toBeVisible();
  await page.locator('#cancel').click();
  await expect(page.locator('#status')).toHaveText(/^Отменено/);
  const cancelled = await pixels(page);
  // Observe beyond a worker message turn to catch stale frames after cancellation.
  await page.waitForTimeout(500);
  expect((await pixels(page)).encoded).toBe(cancelled.encoded);
  await page.locator('#close-preview').click();
  await expect(page.locator('#image-viewer')).not.toBeVisible();
});

test('1024px render keeps updating while expanded and survives closing', async ({
  page,
}, testInfo) => {
  await page.locator('#render').click();
  await page.locator('#open-preview').click();
  const first = await pixels(page);
  await page.waitForFunction(() => {
    const canvas = document.querySelector('#viewer-frame canvas');
    return (
      canvas?.dataset.renderSize === '1024' &&
      Number(canvas.dataset.renderRow) > 0 &&
      Number(canvas.dataset.renderRow) < 1024
    );
  });
  const partial = await page.locator('canvas').evaluate((canvas) => ({
    frame: Number(canvas.dataset.frame),
    row: Number(canvas.dataset.renderRow),
  }));
  await page.waitForFunction(
    (frame) => Number(document.querySelector('canvas')?.dataset.frame) > frame + 2,
    partial.frame,
  );
  expect(Number(await page.locator('canvas').getAttribute('data-render-row'))).toBeGreaterThan(
    partial.row,
  );
  await expect(page.locator('#viewer-footer #status')).toBeVisible();
  await expect(page.locator('#status')).toHaveText(/^Готово · 1024 × 1024 · 16 лучей/, {
    timeout: 90_000,
  });
  const final = await pixels(page);
  expect(final.width).toBe(1024);
  expect(final.nonBlack).toBe(true);
  expect(final.encoded).not.toBe(first.encoded);
  const bounds = await page.locator('canvas').boundingBox();
  const viewport = page.viewportSize();
  expect(bounds.x).toBeGreaterThanOrEqual(0);
  expect(bounds.y).toBeGreaterThanOrEqual(0);
  expect(bounds.x + bounds.width).toBeLessThanOrEqual(viewport.width + 1);
  expect(bounds.y + bounds.height).toBeLessThanOrEqual(viewport.height + 1);
  await testInfo.attach('expanded-render', {
    body: await page.screenshot(),
    contentType: 'image/png',
  });
  await page.locator('#close-preview').click();
  await expect(page.locator('#open-preview canvas')).toHaveCount(1);
  expect((await pixels(page)).encoded).toBe(final.encoded);
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
});
