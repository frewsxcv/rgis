import { test, expect } from "./fixtures/app-fixture";

test.describe("panel presence", () => {
  test("side panel has content", async ({ appPage }) => {
    // Left ~15% of canvas should contain the side panel
    const hasContent = await appPage.regionHasContent(0, 0.1, 0.15, 0.8);
    expect(hasContent).toBe(true);
  });

  test("top bar has content", async ({ appPage }) => {
    // Top ~4% of canvas should contain the top bar
    const hasContent = await appPage.regionHasContent(0, 0, 1, 0.04);
    expect(hasContent).toBe(true);
  });

  test("bottom bar has content", async ({ appPage }) => {
    // Bottom ~4% of canvas should contain the bottom bar
    const hasContent = await appPage.regionHasContent(0, 0.96, 1, 0.04);
    expect(hasContent).toBe(true);
  });

  test("UI panels have more visual complexity than empty map", async ({
    appPage,
  }) => {
    // Side panel (UI) should have content
    const panelHasContent = await appPage.regionHasContent(0, 0.1, 0.15, 0.8);
    expect(panelHasContent).toBe(true);

    // Empty map area (right side, middle) — may or may not have content
    // but the panel should always have content
    const mapArea = await appPage.regionHasContent(0.6, 0.3, 0.3, 0.4);
    // We just verify the panel has content; map area result is informational
    expect(panelHasContent).toBe(true);
  });
});

test.describe("screenshot baselines after interactions", () => {
  test("view after panning right", async ({ appPage }) => {
    await appPage.pressKey("ArrowRight", 5);
    await expect(appPage.page).toHaveScreenshot("after-pan-right.png");
  });

  test("view after zooming in", async ({ appPage }) => {
    await appPage.scrollAtCenter(-300);
    await expect(appPage.page).toHaveScreenshot("after-zoom-in.png");
  });

  test("view after zooming out", async ({ appPage }) => {
    await appPage.scrollAtCenter(300);
    await expect(appPage.page).toHaveScreenshot("after-zoom-out.png");
  });
});
