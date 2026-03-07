import { test } from "./fixtures/app-fixture";

test.describe("antimeridian handling", () => {
  test("US states layer renders correctly with antimeridian fix", async ({
    appPage,
  }) => {
    test.setTimeout(60000);

    await appPage.addLibraryLayer("USA", "States");
    await appPage.closeWindow("Add Layer");

    await appPage.expectScreenshot("us-states-antimeridian.png");
  });
});
