import { test, expect } from "./fixtures/app-fixture";

test.describe("change CRS window", () => {
  test("CRS window shows input field and Set button", async ({ appPage }) => {
    // Close other windows first by clicking empty canvas
    await appPage.clickOnCanvas(0.7, 0.5);
    await appPage.page.waitForTimeout(500);

    // Click the CRS edit button in bottom panel
    await appPage.clickWidget("Edit CRS");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("change-crs-window.png");
  });
});
