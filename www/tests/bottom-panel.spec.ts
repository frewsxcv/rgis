import { test, expect } from "./fixtures/app-fixture";

test.describe("bottom panel - status bar", () => {
  test("bottom panel shows CRS and mouse position", async ({ appPage }) => {
    await expect(appPage.page).toHaveScreenshot("bottom-panel-default.png");
  });

  test("clicking CRS edit button opens Change CRS window", async ({
    appPage,
  }) => {
    // Close the welcome/add layer windows first by clicking on empty canvas area
    await appPage.clickOnCanvas(0.7, 0.5);
    await appPage.page.waitForTimeout(500);

    // Click the ✏ (pencil) button next to CRS display in bottom-right
    // It's at far right of the bottom bar, roughly x=1260, y=710 in 1280x720
    await appPage.clickOnCanvas(0.984, 0.986);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("change-crs-window-open.png");
  });
});
