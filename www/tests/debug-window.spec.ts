import { test, expect } from "./fixtures/app-fixture";

test.describe("debug window", () => {
  test("opening debug window from Help menu", async ({ appPage }) => {
    // Click "Help" menu
    await appPage.clickWidget("Help");
    await appPage.page.waitForTimeout(500);

    // Click "Debug stats" in dropdown
    await appPage.clickWidget("Debug stats");
    await appPage.page.waitForTimeout(1000);
    await expect(appPage.page).toHaveScreenshot("debug-window-open.png");
  });
});
