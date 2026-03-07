import { test } from "./fixtures/app-fixture";

test("polygon fill with transparency", async ({ appPage }) => {
  test.setTimeout(60000);

  await appPage.addLibraryLayer("World", "Countries");
  await appPage.closeWindow("Add Layer");

  // Set fill color to semi-transparent blue (linear RGBA)
  await appPage.setFirstLayerFillColor(0.0, 0.0, 1.0, 0.3);

  await appPage.expectScreenshot("transparent-fill.png");
});
