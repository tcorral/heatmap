# Review: leftover heatmap sdk

**Scope:** 72 files, leftover V1 on main (HEAD `46e70b697887b54c4247701d6fda269d756ddbd1`; 14 commits since prior review `820afce`; no branch diff vs origin/main)

## Summary

All three lenses report remaining gaps after leftover work. Code review reports six defects including a critical C ABI disposal race, UI-thread blocking, ignored platform filter, false Compose scroll, Apple simulator-only package, and unpersisted oversized-event drops. Spec review identifies two critical packaging gaps (Android AAR without native core; Apple simulator-only iOS binary) plus Retry-After ignore, blocking lifecycle flush, and Compose pointer-as-scroll. Product review confirms five acceptance gaps: platform filter regression, incomplete Apple/Android clean-consumer install, capture blocked behind network-held SDK locks, and Compose false scroll. Core delivery, durability, timeout, five-platform Pay comparison, and optional-still behaviors are implemented.

_Each finding's checkbox tracks state through arbitration and resolution. Any agent addressing findings (engineer, implementer) updates the checkbox accordingly:_

- _`[ ]` — open_
- _`[~]` — deferred (engineer arbitration)_
- _`[-]` — rejected as false positive (engineer arbitration)_
- _`[x]` — done (finding addressed)_
- _`[!]` — regression (was `[x]`, surfaced again)_

_Each finding should carry enough context for arbitration without becoming a full essay: short title, concrete issue, impact, and suggested action._

## Code Review

Verdict: Changes requested.

- [x] **Critical** — `core/heatmap-ffi/src/c_abi.rs:69` — Concurrent disposal still permits use-after-free
  - **Issue:** Caller dereferences raw handle before registering in-flight use; `heatmap_sdk_free` can close and deallocate between those operations.
  - **Why it matters:** Concurrent capture/poll and disposal can access freed mutex/SDK memory, causing undefined behavior.
  - **Suggested action:** Serialize disposal with handle acquisition outside allocation being freed, or enforce ownership in managed wrappers so no call can overlap disposal. Add concurrent-call-versus-free test.

- [ ] **Finding** — `sdk/android/heatmap-android/src/main/kotlin/com/heatmap/sdk/HeatmapFlushScheduler.kt:22` — Background execution still blocks calling UI thread
  - **Issue:** Android waits on `Future.get()`, Swift waits on a semaphore at `sdk/apple/shared/Sources/HeatmapAppleShared/HeatmapFlushScheduler.swift:20`, and Windows waits at `sdk/windows/src/Heatmap.Sdk/HeatmapFlushScheduler.cs:50`.
  - **Why it matters:** Network latency still freezes caller despite work executing on another thread, violating non-blocking UI requirement. Capture also synchronously persists queue data.
  - **Suggested action:** Make scheduling asynchronous, expose async completion where required, and move persistence behind bounded background queue.

- [ ] **Finding** — `dashboard/src/api/buildElementsUrl.ts:6` — Platform filter is ignored
  - **Issue:** URL builder no longer sends `filters.platform`, although dashboard still presents platform selector.
  - **Why it matters:** Element totals, unique sessions, and top elements cannot be restricted by selected platform.
  - **Suggested action:** Keep filtered elements request platform-specific and issue separate unfiltered request for five-platform Pay comparison.

- [ ] **Finding** — `sdk/android/heatmap-android/src/main/kotlin/com/heatmap/sdk/capture/compose/ComposeScrollObserver.kt:8` — Pointer movement is misclassified as scrolling
  - **Issue:** Every `ACTION_MOVE` records scroll, including tap jitter and drags on non-scrollable content.
  - **Why it matters:** Produces many false scroll events and records drag/movement behavior excluded from V1.
  - **Suggested action:** Observe actual Compose scroll-state or nested-scroll deltas and coalesce meaningful scroll activity.

- [ ] **Finding** — `sdk/apple/Package.swift:51` — Apple package remains simulator/host specific
  - **Issue:** Package links raw dylibs through absolute unsafe search paths; included iOS binary is simulator-only, while macOS binary is arm64-only.
  - **Why it matters:** Device iOS and Intel macOS consumers cannot use package normally, and raw dylib/rpath setup is unsuitable for distributable SwiftPM applications.
  - **Suggested action:** Ship signed multi-platform XCFramework as SwiftPM `binaryTarget`, covering iOS device, simulator, and supported macOS architectures.

- [ ] **Finding** — `core/heatmap-core/src/sdk.rs:425` — Oversized-event removal is not persisted
  - **Issue:** When all queued events are oversized, method returns `EmptyQueue` before synchronizing modified queue.
  - **Why it matters:** Dropped events remain on disk and reappear after process restart, causing repeated drops and stale persistence.
  - **Suggested action:** Persist queue after oversized removal before returning; extend test with store reload assertion.

## Spec Review

Verdict: Changes required — 2 critical, 3 findings.

- [ ] **Critical** — `sdk/android/README.md:66` — AC: Installable SDKs; Design: pre-release Android binary — Android AAR ships without native core
  - **Issue:** `jniLibs` is empty at HEAD. Consumer smoke skips when `libheatmap_ffi.so` is missing.
  - **Why it matters:** Clean consumer resolves AAR but cannot initialize SDK or capture events.
  - **Suggested action:** Package required ABI libraries in published AAR and make missing-native smoke fail.

- [ ] **Critical** — `sdk/apple/Package.swift:54` — AC: Installable SDKs; Design: pre-release Apple binary — Apple binary supports simulator-only iOS
  - **Issue:** iOS links `vendor/ios-sim`; shipped binary is arm64 simulator dylib, not device-capable XCFramework. macOS binary is arm64-only.
  - **Why it matters:** Clean physical-iOS consumer cannot link or complete capture-to-dashboard path.
  - **Suggested action:** Ship signed multi-slice XCFramework through SwiftPM and test external device/simulator consumers.

- [ ] **Finding** — `core/heatmap-wasm/src/public_api.rs:87` — Design: web HTTP classification; Delivery failures fail closed — Web retries before `Retry-After`
  - **Issue:** `complete_flush_http(429, retry_after)` records retry deadline, but `begin_flush_batch_json()` ignores it. Click-triggered or timed flush can immediately resend.
  - **Why it matters:** Violates required 429 backoff and can amplify rate limiting.
  - **Suggested action:** Gate async batch acquisition on retry deadline and test attempts before and after expiry.

- [ ] **Finding** — `sdk/apple/ios/Sources/HeatmapAppleIOS/HeatmapIosRustCaptureBackend.swift:73` — NFR: Performance; Design: native background flush — Background lifecycle flush still blocks caller
  - **Issue:** Apple waits on a semaphore; Android and Windows equivalents synchronously wait on worker completion.
  - **Why it matters:** Network runs on another thread, but lifecycle/UI caller remains blocked for full request duration.
  - **Suggested action:** Enqueue lifecycle flush onto long-lived scheduler and return immediately; test caller does not wait.

- [ ] **Finding** — `sdk/android/heatmap-android/src/main/kotlin/com/heatmap/sdk/capture/compose/ComposeScrollObserver.kt:13` — AC: Android View and Compose; Design: adapter-owned Compose scroll — Pointer movement is misclassified as scroll
  - **Issue:** Every `ACTION_MOVE` emits scroll without confirming scrollable content moved.
  - **Why it matters:** Drags and ordinary pointer motion create false scroll events and potentially many events per gesture.
  - **Suggested action:** Observe actual Compose scroll/nested-scroll changes and coalesce each scroll gesture appropriately.

## Product Review

Verdict: Changes required. Five actionable product gaps remain at HEAD `46e70b6`.

**Implemented:**
- V1 interaction set; capture starts on init; local disable — `core/heatmap-core/src/event.rs:38`, `core/heatmap-core/src/sdk.rs:208`.
- Click/tap equivalence — `test/contract/pay-equivalence.test.mjs:44`.
- Anonymous/user identity and sign-out — `core/heatmap-core/src/sdk.rs:222`, `core/heatmap-core/src/sdk.rs:739`.
- Session background, timeout, and restart-after-expiry `session_end` — `core/heatmap-core/src/sdk.rs:562`, `core/heatmap-core/tests/session_timeout.rs:158`.
- Screen identity and layout separation — `core/heatmap-core/src/capture.rs:130`, `backend/tests/screen_layout_filters.rs:135`.
- Stable/unmarked elements and normalized coordinates — `backend/src/db.rs:163`, `core/heatmap-core/tests/exclude_mask_capture.rs:110`, `core/heatmap-core/src/event.rs:192`.
- Platform identity conventions and View/Compose Pay capture — `sdk/android/heatmap-android/src/main/kotlin/com/heatmap/sdk/compose/HeatmapComposeCapture.kt:80`, `poc/android/app/src/androidTest/kotlin/com/heatmap/checkout/e2e/CheckoutJourneyInstrumentedTest.kt:29`.
- Privacy and secret-field exclusion — `core/heatmap-core/tests/exclude_mask_capture.rs:158`, `test/privacy/cross-platform-secrets.test.mjs:19`.
- Default/all-or-nothing session sampling — `core/heatmap-core/tests/session_sampling.rs:74`, `core/heatmap-core/tests/session_sampling.rs:120`.
- Offline/process-death survival and persisted in-flight batch — `core/heatmap-core/src/persistent_queue.rs:44`, `core/heatmap-core/src/sdk.rs:415`, `core/heatmap-core/tests/persist_queue.rs:264`.
- Drop-oldest buffering — `core/heatmap-core/src/persistent_queue.rs:110`.
- Permanent versus retryable delivery failures — `core/heatmap-core/src/sdk.rs:488`, `core/heatmap-core/tests/flush_triggers.rs:312`.
- Five-platform Checkout → Pay comparison — `backend/src/db.rs:220`, `dashboard/src/components/PlatformPayCountsTable.tsx:10`, `test/e2e/tests/five-platform-dashboard-acceptance.spec.ts:49`.
- Per-platform coordinate overlay — `backend/src/db.rs:360`, `test/e2e/tests/five-platform-dashboard-acceptance.spec.ts:87`.
- Counts without reference still; no still-upload UI required — `dashboard/src/components/QueryResultsPanel.tsx:30`, `dashboard/src/components/HeatmapOverlayPanel.tsx:11`.
- No scroll-map dashboard visualization — `dashboard/test/dashboard.test.tsx:140`.
- Shared journey and contract equivalence — `test/contract/pay-equivalence.test.mjs:44`, `test/e2e/tests/five-platform-dashboard-acceptance.spec.ts:49`.
- Accessibility and security NFRs — `dashboard/test/dashboard.test.tsx:106`, `backend/src/handlers.rs:69`.

**Partially implemented:**
- Android adapter owns scroll capture, but scroll detection is not semantically correct.
- Dashboard filters work for overlays, but platform no longer restricts totals.
- Apple/Android package definitions exist, but clean-consumer distribution remains incomplete.
- Native HTTP executes on worker threads, but capture can still block behind network-held SDK locks.

- [ ] **Major** — `dashboard/src/api/buildElementsUrl.ts:7` — AC: Dashboard filters — Platform filter is ignored for totals
  - **Issue:** Elements request no longer includes `filters.platform`; selected platform still affects heatmap/still queries. Totals, unique sessions, and top elements therefore remain cross-platform.
  - **Why it matters:** Analyst selecting iOS can see an iOS overlay alongside all-platform totals, violating selected-slice behavior.
  - **Suggested action:** Fetch comparison data separately. Preserve platform on normal totals query while using an unfiltered request for five-platform Pay comparison.

- [ ] **Major** — `sdk/apple/Package.swift:52` — AC: Installable SDKs — Apple package is not distributable to normal clean consumers
  - **Issue:** Package uses unsafe linker flags and raw dylib search paths. Consumer smoke uses local path dependency and macOS only at `sdk/apple/consumer-smoke-test/Package.swift:5`. Vendored iOS binary supports Apple Silicon simulator only at `sdk/apple/vendor/README.md:3`.
  - **Why it matters:** Remote SwiftPM consumers and physical iOS devices cannot complete install → initialization without repository-local assumptions.
  - **Suggested action:** Ship signed XCFramework binary targets containing iOS device, simulator, and macOS slices; verify through versioned remote SwiftPM consumer.

- [ ] **Major** — `sdk/android/README.md:64` — AC: Installable SDKs — Android artifact lacks shipped native runtime
  - **Issue:** Maven publication is configured, but `jniLibs` remains empty and docs require producer-side Rust/NDK build plus `publishToMavenLocal` at `docs/android-sdk.md:7`.
  - **Why it matters:** Clean consumer cannot resolve a ready-to-run AAR without cloning/building producer repository.
  - **Suggested action:** Publish prerelease AAR containing supported ABI `.so` files to a consumer-accessible Maven repository and test resolution without producer checkout or Maven Local preparation.

- [ ] **Major** — `core/heatmap-ffi/src/uniffi_api.rs:90` — NFR: Performance — Background network operation still blocks capture
  - **Issue:** `flush`/`poll_flush` holds SDK mutex throughout blocking HTTP, while capture needs same mutex at `core/heatmap-ffi/src/uniffi_api.rs:123`. Native transport is synchronous at `core/heatmap-core/src/transport.rs:74`. C ABI has equivalent locking at `core/heatmap-ffi/src/c_abi.rs:69`.
  - **Why it matters:** UI-thread tap/click callback can wait behind slow HTTP despite scheduler running transport on another thread.
  - **Suggested action:** Take and persist batch under short lock, release lock before HTTP, then reacquire for acknowledgement/failure handling. Add blocked-transport concurrency test proving capture returns promptly.

- [ ] **Finding** — `sdk/android/heatmap-android/src/main/kotlin/com/heatmap/sdk/capture/compose/ComposeScrollObserver.kt:9` — AC: V1 interaction set / Android Compose scroll — Every pointer move is reported as scroll
  - **Issue:** Any `ACTION_MOVE` emits scroll, including drag/gesture movement where no composable scrolled.
  - **Why it matters:** Non-scroll gestures become false scroll events, weakening journey and interaction semantics.
  - **Suggested action:** Observe actual Compose scroll state or nested-scroll consumption and emit only when viewport content changes.

---

## Review Notes

- [code-reviewer/code] No IDE linter errors found in inspected files.
- [code-reviewer/code] Existing tests cover concurrent C ABI calls, but free occurs only after worker threads join; disposal race remains untested.
- [spec-reviewer/spec] Core persistence, retry payload identity, session timeout rotation, 256 KiB envelope bound, nested-field rejection, optional platform query, dashboard comparison, and no-upload UI match active design.
- [spec-reviewer/external constraints] Supplied web, Apple, Android, macOS, and Windows identity constraints are satisfied.
- [spec-reviewer/standards] Technical Standards MCP unavailable; no TS identifiers inferred.
- [spec-reviewer/spec] Five-platform dashboard acceptance seeds native fixtures; native journey validation remains delegated to platform-specific suites.
- [product-reviewer/out-of-scope] No session replay, text capture, V2 visualization, remote configuration, or production control-plane drift found.
- [product-reviewer/validation] Static review against tracked HEAD `46e70b697887b54c4247701d6fda269d756ddbd1`; no files modified.
