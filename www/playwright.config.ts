import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: "html",

  expect: {
    toHaveScreenshot: {
      maxDiffPixelRatio: 0.01,
      threshold: 0.2,
    },
  },

  use: {
    baseURL: "http://localhost:8081",
    trace: "on-first-retry",
    viewport: { width: 1280, height: 720 },
  },

  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        viewport: { width: 1280, height: 720 },
        launchOptions: {
          args: ["--enable-gpu", "--use-angle=default"],
        },
      },
    },
  ],

  webServer: {
    command: process.env.CI
      ? "npx serve dist -l 8081"
      : "npx webpack-dev-server --port 8081",
    url: "http://localhost:8081",
    reuseExistingServer: !process.env.CI,
  },
});
