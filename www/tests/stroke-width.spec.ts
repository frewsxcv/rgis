import { test, expect } from "./fixtures/app-fixture";

test.describe("stroke width", () => {
  test.beforeEach(async ({ appPage }) => {
    test.setTimeout(60000);

    // Open the Add Layer window
    await appPage.openAddLayerWindow();

    // Load Tectonic Plate Boundaries from library (line geometry)
    await appPage.clickWidget("Library");
    await appPage.clickWidget("World");
    await appPage.clickWidget("Add:Tectonic Plate Boundaries");

    // Wait for layer to load (remote dataset may take longer)
    await appPage.waitForLayerRender(undefined, 50000);

    // Close the Add Layer window
    await appPage.clickOnCanvas(0.222, 0.076);

    // Expand the layer header
    await appPage.clickWidget("World: Tectonic Plate Boundaries");
  });

  test("manage layer window shows stroke width slider", async ({
    appPage,
  }) => {
    await appPage.clickWidget("Manage");

    const rect = await appPage.page.evaluate(
      (l) => (window as any).get_widget_rect(l),
      "Stroke width",
    );
    expect(rect).toBeTruthy();

    await expect(appPage.page).toHaveScreenshot(
      "manage-layer-stroke-width.png",
    );
  });
});
