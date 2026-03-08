import { test } from "./fixtures/app-fixture";

test.describe("welcome window", () => {
  test("welcome window is visible on app start", async ({ appPage }) => {
    await appPage.expectScreenshot("welcome-window-visible.png");
  });

  test("welcome window can be closed", async ({ appPage }) => {
    await appPage.closeWindow("Welcome");
    await appPage.expectScreenshot("welcome-window-closed.png");
  });
});
