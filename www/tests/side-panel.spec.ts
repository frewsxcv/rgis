import { test, expect } from "./fixtures/app-fixture";

test.describe("side panel - layers panel", () => {
  test("side panel shows Layers heading and Add Layer button", async ({
    appPage,
  }) => {
    await expect(appPage.page).toHaveScreenshot("side-panel-default.png");
  });

  test("clicking Add Layer button opens Add Layer window", async ({
    appPage,
  }) => {
    // First close the Add Layer window that opens by default
    // Click its X button at ~284px x, ~55px y
    await appPage.clickOnCanvas(0.222, 0.076);
    await appPage.page.waitForTimeout(500);

    // Now click the "+ Add Layer" button at ~100px x, ~57px y
    await appPage.clickOnCanvas(0.078, 0.079);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "add-layer-window-from-button.png",
    );
  });
});
