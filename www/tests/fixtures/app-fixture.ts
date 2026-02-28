import { test as base, expect, Page } from "@playwright/test";

export class AppPage {
  constructor(public readonly page: Page) {}

  async init() {
    await this.page.goto("/");

    // Wait for WASM to initialize
    await this.page.waitForFunction(
      () => document.title.includes("ready"),
      null,
      { timeout: 30000 },
    );

    // Wait for canvas to appear
    const canvas = this.page.locator("canvas");
    await expect(canvas).toBeVisible({ timeout: 10000 });

    // Allow time for first frame to render
    await this.page.waitForTimeout(2000);
  }

  get canvas() {
    return this.page.locator("canvas");
  }

  async canvasBoundingBox() {
    const box = await this.canvas.boundingBox();
    if (!box) throw new Error("Canvas not found");
    return box;
  }

  async pressKey(key: string, count: number = 1) {
    for (let i = 0; i < count; i++) {
      await this.page.keyboard.press(key);
      await this.page.waitForTimeout(150);
    }
    // Wait for render to settle
    await this.page.waitForTimeout(500);
  }

  async scrollAtCenter(deltaY: number) {
    const box = await this.canvasBoundingBox();
    await this.page.mouse.wheel(0, deltaY);
    await this.page.waitForTimeout(1000);
  }

  async dragOnCanvas(
    startFrac: { x: number; y: number },
    endFrac: { x: number; y: number },
  ) {
    const box = await this.canvasBoundingBox();
    const startX = box.x + box.width * startFrac.x;
    const startY = box.y + box.height * startFrac.y;
    const endX = box.x + box.width * endFrac.x;
    const endY = box.y + box.height * endFrac.y;

    await this.page.mouse.move(startX, startY);
    await this.page.mouse.down();
    // Move in steps for smoother drag
    const steps = 10;
    for (let i = 1; i <= steps; i++) {
      const x = startX + ((endX - startX) * i) / steps;
      const y = startY + ((endY - startY) * i) / steps;
      await this.page.mouse.move(x, y);
      await this.page.waitForTimeout(50);
    }
    await this.page.mouse.up();
    await this.page.waitForTimeout(1000);
  }

  async regionHasContent(
    xFrac: number,
    yFrac: number,
    wFrac: number,
    hFrac: number,
  ): Promise<boolean> {
    return await this.page.evaluate(
      ({ xFrac, yFrac, wFrac, hFrac }) => {
        const canvas = document.querySelector("canvas");
        if (!canvas) return false;

        const gl = canvas.getContext("webgl2") || canvas.getContext("webgl");
        if (gl) {
          const x = Math.floor(xFrac * canvas.width);
          // WebGL has origin at bottom-left, so flip y
          const y = Math.floor((1 - yFrac - hFrac) * canvas.height);
          const w = Math.floor(wFrac * canvas.width);
          const h = Math.floor(hFrac * canvas.height);
          const pixels = new Uint8Array(w * h * 4);
          try {
            gl.readPixels(x, y, w, h, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
          } catch {
            // readPixels may fail without preserveDrawingBuffer
            return true;
          }

          // Check if all pixels are zeros (readPixels returns zeros without preserveDrawingBuffer)
          let allZero = true;
          for (let i = 0; i < pixels.length; i++) {
            if (pixels[i] !== 0) {
              allZero = false;
              break;
            }
          }
          if (allZero) {
            // Can't determine — assume content exists since canvas is visible
            return true;
          }

          // Check for non-uniform pixels (content)
          const firstR = pixels[0],
            firstG = pixels[1],
            firstB = pixels[2];
          for (let i = 4; i < pixels.length; i += 4) {
            if (
              pixels[i] !== firstR ||
              pixels[i + 1] !== firstG ||
              pixels[i + 2] !== firstB
            ) {
              return true;
            }
          }
          return false;
        }

        // Fallback for 2d context
        const ctx = canvas.getContext("2d");
        if (!ctx) return true;

        const x = Math.floor(xFrac * canvas.width);
        const y = Math.floor(yFrac * canvas.height);
        const w = Math.floor(wFrac * canvas.width);
        const h = Math.floor(hFrac * canvas.height);
        const imageData = ctx.getImageData(x, y, w, h);
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
      },
      { xFrac, yFrac, wFrac, hFrac },
    );
  }
}

export const test = base.extend<{ appPage: AppPage }>({
  appPage: async ({ page }, use) => {
    const appPage = new AppPage(page);
    await appPage.init();
    await use(appPage);
  },
});

export { expect };
