import { test, expect } from "./fixtures/app-fixture";

test.describe("adding layer from text input", () => {
  test("enter GeoJSON text and add layer", async ({ appPage }) => {
    test.setTimeout(30000);

    // Click Text radio
    await appPage.clickOnCanvas(0.148, 0.186);
    await appPage.page.waitForTimeout(500);

    // Select GeoJSON format
    await appPage.clickOnCanvas(0.148, 0.24);
    await appPage.page.waitForTimeout(500);

    // Click on the text input area to focus it
    await appPage.clickOnCanvas(0.22, 0.42);
    await appPage.page.waitForTimeout(300);

    // Type GeoJSON text - a simple point
    await appPage.page.keyboard.type(
      '{"type":"FeatureCollection","features":[{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]},"properties":{}}]}',
      { delay: 10 },
    );
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot(
      "text-input-geojson-entered.png",
    );
  });

  test("enter WKT text and see hint", async ({ appPage }) => {
    // Click Text radio
    await appPage.clickOnCanvas(0.148, 0.186);
    await appPage.page.waitForTimeout(500);

    // Select WKT format
    await appPage.clickOnCanvas(0.148, 0.28);
    await appPage.page.waitForTimeout(500);

    // The text area should show WKT hint text
    await expect(appPage.page).toHaveScreenshot("text-input-wkt-hint.png");
  });
});
