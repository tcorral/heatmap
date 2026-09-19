import { useEffect, useRef } from "react";

import type { HeatmapBin, OverlayQueryState } from "../api/types";
import { drawDensityBins } from "../overlay/drawDensityBins";
import { stillDataUrl } from "../overlay/stillDataUrl";

type HeatmapOverlayPanelProps = {
  overlayState: OverlayQueryState;
};

export function HeatmapOverlayPanel({ overlayState }: HeatmapOverlayPanelProps) {
  if (overlayState.status === "idle" || overlayState.status === "no_still") {
    return null;
  }
  if (overlayState.status === "loading") {
    return (
      <p role="status" aria-label="Loading density overlay">
        Loading density overlay…
      </p>
    );
  }
  if (overlayState.status === "error") {
    return (
      <p role="alert" aria-label="Density overlay error">
        {overlayState.message}
      </p>
    );
  }

  return (
    <OverlayCanvas
      stillSrc={stillDataUrl(overlayState.still)}
      bins={overlayState.heatmap.bins}
    />
  );
}

type OverlayCanvasProps = {
  stillSrc: string;
  bins: HeatmapBin[];
};

function OverlayCanvas({ stillSrc, bins }: OverlayCanvasProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) {
      return;
    }
    drawDensityBins(canvas, bins, stillSrc);
  }, [bins, stillSrc]);

  return (
    <section
      data-testid="heatmap-overlay"
      aria-label="Density overlay"
      tabIndex={0}
    >
      <img aria-label="Reference still" src={stillSrc} alt="" />
      <canvas aria-hidden="true" ref={canvasRef} />
    </section>
  );
}
