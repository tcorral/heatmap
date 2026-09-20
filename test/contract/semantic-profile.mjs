/** Shared Pay activation semantics for cross-platform contract checks. */

export const PAY_ELEMENT_ID = "checkout.pay";
export const PAY_SCREEN_ID = "Checkout";
export const PAY_EVENT_TYPE = "click";
export const PAY_WINDOW_CENTER = { x: 0.5, y: 0.5 };

export const PAY_CONTRACT_PLATFORMS = [
  "web",
  "ios",
  "android",
  "macos",
  "windows",
];

export const PAY_GOLDEN_FIXTURES = PAY_CONTRACT_PLATFORMS.map(
  (platform) => `click-checkout-pay-${platform}.json`,
);

/**
 * @param {unknown} event
 * @returns {{
 *   platformType: string;
 *   eventType: string;
 *   elementId: string | undefined;
 *   screenId: string | undefined;
 *   position: { x: number; y: number } | undefined;
 * }}
 */
export function extractPayActivationSemantics(event) {
  if (!event || typeof event !== "object") {
    throw new TypeError("event must be an object");
  }
  const record = /** @type {Record<string, unknown>} */ (event);
  const platform = /** @type {Record<string, unknown>} */ (record.platform ?? {});
  const screen = /** @type {Record<string, unknown>} */ (record.screen ?? {});
  const interaction = /** @type {Record<string, unknown>} */ (record.event ?? {});
  const position = /** @type {Record<string, unknown>} */ (record.position ?? {});

  const x = position.x;
  const y = position.y;
  const hasPosition =
    typeof x === "number" &&
    typeof y === "number" &&
    x >= 0 &&
    x <= 1 &&
    y >= 0 &&
    y <= 1;

  return {
    platformType: String(platform.type ?? ""),
    eventType: String(interaction.type ?? ""),
    elementId:
      typeof interaction.element_id === "string"
        ? interaction.element_id
        : undefined,
    screenId: typeof screen.id === "string" ? screen.id : undefined,
    position: hasPosition ? { x, y } : undefined,
  };
}

/**
 * @param {ReturnType<typeof extractPayActivationSemantics>} semantics
 */
export function assertPayActivationSemantics(semantics) {
  if (semantics.eventType !== PAY_EVENT_TYPE) {
    throw new Error(
      `expected event type ${PAY_EVENT_TYPE}, got ${semantics.eventType}`,
    );
  }
  if (semantics.elementId !== PAY_ELEMENT_ID) {
    throw new Error(
      `expected element_id ${PAY_ELEMENT_ID}, got ${semantics.elementId ?? "missing"}`,
    );
  }
  if (semantics.screenId !== PAY_SCREEN_ID) {
    throw new Error(
      `expected screen.id ${PAY_SCREEN_ID}, got ${semantics.screenId ?? "missing"}`,
    );
  }
  if (!semantics.position) {
    throw new Error("expected normalized position in [0, 1]");
  }
  if (
    semantics.position.x !== PAY_WINDOW_CENTER.x ||
    semantics.position.y !== PAY_WINDOW_CENTER.y
  ) {
    throw new Error(
      `expected window-center position ${PAY_WINDOW_CENTER.x}/${PAY_WINDOW_CENTER.y}, got ${semantics.position.x}/${semantics.position.y}`,
    );
  }
  if (!PAY_CONTRACT_PLATFORMS.includes(semantics.platformType)) {
    throw new Error(`unexpected platform.type ${semantics.platformType}`);
  }
}

/**
 * @param {ReturnType<typeof extractPayActivationSemantics>[]} allSemantics
 */
export function assertPlatformIsFilterDimension(allSemantics) {
  const types = allSemantics.map((s) => s.platformType);
  const unique = new Set(types);
  if (unique.size !== types.length) {
    throw new Error("duplicate platform.type in Pay contract set");
  }
  if (unique.size !== PAY_CONTRACT_PLATFORMS.length) {
    throw new Error(
      `expected ${PAY_CONTRACT_PLATFORMS.length} platforms, got ${unique.size}`,
    );
  }
  for (const platform of PAY_CONTRACT_PLATFORMS) {
    if (!unique.has(platform)) {
      throw new Error(`missing platform.type ${platform}`);
    }
  }
}

/**
 * Adapter unit tests use window-relative pixel taps; core stores 0–1 fractions.
 *
 * @param {{
 *   locationInWindowX: number;
 *   locationInWindowY: number;
 *   windowWidth: number;
 *   windowHeight: number;
 * }} tap
 */
export function normalizedPositionFromAdapterTap(tap) {
  const { locationInWindowX, locationInWindowY, windowWidth, windowHeight } =
    tap;
  if (windowWidth <= 0 || windowHeight <= 0) {
    throw new Error("window dimensions must be positive");
  }
  return {
    x: locationInWindowX / windowWidth,
    y: locationInWindowY / windowHeight,
  };
}
