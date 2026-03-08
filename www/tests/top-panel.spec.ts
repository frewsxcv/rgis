import { test } from "./fixtures/app-fixture";

test.describe("top panel - menu bar", () => {
  test("top panel is visible with rgis label", async ({ appPage }) => {
    await appPage.expectScreenshot("top-panel-default.png");
  });

  test("File menu opens", async ({ appPage }) => {
    await appPage.clickWidget("File");
    await appPage.expectScreenshot("file-menu-open.png");
  });

  test("View menu opens", async ({ appPage }) => {
    await appPage.clickWidget("View");
    await appPage.expectScreenshot("view-menu-open.png");
  });

  test("Help menu opens", async ({ appPage }) => {
    await appPage.clickWidget("Help");
    await appPage.expectScreenshot("help-menu-open.png");
  });
});

test.describe("top panel - tool buttons", () => {
  test("Pan Tool is selected by default", async ({ appPage }) => {
    await appPage.expectScreenshot("pan-tool-default.png");
  });

  test("clicking Query Tool switches active tool", async ({ appPage }) => {
    await appPage.clickWidget("Query Tool");
    await appPage.expectScreenshot("query-tool-selected.png");
  });

  test("clicking Measure Tool switches active tool", async ({ appPage }) => {
    await appPage.clickWidget("Measure Tool");
    await appPage.expectScreenshot("measure-tool-selected.png");
  });
});
