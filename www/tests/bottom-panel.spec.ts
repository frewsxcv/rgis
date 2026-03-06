import { test, expect } from "./fixtures/app-fixture";

test.describe("bottom panel - status bar", () => {
  test("bottom panel shows CRS and mouse position", async ({ appPage }) => {
    await appPage.stabilizeForScreenshot();
    await expect(appPage.page).toHaveScreenshot("bottom-panel-default.png");
  });

  test("clicking CRS edit button opens Change CRS window", async ({
    appPage,
  }) => {
    // Close the welcome window first
    await appPage.closeWindow("Welcome");

    // Click the CRS edit button
    await appPage.clickWidget("Edit CRS");
    await appPage.stabilizeForScreenshot();
    await expect(appPage.page).toHaveScreenshot("change-crs-window-open.png");
  });
});
