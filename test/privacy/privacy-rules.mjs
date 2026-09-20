/** Shared privacy gate rules for cross-platform fixture scans. */

import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { PAY_CONTRACT_PLATFORMS, PAY_GOLDEN_FIXTURES } from "../contract/semantic-profile.mjs";

const privacyDir = dirname(fileURLToPath(import.meta.url));
const testDir = join(privacyDir, "..");
const fixturesDir = join(testDir, "fixtures");

/** Literals that must never appear in event JSON (passwords, coupons, card PANs). */
export const FORBIDDEN_SECRET_LITERALS = [
  "E2E_SECRET_PASSWORD_9f3a",
  "E2E_SECRET_COUPON_z81k",
  "SuperSecret123",
  "4111 1111 1111 1111",
  "4111111111111111",
];

/**
 * Element ids that must never appear on wire when app excludes or masks secrets.
 * Aligns with checkout POC privacy controls and core exclude/mask tests.
 */
export const EXCLUDED_ELEMENT_IDS = ["checkout.password", "checkout.card"];

export { PAY_CONTRACT_PLATFORMS, PAY_GOLDEN_FIXTURES };

function loadJsonFixture(name) {
  const raw = readFileSync(join(fixturesDir, name), "utf8");
  return JSON.parse(raw);
}

/** @returns {unknown[]} events and nested batch events eligible for privacy scan */
export function collectPrivacyScanEvents() {
  /** @type {unknown[]} */
  const events = [];

  for (const file of PAY_GOLDEN_FIXTURES) {
    events.push({ label: file, event: loadJsonFixture(file) });
  }

  const fixtureFiles = readdirSync(fixturesDir)
    .filter((name) => name.endsWith(".json"))
    .sort();

  for (const file of fixtureFiles) {
    if (PAY_GOLDEN_FIXTURES.includes(file)) {
      continue;
    }
    const data = loadJsonFixture(file);
    if (Array.isArray(data.events)) {
      for (const [index, event] of data.events.entries()) {
        events.push({ label: `${file}[${index}]`, event });
      }
    } else {
      events.push({ label: file, event: data });
    }
  }

  for (const platform of PAY_CONTRACT_PLATFORMS) {
    events.push({
      label: `synthesized-pay-${platform}`,
      event: synthesizeContractPayEvent(platform),
    });
  }

  return events;
}

/**
 * @param {string} platformType
 */
function synthesizeContractPayEvent(platformType) {
  return {
    schema_version: 1,
    session_id: `sess_privacy_${platformType}`,
    anonymous_id: `anon_privacy_${platformType}`,
    timestamp: 1_726_771_205_000,
    sdk: { name: `heatmap-${platformType}`, version: "0.1.0" },
    platform: {
      type: platformType,
      os_version: `${platformType} privacy-gate`,
      app_version: "1.0.0",
    },
    screen: { id: "Checkout", layout_version: "2024-09" },
    event: { type: "click", element_id: "checkout.pay" },
    position: { x: 0.5, y: 0.5 },
    viewport: { width: 800, height: 600 },
  };
}

/**
 * @param {unknown} event
 * @param {string} label
 */
export function assertEventJsonPrivacy(event, label) {
  const serialized = JSON.stringify(event);

  for (const secret of FORBIDDEN_SECRET_LITERALS) {
    if (serialized.includes(secret)) {
      throw new Error(`${label} must not contain secret literal ${secret}`);
    }
  }

  const elementId = readElementId(event);
  if (elementId && EXCLUDED_ELEMENT_IDS.includes(elementId)) {
    throw new Error(`${label} must not include excluded element_id ${elementId}`);
  }
}

/**
 * @param {unknown} value
 * @returns {string | undefined}
 */
function readElementId(value) {
  if (!value || typeof value !== "object") {
    return undefined;
  }
  const record = /** @type {Record<string, unknown>} */ (value);
  const interaction = record.event;
  if (!interaction || typeof interaction !== "object") {
    return undefined;
  }
  const eventRecord = /** @type {Record<string, unknown>} */ (interaction);
  return typeof eventRecord.element_id === "string" ? eventRecord.element_id : undefined;
}
