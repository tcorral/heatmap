/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_HEATMAP_PROJECT_KEY: string;
  readonly VITE_HEATMAP_ENDPOINT: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
