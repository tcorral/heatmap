import type { StillResponse } from "../api/types";

export function stillDataUrl(still: StillResponse): string {
  return `data:${still.content_type};base64,${still.image_base64}`;
}
