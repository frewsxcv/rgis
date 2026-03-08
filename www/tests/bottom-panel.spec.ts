import { test } from "./fixtures/app-fixture";

test.describe("bottom panel - status bar", () => {
  test("bottom panel shows CRS and mouse position", async ({ appPage }) => {
    await appPage.expectScreenshot("bottom-panel-default.png");
  });

  test("clicking CRS edit button opens Change CRS window", async ({
    appPage,
  }) => {
    // Close the welcome window first
    await appPage.closeWindow("Welcome");

    // Click the CRS edit button
    await appPage.clickWidget("Edit CRS");
    await appPage.expectScreenshot("change-crs-window-open.png");
  });
});
