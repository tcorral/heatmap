export const COMPARISON_PLATFORMS = [
  { id: "web", label: "web" },
  { id: "ios", label: "iOS" },
  { id: "android", label: "Android" },
  { id: "macos", label: "macOS" },
  { id: "windows", label: "Windows" },
] as const;

const PAY_ELEMENT_ID = "checkout.pay";

export function payCountForPlatform(
  rows: Array<{ platform?: string; element_id: string; click_count: number }>,
  platformId: string,
): number {
  return rows
    .filter((row) => row.element_id === PAY_ELEMENT_ID && row.platform === platformId)
    .reduce((sum, row) => sum + row.click_count, 0);
}
