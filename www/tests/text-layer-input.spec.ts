import { test, expect } from "./fixtures/app-fixture";

test.describe("text layer input", () => {
  test("selecting Text source shows format options", async ({ appPage }) => {
    // Click "Text" radio button at ~170px x, ~147px y
    await appPage.clickOnCanvas(0.133, 0.204);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("text-tab-selected.png");
  });

  test("selecting GeoJSON format in text tab shows text area", async ({
    appPage,
  }) => {
    // Click "Text" radio
    await appPage.clickOnCanvas(0.133, 0.204);
    await appPage.page.waitForTimeout(500);

    // Select GeoJSON format - first radio under format section
    await appPage.clickOnCanvas(0.133, 0.26);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "text-tab-geojson-textarea.png",
    );
  });

  test("selecting WKT format in text tab shows WKT hint", async ({
    appPage,
  }) => {
    // Click "Text" radio
    await appPage.clickOnCanvas(0.133, 0.204);
    await appPage.page.waitForTimeout(500);

    // Select WKT format - third radio under format section
    await appPage.clickOnCanvas(0.133, 0.30);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("text-tab-wkt-textarea.png");
  });
});
