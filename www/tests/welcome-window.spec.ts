import { test, expect } from "./fixtures/app-fixture";

test.describe("welcome window", () => {
  test("welcome window is visible on app start", async ({ appPage }) => {
    await expect(appPage.page).toHaveScreenshot("welcome-window-visible.png");
  });

  test("welcome window can be closed", async ({ appPage }) => {
    // Close welcome window by clicking its X button
    // X is at top-right of welcome window: ~778px x, ~340px y in 1280x720
    await appPage.clickOnCanvas(0.608, 0.472);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("welcome-window-closed.png");
  });
});
