import type { HeatmapBin } from "../api/types";

const GRID = 100;

export function drawDensityBins(
  canvas: HTMLCanvasElement,
  bins: HeatmapBin[],
  stillSrc: string,
): void {
  const context = readCanvasContext(canvas);
  if (!context) {
    return;
  }
  const image = new Image();
  image.onload = () => {
    canvas.width = image.naturalWidth || GRID;
    canvas.height = image.naturalHeight || GRID;
    context.drawImage(image, 0, 0, canvas.width, canvas.height);
    paintBins(context, bins, canvas.width, canvas.height);
  };
  image.src = stillSrc;
}

function readCanvasContext(
  canvas: HTMLCanvasElement,
): CanvasRenderingContext2D | null {
  try {
    return canvas.getContext("2d");
  } catch {
    return null;
  }
}

function paintBins(
  context: CanvasRenderingContext2D,
  bins: HeatmapBin[],
  width: number,
  height: number,
): void {
  const maxCount = bins.reduce((max, bin) => Math.max(max, bin.count), 0);
  if (maxCount === 0) {
    return;
  }
  const cellW = width / GRID;
  const cellH = height / GRID;
  for (const bin of bins) {
    const alpha = 0.25 + (bin.count / maxCount) * 0.55;
    context.fillStyle = `rgba(220, 38, 38, ${alpha})`;
    context.fillRect(bin.bin_x * cellW, bin.bin_y * cellH, cellW, cellH);
  }
}
