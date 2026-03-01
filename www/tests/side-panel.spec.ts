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
    // Click the "Add Layer..." button in the side panel
    await appPage.openAddLayerWindow();
    await expect(appPage.page).toHaveScreenshot(
      "add-layer-window-from-button.png",
    );
  });
});
