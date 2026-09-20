import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";
import Ajv2020 from "ajv/dist/2020.js";
import {
  PAY_GOLDEN_FIXTURES,
  assertPayActivationSemantics,
  assertPlatformIsFilterDimension,
  extractPayActivationSemantics,
} from "./semantic-profile.mjs";

const contractDir = dirname(fileURLToPath(import.meta.url));
const testDir = join(contractDir, "..");
const repoRoot = join(testDir, "..");
const fixturesDir = join(testDir, "fixtures");
const schemasDir = join(repoRoot, "schemas");

const eventSchema = JSON.parse(
  readFileSync(join(schemasDir, "event-v1.json"), "utf8"),
);
const ajv = new Ajv2020({ allErrors: true, strict: false });
ajv.addSchema(eventSchema, "event-v1.json");
const validateEvent = ajv.getSchema("event-v1.json");
assert.ok(validateEvent, "event schema must compile");

function loadFixture(name) {
  const raw = readFileSync(join(fixturesDir, name), "utf8");
  return JSON.parse(raw);
}

test("Pay golden fixtures validate against event-v1 schema", () => {
  for (const file of PAY_GOLDEN_FIXTURES) {
    const event = loadFixture(file);
    const valid = validateEvent(event);
    assert.equal(
      valid,
      true,
      `${file} must validate: ${JSON.stringify(validateEvent.errors)}`,
    );
  }
});

test("five-platform Pay activations share click, screen, element_id, and center position", () => {
  const semantics = PAY_GOLDEN_FIXTURES.map((file) => {
    const event = loadFixture(file);
    const extracted = extractPayActivationSemantics(event);
    assertPayActivationSemantics(extracted);
    return extracted;
  });

  assertPlatformIsFilterDimension(semantics);

  const reference = semantics[0];
  for (const other of semantics.slice(1)) {
    assert.equal(other.eventType, reference.eventType);
    assert.equal(other.elementId, reference.elementId);
    assert.equal(other.screenId, reference.screenId);
    assert.deepEqual(other.position, reference.position);
    assert.notEqual(other.platformType, reference.platformType);
  }
});
