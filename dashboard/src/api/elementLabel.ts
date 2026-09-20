const KNOWN_LABELS: Record<string, string> = {
  "checkout.pay": "Checkout → Pay",
};

export function formatElementLabel(elementId: string): string {
  return KNOWN_LABELS[elementId] ?? elementId;
}
