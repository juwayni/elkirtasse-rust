import { test, expect } from '@playwright/test';

test('verify application UI and navigation', async ({ page }) => {
  await page.goto('http://localhost:5173');

  // 1. Dashboard should be visible
  await expect(page.locator('h1')).toContainText('Your Digital Sanctuary');
  await page.screenshot({ path: 'dashboard.png' });

  // 2. Navigate to Library
  await page.click('div:has-text("LIBRARY")');
  await expect(page.locator('h2')).toContainText('Categories');
  await page.screenshot({ path: 'library.png' });

  // 3. Navigate to Quran
  await page.click('div:has-text("QURAN")');
  await expect(page.locator('h1')).toContainText('The Holy Quran');
  await page.screenshot({ path: 'quran.png' });

  // 4. Navigate to Narrators
  await page.click('div:has-text("NARRATORS")');
  await expect(page.locator('h1')).toContainText('Narrators');
  await page.screenshot({ path: 'narrators.png' });

  // 5. Navigate to Search
  await page.click('div:has-text("SEARCH")');
  await expect(page.locator('button')).toContainText('SEARCH');
  await page.screenshot({ path: 'search.png' });
});
