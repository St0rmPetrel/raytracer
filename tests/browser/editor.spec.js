import { test, expect, render, pixels } from './fixtures.js';

test('object edits, debounced preview and lighting update the scene', async ({ page }) => {
  await page.locator('#preset').selectOption('candy');
  while (await page.locator('#add-object').isEnabled()) await page.locator('#add-object').click();
  await expect(page.locator('#object option')).toHaveCount(8);
  await page.locator('#remove-object').click();
  await expect(page.locator('#object option')).toHaveCount(7);
  await page.locator('#color').fill('#123456');
  await page.locator('#radius').fill('2.5');
  await expect(page.locator('#scene')).toHaveValue(/color = \[18, 52, 86\]/);
  await expect(page.locator('#scene')).toHaveValue(/radius = 2.5/);
  await page.locator('#live-preview').check();
  await page.locator('#x').fill('3');
  await page.locator('#x').fill('4');
  await expect(page.locator('#status')).toHaveText(/^Превью ·/);
  await expect(page.locator('#scene')).toHaveValue(/center = \[4,/);
  await page.locator('#live-preview').uncheck();
  await page.locator('summary').filter({ hasText: 'Камера и свет' }).click();
  await page.locator('#exposure').fill('2.5');
  await page.locator('#light-power').fill('180');
  await expect(page.locator('#scene')).toHaveValue(/exposure = 2.5/);
  await expect(page.locator('#scene')).toHaveValue(/power = 180/);
});

test('invalid manual TOML reports an error and the editor recovers', async ({ page }) => {
  await page.locator('summary').filter({ hasText: 'Для продвинутых' }).click();
  await page.locator('#manual-toml').check();
  await page.locator('#scene').fill('invalid TOML');
  await page.locator('#render').click();
  await expect(page.locator('#status')).toHaveText(/^Не удалось построить/);
  await expect(page.locator('#render')).toBeEnabled();
  await page.locator('#manual-toml').uncheck();
  await render(page);
  expect((await pixels(page)).nonBlack).toBe(true);
});
