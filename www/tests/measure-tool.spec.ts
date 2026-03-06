import { test, expect } from "./fixtures/app-fixture";

test.describe("measure tool", () => {
  test("measure from San Francisco to New York City", async ({ appPage }) => {
    test.setTimeout(60000);

    // Close the Welcome dialog
    await appPage.closeWindow("Welcome");

    // Load the USA States layer from the built-in library
    await appPage.addLibraryLayer("USA", "States");

    // Close the Add Layer window
    await appPage.closeWindow("Add Layer");

    // Expand the layer in the side panel and zoom to extent
    await appPage.clickWidget("USA: States");
    await appPage.clickWidget("Zoom to extent");
    await appPage.waitForNextFrame();

    // Select the Measure Tool
    await appPage.clickWidget("Measure Tool");

    // Move mouse to the San Francisco area on the map. Multiple moves
    // with waits are needed to update egui's pointer state (which has a
    // one-frame delay on is_pointer_over_area()).
    const box = await appPage.canvasBoundingBox();
    const sfX = box.x + box.width * 0.649;
    const sfY = box.y + box.height * 0.692;

    await appPage.page.mouse.move(sfX, sfY);
    await appPage.waitForNextFrame();
    await appPage.page.mouse.move(sfX + 1, sfY);
    await appPage.waitForNextFrame();
    await appPage.page.mouse.move(sfX, sfY);
    await appPage.waitForNextFrame();

    // Click to set measurement start point
    await appPage.page.mouse.click(sfX, sfY);
    await appPage.waitForNextFrame();

    // Click on New York City area to lock the end point
    const nycX = box.x + box.width * 0.898;
    const nycY = box.y + box.height * 0.622;
    await appPage.page.mouse.move(nycX, nycY);
    await appPage.waitForNextFrame();
    await appPage.page.mouse.click(nycX, nycY);
    await appPage.waitForNextFrame();

    // Move mouse away so it doesn't obscure the endpoint handle
    await appPage.page.mouse.move(box.x + box.width * 0.5, box.y + box.height * 0.3);
    await appPage.waitForNextFrame();

    await expect(appPage.page).toHaveScreenshot("measure-tool-line.png");
  });
});
