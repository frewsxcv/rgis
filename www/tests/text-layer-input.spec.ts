import { test, expect } from "./fixtures/app-fixture";

test.describe("text layer input", () => {
  test("selecting Text source shows format options", async ({ appPage }) => {
    await appPage.clickWidget("Text");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("text-tab-selected.png");
  });

  test("selecting GeoJSON format in text tab shows text area", async ({
    appPage,
  }) => {
    await appPage.clickWidget("Text");
    await appPage.page.waitForTimeout(500);

    await appPage.clickWidget("GeoJSON");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "text-tab-geojson-textarea.png",
    );
  });

  test("selecting WKT format in text tab shows WKT hint", async ({
    appPage,
  }) => {
    await appPage.clickWidget("Text");
    await appPage.page.waitForTimeout(500);

    await appPage.clickWidget("WKT");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("text-tab-wkt-textarea.png");
  });
});
