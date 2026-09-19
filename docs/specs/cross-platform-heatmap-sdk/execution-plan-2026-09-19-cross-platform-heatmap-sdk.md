# Execution Plan: Cross-Platform Heatmap SDK

## Source Context

- **Solution Design:** docs/specs/cross-platform-heatmap-sdk/solution-design-2026-09-19-cross-platform-heatmap-sdk.md
- **Task:** docs/specs/cross-platform-heatmap-sdk/task-2026-09-19-cross-platform-heatmap-sdk.md
- **Supporting Artifacts:** docs/specs/cross-platform-heatmap-sdk/research-2026-09-19-cross-platform-heatmap-sdk.md; docs/specs/cross-platform-heatmap-sdk/references-2026-09-19-cross-platform-heatmap-sdk.md; docs/tasks/Cross-Platform Heatmap SDK Build/plan.md (background only; task and solution design win on conflict)

## Outline

Greenfield V1 POC: lock the nested event contract, prove a synthetic click through core, ingest, and query, then complete the web path to dashboard before multiplying native SDKs.

- Contract and synthetic tracer prove schema, batch, persist, and element totals without UI.
- Web WASM SDK, React checkout POC, and dashboard counts prove the first real consumer.
- Shared core depth (identity, session, privacy, sampling, queue, retry, scroll, bins) lands on that path before other platforms.
- Apple, Android, and Windows adapters then proceed in parallel from one FFI surface.
- Five-platform contract, acceptance journey, privacy gate, and CI close the POC.

## Steps

- [x] **Step 1: Lock event and batch schemas** — TDD: No
  - **Description:** Add the nested V1 event schema, batch wrapper schema, and golden fixtures for the allowed event types and platforms.
  - **Files:** schemas/event-v1.json; schemas/batch-v1.json; test/fixtures/; docs/ for schema documentation
  - **Tests:** Fixtures validate against the schemas; allowed types are session_start, session_end, screen_view, click, scroll only; click is the shared activation type; position is 0–1 window fraction with no in-control fields; user_id, screen, position, viewport, and window_id may be omitted; flatter plan.md sketch is not a second wire schema
  - **Depends:** None

- [x] **Step 2: Serialize canonical events in core** — TDD: Yes
  - **Description:** Create the heatmap-core crate and event model that serializes to the golden nested envelope.
  - **Files:** Cargo.toml (workspace); core/heatmap-core/
  - **Tests:** Serialized session_start, screen_view, click, scroll, and session_end match fixtures; unknown or out-of-scope types are rejected; omitted optional fields stay omitted; introduce `cargo test -p heatmap-core`
  - **Depends:** Step 1

- [x] **Step 3: Init, capture gate, and in-memory batch** — TDD: Yes
  - **Description:** Accept init config, start capture when tracking is enabled, enqueue allowed candidates, and build a batch with a new batch_id.
  - **Files:** core/heatmap-core/
  - **Tests:** Successful init makes later candidates eligible without a separate enable call; disable stops capture and enable resumes it; sampled-in session can enqueue a click; batch body matches batch schema; host-facing APIs stay fail-closed
  - **Depends:** Step 2

- [x] **Step 4: Ingest a batch and query element totals** — TDD: Yes
  - **Description:** Stand up the Rust ingest/query process so a valid batch persists raw events and a Pay click appears in element totals.
  - **Files:** backend/
  - **Tests:** Valid batch with X-Project-Key returns 202 and accepted_count; invalid project key returns 401/403; duplicate batch_id returns 202 with no new writes; click with element_id upserts totals; click without element_id stores raw event and does not create a total row; introduce backend `cargo test`
  - **Depends:** Step 2

- [x] **Step 5: Flush one synthetic click to query** — TDD: Yes
  - **Description:** Wire core transport so an explicit flush delivers one Pay click to local ingest and the elements query shows it.
  - **Files:** core/heatmap-core/; test/integration/; backend/
  - **Tests:** Synthetic checkout.pay click at window center uploads once and appears under Checkout → Pay; transport uses the configured HTTPS ingest base; host remains up if ingest is down
  - **Depends:** Step 3, Step 4

- [x] **Step 6: Expose public API through WASM** — TDD: No
  - **Description:** Add heatmap-wasm bindings for the public SDK surface used by web.
  - **Files:** core/heatmap-wasm/
  - **Tests:** Binding surface covers init, identify, sign-out, screen with optional layout, setTrackingEnabled, and flush; no public track/custom API
  - **Depends:** Step 5

- [x] **Step 7: Capture web clicks with stable ids** — TDD: Yes
  - **Description:** Add the web TypeScript SDK that auto-captures activations, reads data-heatmap-id, and sends window-normalized positions.
  - **Files:** sdk/web/
  - **Tests:** Click on Pay with data-heatmap-id=checkout.pay emits click with that element_id; unmarked non-secret control emits click with position and no element_id; center of the visible window stores ~0.5, ~0.5; after resize, new events use the new window; adapter does not read control text or input values; introduce the sdk/web package test script
  - **Depends:** Step 6

- [ ] **Step 8: Build React checkout POC** — TDD: No
  - **Description:** Add the React sample app that completes the shared checkout journey and initializes the web SDK.
  - **Files:** poc/react/
  - **Tests:** App can init with project key and endpoint; journey includes home, product, checkout with scroll and coupon, Pay, and confirmation; Pay uses data-heatmap-id=checkout.pay; Cancel is present and unmarked
  - **Depends:** Step 7

- [ ] **Step 9: Show element totals in the dashboard** — TDD: Yes
  - **Description:** Add the React dashboard so Checkout → Pay counts render from query data without a reference still.
  - **Files:** dashboard/
  - **Tests:** Totals, unique sessions, and top elements render for a slice that has clicks; empty slice shows zero counts and no overlay; query error is visible and does not crash; filters and counts are keyboard-reachable with visible names; no login wall
  - **Depends:** Step 4, Step 8

- [ ] **Step 10: Prove web checkout through dashboard** — TDD: Yes
  - **Description:** Add the first per-platform E2E so the React journey lands in ingest and dashboard counts.
  - **Files:** test/e2e/
  - **Tests:** Sequence includes session_start, screen_view(home), click(product), screen_view(checkout), scroll, click(checkout.coupon), click(checkout.pay), screen_view(confirmation); dashboard shows web Pay totals; introduce the web E2E runner
  - **Depends:** Step 8, Step 9

- [ ] **Step 11: Persist anonymous and optional user identity** — TDD: Yes
  - **Description:** Keep a stable anonymous_id per install and add identify / sign-out that only add or remove user_id.
  - **Files:** core/heatmap-core/; sdk/web/
  - **Tests:** Events before and after identify share the same anonymous_id; events after identify include user_id; events after sign-out keep anonymous_id and omit user_id
  - **Depends:** Step 5, Step 7

- [ ] **Step 12: Rotate sessions on timeout, not background** — TDD: Yes
  - **Description:** Create a session on first capture, keep it across background/foreground before timeout, and start a new session after inactivity timeout or expiry.
  - **Files:** core/heatmap-core/
  - **Tests:** Default timeout is 1,800,000 ms and is overridable; captured event resets the inactivity window; background then resume before timeout keeps session_id; interaction after timeout or expiry starts a new session; tests use a short override
  - **Depends:** Step 11

- [ ] **Step 13: Inherit screen and layout version** — TDD: Yes
  - **Description:** Attach the app-declared screen and optional layout_version to later events until screen() changes, and keep layout versions out of one overlay bucket.
  - **Files:** core/heatmap-core/; backend/
  - **Tests:** Events after screen(Checkout) group under Checkout; later screen(Confirmation) does not mix; events with no screen still send and are omitted from screen-filtered queries; Checkout layout A and layout B do not share one overlay key; missing layout uses the unspecified-layout bucket
  - **Depends:** Step 12, Step 4

- [ ] **Step 14: Apply exclude, mask, and no-text rules** — TDD: Yes
  - **Description:** Drop excluded elements and screens, strip element_id on masked controls while keeping coordinates, and keep text and typed values out of payloads, logs, queue data, and diagnostics.
  - **Files:** core/heatmap-core/; test/fixtures/
  - **Tests:** Excluded control or screen is not captured; masked control stores coordinates and omits element_id; serialized events and diagnostics contain no label text or typed values; core does not require adapters to supply text
  - **Depends:** Step 13

- [ ] **Step 15: Sample whole sessions only** — TDD: Yes
  - **Description:** Capture every session by default; when a sample rate is set, decide once at session start and persist that decision.
  - **Files:** core/heatmap-core/
  - **Tests:** Default sample_rate 1.0 enqueues the session; sampled-out session enqueues nothing and does not appear in dashboard queries; decision does not change mid-session; local diagnostic records sampled-out
  - **Depends:** Step 12

- [ ] **Step 16: Persist the queue with dual-cap overflow** — TDD: Yes
  - **Description:** Survive offline and process death with a persistent queue that drops oldest events at 10,000 events or 5 MiB and keeps capturing.
  - **Files:** core/heatmap-core/
  - **Tests:** Events captured offline upload once after restore; relaunch with pending events does not crash; at cap, oldest events drop, newer events enqueue, host stays up; local diagnostic counts drops
  - **Depends:** Step 14, Step 15

- [ ] **Step 17: Flush on triggers and classify delivery failures** — TDD: Yes
  - **Description:** Flush on 50 events, 64 KiB, 10 seconds, background, or flush(); retry only transient failures; treat invalid key, bad schema, and oversize as permanent.
  - **Files:** core/heatmap-core/
  - **Tests:** Each flush trigger builds a batch with a new batch_id; 5xx and 429 keep the queue and retry with the same batch_id, honoring Retry-After; 401/403, 400, and 413 drop that batch, increment a diagnostic, and do not retry-loop; host stays up
  - **Depends:** Step 16, Step 5

- [ ] **Step 18: Record scroll without a scroll map** — TDD: Yes
  - **Description:** Accept scroll on the current screen as type scroll only; store raw scroll and keep it out of bins and dashboard overlay.
  - **Files:** core/heatmap-core/; backend/; dashboard/
  - **Tests:** Scroll after screen(Checkout) is stored on Checkout; scroll is not binned; dashboard Checkout view shows click/tap counts, unique sessions, top elements, density overlay, and platform breakdown, not a scroll-depth map
  - **Depends:** Step 13, Step 9

- [ ] **Step 19: Enforce ingest limits and build click bins** — TDD: Yes
  - **Description:** Reject oversized, malformed, and rate-limited batches as specified; persist valid events; upsert 100-by-100 click bins and expose heatmap and screens queries.
  - **Files:** backend/
  - **Tests:** Oversize returns 413; malformed or unsupported schema_version returns 400; ~120 batches/minute/key returns 429; unknown fields are ignored; missing required fields reject that event; valid sibling events in the same batch still persist; click bins use min(99, floor(x * 100)) and the same for y; overlay query key is project + platform + screen + layout_version, not window_id; GET /v1/screens lists known screens; GET /v1/heatmap returns bins for the slice
  - **Depends:** Step 4, Step 18

- [ ] **Step 20: Overlay coordinates on optional stills with filters** — TDD: Yes
  - **Description:** Let the analyst filter by platform, screen, layout or app version, and date range; draw the per-platform density overlay on a still when one exists.
  - **Files:** backend/; dashboard/; poc/stills/
  - **Tests:** Filters restrict counts and overlay to the selected slice; missing still still shows counts and no overlay image; present still for that screen + layout + platform is the overlay background; different layout versions are not drawn together; pending vs empty vs error states are distinct; filter and overlay controls stay keyboard-reachable
  - **Depends:** Step 19, Step 9

- [ ] **Step 21: Finish web capture, package, and docs** — TDD: Yes
  - **Description:** Complete web secret-field dropping, scroll, disable, and installable NPM packaging with integrator docs.
  - **Files:** sdk/web/; docs/
  - **Tests:** Password and common payment/password autocomplete fields are dropped even when unmarked; tracking disabled captures nothing further; a clean consumer project can install the pre-release web package and init; docs cover install, init, screen, annotate, disable, identify, and sign-out
  - **Depends:** Step 7, Step 14, Step 17, Step 18

- [ ] **Step 22: Cover web failure and privacy E2E** — TDD: Yes
  - **Description:** Extend web E2E for offline upload, invalid key, full buffer, and no-text privacy.
  - **Files:** test/e2e/; sdk/web/
  - **Tests:** Offline then restore uploads once with no dashboard double-count; invalid project key does not crash the host or retry-loop; buffer-at-capacity drops oldest and keeps the host up; password and typed values never appear in payloads, logs, or queue files
  - **Depends:** Step 10, Step 21, Step 17

- [ ] **Step 23: Expose the same public API through native FFI** — TDD: No
  - **Description:** Add heatmap-ffi with UniFFI for Apple and Android and a C ABI for Windows, matching the public SDK surface.
  - **Files:** core/heatmap-ffi/
  - **Tests:** Exported surface covers init, identify, sign-out, screen with optional layout, setTrackingEnabled, and flush; no public track/custom API; generated bindings are callable from a smoke host on each intended path
  - **Depends:** Step 17

- [ ] **Step 24: Add shared Swift API and package** — TDD: No
  - **Description:** Add the shared Swift API over UniFFI and a Swift package with separate iOS and macOS targets.
  - **Files:** sdk/apple/shared/; Package.swift or sdk/apple/ package manifest
  - **Tests:** One package version exposes the shared API to both targets; a clean consumer project can depend on the pre-release package
  - **Depends:** Step 23

- [ ] **Step 25: Capture iOS UIKit activations** — TDD: Yes
  - **Description:** Add iOS capture for taps, screen views, scrolls, lifecycle, window-normalized positions, and accessibilityIdentifier identity.
  - **Files:** sdk/apple/ios/
  - **Tests:** Pay with accessibilityIdentifier=checkout.pay is attributed as Pay; unmarked non-secret control paints coordinates only; secure text and password/payment content types are dropped even when unmarked; background then foreground before timeout keeps the session; adapter does not read labels or typed values; introduce XCTest
  - **Depends:** Step 24

- [ ] **Step 26: Build iOS checkout POC and docs** — TDD: No
  - **Description:** Add the iOS sample app for the shared checkout journey and iOS integrator docs.
  - **Files:** poc/ios/; docs/
  - **Tests:** Journey matches the shared sequence; Pay uses accessibilityIdentifier=checkout.pay; docs cover install, init, screen, annotate, disable, identify, and sign-out
  - **Depends:** Step 25

- [ ] **Step 27: Prove iOS checkout through ingest** — TDD: Yes
  - **Description:** Add iOS E2E for the checkout journey, background/resume, and fail-closed delivery.
  - **Files:** test/e2e/
  - **Tests:** Shared journey sequence is stored; background/resume stays one session; coordinates stay 0–1 of the current window; host stays up when ingest is down; introduce the iOS E2E runner
  - **Depends:** Step 26, Step 19

- [ ] **Step 28: Capture macOS AppKit activations** — TDD: Yes
  - **Description:** Add macOS capture with accessibilityIdentifier then view identifier lookup, multi-window attribution, and secret-field dropping.
  - **Files:** sdk/apple/macos/
  - **Tests:** Pay identity uses accessibilityIdentifier first and identifier.rawValue when empty; events carry opaque window_id for the window used; coordinates are relative to that window; resize and scale change affect only new events; secure text fields are dropped; adapter does not send window titles or control text; introduce XCTest for macOS
  - **Depends:** Step 24

- [ ] **Step 29: Build macOS checkout POC and docs** — TDD: No
  - **Description:** Add the macOS sample app for the shared checkout journey and macOS integrator docs.
  - **Files:** poc/macos/; docs/
  - **Tests:** Journey matches the shared sequence; Pay uses the agreed macOS identity; docs cover install, init, screen, annotate, disable, identify, and sign-out
  - **Depends:** Step 28

- [ ] **Step 30: Prove macOS checkout through ingest** — TDD: Yes
  - **Description:** Add macOS E2E for checkout, resize, and multiple windows.
  - **Files:** test/e2e/
  - **Tests:** Shared journey sequence is stored; interaction after resize sits on the new window; interaction in a second window is attributable to that window; V1 overlay still does not split by window; introduce the macOS E2E runner
  - **Depends:** Step 29, Step 19

- [ ] **Step 31: Add Android Kotlin API and AAR** — TDD: No
  - **Description:** Package the Android SDK API over UniFFI as a pre-release AAR/Maven artifact.
  - **Files:** sdk/android/
  - **Tests:** A clean consumer project can depend on the artifact and call the public API; no public track/custom API; introduce Gradle test wiring
  - **Depends:** Step 23

- [ ] **Step 32: Capture Android View activations** — TDD: Yes
  - **Description:** Add the View adapter with dedicated heatmap identity, window-normalized positions, lifecycle, and View secret-field dropping.
  - **Files:** sdk/android/
  - **Tests:** Pay with the dedicated View heatmap identity is attributed as Pay; resource name and content description are not used as identity; unmarked non-secret View paints coordinates only; password input types and autofill hints are dropped; background/resume before timeout keeps the session; adapter does not read text
  - **Depends:** Step 31

- [ ] **Step 33: Capture Android Compose activations** — TDD: Yes
  - **Description:** Add the Compose adapter that reads a dedicated heatmap identity with test-tag fallback and does not rely on View hit-testing.
  - **Files:** sdk/android/
  - **Tests:** Pay with dedicated Compose identity is attributed as Pay; testTag is used when dedicated identity is absent; unmarked non-secret composable may contribute coordinates and has no element total; password visual transformation / password semantics are dropped; adapter does not read text
  - **Depends:** Step 31

- [ ] **Step 34: Build Android checkout POC and docs** — TDD: No
  - **Description:** Add the Android sample app with both View and Compose screens for the shared journey, plus Android integrator docs.
  - **Files:** poc/android/; docs/
  - **Tests:** View screen and Compose screen each have Pay with the same business identity; journey matches the shared sequence; docs cover install, init, screen, annotate, disable, identify, and sign-out
  - **Depends:** Step 32, Step 33

- [ ] **Step 35: Prove Android checkout and offline queue** — TDD: Yes
  - **Description:** Add Android E2E for View and Compose Pay, offline persist, and fail-closed delivery.
  - **Files:** test/e2e/
  - **Tests:** Both Pay activations are captured as Pay; offline then restore uploads once; process death with pending events uploads later without crashing relaunch; host stays up when ingest is down; introduce the Android E2E runner
  - **Depends:** Step 34, Step 16, Step 19

- [ ] **Step 36: Add Windows C# API and NuGet package** — TDD: No
  - **Description:** Add the C# P/Invoke surface over the C ABI and a pre-release NuGet package.
  - **Files:** sdk/windows/
  - **Tests:** A clean consumer project can install the package and call the public API; no public track/custom API; introduce `dotnet test`
  - **Depends:** Step 23

- [ ] **Step 37: Capture WPF activations** — TDD: Yes
  - **Description:** Add WPF capture for clicks, scrolls, AutomationId identity, DPI/scale, multi-window attribution, and password-box dropping.
  - **Files:** sdk/windows/
  - **Tests:** Pay with AutomationProperties.AutomationId=checkout.pay is attributed as Pay; unmarked non-secret control paints coordinates only; password boxes are dropped; events carry opaque window_id; coordinates are relative to that window and stay 0–1 after DPI/scale or resize; adapter does not read text or window titles
  - **Depends:** Step 36

- [ ] **Step 38: Build Windows checkout POC and docs** — TDD: No
  - **Description:** Add the WPF sample app for the shared checkout journey and Windows integrator docs.
  - **Files:** poc/windows/; docs/
  - **Tests:** Journey matches the shared sequence; Pay uses AutomationId=checkout.pay; docs cover install, init, screen, annotate, disable, identify, and sign-out
  - **Depends:** Step 37

- [ ] **Step 39: Prove Windows checkout through ingest** — TDD: Yes
  - **Description:** Add Windows E2E for checkout, resize, DPI/scale, and multiple windows.
  - **Files:** test/e2e/
  - **Tests:** Shared journey sequence is stored; new events after resize or DPI/scale use the current window; second-window interaction is attributable to that window; host stays up when ingest is down; introduce the Windows E2E runner
  - **Depends:** Step 38, Step 19

- [ ] **Step 40: Assert five-platform Pay contract** — TDD: Yes
  - **Description:** Add contract tests that the five SDKs emit semantically equivalent nested events for the same annotated Pay activation.
  - **Files:** test/contract/; test/fixtures/
  - **Tests:** Web click and iOS/Android/macOS/Windows activations of Pay agree on click type, screen, element_id checkout.pay, and 0–1 position meaning; platform remains a filter dimension; introduce the contract runner
  - **Depends:** Step 22, Step 27, Step 30, Step 35, Step 39

- [ ] **Step 41: Accept the five-app dashboard journey** — TDD: Yes
  - **Description:** Run the shared checkout journey on all five sample apps and show cross-platform Pay totals plus per-platform coordinate heatmaps in one dashboard.
  - **Files:** test/e2e/; dashboard/; poc/stills/
  - **Tests:** All five apps complete install → init → journey → upload; Checkout → Pay shows counts for web, iOS, Android, macOS, and Windows; each platform overlay can be viewed separately for the same screen and layout; this is the cross-platform acceptance test
  - **Depends:** Step 20, Step 40

- [ ] **Step 42: Gate privacy on every surface** — TDD: Yes
  - **Description:** Add the mandatory privacy CI check that secret fixtures never leak through any SDK, queue, log, or ingest path.
  - **Files:** test/; CI workflow files as needed for this gate
  - **Tests:** Password and payment secret fixtures never appear in payloads, logs, queued data, or diagnostics on web, iOS, Android, macOS, and Windows; app-excluded controls also stay out
  - **Depends:** Step 22, Step 27, Step 30, Step 35, Step 39

- [ ] **Step 43: Add CI for format, unit, contract, builds, and E2E** — TDD: No
  - **Description:** Add CI workflows that run formatting, unit tests, contract tests, package builds, per-platform E2E, and the cross-platform acceptance test.
  - **Files:** .github/workflows/
  - **Tests:** Pipeline covers cargo test for core and backend, sdk/web tests, XCTest, Gradle test, dotnet test, contract runner, privacy gate, and E2E runners introduced in earlier steps
  - **Depends:** Step 41, Step 42

## Notes

- Task and solution design win over plan.md: one wire type click, no public track API, no in-control coordinates, no remote disable, no scroll map, no visibility/custom/hover types, repository root is heatmap-tool not a nested heatmap/ folder.
- Step 3 and Step 4 may run in parallel after Step 2; they merge at Step 5.
- After Step 23, Apple (Steps 24–30), Android (Steps 31–35), and Windows (Steps 36–39) may run in parallel. After Step 24, Step 25 and Step 28 may run in parallel.
- Validation commands from the solution design are introduced with the first code for that surface: core tests in Step 2, backend tests in Step 4, web package tests in Step 7, web E2E in Step 10, XCTest in Step 25, Gradle in Step 31, dotnet in Step 36, contract runner in Step 40.
- Each TDD: Yes step is one RED-GREEN-REFACTOR cycle and must end green.
- Production login, multi-project switching, viewport-class filters, generally-available marketplace listing, and plan.md performance-lab / SAST / signed-release hardening stay out of this plan.
