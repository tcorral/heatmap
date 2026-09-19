# Event and batch schema (V1)

Integrators send heatmap data as JSON events grouped into batches. Machine-readable definitions live in [`schemas/event-v1.json`](../schemas/event-v1.json) and [`schemas/batch-v1.json`](../schemas/batch-v1.json). Golden examples are under [`test/fixtures/`](../test/fixtures/).

## Event envelope

Each captured interaction is one JSON object with a fixed set of required fields and several optional fields. **Optional fields are omitted when they do not apply** — do not send `null` for absent values.

| Field | Required | Notes |
| ----- | -------- | ----- |
| `schema_version` | yes | Must be `1`. |
| `session_id` | yes | Opaque session identifier. |
| `anonymous_id` | yes | Persistent anonymous user id. |
| `user_id` | no | Present after `identify`; omit after sign-out or when anonymous. |
| `timestamp` | yes | Event time (milliseconds since epoch). |
| `window_id` | no | Opaque window identifier (common on desktop). |
| `sdk` | yes | `{ "name", "version" }` of the integrating SDK build. |
| `platform` | yes | `{ "type", "os_version", "app_version" }`. |
| `screen` | no | `{ "id", "layout_version?" }` for the current screen; omit entire object when unknown. |
| `event` | yes | `{ "type", "element_id?" }` — see interaction types below. |
| `position` | no | Window-normalized `{ "x", "y" }`; see coordinates. |
| `viewport` | no | `{ "width", "height" }` in CSS/logical pixels of the window used for `position`. |

The event root may include **additional properties** not listed here. Ingest ignores unknown top-level fields so clients can add forward-compatible metadata. Nested objects (`sdk`, `platform`, `screen`, `event`, `position`, `viewport`) do not allow extra keys.

### Platform `type`

One of: `web`, `ios`, `android`, `macos`, `windows`.

### Interaction `event.type`

V1 allows exactly:

| `event.type` | `element_id` | Typical use |
| ------------ | ------------ | ----------- |
| `session_start` | omit | Session opened. |
| `session_end` | omit | Session closed or timed out. |
| `screen_view` | omit | User navigated to a screen. |
| `click` | optional | Mouse or finger activation (same wire type). |
| `scroll` | omit | Scroll on the current screen. |

There is no separate `tap` type. Touch activations use `click`.

When `element_id` is set (for example `checkout.pay`), the activation counts toward element totals. When it is omitted, the event may still contribute to the coordinate heatmap for that screen.

Types such as `custom`, `hover`, `visibility`, and in-control coordinate fields are **not** part of V1.

### Coordinates (`position`)

When present, `position.x` and `position.y` are fractions of the **visible window**, origin top-left, each in **0.0–1.0 inclusive**. After resize or rotation, new events use the new window geometry.

The schema allows only `x` and `y` on `position`. Do not send control-local coordinates (for example `element_x` / `element_y`).

## Batch wrapper

Upload body shape:

```json
{
  "batch_id": "<client-generated id>",
  "events": [ /* Event objects */ ]
}
```

`batch_id` must be stable when retrying the same upload. The wrapper object has no fields other than `batch_id` and `events`.

## Validate fixtures locally

From the repository root:

```bash
cd test && npm install && npm run validate-fixtures
```

This checks every JSON file in `test/fixtures/` against the event or batch schema.

## Related reading

Product behaviour (privacy, flush limits, dashboard) is described in the solution design and task spec under `docs/specs/cross-platform-heatmap-sdk/`. This document is the integrator-facing contract for the wire format only.
