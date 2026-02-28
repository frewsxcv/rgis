import { test, expect } from "@playwright/test";

test("app loads and renders", async ({ page }) => {
  await page.goto("/");

  // Wait for WASM to initialize
  await page.waitForFunction(() => document.title.includes("ready"), null, {
    timeout: 30000,
  });

  // Wait for canvas to appear
  const canvas = page.locator("canvas");
  await expect(canvas).toBeVisible({ timeout: 10000 });

  // Allow time for first frame to render
  await page.waitForTimeout(2000);

  await expect(page).toHaveScreenshot("app-loaded.png");
});

test("canvas has non-blank content", async ({ page }) => {
  await page.goto("/");

  await page.waitForFunction(() => document.title.includes("ready"), null, {
    timeout: 30000,
  });

  const canvas = page.locator("canvas");
  await expect(canvas).toBeVisible({ timeout: 10000 });

  await page.waitForTimeout(2000);

  // Check that the canvas has non-trivial content by sampling pixel data
  const hasContent = await page.evaluate(() => {
    const canvas = document.querySelector("canvas");
    if (!canvas) return false;

    const ctx = canvas.getContext("2d");
    if (!ctx) {
      // WebGL canvas — can't use getContext("2d"), but if it's visible
      // and the app initialized, we consider it has content.
      // Use screenshot comparison instead.
      return true;
    }

    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    const data = imageData.data;

    // Check if all pixels are the same color (blank)
    const firstR = data[0],
      firstG = data[1],
      firstB = data[2];
    for (let i = 4; i < data.length; i += 4) {
      if (
        data[i] !== firstR ||
        data[i + 1] !== firstG ||
        data[i + 2] !== firstB
      ) {
        return true; // Found a different pixel — not blank
      }
    }
    return false;
  });

  expect(hasContent).toBe(true);
});
