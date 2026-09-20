# Solution Design: Cross-Platform Heatmap SDK

**Inputs:**
- `docs/specs/cross-platform-heatmap-sdk/task-2026-09-19-cross-platform-heatmap-sdk.md` (authoritative for ACs, scope, constraints)
- `docs/specs/cross-platform-heatmap-sdk/research-2026-09-19-cross-platform-heatmap-sdk.md`
- `docs/specs/cross-platform-heatmap-sdk/references-2026-09-19-cross-platform-heatmap-sdk.md`
- `docs/tasks/Cross-Platform Heatmap SDK Build/plan.md` (background only; task wins on conflict)

## Outline

- Greenfield V1 POC: first product, not a change to an existing SDK. No `core/`, `sdk/`, `backend/`, `dashboard/`, schemas, tests, or package manifests exist.
- Shared Rust telemetry core; thin platform adapters capture UI, lifecycle, storage, and network only.
- Five consumers: web (React/NPM), iOS/macOS (one Swift package, two targets), Android (Maven/AAR, View/Compose), Windows (NuGet/WPF).
- One nested event envelope. Mouse and finger activations share wire type `click`. Coordinates are 0–1 fraction of window, top-left origin. No in-control coordinates in V1.
- One Rust ingest/query process with SQLite. Raw events plus 100-by-100 bins and element totals updated on accept. React dashboard, no login. Stills are optional fixtures plus manual upload; SDK does not screenshot.
- Validate with core unit/property tests, golden contract fixtures, adapter tests, ingest tests, privacy gates, per-platform checkout E2E, and cross-platform acceptance journey. No product test toolchain exists on disk yet.
- No open design questions. Production rate limits, viewport-class filters, and scroll-depth maps excluded from V1.

---

## Decisions

| Decision | Why | Rejected Alternatives |
| -------- | --- | --------------------- |
| One Rust core with thin native adapters | Shared telemetry behavior; platform code captures UI, lifecycle, storage, network. Single implementation of session, privacy, sampling, queue, serialization. | Five native SDKs sharing only JSON (drift). Other shared language (weaker Wasm/AppKit/WPF fit). |
| WebAssembly on web; UniFFI on Apple and Android; C ABI on Windows | UniFFI fits Swift/Kotlin. Windows/.NET more reliable with P/Invoke. Browser must not load native library. | One C ABI on every platform (wrapper risk). UniFFI on Windows (less proven for .NET). |
| One wire type `click` for mouse and finger; platform stays the filter | Pay click and Pay tap count as same interaction, attributable by platform. | Two types merged in aggregation. One type plus input-modality field (not for V1). |
| Window-normalized position only | Overlay is window fraction, top-left origin. In-control coordinates add unused capture work. | In-control position when bounds known. Always send both. |
| Opaque `window_id` on event; V1 overlay does not split by window | Desktop events belong to window used; coordinates use that window. Dashboard filters exclude window. Window titles off wire. | Split each overlay by window. Viewport only, no window id. |
| Dedicated heatmap identity on Android View; Compose dedicated identity with test-tag fallback | Totals need same business string as other platforms. Resource names and labels can leak text. | View resource name. Generic View tag field only. |
| Default inactivity timeout 30 minutes, overridable; tests use short override | Required documented default. Captured event resets window. Backgrounding does not start new session. | 15 minutes. 60 minutes. |
| Persistent queue, 10,000 events or 5 MiB, drop oldest | Offline and process-death survival; fail-closed overflow. Dual cap prevents large payload filling disk. | Count cap only. Memory queue persisted only on background or flush. |
| One Rust ingest/query service, SQLite, aggregate on accept | POC test environment, not multi-tenant control plane. Shared event types with core. Client `batch_id` makes retries idempotent. | TypeScript service and SQLite. Separate ingest/aggregation/API with Postgres. |
| React dashboard, no login, canvas overlay on optional still | Totals, per-platform overlay, filters, and counts without still. POC has no sign-in. | Server-rendered pages. Heavier visualization toolkit. |
| Checked-in sample stills plus manual dashboard upload | Overlay sits on still when exists; counts work without one. SDK screenshots are reconstruction/privacy risk. | Fixtures only. SDK-captured screenshot. |
| Drop platform-known secrets and app excludes; mask keeps coordinates and strips element id | Never send text or typed values. Secrets drop even when unmarked. Mask distinct from exclude. Adapters never read text. | Treat mask as exclude. Send placeholder element id. |
| Flush on 50 events, 64 KiB, 10 seconds, background, or explicit flush; retry only transient failures | Offline upload without host crashes. Invalid key, bad schema, and oversized payload must not retry-loop. | Time-only flush. Retry every non-success. |
| Session sampling default 1.0; decide at session start; persist decision; enqueue nothing when sampled out | Capture all sessions unless configured; decision does not change mid-session. | Mid-session re-roll. Per-event sampling (out of scope). |
| Public API: init, identify, sign-out, screen with optional layout, disable/enable, flush; no manual track | Custom events out of V1. Activations from auto-capture. Layout travels with screen to prevent silent overlay splits. | Manual track. Separate layout call. |
| Store raw accepted events and 100-by-100 bins plus element totals | On-accept bins serve overlay; raw events allow aggregate rebuild and contract checks. | Bins only. Raw only, bin at query time. |
| Scroll event is shared envelope plus type `scroll` only | Record scroll on current screen; no scroll map. Offsets would invent second coordinate spec. | Normalized content offset. Scrollable-control identity. |
| One Swift package with separate iOS and macOS targets | Shared UniFFI core and Swift API; capture differs. One version, two targets. | Two Swift packages. One shared target compiling both UIKits. |
| Ingest `HTTPS POST /v1/batches` with project key on request; JSON body | Encrypted transit, project-key auth, JSON for POC. | Compact binary wire in V1. Unauthenticated ingest. |
| Server ceilings: 200 events or 256 KiB per batch; ~120 batches per minute per project key | Above client flush so normal checkout avoids ceiling. | |
| Unknown fields ignored; missing required fields reject event | Backend must not require simultaneous client upgrades. Host app fail-closed. | |
| Diagnostics are local counters only | Dropped, sampled-out, and failure counts aid tests. No diagnostics UI in V1. | |
| Repository is monorepo root | Repo is `heatmap-tool`; plan tree lands here, not nested `heatmap/` folder. | |

---

## Standards & References Compliance

**Technical standards:**

| Standard | How Satisfied |
| -------- | ------------- |
| None loaded | Technical Standards MCP unavailable. No `TS-*` identifiers apply. Platform identity rules follow official docs in references artifact. |

**Repo references:**

| Reference | How Satisfied |
| --------- | ------------- |
| `AGENTS.md` | Design inside task scope; no unrequested features; artifact follows Playbook rules. |
| `.ai-sdlc/docs/ARCHITECTURE.md` | Artifact in `docs/specs/cross-platform-heatmap-sdk/` using solution-design template. |
| `.ai-sdlc/docs/BUDGET-ENGINEERING.md` | Decisions compacted here for execution-plan start. |
| `.ai-sdlc/docs/repo-workspace/README.md` | Feedback channels named in Validation; must be introduced with product. |
| `.ai-sdlc/docs/CLAUDE-CODE.md`, `.ai-sdlc/docs/QUICKSTART.md`, `.ai-sdlc/docs/PROTOTYPE.md`, `.ai-sdlc/docs/CURSOR-CLI.md`, `.cursor/hooks.json` | Harness-only. No product runtime. Hooks are session hooks, not heatmap privacy. |

**External documentation (identity constraints):**

| Source | How Satisfied |
| ------ | ------------- |
| WHATWG / MDN / React `data-*` | Web stable identity is `data-heatmap-id` (dataset key `heatmapId`). |
| Apple `accessibilityIdentifier` (UIKit) | iOS stable identity. Not VoiceOver label. |
| Apple AppKit `accessibilityIdentifier` then `NSView.identifier` | macOS lookup order. Dot allowed; `/`, `\`, `:`, leading `_` reserved. |
| Android Compose `Modifier.testTag` / semantics tree | Separate adapter. Dedicated identity first; test tag fallback. |
| WPF `AutomationProperties.AutomationId` | Windows stable identity. Unique among siblings. |

---

## Design

### Structure

```
heatmap-tool (monorepo root)
│
├── schemas/                         canonical JSON Schema + golden fixtures
├── core/
│   ├── heatmap-core/                Rust: config, session, identity, privacy,
│   │                                sampling, queue, batch, retry, serialize
│   ├── heatmap-wasm/                wasm-bindgen for web
│   └── heatmap-ffi/                 UniFFI (Apple, Android) + C ABI (Windows)
├── sdk/
│   ├── web/                         TypeScript: DOM capture, NPM package
│   ├── apple/
│   │   ├── shared/                  Swift API over UniFFI
│   │   ├── ios/                     UIKit capture
│   │   └── macos/                   AppKit capture
│   ├── android/                     Kotlin: View adapter + Compose adapter, AAR
│   └── windows/                     C# P/Invoke + WPF capture, NuGet
├── poc/
│   ├── react|ios|android|macos|windows   same checkout journey
│   └── stills/                      optional reference images per screen/layout/platform
├── backend/                         one Rust process: ingest + query + SQLite
├── dashboard/                       React SPA, no login
├── test/
│   ├── fixtures/                    golden events (nested envelope)
│   ├── contract/
│   ├── integration/
│   └── e2e/                         per-platform + cross-platform checkout
└── docs/                            install/config per SDK + schema docs
```

Core owns session, identity, exclude/mask/sampling, queue, batching, retry class, serialization. Adapters own hit-testing, platform identity, secret detection, window/viewport/coordinates, lifecycle, persistent storage, HTTPS. Adapters never read control text or input values.

Identity lookup: web `data-heatmap-id`; iOS `accessibilityIdentifier`; Android View dedicated heatmap identity only (not resource name or content description); Android Compose dedicated identity then `testTag`; macOS `accessibilityIdentifier` then `identifier.rawValue`; Windows `AutomationProperties.AutomationId`. Missing identity still stores a click position; no element-total row. Generated view paths are not identities.

Coordinate rules: `position.x` / `position.y` are 0.0–1.0 inclusive of the current visible window, origin top-left. After resize, rotation, or DPI/scale change, new events use the new window. Viewport width/height are CSS/logical pixels of that window. Bin: `min(99, floor(x * 100))`, same for y. Overlay key: project + platform + screen + layout_version (unspecified if missing). Not `window_id`.

Privacy catalog (adapters drop before core): web password inputs and common payment/password autocomplete; iOS secure text entry and password/payment content types; Android View password input types and autofill hints; Compose password visual transformation / password semantics; macOS secure text fields; Windows password boxes. No label text, typed values, or window titles in payloads, logs, queue files, or diagnostics.

Session rules: new session on first capture after init, after inactivity timeout, or after expiry. Background/foreground before timeout keeps the session (iOS, Android, macOS, Windows). Default timeout 1,800,000 ms.

### Data Flow

```
App calls init(config)
  → core persists/creates anonymous_id
  → core starts session if tracking enabled and sampled in
  → adapters attach auto-capture
User navigates / activates / scrolls
  → adapter builds capture candidate (no text)
  → adapter drops platform-known secrets
  → core drops if disabled, excluded, or sampled out
  → core applies mask (strip element_id, keep coordinates)
  → core attaches session/user/anonymous IDs and screen/layout
  → core normalizes position 0–1 for click type
  → core enqueues to persistent queue
Flush trigger (50 events / 64 KiB / 10s / background / flush())
  → core builds batch with new batch_id
  → adapter HTTPS POST /v1/batches
  → 2xx: drop queued events
  → Transient failure: keep queue, backoff retry, same batch_id
  → Permanent failure: drop batch, increment diagnostic, no retry-loop

POST /v1/batches + X-Project-Key
  → reject invalid key (401/403), oversized (413), rate limit (429), malformed (400)
  → duplicate batch_id: 202, no new writes
  → persist valid raw events; skip invalid events; host stays up
  → upsert element totals for click with element_id; upsert 100×100 bins for click with position
  → store raw scroll; do not bin scroll
Analyst opens dashboard
  → GET elements / heatmap / screens with filters: platform, screen, layout version, app version, date range
  → counts without a still; overlay when a still exists for that screen + layout + platform

Checkout journey (all five POCs):
  session_start → screen_view(home) → click(product) → screen_view(checkout) → scroll
  → click(checkout.coupon) → click(checkout.pay) → screen_view(confirmation)
```

### State

| State | Location | Why |
| ----- | -------- | --- |
| anonymous_id | Platform store via adapter | Stable per install; survives identify/sign-out |
| user_id | Core, persisted | Optional; removed on sign-out only |
| session_id, last_event_at, sample_in | Core, persisted | Background keeps session; 30m timeout; sample bit fixed |
| screen id + layout_version | Core memory, persisted | Inherited until screen() changes; unspecified-layout bucket |
| tracking_enabled | Core | Init starts capture; app can disable without kill switch |
| exclude/mask lists | Core config | Evaluated before persist |
| event queue | Persistent store, dual cap | Offline and process-death; drop oldest |
| in-flight batch_id | Queue metadata | Idempotent retry |
| raw events, totals, bins, stills | Backend SQLite | Query + rebuild; stills optional |
| window_id | On event | Required for macOS/Windows multi-window |

---

## Interfaces

**API:**

Public SDK (same mental model; names follow platform convention). No public `track` / custom event API in V1.

```
init(config)
identify(user_id)
signOut()
screen(screen_id, layout_version?)
setTrackingEnabled(enabled)
flush()
```

`config`: `project_key` (required), `endpoint` (required, HTTPS ingest base), `session_timeout_ms` (default 1800000), `sample_rate` (default 1.0), `exclude_elements`, `exclude_screens`, `mask_elements` (default empty).

Ingest:

```
POST /v1/batches
Headers: X-Project-Key, Content-Type: application/json
Body: { "batch_id": "<client uuid>", "events": [ Event, ... ] }
Limits: ≤ 200 events, ≤ 262144 bytes; ≈ 120 batches / minute / project key (POC)
202 { batch_id, accepted_count, rejected_count }  success or duplicate batch_id
400 malformed or unsupported schema_version
401/403 invalid project key (SDK: permanent, no retry-loop)
413 oversize (SDK: permanent for that batch)
429 rate limit (SDK: retry, honor Retry-After)
5xx retryable
```

Query (POC, no auth product surface):

```
GET /v1/elements?platform&screen&layout_version&app_version&from&to
GET /v1/heatmap?platform&screen&layout_version&app_version&from&to
GET /v1/screens
GET /v1/stills?screen&layout_version&platform
POST /v1/stills
```

TLS required for ingest. Batch wrapper: `{ "batch_id": string, "events": Event[] }`. Flatter `{ type, screen, element_id, x, y }` in plan.md is a contract-test projection, not a second wire schema.

**Types:**

Canonical Event:
```json
{
  "schema_version": 1,
  "session_id": "string",
  "anonymous_id": "string",
  "user_id": "string | omitted",
  "timestamp": "number",
  "window_id": "string | omitted",
  "sdk": { "name": "string", "version": "string" },
  "platform": {
    "type": "web | ios | android | macos | windows",
    "os_version": "string",
    "app_version": "string"
  },
  "screen": {
    "id": "string",
    "layout_version": "string | omitted"
  },
  "event": {
    "type": "session_start | session_end | screen_view | click | scroll",
    "element_id": "string | omitted"
  },
  "position": { "x": "number", "y": "number" } | "omitted",
  "viewport": { "width": "number", "height": "number" } | "omitted"
}
```

**Component I/O:**

| @Input/@Output | Name | Type | Purpose |
| -------------- | ---- | ---- | ------- |
| Input | platform filter | enum | Restrict totals and overlay |
| Input | screen filter | string | Required for overlay |
| Input | version filter | layout/app version | Prevent layout mixing |
| Input | date range | from/to | Slice counts and overlay |
| Input | still | image | Optional overlay background |
| Output | element totals | table | Element_id counts per platform |
| Output | unique sessions | number | Selected slice sessions |
| Output | top elements | list | Identified element click totals |
| Output | density overlay | canvas | Click bins for slice |
| Output | platform breakdown | counts | Comparative checkout metrics |

---

## Changes

| Action | File | Change | AC |
| ------ | ---- | ------ | -- |
| Add | `schemas/event-v1.json` | Nested event JSON Schema | AC-contract, AC-coordinates |
| Add | `schemas/batch-v1.json` | Batch wrapper schema | AC-offline, AC-fail-closed |
| Add | `test/fixtures/*.json` | Golden events per type/platform | AC-contract, AC-journey |
| Add | `core/heatmap-core/` | Session, identity, privacy, sampling, queue, batch, retry, serialize | AC-init-capture to AC-buffer-full |
| Add | `core/heatmap-wasm/` | WebAssembly bindings | AC-install (web) |
| Add | `core/heatmap-ffi/` | UniFFI + C ABI | AC-install (native) |
| Add | `sdk/web/` | DOM capture, data-heatmap-id, NPM package | AC-platform-ids, AC-install |
| Add | `sdk/apple/shared/` | Shared Swift API | AC-anonymous-id, AC-user-id, AC-screen |
| Add | `sdk/apple/ios/` | UIKit capture, accessibilityIdentifier | AC-platform-ids, AC-session-background |
| Add | `sdk/apple/macos/` | AppKit capture, id lookup, multi-window | AC-platform-ids, multi-window |
| Add | `sdk/android/` | View + Compose adapters, AAR | AC-android-surfaces, AC-platform-ids |
| Add | `sdk/windows/` | WPF capture, AutomationId, NuGet | AC-platform-ids, multi-window |
| Add | `poc/react…poc/windows` | Shared checkout journey | AC-journey |
| Add | `poc/stills/` | Optional reference stills | AC-still-optional |
| Add | `backend/` | Rust ingest/query + SQLite | AC-offline, AC-dashboard-*, AC-fail-closed |
| Add | `dashboard/` | React SPA overlay + filters | AC-dashboard-*, AC-filters, AC-no-scroll-viz |
| Add | `test/contract/` | Cross-SDK semantic equivalence | AC-contract, AC-click-tap |
| Add | `test/e2e/` | Per-platform + acceptance | AC-journey |
| Add | `docs/` | Integrator docs + schema | AC-docs |
| Add | CI workflows | Format, UT, contract, builds, E2E | NFR performance/privacy/security |

---

## Edge Cases

| Scenario | User Sees | Implementation |
| -------- | --------- | -------------- |
| Loading | Filters usable; pending data state | Query in flight; distinguish empty vs error |
| Empty | Zero counts, no overlay | Valid empty aggregates |
| Error | Visible failure; no crash | Surface query error; still optional |
| No screen | Captured events; omitted from filtered views | `screen` omitted; query filters exclude |
| Unmarked control | Coordinate heatmap; no total row | Stored with position, no `element_id` |
| Known secret | No point and no total | Adapter drops before core; no text captured |
| Excluded app control | Not captured | Core drops based on exclude lists |
| Masked control | Coordinate only | Core strips `element_id` |
| Tracking disabled | No capture; host stays up | Core rejects candidates until enabled |
| Buffer full | Host stays up; newest kept | Drop oldest at limit |
| Sign-out after identify | Later events keep anonymous_id, lose user_id | Core clears user_id only |
| Background then resume before timeout | Same session | Core does not rotate on background |
| Inactivity timeout or expired session | New session on next interaction | Default 30 minutes from last captured event |
| Resize, rotation, DPI / scale change | New points sit on the new window | Adapter recomputes viewport; normalize 0–1 |
| Multiple windows (macOS, Windows) | Interaction belongs to that window | Opaque `window_id` + that viewport; V1 overlay does not split by window |
| Invalid project key | Host stays up; no retry loop | Permanent failure; diagnostic increment |
| Sampled-out session | Nothing from that session in the dashboard | Decision at session start; enqueue nothing |
| Missing reference still | Counts available; no overlay image | Tables render; canvas waits |
| Different layout versions | Separate overlays | Aggregation key includes layout_version |
| Process death with pending events | Events upload after relaunch when persisted | Persistent queue; relaunch must not crash |
| Android Compose unmarked composable | Coordinates unless secret; no total | Compose adapter; no generated path identity |
| Host ingest down | Host app remains usable | All SDK paths fail closed; no UI-thread block |
| Network loss | Host stays up; delayed upload | Persistent queue; retry with backoff |

---

## Tests

### Scenarios

| Component/Service | Scenarios |
| ----------------- | --------- |
| heatmap-core session | Init creation; background persistence; timeout/expiry rotation; configurable timeout |
| heatmap-core identity | Stable anonymous_id; user_id behavior on identify/sign-out |
| heatmap-core events | Accepted types only; click/scroll variants; position rules |
| heatmap-core privacy | Secret/exclude/mask behavior; no text leak |
| heatmap-core sampling | Start-of-session fixed decision; persist decision; sampled-out enqueue |
| heatmap-core queue | FIFO; dual-cap overflow; persist/restore |
| Contract (all SDKs) | Pay activation equivalence across five platforms; nested envelope fixtures |
| Adapters | Platform-specific ID discovery; secret dropping; lifecycle integration; WASM fail-closed |
| Ingest/Backend | Auth; idempotency; partial invalid; limits; 429; aggregation correctness |
| Dashboard | Multi-platform breakdown; overlay filters; empty/missing still states; keyboard-reachable |
| E2E | Cross-platform shared checkout journey sequence |

### Testing Approach

| Decision | Why | Source |
| -------- | --- | ------ |
| Rust core unit + property tests | Shared logic (session, privacy, coordinates, sampling); fixes multiply efficiency | Task NFRs, Plan module list |
| Golden nested-envelope fixtures | Locks nested model; flatness is projection only | Task constraints |
| Platform-specific adapter units | Identifiers and capture differ significantly | AC-platform-ids |
| Semantic contract tests | Click/tap equivalence and comparison | AC-click-tap, AC-contract |
| Privacy CI gate | Mandatory check; text/values must never leak | AC-no-text, AC-secrets, NFR security |
| Per-platform + shared E2E | POC is incomplete if any consumer cannot finish journey | AC-journey |

---

## Validation

No product toolchain exists on disk today. Commands below are not runnable until the matching manifests exist. Execution-plan must introduce each channel with the first code for that surface.

- **Rust core test**: `cargo test -p heatmap-core`
- **Rust backend test**: `cargo test` in `backend`
- **Web SDK test**: package test script in `sdk/web`
- **Contract**: test runner over `test/contract` against `test/fixtures`
- **Privacy gate**: secret fixtures must never appear in payloads, logs, or queue files
- **Apple / Android / Windows**: `XCTest`, `Gradle test`, `dotnet test` once packages exist
- **E2E**: platform runners in `test/e2e` against ingest + dashboard
- **Harness hooks**: `.cursor/hooks.json` session hooks only; not a product validation channel

---

## Open Questions

| Question | Owner | Status |
| -------- | ----- | ------ |
| None | — | closed 2026-09-19 |

---

## Implementation Notes

### Note 1

- **What changed:** Init accepts `http://127.0.0.1` and `http://localhost` ingest bases (optional port/path) in addition to `https://`. Remote `http://` endpoints stay rejected.
- **Why:** The React POC / Playwright E2E posts batches to a local axum process. Terminating TLS on every developer machine was not practical for that path.
- **Supersedes:** “TLS required for ingest” and “endpoint must be HTTPS” as a universal init rule. Remote production ingest remains HTTPS-only. Adapters still POST `{base}/v1/batches`.

### Note 2

- **What changed:** WASM init sets `defer_network_flush`. Enqueue, `poll_flush`, and background no longer call `BatchTransport` (sync XHR). Hosts upload with `beginFlushBatchJson` / `completeFlush` over async `fetch`, on a 10s timer and after capture.
- **Why:** Cross-origin XHR is blocked; sync XHR on the UI thread violates the performance NFR. Shared `BatchTransport` stays synchronous for native.
- **Supersedes:** Browser auto-flush and `pollFlush` delivering batches through `FetchBatchTransport` on the main thread.
