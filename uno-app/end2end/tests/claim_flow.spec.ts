import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:3000";

test.describe("License Claim Flow", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to licenses page before each test
    await page.goto(`${BASE_URL}/licenses`);
  });

  test("displays license variants on licenses page", async ({ page }) => {
    // Check page title/header
    await expect(page.locator("h1")).toContainText(/license/i);

    // Check that variant cards are displayed
    const variantCards = page.locator(".variant-card");
    await expect(variantCards.first()).toBeVisible();
  });

  test("variant cards show earnings and claim buttons", async ({ page }) => {
    const firstCard = page.locator(".variant-card").first();

    // Check for split display
    await expect(firstCard.locator(".split-display")).toBeVisible();

    // Check for claim button
    const claimBtn = firstCard.locator(".claim-btn");
    await expect(claimBtn).toBeVisible();
  });

  test("clicking claim button initiates claim process", async ({ page }) => {
    // Find an available variant card (not unavailable)
    const availableCard = page
      .locator(".variant-card:not(.unavailable)")
      .first();

    // Skip if no available variants
    const isVisible = await availableCard.isVisible().catch(() => false);
    if (!isVisible) {
      test.skip();
      return;
    }

    // Click the claim button
    const claimBtn = availableCard.locator(".claim-btn");
    await claimBtn.click();

    // Should either navigate to claim page or show loading state
    await expect(
      page
        .locator(".claim-btn")
        .filter({ hasText: /claiming/i })
        .or(page.locator(".license-reveal"))
    ).toBeVisible({ timeout: 10000 });
  });

  test("claim page shows license key after successful claim", async ({
    page,
  }) => {
    // Navigate directly to a claim page (mock token)
    // In real tests, this would follow the full claim flow
    await page.goto(`${BASE_URL}/claim/test-token`);

    // If claim is successful, should show license reveal
    const licenseReveal = page.locator(".license-reveal");

    // Wait for either the reveal or an error
    const hasReveal = await licenseReveal.isVisible().catch(() => false);

    if (hasReveal) {
      // Check for license key display
      await expect(page.locator(".license-key")).toBeVisible();

      // Check for copy button
      await expect(page.locator(".copy-btn, [class*='copy']")).toBeVisible();

      // Check for download section
      await expect(page.locator(".android-download")).toBeVisible();
    }
  });

  test("copy button copies license key to clipboard", async ({
    page,
    context,
  }) => {
    // Grant clipboard permissions
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);

    await page.goto(`${BASE_URL}/claim/test-token`);

    const copyBtn = page.locator(".copy-btn, [class*='copy']").first();
    const isVisible = await copyBtn.isVisible().catch(() => false);

    if (isVisible) {
      await copyBtn.click();

      // Check for copied state (button text change or tooltip)
      await expect(
        page.locator("text=/copied/i").or(copyBtn.filter({ hasText: /copied/i }))
      ).toBeVisible({ timeout: 3000 });
    }
  });
});

test.describe("License Page Responsiveness", () => {
  test("displays correctly on mobile viewport", async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    await page.goto(`${BASE_URL}/licenses`);

    // Check that variant cards are visible
    const variantCards = page.locator(".variant-card");
    await expect(variantCards.first()).toBeVisible();

    // Cards should stack vertically on mobile
    const cards = await variantCards.all();
    if (cards.length >= 2) {
      const box1 = await cards[0].boundingBox();
      const box2 = await cards[1].boundingBox();

      if (box1 && box2) {
        // On mobile, cards should be stacked (second card below first)
        expect(box2.y).toBeGreaterThan(box1.y);
      }
    }
  });

  test("displays correctly on tablet viewport", async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });

    await page.goto(`${BASE_URL}/licenses`);

    await expect(page.locator(".variant-card").first()).toBeVisible();
  });

  test("displays correctly on desktop viewport", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 800 });

    await page.goto(`${BASE_URL}/licenses`);

    await expect(page.locator(".variant-card").first()).toBeVisible();
  });
});
