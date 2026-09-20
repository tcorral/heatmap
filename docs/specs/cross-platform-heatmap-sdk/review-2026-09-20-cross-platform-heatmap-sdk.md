# Review: cross-platform-heatmap-sdk

**Scope:** 489 git-tracked files, full V1 POC on main (HEAD `820afceb1affb14c17a4058352a111b835883512`; no branch diff vs merge-base)

## Summary

All three lenses report open findings. Code review reports six defects including four critical retry, persistence, and Windows FFI issues. Spec review identifies eight design-drift findings around retry identity, web failure classification, native UI-thread networking, session_end on restart, batch size, ingest unknown-field rejection, missing dashboard surfaces, and Compose scroll capture. Product review confirms three acceptance gaps: no five-platform element breakdown, incomplete Apple/Android clean-consumer packaging, and blocking Apple capture/upload work.

_Each finding's checkbox tracks state through arbitration and resolution. Any agent addressing findings (engineer, implementer) updates the checkbox accordingly:_

- _`[ ]` — open_
- _`[~]` — deferred (engineer arbitration)_
- _`[-]` — rejected as false positive (engineer arbitration)_
- _`[x]` — done (finding addressed)_
- _`[!]` — regression (was `[x]`, surfaced again)_

_Each finding should carry enough context for arbitration without becoming a full essay: short title, concrete issue, impact, and suggested action._

## Code Review

- [x] **Critical** — `core/heatmap-core/src/sdk.rs:471` — Failed batches are restored behind newer queued events
  - **Issue:** Failed events append to queue tail while `retry_batch_id` is reused for next front slice.
  - **Why it matters:** Retry can send different payload under same batch ID, losing newer events and later duplicating older events.
  - **Suggested action:** Restore failed batch at queue front in original order and add regression test with more than 200 queued events.

- [x] **Critical** — `core/heatmap-core/src/sdk.rs:415` — In-flight events removed from persistent storage before acknowledgment
  - **Issue:** `take_batch_for_upload` drains events and immediately persists reduced queue; pending batch exists only in memory.
  - **Why it matters:** Process termination during upload permanently loses unacknowledged events.
  - **Suggested action:** Persist in-flight batch until successful acknowledgment; test restart between batch creation and completion.

- [x] **Critical** — `core/heatmap-ffi/src/c_abi.rs:35` — Windows C ABI handle is not concurrency-safe
  - **Issue:** `with_sdk` creates unsynchronized mutable references from raw pointer while Windows timer and WPF callbacks can call same handle concurrently; disposal can also free it during a call.
  - **Why it matters:** Aliased Rust mutable references and use-after-free cause undefined behavior.
  - **Suggested action:** Add synchronized, lifetime-safe native handle ownership and make managed disposal wait for active callbacks.

- [x] **Critical** — `core/heatmap-core/src/identity_store.rs:91` — File persistence updates fail on Windows
  - **Issue:** `fs::rename(tmp, destination)` does not replace existing destination on Windows, and rename errors are discarded.
  - **Why it matters:** Subsequent queue, identity, and session writes silently retain stale data.
  - **Suggested action:** Implement platform-safe atomic replacement, propagate/report failures, and test repeated writes on Windows.

- [x] **Finding** — `core/heatmap-core/src/sdk.rs:433` — Batch byte limit excludes JSON envelope overhead
  - **Issue:** Batch construction counts event bytes only, omitting batch fields, array delimiters, and commas.
  - **Why it matters:** Near-limit batches exceed backend’s 262,144-byte cap, receive permanent 413, and are dropped.
  - **Suggested action:** Bound using complete serialized batch size and add exact-boundary tests.

- [ ] **Finding** — `poc/react/src/heatmap/init.ts:79` — Web uploader treats permanent HTTP failures as transient
  - **Issue:** Every non-2xx response calls `completeFlush(false)`, which restores batch as generic transport failure without status classification.
  - **Why it matters:** 400/401/403/413 batches retry indefinitely every ten seconds. Existing test waits only 2.5 seconds.
  - **Suggested action:** Pass HTTP status semantics into completion and extend test beyond scheduled retry interval.

## Spec Review

- [x] **Critical** — `core/heatmap-core/src/sdk.rs:412` — Retry identity is not bound to its original payload
  - **Issue:** Failed events are appended behind remaining queued events at lines 471–477, while `retry_batch_id` is reused for next front batch and is not persisted across restart.
  - **Why it matters:** Same batch ID can identify different events, causing idempotency collisions, duplicate writes, or unsent-event loss.
  - **Suggested action:** Persist in-flight batch ID and exact event payload; retry them together before later events.

- [ ] **Finding** — `core/heatmap-wasm/src/public_api.rs:106` — Web async flush cannot distinguish permanent failures
  - **Issue:** `complete_flush` accepts only a boolean and converts every rejection into transient `FlushError::Transport`. React integration passes `false` for every non-2xx response at `poc/react/src/heatmap/init.ts:79`.
  - **Why it matters:** Web retries 400/401/403/413 and cannot honor 429 `Retry-After`, contrary to delivery design.
  - **Suggested action:** Pass HTTP status and retry metadata into completion handling.

- [ ] **Finding** — `core/heatmap-core/src/sdk.rs:599` — Native auto-flush performs network I/O on capture thread
  - **Issue:** Enqueue calls `maybe_auto_flush` synchronously; native transport performs blocking HTTP at `core/heatmap-core/src/transport.rs:74`.
  - **Why it matters:** Threshold-triggering UI events can block host UI, violating no-UI-thread-block constraint.
  - **Suggested action:** Dispatch native delivery to background worker while preserving bounded persistent handoff.

- [ ] **Finding** — `core/heatmap-core/src/sdk.rs:700` — Restart after inactivity loses required `session_end`
  - **Issue:** Initialization clears expired persisted session before next capture, preventing rotation logic from emitting `session_end` for prior session.
  - **Why it matters:** Implementation Note 3 requires prior-session `session_end` on next capture after inactivity.
  - **Suggested action:** Retain expired prior session metadata until next capture performs closure and rotation.

- [x] **Finding** — `core/heatmap-core/src/sdk.rs:425` — Batch construction does not enforce serialized 256 KiB ceiling
  - **Issue:** Limit uses summed event sizes, excludes envelope overhead, and always accepts first event even when it alone exceeds limit.
  - **Why it matters:** SDK can create oversized batches that backend rejects with 413 and permanently drops.
  - **Suggested action:** Bound complete serialized batch and explicitly handle individually oversized events.

- [ ] **Finding** — `backend/src/validate.rs:60` — Nested unknown fields are rejected
  - **Issue:** `object_only_keys` rejects unknown SDK, platform, screen, event, position, and viewport fields.
  - **Why it matters:** Conflicts with design decision that unknown fields are ignored for forward compatibility.
  - **Suggested action:** Ignore unknown nested fields or arbitrate and update conflicting design/schema constraints.

- [ ] **Finding** — `dashboard/src/DashboardApp.tsx:29` — Specified dashboard surfaces are absent
  - **Issue:** Dashboard exposes filters, totals, and overlay only; no manual still-upload UI or platform-breakdown output exists.
  - **Why it matters:** Design explicitly requires manual dashboard upload and comparative platform counts.
  - **Suggested action:** Add minimal still-upload and platform-breakdown surfaces, or document approved scope reduction.

- [ ] **Finding** — `sdk/android/heatmap-android/src/main/kotlin/com/heatmap/sdk/compose/HeatmapComposeCapture.kt:45` — Compose adapter does not capture scrolling
  - **Issue:** Compose adapter installs touch and lifecycle handling only. POC manually observes scroll state at `poc/android/app/src/main/kotlin/com/heatmap/checkout/ui/screens/ComposeCheckoutScreen.kt:44`.
  - **Why it matters:** Capture responsibility leaks into host application, contrary to adapter-owned Compose capture design.
  - **Suggested action:** Observe Compose scroll semantics/state within adapter integration.

## Product Review

- V1 interaction set, capture start/disable — Implemented at `core/heatmap-core/src/event.rs:38`, `core/heatmap-core/src/sdk.rs:164`, `core/heatmap-core/src/sdk.rs:208`.
- Click/tap equivalence — Implemented at `test/contract/pay-equivalence.test.mjs:44`.
- Anonymous/optional identity — Implemented at `core/heatmap-core/src/sdk.rs:222`, `core/heatmap-core/src/sdk.rs:739`.
- Session background/timeout — Implemented at `core/heatmap-core/tests/session_timeout.rs:104`, `core/heatmap-core/tests/session_timeout.rs:125`.
- Screen identity/layout separation — Implemented at `core/heatmap-core/src/capture.rs:130`, `backend/tests/screen_layout_filters.rs:135`.
- Stable/unmarked element behavior — Implemented at `backend/src/db.rs:250`, `core/heatmap-core/tests/exclude_mask_capture.rs:110`.
- Normalized cross-platform coordinates — Implemented at `core/heatmap-core/src/event.rs:192`, `test/contract/pay-equivalence.test.mjs:44`.
- Platform identity conventions — Implemented in platform capture adapters, including `sdk/apple/ios/Sources/HeatmapAppleIOS/HeatmapIosCapture.swift:72` and `sdk/windows/src/Heatmap.Sdk.Wpf/HeatmapWpfCapture.cs:76`.
- Android View and Compose — Implemented and exercised at `poc/android/app/src/androidTest/kotlin/com/heatmap/checkout/e2e/CheckoutJourneyInstrumentedTest.kt:29`.
- Privacy and secret-field exclusion — Implemented at `core/heatmap-core/tests/exclude_mask_capture.rs:158` and `test/privacy/cross-platform-secrets.test.mjs:19`.
- Default/session sampling — Implemented at `core/heatmap-core/tests/session_sampling.rs:74`, `core/heatmap-core/tests/session_sampling.rs:120`.
- Offline survival/process recovery — Implemented at `core/heatmap-core/src/sdk.rs:599` and `poc/android/app/src/androidTest/kotlin/com/heatmap/checkout/e2e/CheckoutJourneyInstrumentedTest.kt:67`.
- Buffer drop-oldest — Implemented at `core/heatmap-core/src/persistent_queue.rs:64`.
- Fail-closed delivery — Implemented at `core/heatmap-core/src/transport.rs:69` and `core/heatmap-core/src/sdk.rs:459`.
- Cross-platform element comparison — **Missing** — API and dashboard expose one platform slice at a time at `backend/src/db.rs:250`, `dashboard/src/components/filterFields.tsx:8`.
- Per-platform coordinate heatmap — Implemented at `backend/src/db.rs:360`.
- Optional reference still — Implemented at `dashboard/src/components/HeatmapOverlayPanel.tsx:11`.
- No scroll-map visualization — Implemented at `dashboard/test/dashboard.test.tsx:117`.
- Dashboard filters — Implemented at `dashboard/src/components/filterFields.tsx:10`.
- Shared checkout journey — Implemented across native E2E suites, including `poc/ios/HeatmapCheckoutUITests/CheckoutJourneyUITests.swift:10` and `test/e2e/windows/HeatmapCheckout.E2ETests/CheckoutJourneyE2ETests.cs:9`.
- Contract equivalence — Implemented at `test/contract/pay-equivalence.test.mjs:44`.
- Installable SDKs — **Missing for Apple and Android clean consumers** at `sdk/apple/Package.swift:3`, `sdk/android/heatmap-android/build.gradle.kts:1`.
- Integrator documentation — Implemented in `docs/web-sdk.md`, `docs/ios-sdk.md`, `docs/android-sdk.md`, `docs/macos-sdk.md`, and `docs/windows-sdk.md`.
- Accessibility NFR — Implemented at `dashboard/test/dashboard.test.tsx:84`.
- Performance NFR — **Missing for native Apple capture/upload paths** at `sdk/apple/shared/Sources/HeatmapAppleShared/HeatmapClient.swift:51`, `sdk/apple/ios/Sources/HeatmapAppleIOS/HeatmapIosRustCaptureBackend.swift:23`.
- Security NFR — Implemented at `core/heatmap-core/src/config.rs:42`, `backend/src/handlers.rs:68`.

- [ ] **Major** — `dashboard/src/components/filterFields.tsx:8` — AC: Cross-platform element comparison — Dashboard lacks five-platform element breakdown
  - **Issue:** Platform is a single-select filter. Backend totals query requires one platform and groups only by element.
  - **Why it matters:** Analyst cannot see Pay counts broken down across web, iOS, Android, macOS, and Windows in one dashboard result as required.
  - **Suggested action:** Add cross-platform totals query grouped by platform and render all five platform counts together.

- [ ] **Major** — `sdk/apple/Package.swift:3` — AC: Installable SDKs — Apple package depends on repository-local debug libraries
  - **Issue:** SwiftPM package links `libheatmap_ffi` from `../../target/debug` or simulator-specific build output. It contains no binary target/XCFramework. Android docs likewise require source-module inclusion or hypothetical published AAR, but no Maven publication is configured at `sdk/android/heatmap-android/build.gradle.kts:1`.
  - **Why it matters:** Clean consumers cannot install these SDKs through normal package-manager dependency resolution and complete initialization without cloning repository and manually building native artifacts.
  - **Suggested action:** Package Apple XCFramework through SwiftPM and publish/configure Android AAR Maven artifact.

- [ ] **Major** — `sdk/apple/shared/Sources/HeatmapAppleShared/HeatmapClient.swift:51` — NFR: Performance — Apple work can block main UI thread
  - **Issue:** Run-loop timer synchronously calls `sdk.pollFlush()`. Tap capture also synchronously crosses FFI and persists queue data through `sdk/apple/ios/Sources/HeatmapAppleIOS/HeatmapIosRustCaptureBackend.swift:23` and `core/heatmap-core/src/sdk.rs:599`.
  - **Why it matters:** Disk or network latency can stall host UI, violating non-blocking interaction and fail-closed burst-load requirements.
  - **Suggested action:** Move persistence and transport behind bounded background workers; keep UI callbacks limited to non-blocking enqueue.

---

## Review Notes

- [spec-reviewer/standards] N/A as directed; no technical-standard IDs invented.
- [spec-reviewer/repo references] No product-runtime drift found against cited harness references.
- [spec-reviewer/external constraints] Implementations match supplied web, Apple, Android, and Windows identity constraints.
- [product-reviewer/out-of-scope] No session replay, text capture, V2 interactions, scroll-map visualization, remote configuration, or production dashboard control-plane drift found.
- [product-reviewer/validation] Reviewed tracked HEAD `820afceb1affb14c17a4058352a111b835883512`; no files modified.
