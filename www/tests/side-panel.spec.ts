import { test } from "./fixtures/app-fixture";

test.describe("side panel - layers panel", () => {
  test("side panel shows Layers heading and Add Layer button", async ({
    appPage,
  }) => {
    await appPage.expectScreenshot("side-panel-default.png");
  });

  test("clicking Add Layer button opens Add Layer window", async ({
    appPage,
  }) => {
    // Click the "Add Layer..." button in the side panel
    await appPage.openAddLayerWindow();
    await appPage.expectScreenshot(
      "add-layer-window-from-button.png",
    );
  });
});
