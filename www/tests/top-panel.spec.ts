import { test, expect } from "./fixtures/app-fixture";

test.describe("top panel - menu bar", () => {
  test("top panel is visible with rgis label", async ({ appPage }) => {
    await expect(appPage.page).toHaveScreenshot("top-panel-default.png");
  });

  test("File menu opens", async ({ appPage }) => {
    await appPage.clickWidget("File");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("file-menu-open.png");
  });

  test("View menu opens", async ({ appPage }) => {
    await appPage.clickWidget("View");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("view-menu-open.png");
  });

  test("Help menu opens", async ({ appPage }) => {
    await appPage.clickWidget("Help");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("help-menu-open.png");
  });
});

test.describe("top panel - tool buttons", () => {
  test("Pan Tool is selected by default", async ({ appPage }) => {
    await expect(appPage.page).toHaveScreenshot("pan-tool-default.png");
  });

  test("clicking Query Tool switches active tool", async ({ appPage }) => {
    await appPage.clickWidget("Query Tool");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("query-tool-selected.png");
  });

  test("clicking Measure Tool switches active tool", async ({ appPage }) => {
    await appPage.clickWidget("Measure Tool");
    await appPage.page.waitForTimeout(500);
    await expect(appPage.page).toHaveScreenshot("measure-tool-selected.png");
  });
});
