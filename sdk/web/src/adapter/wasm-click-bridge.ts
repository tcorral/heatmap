import { startDomClickCapture } from "./dom-click-capture.js";

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

export function wireDomClicksToWasm(recorder: WasmActivationRecorder): () => void {
  return startDomClickCapture({
    recordClick: (click) => {
      recorder.recordActivationClick(
        click.position.x,
        click.position.y,
        click.elementId,
        click.viewport.width,
        click.viewport.height,
        Date.now(),
      );
    },
  });
}
