import { test } from "./fixtures/app-fixture";

test.describe("attribute table", () => {
  test.beforeEach(async ({ appPage }) => {
    test.setTimeout(60000);

    await appPage.addLibraryLayer("World", "Countries");

    // Close the Add Layer window
    await appPage.closeWindow("Add Layer");
  });

  test("attribute table window shows all feature attributes", async ({
    appPage,
  }) => {
    // Expand the layer header
    await appPage.clickWidget("World: Countries");

    // Open the Manage Layer window
    await appPage.clickWidget("Manage");

    // Click the Attribute Table button
    await appPage.clickWidget("Attribute Table");

    await appPage.expectScreenshot("attribute-table.png");
  });
});
