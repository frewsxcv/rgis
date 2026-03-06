import { test, expect } from "./fixtures/app-fixture";

test.describe("change CRS window", () => {
  test("CRS window shows input field and Set button", async ({ appPage }) => {
    // Close welcome window first
    await appPage.closeWindow("Welcome");

    // Click the CRS edit button in bottom panel
    await appPage.clickWidget("Edit CRS");
    await appPage.stabilizeForScreenshot();
    await expect(appPage.page).toHaveScreenshot("change-crs-window.png");
  });
});
