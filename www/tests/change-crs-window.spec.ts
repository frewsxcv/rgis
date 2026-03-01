import { test, expect } from "./fixtures/app-fixture";

test.describe("change CRS window", () => {
  test("CRS window shows input field and Set button", async ({ appPage }) => {
    // Close other windows first by clicking empty canvas
    await appPage.clickOnCanvas(0.7, 0.5);
    await appPage.page.waitForTimeout(500);

    // Click the ✏ button in bottom panel to open Change CRS window
    // Pencil button at far right bottom: ~1260px x, ~710px y in 1280x720
    await appPage.clickOnCanvas(0.984, 0.986);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("change-crs-window.png");
  });
});
