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
    // Click "File" radio button
    await appPage.clickOnCanvas(0.148, 0.166);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("add-layer-file-view.png");
  });

  test("selecting Text source shows format and text input", async ({
    appPage,
  }) => {
    // Click "Text" radio button
    await appPage.clickOnCanvas(0.148, 0.186);
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
    // Click File radio
    await appPage.clickOnCanvas(0.148, 0.166);
    await appPage.page.waitForTimeout(500);

    // Select GeoJSON format radio
    await appPage.clickOnCanvas(0.148, 0.24);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "file-tab-geojson-selected.png",
    );
  });

  test("selecting Shapefile format in file tab", async ({ appPage }) => {
    // Click File radio
    await appPage.clickOnCanvas(0.148, 0.166);
    await appPage.page.waitForTimeout(500);

    // Select Shapefile format radio (third option)
    await appPage.clickOnCanvas(0.148, 0.28);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "file-tab-shapefile-selected.png",
    );
  });
});

test.describe("add layer window - text tab", () => {
  test("selecting GeoJSON format shows hint text", async ({ appPage }) => {
    // Click Text radio
    await appPage.clickOnCanvas(0.148, 0.186);
    await appPage.page.waitForTimeout(500);

    // Select GeoJSON format
    await appPage.clickOnCanvas(0.148, 0.24);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "text-tab-geojson-format.png",
    );
  });

  test("selecting WKT format shows hint text", async ({ appPage }) => {
    // Click Text radio
    await appPage.clickOnCanvas(0.148, 0.186);
    await appPage.page.waitForTimeout(500);

    // Select WKT format (third option)
    await appPage.clickOnCanvas(0.148, 0.28);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("text-tab-wkt-format.png");
  });
});
