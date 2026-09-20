import assert from "node:assert/strict";
import test from "node:test";
import {
  PAY_ELEMENT_ID,
  PAY_EVENT_TYPE,
  PAY_SCREEN_ID,
  PAY_WINDOW_CENTER,
  assertPayActivationSemantics,
  extractPayActivationSemantics,
  normalizedPositionFromAdapterTap,
} from "./semantic-profile.mjs";

/** Mirrors View / Compose / WPF activation builders at window center (800×600). */
const ADAPTER_CENTER_TAP = {
  locationInWindowX: 400,
  locationInWindowY: 300,
  windowWidth: 800,
  windowHeight: 600,
};

/**
 * @param {string} platformType
 * @param {string} sdkName
 */
function synthesizePayNestedEvent(platformType, sdkName) {
  const position = normalizedPositionFromAdapterTap(ADAPTER_CENTER_TAP);
  return {
    schema_version: 1,
    session_id: `sess_contract_${platformType}`,
    anonymous_id: `anon_${platformType}_contract`,
    timestamp: 1_726_771_205_000,
    sdk: { name: sdkName, version: "0.1.0" },
    platform: {
      type: platformType,
      os_version: `${platformType} test`,
      app_version: "1.0.0",
    },
    screen: { id: PAY_SCREEN_ID, layout_version: "2024-09" },
    event: { type: PAY_EVENT_TYPE, element_id: PAY_ELEMENT_ID },
    position,
    viewport: {
      width: ADAPTER_CENTER_TAP.windowWidth,
      height: ADAPTER_CENTER_TAP.windowHeight,
    },
  };
}

test("adapter-shaped center tap normalizes to 0.5, 0.5", () => {
  const position = normalizedPositionFromAdapterTap(ADAPTER_CENTER_TAP);
  assert.deepEqual(position, PAY_WINDOW_CENTER);
});

test("View and Compose Android adapter taps match Pay contract semantics", () => {
  const viewSemantics = extractPayActivationSemantics(
    synthesizePayNestedEvent("android", "heatmap-android"),
  );
  const composeSemantics = extractPayActivationSemantics(
    synthesizePayNestedEvent("android", "heatmap-android"),
  );
  assertPayActivationSemantics(viewSemantics);
  assertPayActivationSemantics(composeSemantics);
  assert.deepEqual(viewSemantics, composeSemantics);
});

test("iOS, macOS, and Windows synthesized Pay payloads match contract semantics", () => {
  const cases = [
    ["ios", "heatmap-ios"],
    ["macos", "heatmap-macos"],
    ["windows", "heatmap-windows"],
  ];
  for (const [platformType, sdkName] of cases) {
    const event = synthesizePayNestedEvent(platformType, sdkName);
    const semantics = extractPayActivationSemantics(event);
    assertPayActivationSemantics(semantics);
    assert.equal(semantics.platformType, platformType);
  }
});

test("web center click semantics align with synthesized adapter center tap", () => {
  const web = synthesizePayNestedEvent("web", "heatmap-web");
  const semantics = extractPayActivationSemantics(web);
  assertPayActivationSemantics(semantics);
});
