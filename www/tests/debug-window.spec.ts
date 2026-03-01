import { test, expect } from "./fixtures/app-fixture";

test.describe("debug window", () => {
  test("opening debug window from Help menu", async ({ appPage }) => {
    // Click "Help" menu at ~120px from left, ~11px from top
    await appPage.clickOnCanvas(0.094, 0.015);
    await appPage.page.waitForTimeout(500);

    // Click "Debug stats" in dropdown - it's the first item below Help
    // Menu items appear below the menu button, roughly at ~120px x, ~35px y
    await appPage.clickOnCanvas(0.094, 0.049);
    await appPage.page.waitForTimeout(1000);
    await expect(appPage.page).toHaveScreenshot("debug-window-open.png");
  });
});
