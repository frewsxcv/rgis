import { test, expect } from "./fixtures/app-fixture";

test.describe("raster layer - GeoTIFF support", () => {
  test("GeoTIFF radio button appears in file format list", async ({
    appPage,
  }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("File");
    await appPage.page.waitForTimeout(500);

    await appPage.clickWidget("GeoTIFF");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "file-tab-geotiff-selected.png",
    );
  });

  test("load GeoTIFF file via file chooser", async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("File");
    await appPage.page.waitForTimeout(500);

    // Select GeoTIFF format
    await appPage.clickWidget("GeoTIFF");
    await appPage.page.waitForTimeout(500);

    // Trigger file chooser
    const fileChooserPromise = appPage.page.waitForEvent("filechooser");
    await appPage.clickWidget("Select file");
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles("./dist/test-data/sample.tif");
    await appPage.page.waitForTimeout(1000);

    // Click "Add layer"
    await appPage.clickWidget("Add layer");
    await appPage.page.waitForTimeout(3000);

    await expect(appPage.page).toHaveScreenshot("raster-layer-loaded.png");
  });
});
