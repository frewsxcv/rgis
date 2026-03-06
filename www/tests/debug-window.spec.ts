import { test, expect } from "./fixtures/app-fixture";

test.describe("debug window", () => {
  test("opening debug window from Help menu", async ({ appPage }) => {
    // Click "Help" menu
    await appPage.clickWidget("Help");

    // Click "Debug stats" in dropdown
    await appPage.clickWidget("Debug stats");
    await appPage.waitForNextFrame();

    // Verify the Debug window is open by checking its widget rect exists
    const debugRect = await appPage.page.evaluate(
      () => (window as any).get_widget_rect("Debug"),
    );
    expect(debugRect).not.toBeNull();
  });
});
