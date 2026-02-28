import { test, expect } from "./fixtures/app-fixture";

test.describe("top panel - menu bar", () => {
  test("top panel is visible with rgis label", async ({ appPage }) => {
    await expect(appPage.page).toHaveScreenshot("top-panel-default.png");
  });

  test("File menu opens", async ({ appPage }) => {
    // "File" is at ~40px from left, ~11px from top in 1280x720
    await appPage.clickOnCanvas(0.031, 0.015);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("file-menu-open.png");
  });

  test("View menu opens", async ({ appPage }) => {
    // "View" is at ~78px from left
    await appPage.clickOnCanvas(0.061, 0.015);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("view-menu-open.png");
  });

  test("Help menu opens", async ({ appPage }) => {
    // "Help" is at ~120px from left
    await appPage.clickOnCanvas(0.094, 0.015);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("help-menu-open.png");
  });
});

test.describe("top panel - tool buttons", () => {
  test("Pan Tool is selected by default", async ({ appPage }) => {
    await expect(appPage.page).toHaveScreenshot("pan-tool-default.png");
  });

  test("clicking Query Tool switches active tool", async ({ appPage }) => {
    // "Query Tool" is at ~280px from left
    await appPage.clickOnCanvas(0.219, 0.015);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("query-tool-selected.png");
  });

  test("clicking Measure Tool switches active tool", async ({ appPage }) => {
    // "Measure Tool" is at ~380px from left
    await appPage.clickOnCanvas(0.297, 0.015);
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("measure-tool-selected.png");
  });
});
