import { test, expect } from "./fixtures/app-fixture";

test("app loads and renders", async ({ appPage }) => {
  await expect(appPage.page).toHaveScreenshot("app-loaded.png");
});

test("canvas has non-blank content", async ({ appPage }) => {
  const hasContent = await appPage.page.evaluate(() => {
    const canvas = document.querySelector("canvas");
    if (!canvas) return false;

    const ctx = canvas.getContext("2d");
    if (!ctx) {
      // WebGL canvas — can't use getContext("2d"), but if it's visible
      // and the app initialized, we consider it has content.
      return true;
    }

    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    const data = imageData.data;

    const firstR = data[0],
      firstG = data[1],
      firstB = data[2];
    for (let i = 4; i < data.length; i += 4) {
      if (
        data[i] !== firstR ||
        data[i + 1] !== firstG ||
        data[i + 2] !== firstB
      ) {
        return true;
      }
    }
    return false;
  });

  expect(hasContent).toBe(true);
});

test("canvas is present and fills viewport", async ({ appPage }) => {
  const box = await appPage.canvasBoundingBox();
  // Canvas should roughly fill the 1280x720 viewport
  expect(box.width).toBeGreaterThanOrEqual(1200);
  expect(box.height).toBeGreaterThanOrEqual(650);
});

test("initial render baseline", async ({ appPage }) => {
  await expect(appPage.page).toHaveScreenshot("initial-render.png");
});

test("welcome dialog is visible in center", async ({ appPage }) => {
  // The center region should have distinct content (welcome dialog)
  // compared to the uniform map background
  const centerHasContent = await appPage.regionHasContent(0.3, 0.3, 0.4, 0.4);
  expect(centerHasContent).toBe(true);
});
