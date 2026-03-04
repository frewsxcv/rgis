import { test, expect } from "./fixtures/app-fixture";

test.describe("add layer window - source selection", () => {
  test("initial state shows source radio buttons", async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await expect(appPage.page).toHaveScreenshot("add-layer-initial.png");
  });

  test("selecting Library source shows library folders", async ({
    appPage,
  }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("Library");
    await expect(appPage.page).toHaveScreenshot("add-layer-library-view.png");
  });

  test("selecting File source shows format options", async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("File");
    await expect(appPage.page).toHaveScreenshot("add-layer-file-view.png");
  });

  test("selecting Text source shows format and text input", async ({
    appPage,
  }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("Text");
    await expect(appPage.page).toHaveScreenshot("add-layer-text-view.png");
  });
});

test.describe("add layer window - library tab", () => {
  test("expanding Russia folder shows Country entry", async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("Library");

    await appPage.clickWidget("Russia");
    await expect(appPage.page).toHaveScreenshot(
      "library-russia-folder-expanded.png",
    );
  });

  test("expanding USA folder shows States entry", async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("Library");

    await appPage.clickWidget("USA");
    await expect(appPage.page).toHaveScreenshot(
      "library-usa-folder-expanded.png",
    );
  });

  test("expanding World folder shows entries", async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("Library");

    await appPage.clickWidget("World");
    await expect(appPage.page).toHaveScreenshot(
      "library-world-folder-expanded.png",
    );
  });
});

test.describe("add layer window - file tab", () => {
  test("selecting GeoJSON format in file tab", async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("File");

    await appPage.clickWidget("GeoJSON");
    await expect(appPage.page).toHaveScreenshot(
      "file-tab-geojson-selected.png",
    );
  });

  test("selecting Shapefile format in file tab", async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("File");

    await appPage.clickWidget("Shapefile");
    await expect(appPage.page).toHaveScreenshot(
      "file-tab-shapefile-selected.png",
    );
  });
});
