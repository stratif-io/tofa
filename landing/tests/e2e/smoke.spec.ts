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
  await expect(page.locator('meta[name="description"]')).toHaveAttribute('content', /TOTP authenticator/);
  await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', /og\.png$/);
  await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', 'https://tofa.stratif.io');
  const jsonLd = await page.locator('script[type="application/ld+json"]').all();
  expect(jsonLd.length).toBeGreaterThanOrEqual(3);
});

test('404 page renders', async ({ page }) => {
  await page.goto('/does-not-exist');
  await expect(page.locator('h1')).toContainText(/not found/i);
});

test('install tabs render: shell tab is default, macOS tab shows brew command', async ({ page }) => {
  await page.goto('/#install');
  await expect(page.locator('#install').getByText('tofa.stratif.io/install.sh').first()).toBeVisible();
  await page.locator('#install').getByRole('tab', { name: 'macOS' }).click();
  await expect(page.locator('#install').getByText('brew install tofa').first()).toBeVisible();
});
