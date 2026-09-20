import assert from "node:assert/strict";
import test from "node:test";

import {
  PAY_CONTRACT_PLATFORMS,
  PAY_GOLDEN_FIXTURES,
  assertEventJsonPrivacy,
  collectPrivacyScanEvents,
  EXCLUDED_ELEMENT_IDS,
  FORBIDDEN_SECRET_LITERALS,
} from "./privacy-rules.mjs";

test("privacy gate defines forbidden secrets and excluded element ids", () => {
  assert.ok(FORBIDDEN_SECRET_LITERALS.length >= 4);
  assert.ok(EXCLUDED_ELEMENT_IDS.includes("checkout.password"));
  assert.equal(PAY_GOLDEN_FIXTURES.length, PAY_CONTRACT_PLATFORMS.length);
});

test("multi-platform fixture and synthesized events contain no secrets or excluded element ids", () => {
  const targets = collectPrivacyScanEvents();
  assert.ok(
    targets.length >= PAY_CONTRACT_PLATFORMS.length + 1,
    "expected pay goldens plus additional fixtures",
  );

  for (const { label, event } of targets) {
    assertEventJsonPrivacy(event, label);
  }
});
