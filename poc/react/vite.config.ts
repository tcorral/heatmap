import path from "node:path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@heatmap/sdk-web": path.resolve(__dirname, "../../sdk/web/src/index.ts"),
    },
  },
});
