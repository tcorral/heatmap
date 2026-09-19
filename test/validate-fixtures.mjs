import Ajv2020 from "ajv/dist/2020.js";
import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const testDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(testDir, "..");
const schemasDir = join(repoRoot, "schemas");
const fixturesDir = join(testDir, "fixtures");

const eventSchema = JSON.parse(
  readFileSync(join(schemasDir, "event-v1.json"), "utf8")
);
const batchSchema = JSON.parse(
  readFileSync(join(schemasDir, "batch-v1.json"), "utf8")
);

const ajv = new Ajv2020({ allErrors: true, strict: false });
ajv.addSchema(eventSchema, "event-v1.json");
ajv.addSchema(batchSchema, "batch-v1.json");

const validateEvent = ajv.getSchema("event-v1.json");
const validateBatch = ajv.getSchema("batch-v1.json");

if (!validateEvent || !validateBatch) {
  console.error("Failed to compile schemas.");
  process.exit(1);
}

const batchFixtures = new Set(["batch-checkout-journey.json"]);
const fixtureFiles = readdirSync(fixturesDir)
  .filter((name) => name.endsWith(".json"))
  .sort();

let failed = false;

for (const file of fixtureFiles) {
  const path = join(fixturesDir, file);
  const data = JSON.parse(readFileSync(path, "utf8"));
  const validate = batchFixtures.has(file) ? validateBatch : validateEvent;
  const label = batchFixtures.has(file) ? "batch" : "event";

  if (!validate(data)) {
    failed = true;
    console.error(`${file} (${label}) failed validation:`);
    for (const err of validate.errors ?? []) {
      console.error(`  ${err.instancePath || "/"} ${err.message}`);
    }
  } else {
    console.log(`ok ${file} (${label})`);
  }
}

if (failed) {
  process.exit(1);
}

console.log(`Validated ${fixtureFiles.length} fixture(s).`);
