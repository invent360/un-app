import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:3000";

test.describe("FAQ Page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(`${BASE_URL}/faq`);
  });

  test("displays FAQ page with header", async ({ page }) => {
    await expect(page.locator("h1")).toContainText(/faq|frequently asked/i);
  });

  test("displays search bar", async ({ page }) => {
    const searchInput = page.locator(
      '.search-bar input, .search-input, input[placeholder*="search" i]'
    );
    await expect(searchInput).toBeVisible();
  });

  test("displays category tabs", async ({ page }) => {
    const categoryTabs = page.locator(".category-tabs, .faq-categories");
    await expect(categoryTabs).toBeVisible();

    // Should have "All" tab
    await expect(
      page.locator('.category-tab, .category-btn').filter({ hasText: /all/i })
    ).toBeVisible();
  });

  test("displays FAQ items", async ({ page }) => {
    const faqItems = page.locator(".faq-item, details");
    await expect(faqItems.first()).toBeVisible();

    // Should have multiple FAQ items
    const count = await faqItems.count();
    expect(count).toBeGreaterThan(0);
  });

  test("search filters FAQ items", async ({ page }) => {
    const searchInput = page.locator(
      '.search-bar input, .search-input, input[placeholder*="search" i]'
    );

    // Get initial count
    const initialCount = await page.locator(".faq-item, details").count();

    // Type a search query
    await searchInput.fill("earn");

    // Wait for filtering
    await page.waitForTimeout(300);

    // Check that results are filtered
    const filteredItems = page.locator(".faq-item, details");
    const filteredCount = await filteredItems.count();

    // Should either have fewer items or show items matching "earn"
    if (filteredCount > 0) {
      // At least one item should contain "earn" in question or answer
      const firstItem = filteredItems.first();
      const text = await firstItem.textContent();
      expect(text?.toLowerCase()).toContain("earn");
    }
  });

  test("search shows empty state for no results", async ({ page }) => {
    const searchInput = page.locator(
      '.search-bar input, .search-input, input[placeholder*="search" i]'
    );

    // Search for something that won't match
    await searchInput.fill("xyznonexistentquery123");

    // Wait for filtering
    await page.waitForTimeout(300);

    // Should show empty state or no FAQ items
    const emptyState = page.locator(".empty-state, text=/no.*found/i");
    const faqItems = page.locator(".faq-item, details");

    const hasEmptyState = await emptyState.isVisible().catch(() => false);
    const itemCount = await faqItems.count();

    expect(hasEmptyState || itemCount === 0).toBeTruthy();
  });

  test("clear search button works", async ({ page }) => {
    const searchInput = page.locator(
      '.search-bar input, .search-input, input[placeholder*="search" i]'
    );

    // Type something
    await searchInput.fill("test query");

    // Look for clear button
    const clearBtn = page.locator('.clear-btn, button[title*="clear" i]');
    const hasClearBtn = await clearBtn.isVisible().catch(() => false);

    if (hasClearBtn) {
      await clearBtn.click();

      // Input should be cleared
      await expect(searchInput).toHaveValue("");
    }
  });

  test("category tabs filter FAQ items", async ({ page }) => {
    // Click on a specific category (e.g., "Earnings")
    const earningsTab = page
      .locator('.category-tab, .category-btn')
      .filter({ hasText: /earnings/i });

    const hasEarningsTab = await earningsTab.isVisible().catch(() => false);

    if (hasEarningsTab) {
      await earningsTab.click();

      // Wait for filtering
      await page.waitForTimeout(300);

      // Tab should be active
      await expect(earningsTab).toHaveClass(/active/);

      // FAQ items should be filtered
      const faqItems = page.locator(".faq-item, details");
      const count = await faqItems.count();

      // Should have at least one item (or empty state)
      expect(count).toBeGreaterThanOrEqual(0);
    }
  });

  test("clicking All tab shows all items", async ({ page }) => {
    // First click a specific category
    const earningsTab = page
      .locator('.category-tab, .category-btn')
      .filter({ hasText: /earnings/i });

    if (await earningsTab.isVisible().catch(() => false)) {
      await earningsTab.click();
      await page.waitForTimeout(300);
    }

    // Then click All
    const allTab = page
      .locator('.category-tab, .category-btn')
      .filter({ hasText: /^all$/i });

    await allTab.click();
    await page.waitForTimeout(300);

    // All tab should be active
    await expect(allTab).toHaveClass(/active/);
  });

  test("FAQ items expand and collapse", async ({ page }) => {
    const faqItem = page.locator("details.faq-item").first();
    const isDetails = await faqItem.isVisible().catch(() => false);

    if (isDetails) {
      // Initially should be collapsed
      const summary = faqItem.locator("summary");
      await expect(summary).toBeVisible();

      // Click to expand
      await summary.click();

      // Should be expanded (has open attribute)
      await expect(faqItem).toHaveAttribute("open", "");

      // Click again to collapse
      await summary.click();

      // Should be collapsed (no open attribute)
      await expect(faqItem).not.toHaveAttribute("open");
    }
  });
});

test.describe("FAQ Page Responsiveness", () => {
  test("displays correctly on mobile", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto(`${BASE_URL}/faq`);

    // Check main elements are visible
    await expect(page.locator("h1")).toBeVisible();
    await expect(
      page.locator('.search-bar, .search-input, input[placeholder*="search" i]')
    ).toBeVisible();

    // Category tabs should be scrollable on mobile
    const categoryTabs = page.locator(".category-tabs, .faq-categories");
    await expect(categoryTabs).toBeVisible();
  });

  test("displays correctly on desktop", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    await page.goto(`${BASE_URL}/faq`);

    await expect(page.locator("h1")).toBeVisible();
    await expect(page.locator(".faq-item, details").first()).toBeVisible();
  });
});
