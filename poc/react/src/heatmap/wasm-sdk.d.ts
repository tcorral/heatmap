declare module "../wasm/pkg/heatmap_wasm.js" {
  export default function init(): Promise<unknown>;

  export class HeatmapSdkJs {
    static init(configJson: string): HeatmapSdkJs;
    screen(screenId: string, layoutVersion?: string | null): void;
    flush(): void;
    drainBatchJson(): string | undefined;
    recordScroll(): void;
    recordActivationClick(
      x: number,
      y: number,
      elementId: string | undefined,
      viewportWidth: number,
      viewportHeight: number,
      timestampMs: number,
    ): void;
    setQueueLimitsForTesting(maxEvents: number, maxBytes: number): void;
    queuedEventCountForTesting(): number;
  }
}
