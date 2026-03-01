import { test, expect } from "./fixtures/app-fixture";

test.describe("add layer window - source selection", () => {
  test("initial state shows source radio buttons", async ({ appPage }) => {
    // The Add Layer window should be visible on startup
    await expect(appPage.page).toHaveScreenshot("add-layer-initial.png");
  });

  test("selecting Library source shows library folders", async ({
    appPage,
  }) => {
    // Click "Library" radio button
    await appPage.clickOnCanvas(0.148, 0.146);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("add-layer-library-view.png");
  });

  test("selecting File source shows format options", async ({ appPage }) => {
    // Click "File" radio button at ~170px x, ~126px y
    await appPage.clickOnCanvas(0.133, 0.175);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("add-layer-file-view.png");
  });

  test("selecting Text source shows format and text input", async ({
    appPage,
  }) => {
    // Click "Text" radio button at ~170px x, ~147px y
    await appPage.clickOnCanvas(0.133, 0.204);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("add-layer-text-view.png");
  });
});

test.describe("add layer window - library tab", () => {
  test("expanding Russia folder shows Country entry", async ({ appPage }) => {
    // Click Library radio
    await appPage.clickOnCanvas(0.148, 0.146);
    await appPage.page.waitForTimeout(500);

    // Click the Russia folder to expand it
    await appPage.clickOnCanvas(0.137, 0.31);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "library-russia-folder-expanded.png",
    );
  });

  test("expanding USA folder shows States entry", async ({ appPage }) => {
    // Click Library radio
    await appPage.clickOnCanvas(0.148, 0.146);
    await appPage.page.waitForTimeout(500);

    // Click the USA folder to expand it
    await appPage.clickOnCanvas(0.137, 0.338);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "library-usa-folder-expanded.png",
    );
  });

  test("expanding World folder shows entries", async ({ appPage }) => {
    // Click Library radio
    await appPage.clickOnCanvas(0.148, 0.146);
    await appPage.page.waitForTimeout(500);

    // Click the World folder to expand it
    await appPage.clickOnCanvas(0.137, 0.424);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "library-world-folder-expanded.png",
    );
  });
});

test.describe("add layer window - file tab", () => {
  test("selecting GeoJSON format in file tab", async ({ appPage }) => {
    // Click File radio at ~170px x, ~126px y
    await appPage.clickOnCanvas(0.133, 0.175);
    await appPage.page.waitForTimeout(500);

    // Select GeoJSON format radio - first format option
    await appPage.clickOnCanvas(0.133, 0.27);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "file-tab-geojson-selected.png",
    );
  });

  test("selecting Shapefile format in file tab", async ({ appPage }) => {
    // Click File radio at ~170px x, ~126px y
    await appPage.clickOnCanvas(0.133, 0.175);
    await appPage.page.waitForTimeout(500);

    // Select Shapefile format radio (third option)
    await appPage.clickOnCanvas(0.133, 0.33);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "file-tab-shapefile-selected.png",
    );
  });
});

