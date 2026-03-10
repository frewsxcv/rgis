import { defineConfig, devices } from "@playwright/test";
import os from "os";

const ciWorkers = Math.max(1, os.cpus().length - 1);

export default defineConfig({
  testDir: "./tests",
  snapshotPathTemplate: "{testDir}/{testFileDir}/{testFileName}-snapshots/{arg}{ext}",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: process.env.CI ? ciWorkers : undefined,
  reporter: "html",

  expect: {
    toHaveScreenshot: {
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
