import { test, expect } from "./fixtures/app-fixture";

test.describe("keyboard panning", () => {
  test("arrow key changes the view", async ({ appPage }) => {
    const before = await appPage.page.screenshot();
    await appPage.pressKey("ArrowRight", 5);
    const after = await appPage.page.screenshot();

    expect(Buffer.compare(before, after)).not.toBe(0);
  });

  test("pan right then left returns to similar view", async ({ appPage }) => {
    const original = await appPage.page.screenshot();

    await appPage.pressKey("ArrowRight", 5);
    await appPage.pressKey("ArrowLeft", 5);

    await expect(appPage.page).toHaveScreenshot("pan-right-left-return.png", {
      maxDiffPixelRatio: 0.05,
    });
  });

  test("pan up then down returns to similar view", async ({ appPage }) => {
    const original = await appPage.page.screenshot();

    await appPage.pressKey("ArrowUp", 5);
    await appPage.pressKey("ArrowDown", 5);

    await expect(appPage.page).toHaveScreenshot("pan-up-down-return.png", {
      maxDiffPixelRatio: 0.05,
    });
  });

  test("all four arrow directions produce distinct views", async ({
    appPage,
  }) => {
    const center = await appPage.page.screenshot();

    await appPage.pressKey("ArrowRight", 5);
    const right = await appPage.page.screenshot();

    // Return to center
    await appPage.pressKey("ArrowLeft", 5);
    await appPage.pressKey("ArrowLeft", 5);
    const left = await appPage.page.screenshot();

    // Return to center
    await appPage.pressKey("ArrowRight", 5);
    await appPage.pressKey("ArrowUp", 5);
    const up = await appPage.page.screenshot();

    // Return to center
    await appPage.pressKey("ArrowDown", 5);
    await appPage.pressKey("ArrowDown", 5);
    const down = await appPage.page.screenshot();

    // Each direction should produce a different view
    expect(Buffer.compare(right, left)).not.toBe(0);
    expect(Buffer.compare(up, down)).not.toBe(0);
    expect(Buffer.compare(right, up)).not.toBe(0);
  });
});

test.describe("mouse scroll zoom", () => {
  test("scrolling changes the zoom level", async ({ appPage }) => {
    const before = await appPage.page.screenshot();
    await appPage.scrollAtCenter(-300);
    const after = await appPage.page.screenshot();

    expect(Buffer.compare(before, after)).not.toBe(0);
  });

  test("zoom in then out returns to similar view", async ({ appPage }) => {
    await appPage.scrollAtCenter(-300);
    await appPage.scrollAtCenter(300);

    await expect(appPage.page).toHaveScreenshot("zoom-in-out-return.png", {
      maxDiffPixelRatio: 0.05,
    });
  });
});

test.describe("mouse drag pan", () => {
  test("click-drag pans the view", async ({ appPage }) => {
    const before = await appPage.page.screenshot();

    // Drag in center-right area to avoid side panel
    await appPage.dragOnCanvas({ x: 0.7, y: 0.5 }, { x: 0.5, y: 0.3 });

    const after = await appPage.page.screenshot();
    expect(Buffer.compare(before, after)).not.toBe(0);
  });
});
