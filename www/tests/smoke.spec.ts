import { test, expect } from "./fixtures/app-fixture";

test("app loads and renders", async ({ appPage }) => {
  await expect(appPage.page).toHaveScreenshot("app-loaded.png");
});

test("canvas is present and fills viewport", async ({ appPage }) => {
  const box = await appPage.canvasBoundingBox();
  // Canvas should roughly fill the 1280x720 viewport
  expect(box.width).toBeGreaterThanOrEqual(1200);
  expect(box.height).toBeGreaterThanOrEqual(650);
});

test("initial render baseline", async ({ appPage }) => {
  await expect(appPage.page).toHaveScreenshot("initial-render.png");
});

test("clicking Library option changes the view", async ({ appPage }) => {
  await appPage.openAddLayerWindow();
  const before = await appPage.page.screenshot();

  // Click the "Library" radio button in the Add Layer panel
  await appPage.clickWidget("Library");

  const after = await appPage.page.screenshot();
  expect(Buffer.compare(before, after)).not.toBe(0);

  await expect(appPage.page).toHaveScreenshot("library-selected.png");
});

test("add World Countries layer from library", async ({ appPage }) => {
  test.setTimeout(60000);

  // Open the Add Layer window
  await appPage.openAddLayerWindow();

  // Click the "Library" radio button
  await appPage.clickWidget("Library");

  // Expand the "World" folder
  await appPage.clickWidget("World");

  // Click the "+ Add" button next to "Countries"
  await appPage.clickWidget("Add:Countries");

  // Wait for the layer to load from network and render
  await appPage.waitForLayerRender();

  await expect(appPage.page).toHaveScreenshot("world-countries-rendered.png");
});
