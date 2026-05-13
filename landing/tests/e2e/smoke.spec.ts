import { test, expect } from '@playwright/test';

test('home page renders with all key sections', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/TOFA/);
  await expect(page.locator('h1')).toContainText('Stop grabbing your');
  await expect(page.locator('#demos')).toBeVisible();
  await expect(page.locator('#install')).toBeVisible();
  await expect(page.locator('#faq')).toBeVisible();
});

test('SEO metadata is present', async ({ page }) => {
  await page.goto('/');
  await expect(page.locator('meta[name="description"]')).toHaveAttribute('content', /open-source 2FA/);
  await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', /og\.png$/);
  await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', 'https://tofa.stratif.io');
  const jsonLd = await page.locator('script[type="application/ld+json"]').all();
  expect(jsonLd.length).toBeGreaterThanOrEqual(3);
});

test('404 page renders', async ({ page }) => {
  await page.goto('/does-not-exist');
  await expect(page.locator('h1')).toContainText(/not found/i);
});

test('install tabs render and macOS tab shows brew command', async ({ page }) => {
  await page.goto('/#install');
  await expect(page.locator('code').first()).toContainText('brew install tofa');
});
