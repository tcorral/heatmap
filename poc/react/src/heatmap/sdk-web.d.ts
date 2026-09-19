declare module "@heatmap/sdk-web" {
  export type WindowPosition = { x: number; y: number };
  export type WindowViewport = { width: number; height: number };

  export type RecordedWebClick = {
    type: "click";
    elementId?: string;
    position: WindowPosition;
    viewport: WindowViewport;
  };

  export type ClickCaptureSink = {
    recordClick(click: RecordedWebClick): void;
  };

  export type WasmActivationRecorder = {
    recordActivationClick(
      x: number,
      y: number,
      elementId: string | undefined,
      viewportWidth: number,
      viewportHeight: number,
      timestampMs: number,
    ): void;
  };

  export function startDomClickCapture(sink: ClickCaptureSink): () => void;
  export function wireDomClicksToWasm(recorder: WasmActivationRecorder): () => void;
}
