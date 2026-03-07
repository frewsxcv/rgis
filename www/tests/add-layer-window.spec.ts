import { test } from "./fixtures/app-fixture";

test.describe("add layer window - source selection", () => {
  test.beforeEach(async ({ appPage }) => {
    await appPage.openAddLayerWindow();
  });

  test("initial state shows source radio buttons", async ({ appPage }) => {
    await appPage.expectScreenshot("add-layer-initial.png");
  });

  test("selecting Library source shows library folders", async ({
    appPage,
  }) => {
    await appPage.clickWidget("Library");
    await appPage.expectScreenshot("add-layer-library-view.png");
  });

  test("selecting File source shows format options", async ({ appPage }) => {
    await appPage.clickWidget("File");
    await appPage.expectScreenshot("add-layer-file-view.png");
  });

  test("selecting Text source shows format and text input", async ({
    appPage,
  }) => {
    await appPage.clickWidget("Text");
    await appPage.expectScreenshot("add-layer-text-view.png");
  });
});

test.describe("add layer window - library tab", () => {
  test.beforeEach(async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("Library");
  });

  test("expanding Russia folder shows Country entry", async ({ appPage }) => {
    await appPage.clickWidget("Russia");
    await appPage.expectScreenshot(
      "library-russia-folder-expanded.png",
    );
  });

  test("expanding USA folder shows States entry", async ({ appPage }) => {
    await appPage.clickWidget("USA");
    await appPage.expectScreenshot(
      "library-usa-folder-expanded.png",
    );
  });

  test("expanding World folder shows entries", async ({ appPage }) => {
    await appPage.clickWidget("World");
    await appPage.expectScreenshot(
      "library-world-folder-expanded.png",
    );
  });
});

test.describe("add layer window - file tab", () => {
  test.beforeEach(async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("File");
  });

  test("selecting GeoJSON format in file tab", async ({ appPage }) => {
    await appPage.clickWidget("GeoJSON");
    await appPage.expectScreenshot(
      "file-tab-geojson-selected.png",
    );
  });

  test("selecting Shapefile format in file tab", async ({ appPage }) => {
    await appPage.clickWidget("Shapefile");
    await appPage.expectScreenshot(
      "file-tab-shapefile-selected.png",
    );
  });
});
