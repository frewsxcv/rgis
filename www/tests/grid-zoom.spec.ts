import { test, expect } from "./fixtures/app-fixture";

test("grid lines persist after zoom in", async ({ appPage }) => {
  await appPage.expectScreenshot("grid-before-zoom.png");

  await appPage.clickWidget("Zoom In");
  await appPage.page.waitForTimeout(500);
  await appPage.expectScreenshot("grid-after-zoom-in.png");
});

test("grid lines persist after zoom out", async ({ appPage }) => {
  await appPage.clickWidget("Zoom Out");
  await appPage.page.waitForTimeout(500);
  await appPage.expectScreenshot("grid-after-zoom-out.png");
});
