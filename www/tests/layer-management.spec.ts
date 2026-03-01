import { test, expect } from "./fixtures/app-fixture";

test.describe("layer management", () => {
  test.beforeEach(async ({ appPage }) => {
    test.setTimeout(60000);

    // Open the Add Layer window first
    await appPage.openAddLayerWindow();

    // Load World Countries from library
    await appPage.clickWidget("Library");
    await appPage.page.waitForTimeout(500);

    // Expand World folder
    await appPage.clickWidget("World");
    await appPage.page.waitForTimeout(500);

    // Click "+ Add" button next to Countries
    await appPage.clickWidget("Add:Countries");

    // Wait for layer to load
    await appPage.page.waitForTimeout(10000);

    // Close the Add Layer window by clicking its X button
    await appPage.clickOnCanvas(0.222, 0.076);
    await appPage.page.waitForTimeout(500);
  });

  test("loaded layer appears in side panel", async ({
    appPage,
  }) => {
    await expect(appPage.page).toHaveScreenshot(
      "layer-loaded-in-side-panel.png",
    );
  });

  test("right-click layer shows context menu", async ({
    appPage,
  }) => {
    // Right-click on the "World: Countries" layer row to open context menu
    await appPage.rightClickWidget("World: Countries");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("layer-details-expanded.png");
  });

  test("clicking Manage opens manage layer window", async ({ appPage }) => {
    // Right-click to open context menu
    await appPage.rightClickWidget("World: Countries");
    await appPage.page.waitForTimeout(500);

    // Click "Manage..." button
    await appPage.clickWidget("Manage");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("manage-layer-window.png");
  });

  test("layer visibility toggle hides the layer", async ({ appPage }) => {
    // Click the visibility checkbox directly
    await appPage.clickWidget("Toggle Visibility");
    await appPage.page.waitForTimeout(1000);
    await expect(appPage.page).toHaveScreenshot("layer-hidden.png");
  });

  test("zoom to extent centers the map on the layer", async ({ appPage }) => {
    // Right-click to open context menu
    await appPage.rightClickWidget("World: Countries");
    await appPage.page.waitForTimeout(500);

    // Click "Zoom to Extent" button
    await appPage.clickWidget("Zoom to extent");
    await appPage.page.waitForTimeout(1000);
    await expect(appPage.page).toHaveScreenshot("zoom-to-extent.png");
  });

  test("expanding Operations section shows available operations", async ({
    appPage,
  }) => {
    // Right-click to open context menu
    await appPage.rightClickWidget("World: Countries");
    await appPage.page.waitForTimeout(500);

    // Click "Operations" collapsing header
    await appPage.clickWidget("Operations");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("operations-expanded.png");
  });
});
