import { test, expect } from "./fixtures/app-fixture";

test("app loads and renders", async ({ appPage }) => {
  await appPage.expectScreenshot("app-loaded.png");
});

test("canvas has non-blank content", async ({ appPage }) => {
  const hasContent = await appPage.regionHasContent(0, 0, 1, 1);
  expect(hasContent).toBe(true);
});

test("canvas is present and fills viewport", async ({ appPage }) => {
  const box = await appPage.canvasBoundingBox();
  // Canvas should roughly fill the 1280x720 viewport
  expect(box.width).toBeGreaterThanOrEqual(1200);
  expect(box.height).toBeGreaterThanOrEqual(650);
});

test("initial render baseline", async ({ appPage }) => {
  await appPage.expectScreenshot("initial-render.png");
});

test("clicking Library option changes the view", async ({ appPage }) => {
  await appPage.openAddLayerWindow();
  const before = await appPage.page.screenshot();

  // Click the "Library" radio button in the Add Layer panel
  await appPage.clickWidget("Library");

  const after = await appPage.page.screenshot();
  expect(Buffer.compare(before, after)).not.toBe(0);

  await appPage.expectScreenshot("library-selected.png");
});

test("add World Countries layer from library", async ({ appPage }) => {
  test.setTimeout(60000);

  await appPage.addLibraryLayer("World", "Countries");

  await appPage.expectScreenshot("world-countries-rendered.png");
});
