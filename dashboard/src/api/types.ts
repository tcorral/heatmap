export type ElementsFilter = {
  platform: string;
  screen: string;
  appVersion: string;
  layoutVersion: string;
  fromMs?: number;
  toMs?: number;
};

export type ElementTotalRow = {
  element_id: string;
  click_count: number;
  unique_sessions: number;
};

export type ElementsResponse = {
  elements: ElementTotalRow[];
  unique_sessions: number;
};

export type ElementsApiClient = {
  getElements: (filters: ElementsFilter) => Promise<ElementsResponse>;
};

export type ElementsQueryState =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "success"; data: ElementsResponse }
  | { status: "error"; message: string };
