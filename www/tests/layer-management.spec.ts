import { test, expect } from "./fixtures/app-fixture";

test.describe("layer management", () => {
  test.beforeEach(async ({ appPage }) => {
    test.setTimeout(60000);

    // Load World Countries from library
    // Click Library radio at ~190px x, ~106px y
    await appPage.clickOnCanvas(0.148, 0.147);
    await appPage.page.waitForTimeout(500);

    // Expand World folder
    await appPage.clickOnCanvas(0.137, 0.424);
    await appPage.page.waitForTimeout(500);

    // Click "+ Add" button next to Countries
    await appPage.clickOnCanvas(0.162, 0.454);

    // Wait for layer to load
    await appPage.page.waitForTimeout(10000);

    // Close the Add Layer window by clicking its X button (~284px x, ~55px y)
    await appPage.clickOnCanvas(0.222, 0.076);
    await appPage.page.waitForTimeout(500);
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
    // Click "World: Countries" collapsing header triangle at ~15px x, ~78px y
    await appPage.clickOnCanvas(0.055, 0.108);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("layer-details-expanded.png");
  });

  test("clicking Manage opens manage layer window", async ({ appPage }) => {
    // Expand the layer header
    await appPage.clickOnCanvas(0.055, 0.108);
    await appPage.page.waitForTimeout(500);

    // Click "✏ Manage" button at ~100px x, ~117px y
    await appPage.clickOnCanvas(0.078, 0.163);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("manage-layer-window.png");
  });

  test("layer visibility toggle hides the layer", async ({ appPage }) => {
    // Expand the layer header
    await appPage.clickOnCanvas(0.055, 0.108);
    await appPage.page.waitForTimeout(500);

    // Click "👁 Hide" button at ~100px x, ~159px y
    await appPage.clickOnCanvas(0.078, 0.221);
    await appPage.page.waitForTimeout(1000);
    await expect(appPage.page).toHaveScreenshot("layer-hidden.png");
  });

  test("zoom to extent centers the map on the layer", async ({ appPage }) => {
    // Expand the layer header
    await appPage.clickOnCanvas(0.055, 0.108);
    await appPage.page.waitForTimeout(500);

    // Click "🔎 Zoom to extent" button at ~100px x, ~180px y
    await appPage.clickOnCanvas(0.078, 0.250);
    await appPage.page.waitForTimeout(1000);
    await expect(appPage.page).toHaveScreenshot("zoom-to-extent.png");
  });

  test("expanding Operations section shows available operations", async ({
    appPage,
  }) => {
    // Expand the layer header
    await appPage.clickOnCanvas(0.055, 0.108);
    await appPage.page.waitForTimeout(500);

    // Click "⚙ Operations" collapsing header at ~80px x, ~222px y
    await appPage.clickOnCanvas(0.055, 0.308);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("operations-expanded.png");
  });
});
