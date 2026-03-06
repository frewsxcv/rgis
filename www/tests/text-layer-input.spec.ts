import { test } from "./fixtures/app-fixture";

test.describe("text layer input", () => {
  test.beforeEach(async ({ appPage }) => {
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("Text");
  });

  test("selecting Text source shows format options", async ({ appPage }) => {
    await appPage.expectScreenshot("text-tab-selected.png");
  });

  test("selecting GeoJSON format in text tab shows text area", async ({
    appPage,
  }) => {
    await appPage.clickWidget("GeoJSON");
    await appPage.expectScreenshot(
      "text-tab-geojson-textarea.png",
    );
  });

  test("selecting WKT format in text tab shows WKT hint", async ({
    appPage,
  }) => {
    await appPage.clickWidget("WKT");
    await appPage.expectScreenshot("text-tab-wkt-textarea.png");
  });
});
