export type ElementsFilter = {
  platform: string;
  screen: string;
  appVersion: string;
  layoutVersion: string;
  fromMs?: number;
  toMs?: number;
};

export type ElementTotalRow = {
  platform?: string;
  element_id: string;
  click_count: number;
  unique_sessions: number;
};

export type ElementsResponse = {
  elements: ElementTotalRow[];
  unique_sessions: number;
};

export type HeatmapBin = {
  bin_x: number;
  bin_y: number;
  count: number;
};

export type HeatmapResponse = {
  bins: HeatmapBin[];
};

export type StillResponse = {
  content_type: string;
  image_base64: string;
};

export type ElementsApiClient = {
  getElements: (filters: ElementsFilter) => Promise<ElementsResponse>;
};

export type HeatmapApiClient = {
  getHeatmap: (filters: ElementsFilter) => Promise<HeatmapResponse>;
};

export type StillsApiClient = {
  getStill: (filters: ElementsFilter) => Promise<StillResponse | null>;
};

export type DashboardApiClient = ElementsApiClient &
  HeatmapApiClient &
  StillsApiClient;

export type ElementsQueryState =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "success"; data: ElementsResponse }
  | { status: "error"; message: string };

export type OverlayQueryState =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "no_still" }
  | { status: "success"; still: StillResponse; heatmap: HeatmapResponse }
  | { status: "error"; message: string };
