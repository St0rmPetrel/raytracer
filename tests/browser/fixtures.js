import { test as base, expect } from '@playwright/test';

export const test = base.extend({
  page: async ({ page }, use) => {
    const errors = [];
    page.on('pageerror', (error) => errors.push(error.message));
    // The ordinary browser must work without the optional Telegram SDK.
    await page.route('https://telegram.org/**', (route) => route.abort());
    await page.goto('/');
    await expect(page.locator('#status')).toHaveText(/^Превью ·/);
    await page.locator('#live-preview').uncheck();
    await use(page);
    expect(errors, 'No uncaught browser errors').toEqual([]);
  },
});

export { expect };

export async function render(page, { size = '128', quality = '16' } = {}) {
  await page.locator('#size').selectOption(size);
  await page.locator('#quality').selectOption(quality);
  await page.locator('#render').click();
  await expect(page.locator('#status')).toHaveText(/^Готово ·/, {
    timeout: 90_000,
  });
}

export async function pixels(page) {
  return page.locator('#scene-canvas').evaluate((canvas) => {
    const data = canvas.getContext('2d').getImageData(0, 0, canvas.width, canvas.height).data;
    return {
      width: canvas.width,
      nonBlack: data.some((value, index) => index % 4 !== 3 && value > 0),
      encoded: canvas.toDataURL(),
    };
  });
}
