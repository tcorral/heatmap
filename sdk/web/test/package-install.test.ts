import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

const packageRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

describe("package install surface", () => {
  it("declares ESM exports with types for the package root", () => {
    const pkg = JSON.parse(
      readFileSync(resolve(packageRoot, "package.json"), "utf8"),
    ) as {
      name: string;
      exports: Record<string, { import?: string; types?: string }>;
      files?: string[];
    };

    expect(pkg.name).toBe("@heatmap/sdk-web");
    expect(pkg.exports["."].import).toBe("./dist/index.js");
    expect(pkg.exports["."].types).toBe("./dist/index.d.ts");
    expect(pkg.files).toContain("dist");
  });

  it("resolves the public module entry from source", async () => {
    const mod = await import("../src/index.js");

    expect(mod.startDomClickCapture).toBeTypeOf("function");
    expect(mod.startDomScrollCapture).toBeTypeOf("function");
    expect(mod.createHeatmapWebClient).toBeTypeOf("function");
    expect(mod.wireDomCaptureToWasm).toBeTypeOf("function");
  });
});
