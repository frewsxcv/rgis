import { test, expect } from "./fixtures/app-fixture";

test.describe("measure tool", () => {
  test("measure from San Francisco to New York City", async ({ appPage }) => {
    test.setTimeout(60000);

    // Close the Welcome dialog by clicking its X button
    await appPage.clickOnCanvas(0.626, 0.443);
    await appPage.page.waitForTimeout(500);

    // Load the USA States layer from the built-in library
    await appPage.openAddLayerWindow();
    await appPage.clickWidget("Library");
    await appPage.page.waitForTimeout(500);
    await appPage.clickWidget("USA");
    await appPage.page.waitForTimeout(500);
    await appPage.clickWidget("Add:States");
    await appPage.page.waitForTimeout(10000);

    // Close the Add Layer window by clicking the canvas
    await appPage.clickOnCanvas(0.222, 0.076);
    await appPage.page.waitForTimeout(500);

    // Expand the layer in the side panel and zoom to extent
    await appPage.clickWidget("USA: States");
    await appPage.page.waitForTimeout(500);
    await appPage.clickWidget("Zoom to extent");
    await appPage.page.waitForTimeout(1000);

    // Select the Measure Tool
    await appPage.clickWidget("Measure Tool");
    await appPage.page.waitForTimeout(500);

    // Move mouse to the San Francisco area on the map. Multiple moves
    // with waits are needed to update egui's pointer state (which has a
    // one-frame delay on is_pointer_over_area()).
    const box = await appPage.canvasBoundingBox();
    const sfX = box.x + box.width * 0.649;
    const sfY = box.y + box.height * 0.692;

    await appPage.page.mouse.move(sfX, sfY);
    await appPage.page.waitForTimeout(200);
    await appPage.page.mouse.move(sfX + 1, sfY);
    await appPage.page.waitForTimeout(200);
    await appPage.page.mouse.move(sfX, sfY);
    await appPage.page.waitForTimeout(200);

    // Click to set measurement start point
    await appPage.page.mouse.click(sfX, sfY);
    await appPage.page.waitForTimeout(500);

    // Move the mouse to New York City area
    const nycX = box.x + box.width * 0.898;
    const nycY = box.y + box.height * 0.622;
    await appPage.page.mouse.move(nycX, nycY);
    await appPage.page.waitForTimeout(500);

    await expect(appPage.page).toHaveScreenshot("measure-tool-line.png");
  });
});
