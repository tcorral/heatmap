import path from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig } from "@playwright/test";

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, "../..");

const E2E_BACKEND_PORT = process.env.E2E_BACKEND_PORT ?? "18080";
const E2E_POC_PORT = process.env.E2E_POC_PORT ?? "15173";
const E2E_DASHBOARD_PORT = process.env.E2E_DASHBOARD_PORT ?? "15174";
const backendBase = `http://127.0.0.1:${E2E_BACKEND_PORT}`;
const pocBase = `http://127.0.0.1:${E2E_POC_PORT}`;
const dashboardBase = `http://127.0.0.1:${E2E_DASHBOARD_PORT}`;

export default defineConfig({
  testDir: "./tests",
  fullyParallel: false,
  workers: 1,
  timeout: 120_000,
  use: {
    baseURL: pocBase,
    trace: "on-first-retry",
  },
  webServer: [
    {
      command: `bash scripts/build-wasm-for-poc.sh && PORT=${E2E_BACKEND_PORT} HEATMAP_EXPOSE_TEST_EVENTS=1 DATABASE_PATH=heatmap-e2e.db cargo run -p heatmap-backend`,
      cwd: repoRoot,
      url: `${backendBase}/v1/elements?platform=web&screen=Checkout&app_version=1.0.0`,
      reuseExistingServer: false,
      timeout: 180_000,
    },
    {
      command: `VITE_HEATMAP_PROJECT_KEY=test-project-key VITE_HEATMAP_ENDPOINT=${backendBase} npm run dev -- --host 127.0.0.1 --port ${E2E_POC_PORT} --strictPort`,
      cwd: path.join(repoRoot, "poc/react"),
      url: pocBase,
      reuseExistingServer: false,
      timeout: 120_000,
    },
    {
      command: `VITE_API_BASE_URL=${backendBase} npm run dev -- --host 127.0.0.1 --port ${E2E_DASHBOARD_PORT} --strictPort`,
      cwd: path.join(repoRoot, "dashboard"),
      url: dashboardBase,
      reuseExistingServer: false,
      timeout: 120_000,
    },
  ],
});
