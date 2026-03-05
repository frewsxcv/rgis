import { test, expect } from "./fixtures/app-fixture";

test.describe("layer management", () => {
  test.beforeEach(async ({ appPage }) => {
    test.setTimeout(60000);

    // Open the Add Layer window first
    await appPage.openAddLayerWindow();

    // Load World Countries from library
    await appPage.clickWidget("Library");

    // Expand World folder
    await appPage.clickWidget("World");

    // Click "+ Add" button next to Countries
    await appPage.clickWidget("Add:Countries");

    // Wait for layer to load
    await appPage.waitForLayerRender();

    // Close the Add Layer window by clicking its X button
    await appPage.clickOnCanvas(0.222, 0.076);
  });

  test("loaded layer appears in side panel with collapsing header", async ({
    appPage,
  }) => {
    await expect(appPage.page).toHaveScreenshot(
      "layer-loaded-in-side-panel.png",
    );
  });

  test("expanding layer shows layer details and buttons", async ({
    appPage,
  }) => {
    // Click the ▶ toggle arrow on the "World: Countries" collapsing header
    await appPage.clickWidget("World: Countries");
    await expect(appPage.page).toHaveScreenshot("layer-details-expanded.png");
  });

  test("clicking Manage opens manage layer window", async ({ appPage }) => {
    // Expand the layer header
    await appPage.clickWidget("World: Countries");

    // Click "Manage..." button
    await appPage.clickWidget("Manage");
    await expect(appPage.page).toHaveScreenshot("manage-layer-window.png");
  });

  test("layer visibility toggle hides the layer", async ({ appPage }) => {
    // Expand the layer header
    await appPage.clickWidget("World: Countries");

    // Click "Visible" checkbox
    await appPage.clickWidget("Toggle Visibility");
    await appPage.waitForNextFrame();
    await expect(appPage.page).toHaveScreenshot("layer-hidden.png");
  });

  test("zoom to extent centers the map on the layer", async ({ appPage }) => {
    // Expand the layer header
    await appPage.clickWidget("World: Countries");

    // Click "Zoom to Extent" button
    await appPage.clickWidget("Zoom to extent");
    await appPage.waitForNextFrame();
    await expect(appPage.page).toHaveScreenshot("zoom-to-extent.png");
  });

  test("manage layer window shows stroke width control", async ({
    appPage,
  }) => {
    // Expand the layer header
    await appPage.clickWidget("World: Countries");

    // Click "Manage..." button
    await appPage.clickWidget("Manage");

    // Verify the stroke width label is present
    const rect = await appPage.page.evaluate(
      (l) => (window as any).get_widget_rect(l),
      "Stroke width",
    );
    expect(rect).toBeTruthy();

    await expect(appPage.page).toHaveScreenshot(
      "manage-layer-stroke-width.png",
    );
  });

  test("expanding Operations section shows available operations", async ({
    appPage,
  }) => {
    // Expand the layer header
    await appPage.clickWidget("World: Countries");

    // Click "Operations" collapsing header
    await appPage.clickWidget("Operations");
    await expect(appPage.page).toHaveScreenshot("operations-expanded.png");
  });
});
