import { test, expect } from "./fixtures/app-fixture";

test.describe("welcome window", () => {
  test("welcome window is visible on app start", async ({ appPage }) => {
    await expect(appPage.page).toHaveScreenshot("welcome-window-visible.png");
  });

  test("welcome window can be closed", async ({ appPage }) => {
    // Close welcome window by clicking its X button (egui internal)
    await appPage.clickOnCanvas(0.608, 0.472);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("welcome-window-closed.png");
  });
});
