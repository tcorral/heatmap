import { afterEach, describe, expect, it } from "vitest";

import {
  HEATMAP_STORE_KEY_ANONYMOUS_ID,
  HEATMAP_STORE_KEY_USER_ID,
  readIdentityValue,
  writeIdentityValue,
} from "../src/identity/local-storage-store.js";

describe("identity localStorage store", () => {
  afterEach(() => {
    localStorage.removeItem(HEATMAP_STORE_KEY_ANONYMOUS_ID);
    localStorage.removeItem(HEATMAP_STORE_KEY_USER_ID);
  });

  it("should persist anonymous_id across separate read calls", () => {
    writeIdentityValue(HEATMAP_STORE_KEY_ANONYMOUS_ID, "anon_test_001");
    expect(readIdentityValue(HEATMAP_STORE_KEY_ANONYMOUS_ID)).toBe(
      "anon_test_001",
    );
  });

  it("should remove user_id when cleared without touching anonymous_id", () => {
    writeIdentityValue(HEATMAP_STORE_KEY_ANONYMOUS_ID, "anon_stable");
    writeIdentityValue(HEATMAP_STORE_KEY_USER_ID, "user-7");
    writeIdentityValue(HEATMAP_STORE_KEY_USER_ID, null);
    expect(readIdentityValue(HEATMAP_STORE_KEY_ANONYMOUS_ID)).toBe(
      "anon_stable",
    );
    expect(readIdentityValue(HEATMAP_STORE_KEY_USER_ID)).toBeNull();
  });
});
