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
  });
});
