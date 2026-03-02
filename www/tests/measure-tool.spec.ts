import { test, expect } from "./fixtures/app-fixture";

test.describe("measure tool", () => {
  test("selecting measure tool and clicking canvas shows measurement line", async ({
    appPage,
  }) => {
    // Select the Measure Tool
    await appPage.clickWidget("Measure Tool");
    await appPage.page.waitForTimeout(500);

    // Click on the canvas to set the start point
    await appPage.clickOnCanvas(0.3, 0.5);

    // Move the mouse to a different position to draw the measurement line
    const box = await appPage.canvasBoundingBox();
    await appPage.page.mouse.move(
      box.x + box.width * 0.7,
      box.y + box.height * 0.5,
    );
    await appPage.page.waitForTimeout(500);

    await expect(appPage.page).toHaveScreenshot("measure-tool-line.png");
  });
});
