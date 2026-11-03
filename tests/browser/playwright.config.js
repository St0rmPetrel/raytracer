import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: '.',
  timeout: 120_000,
  expect: { timeout: 10_000 },
  forbidOnly: Boolean(process.env.CI),
  workers: 1,
  reporter: [['list'], ['html', { outputFolder: '../../review-output/report', open: 'never' }]],
  outputDir: '../../review-output/test-results',
  use: {
    baseURL: process.env.PREVIEW_URL || 'http://127.0.0.1:8000',
    launchOptions: { executablePath: process.env.CHROME_PATH || undefined },
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure',
  },
  projects: [
    {
      name: 'desktop',
      testIgnore: '**/telegram.spec.js',
      use: { browserName: 'chromium', viewport: { width: 1280, height: 900 } },
    },
    {
      name: 'mobile',
      use: { ...devices['iPhone 13'], browserName: 'chromium' },
    },
  ],
});
