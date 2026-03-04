import { test, expect } from "./fixtures/app-fixture";

test.describe("raster layer - GeoTIFF support", () => {
  test("GeoTIFF radio button appears in file format list", async ({
    appPage,
  }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("File");

    await appPage.clickWidget("GeoTIFF");
    await expect(appPage.page).toHaveScreenshot(
      "file-tab-geotiff-selected.png",
    );
  });

  test("load GeoTIFF file via file chooser and render on map", async ({
    appPage,
  }) => {
    test.setTimeout(60000);

    await appPage.openAddLayerWindow();
    await appPage.clickWidget("File");

    // Select GeoTIFF format
    await appPage.clickWidget("GeoTIFF");

    // Trigger file chooser
    const fileChooserPromise = appPage.page.waitForEvent("filechooser");
    await appPage.clickWidget("Select file");
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles("./dist/test-data/sample.tif");
    await appPage.waitForNextFrame();

    // Dismiss the rfd file dialog by clicking its Ok button
    await appPage.page.locator("#rfd-overlay .rfd-button", { hasText: "Ok" }).click();
    await appPage.page.waitForTimeout(1000);

    // Click "Add layer"
    await appPage.clickWidget("Add layer");
    await appPage.page.waitForTimeout(5000);

    await expect(appPage.page).toHaveScreenshot(
      "raster-layer-rendered.png",
    );
  });
});
