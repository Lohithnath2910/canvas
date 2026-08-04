# CANVAS — Master Execution Blueprint
**A native, offline-first, multiplayer desktop widget platform**
Version 1.0 — Prepared for autonomous execution by Gemini 3.1 Pro (High) inside Antigravity IDE

---

## 0. How To Use This Document (read this first, agent)

1. This is a **single-pass, sequential build spec**. Build the milestones in Section 9 **in order**. Do not skip ahead to a later milestone until the current one passes its Definition of Done (DoD) checklist.
2. Section 2 is a **closed decision set** — the tech stack is final, not a menu. Do not substitute alternatives (e.g. do not swap in Electron, Tauri, Automerge, Diesel, etc.) without flagging it to the human first.
3. Section 8 is a **risk register** — read it before starting M2 (window management) and M4 (sync), the two hardest subsystems. It exists so you don't silently attempt something structurally impossible and burn a milestone finding out.
4. Every milestone in Section 9 ends with a DoD checklist. Treat these as hard gates: if an item can't be checked truthfully, the milestone is not done.
5. Section 13 lists every assumption made while writing this doc that the human has not explicitly confirmed. If any of these are wrong for the human's actual intent, stop and ask before building on top of them.
6. Section 14 is a ready-to-paste system prompt for the agent session itself.

---

## 1. Product Summary

**Canvas** turns the desktop itself into a collaborative workspace. Instead of opening an app to check notes/todos/docs, small native always-on-top widgets (sticky notes, todo lists, shared docs, whiteboards, kanban boards, calendars) live directly on the desktop, persist locally, and sync in real time with collaborators when online.

**Non-goals (explicit exclusions — do not implement these):**
- No Electron, no Chromium, no embedded webview of any kind, anywhere in the stack.
- No requirement for network connectivity to create, edit, or view any widget.
- No "Save" button or explicit save action anywhere in the UI — all persistence is automatic.
- No last-write-wins conflict resolution — all multi-user state uses CRDTs.

**Hard resource budgets (apply to every build from M1 onward, verified in CI — see Section 11):**

| Component | Budget |
|---|---|
| Single widget (idle, in memory) | 5–15 MB |
| Full application (idle, no widgets open) | 20–40 MB |
| Background sync service | < 10 MB |
| Application total (typical use, several widgets) | < 50 MB |
| Cold launch to first widget visible | < 300ms |

---

## 2. Committed Technology Stack (final — do not deviate)

| Layer | Choice | Notes |
|---|---|---|
| Desktop UI framework | **Rust + Slint** | Native rendering, no browser runtime |
| Local persistence | **SQLite via `rusqlite`** | Synchronous, runs on UI thread; fine because ops are small |
| CRDT engine | **`yrs`** (Rust port of Yjs) | Text, Map, Array types cover all widget types below |
| App-side networking | **`tokio`** on a dedicated background thread | Bridged to Slint's event loop via channels + `invoke_from_event_loop` |
| WebSocket client/server | **`tokio-tungstenite`** (client) / **Axum's built-in WS** (server) | |
| Backend framework | **Axum** (Rust) | |
| Backend database | **PostgreSQL via `sqlx`** (async, compile-time checked queries) | |
| Cache / pub-sub (multi-instance presence fan-out) | **Redis** | |
| Object storage | **S3-compatible** — **MinIO** for local/dev, real S3 (or compatible) in prod | |
| Auth | **JWT** (`jsonwebtoken` crate) + **OAuth2 Authorization Code flow** | Google + GitHub in MVP; Microsoft + Apple deferred to Phase 4 |
| Local dev infra | **Docker Compose** (Postgres, Redis, MinIO, backend) | |
| Orchestration (later) | Kubernetes — **Phase 4 only, not before** | |
| CI / cross-platform builds | **GitHub Actions matrix**: `windows-latest`, `macos-latest`, `ubuntu-latest` | Do **not** attempt to cross-compile the GUI binary from one host — native windowing APIs make this unreliable. Build natively on each OS runner. |
| Packaging | `cargo-packager` (installers), platform code signing in M8 | |

---

## 3. Repository Structure (single Cargo workspace)

```
canvas/
├── Cargo.toml                 # workspace root
├── crates/
│   ├── canvas-app/            # Slint desktop entrypoint, widget manager, window manager
│   ├── canvas-ui/              # Slint components, theme engine, animation engine
│   ├── canvas-doc/              # Document model — thin wrapper around yrs per widget type
│   ├── canvas-storage/          # SQLite local persistence (rusqlite)
│   ├── canvas-sync/              # Sync engine: op queue, WS client, reconciliation
│   ├── canvas-protocol/          # Shared wire-protocol types (serde), used by app AND server
│   ├── canvas-server/            # Axum backend: REST + WS gateway
│   └── canvas-auth/              # JWT + OAuth logic (server-side)
├── migrations/                 # sqlx-cli Postgres migrations
├── docker/
│   └── docker-compose.yml      # postgres, redis, minio, canvas-server
├── .github/workflows/
│   ├── ci.yml                  # test + lint, all 3 OS
│   └── release.yml             # build installers, all 3 OS
└── docs/                       # architecture notes, ADRs
```

`canvas-protocol` is the seam that keeps client and server in sync — never duplicate message types between `canvas-app` and `canvas-server`; both depend on this crate.

---

## 4. Data & Document Model

Every widget is backed by exactly one `yrs::Doc`. Widget type determines the CRDT shape used inside it:

| Widget | yrs structure |
|---|---|
| Sticky Note | `YText` (rich text as delta-formatted spans) |
| Todo | `YArray<YMap>` — each item: `{id, text: YText, done: bool, due: i64?, assignee: String?, children: YArray}` |
| Shared Document | `YText` + `YMap` for metadata; cursor positions via yrs **relative positions** (not absolute offsets — survives concurrent edits) |
| Whiteboard | `YMap<shape_id, YMap>` for shapes (`type,x,y,w,h,rotation,style`) + `YArray` of point-lists for freehand strokes |
| Calendar | `YArray<YMap>` — events: `{id, title, start, end, attendees}` |
| Kanban | Cards use **fractional indexing** (a sortable string/float position field), not nested-array position — nested array reordering under concurrent edits is a known CRDT footgun. Columns: `YArray<YMap{id,title}>`. Cards: `YMap<card_id, YMap{column_id, position, title, ...}>` |
| Code Snippet | `YText` + `YMap` for language/metadata |
| Dashboard | `YMap` of widget-defined key/value bindings (read-mostly, low sync priority) |

**Canonical operations** (all state changes reduce to yrs updates, never whole-document replacement): insert/delete text range, set map key, insert/delete array item, move (via fractional reindex, never in-place array move).

---

## 5. Local Storage Schema (SQLite, `canvas-storage`)

```sql
CREATE TABLE workspaces (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    is_local_only INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL
);

CREATE TABLE widgets (
    id TEXT PRIMARY KEY,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id),
    widget_type TEXT NOT NULL,       -- 'sticky_note' | 'todo' | 'doc' | 'whiteboard' | 'calendar' | 'kanban' | 'code' | 'dashboard'
    title TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE documents (
    widget_id TEXT PRIMARY KEY REFERENCES widgets(id),
    yrs_snapshot BLOB NOT NULL,      -- yrs::Doc encoded state (encode_state_as_update)
    yrs_state_vector BLOB NOT NULL,  -- for diffing against remote peers
    schema_version INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE window_positions (
    widget_id TEXT PRIMARY KEY REFERENCES widgets(id),
    x INTEGER, y INTEGER, width INTEGER, height INTEGER,
    monitor_id TEXT,
    z_order INTEGER
);

CREATE TABLE revision_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    widget_id TEXT NOT NULL REFERENCES widgets(id),
    yrs_update BLOB NOT NULL,        -- individual update, for undo / audit, GC'd periodically
    created_at INTEGER NOT NULL
);

CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE assets_cache (hash TEXT PRIMARY KEY, local_path TEXT NOT NULL, size_bytes INTEGER, last_accessed INTEGER);
```

Periodically compact `documents.yrs_snapshot` (squash `revision_history` into the snapshot and prune) to prevent unbounded growth — required before M4 ships, not optional.

---

## 6. Sync Protocol (`canvas-protocol` + `canvas-sync`)

WebSocket connection: one per open workspace, endpoint `wss://.../ws/{workspace_id}`.

**Message envelope** (JSON control frame; large payloads use a binary companion frame referenced by ID):

```rust
enum ClientMessage {
    Auth { token: String },
    Subscribe { widget_id: String, state_vector: Vec<u8> },
    Op { widget_id: String, update: Vec<u8> },      // raw yrs update bytes
    Presence { widget_id: String, cursor: Option<CursorPos> },
    Ping,
}

enum ServerMessage {
    Authed { user_id: String },
    Snapshot { widget_id: String, update: Vec<u8> }, // diff since client's state_vector
    Op { widget_id: String, from_user: String, update: Vec<u8> },
    Presence { widget_id: String, user_id: String, cursor: Option<CursorPos> },
    Ack { op_id: String },
    Error { code: String, message: String },
    Pong,
}
```

**Flow:**
1. Local edit → applied to local `yrs::Doc` immediately → UI updates instantly (no network wait) → SQLite write.
2. Update bytes pushed onto an outbound queue in `canvas-sync`.
3. If online: send `Op`. If offline: queue persists in `revision_history`; flushed in order on reconnect.
4. Server broadcasts `Op` to all other subscribed clients (via Redis pub/sub if server is horizontally scaled).
5. Remote update applied to local `yrs::Doc` — yrs merges automatically, no manual conflict resolution code needed.
6. On reconnect after being offline: client sends `Subscribe` with its state vector; server replies with a `Snapshot` diff, not the full document.

---

## 7. Backend API (`canvas-server`)

**REST:**
- `POST /auth/oauth/:provider/start`, `GET /auth/oauth/:provider/callback`, `POST /auth/refresh`
- `GET/POST /workspaces`, `GET/PATCH/DELETE /workspaces/:id`
- `GET/POST /workspaces/:id/widgets`, `GET/DELETE /widgets/:id`
- `POST /widgets/:id/snapshot` (periodic full-state checkpoint from client)
- `POST /uploads/presign` → returns presigned S3/MinIO URL for attachments

**WebSocket:** `GET /ws/:workspace_id` (upgrade), authenticated via token in query param or initial `Auth` message.

**Permissions** enforced server-side on every write: `workspace_members(user_id, workspace_id, role)` where `role ∈ {owner, editor, commenter, viewer}`. Reject any `Op` from a `viewer`/`commenter` role at the gateway before it touches Redis/Postgres.

---

## 8. Risk Register — read before M2 and M4

| Risk | Why it's hard | Mitigation for this build |
|---|---|---|
| **True desktop-layer widget embedding** (living behind icons, like Rainmeter/wallpaper engine) | Deeply OS-specific: Win32 `Progman`/`WorkerW` injection on Windows, `NSWindow` level tricks on macOS, X11/Wayland compositor differences on Linux. Wayland in particular restricts this by design. | **Phase 1 approximation:** ship widgets as borderless, always-on-top native windows (`WS_EX_TOOLWINDOW`-style / floating panel) — not literally embedded in the desktop layer. True embedding is an explicit **per-OS stretch goal**, tackled only after M8, one OS at a time, starting with Windows (most tractable). |
| **yrs document growth** | Update log grows unboundedly without compaction | Mandatory snapshot compaction job, see Section 5 — build this in M4, not later |
| **Slint cross-platform rendering quirks** | Younger ecosystem than Qt; some platform-specific rendering bugs are expected | Budget explicit QA time per OS in M8; do not assume "works on Linux" implies "works on Windows/macOS" |
| **Relative-position cursors in concurrent rich text edits** | Naive absolute offsets break under concurrent inserts | Use yrs's built-in relative position API from the start in M5 — do not build shared-doc cursors on absolute indices even for the MVP |
| **Code signing / notarization** | macOS Gatekeeper and Windows SmartScreen both block unsigned installers | Budget this explicitly in M8; requires an Apple Developer account and a Windows code-signing certificate — **human action required**, not something the agent can complete alone |
| **End-to-end encryption vs. server-side collaboration** | E2EE is in tension with server-mediated presence/search; doing both correctly is a research-grade problem | Explicitly deferred to Phase 4/stretch. Do not attempt to bolt on E2EE during M3–M7. |

---

## 9. Milestone Roadmap (build strictly in this order)

### M0 — Bootstrap
Cargo workspace created per Section 3. Empty Slint window launches on all three OSes via CI matrix. Binary size and idle RSS measured and logged.
**DoD:** CI green on Windows + macOS + Linux · idle RSS < 40MB · cold launch < 300ms · no network code exists yet.

### M1 — Single Offline Sticky Note
One widget type (sticky note) fully working: create, edit rich text, close, reopen — persists via SQLite/yrs with zero network dependency.
**DoD:** Force-quit app mid-edit → relaunch → content is exactly as last edited · works with network disabled · widget RSS 5–15MB.

### M2 — Multi-Widget Desktop
Widget Manager: spawn multiple sticky notes, drag to reposition, positions persist across restarts (per Section 8 risk register — always-on-top approximation, not true desktop embedding). Theme engine + basic open/close animations.
**DoD:** 10 widgets open simultaneously stay within the 50MB app-wide budget · window positions survive restart and multi-monitor changes.

### M3 — Backend Skeleton
`canvas-server` running via Docker Compose (Postgres + Redis + MinIO). Email/password auth issuing JWTs. REST CRUD for workspaces/widgets. No realtime sync yet — this milestone is API plumbing only.
**DoD:** `docker compose up` brings up a working stack · integration tests cover auth + CRUD · Postgres migrations run cleanly from empty DB.

### M4 — Sync Engine
Full protocol from Section 6 implemented both sides. Two client instances editing the same sticky note converge correctly, online and after simulated offline edits on both sides. Snapshot compaction job running.
**DoD:** Automated multi-client convergence test passes (see Section 11) · offline-then-reconnect test passes · presence indicators show correctly · document size stays bounded over 1000 simulated edits.

### M5 — Todo, Shared Document, Calendar widgets
Adds three widget types on top of the now-proven document/sync pipeline. Shared Document includes realtime multi-cursor via yrs relative positions.
**DoD:** Each widget type has the same offline + convergence guarantees as M1/M4 · cursor positions remain correct under concurrent typing by two simulated clients.

### M6 — Whiteboard + Kanban widgets
Canvas-based drawing/shapes; Kanban with fractional-indexed drag-and-drop.
**DoD:** Concurrent drag-and-drop of the same card by two clients converges without duplication or loss · whiteboard strokes sync incrementally, not as full-canvas re-sends.

### M7 — OAuth, Permissions, Object Storage
Google + GitHub OAuth flows. Full role-based permission enforcement server-side. Image/attachment upload via presigned S3/MinIO URLs.
**DoD:** A `viewer`-role user's write attempts are rejected server-side even if the client is modified to send them · uploaded assets appear across devices.

### M8 — Packaging & Distribution
Installers for all three OSes via `cargo-packager`, auto-update mechanism, code signing (human-provided certificates — see Risk Register).
**DoD:** Fresh install → launch → sign in → widgets sync, on a clean VM for each OS.

### M9 — Stretch / Phase 3–4 (not required for MVP)
Plugin API, AI summaries, true per-OS desktop-layer embedding, E2E encryption, admin console, audit logs, Kubernetes deployment.

---

## 10. Definition of Done — general template

Apply this checklist shape to every milestone above, not just the specific bullets given:
- [ ] Builds and passes CI on all three target OSes
- [ ] Meets the resource budgets in Section 1 for anything currently in scope
- [ ] Has automated tests covering the offline case and (from M4 onward) the concurrent-edit case
- [ ] No new dependency introduced outside Section 2's stack without flagging it
- [ ] Manual smoke test performed on at least one real OS, not just CI

---

## 11. Testing Strategy

- **Unit tests** per crate, standard `cargo test`.
- **Convergence test harness** (`canvas-sync` integration tests, required from M4): spin up N in-process simulated clients sharing a `yrs::Doc` each, apply randomized concurrent edits, simulate offline gaps, assert all N converge to identical document state.
- **Resource budget checks in CI:** measure RSS after launch and after opening N widgets; fail the build if over the Section 1 budgets.
- **Manual OS-specific QA checklist** (multi-monitor behavior, DPI scaling, always-on-top interaction with OS features) — required before M8 sign-off.

---

## 12. Non-Negotiable Constraints (recap)

- No Electron / Chromium / any embedded browser, anywhere.
- Editing never blocks on network, at any milestone.
- All multi-user conflict resolution is CRDT-based — no last-write-wins code path anywhere.
- Resource budgets in Section 1 are enforced by CI, not aspirational.

---

## 13. Assumptions Made — review before sending to Gemini

These are implementation decisions made to close every ambiguity in the original vision doc. If any is wrong for your intent, edit this section before handing the file over:

1. **CRDT library = `yrs`**, not Automerge or a custom engine.
2. **Local storage = synchronous `rusqlite`**, with a separate tokio thread for networking — not a fully async local storage layer.
3. **"Desktop widget" = always-on-top floating window**, not literal desktop-layer embedding, for Phase 1 (see Risk Register).
4. **MVP widget set** for the first fully-synced release is Sticky Note → Todo → Shared Document → Calendar (M1–M5); Whiteboard/Kanban follow in M6; Code Snippet and Dashboard widgets are not scheduled until Phase 3.
5. **OAuth providers**: Google + GitHub only for MVP; Microsoft and Apple deferred to Phase 4.
6. **Infra**: Docker Compose for local/self-hosted dev; no managed cloud provider (AWS/GCP/Supabase/etc.) has been chosen — that decision is left open for you to make at deployment time.
7. **E2E encryption** is explicitly deferred, not part of MVP, per the Risk Register.
8. **Cross-platform builds** happen via CI matrix on real OS runners, not cross-compilation from a single host.

---

## 14. Prompt Preamble — paste this into Antigravity to start the session

```
You are building "Canvas" from the attached master blueprint (CANVAS_Master_Blueprint.md).

Rules:
1. Build milestones M0 through M8 strictly in order. Do not start a milestone
   until the previous one's Definition of Done checklist is fully satisfied.
2. The tech stack in Section 2 is final — do not substitute any component.
3. Before starting M2 and M4, re-read Section 8 (Risk Register) in full.
4. After completing each milestone, explicitly output the DoD checklist for
   that milestone with a pass/fail for each item before proceeding.
5. If you hit a decision not covered by this document, stop and ask rather
   than guessing — do not silently deviate from the spec.
6. Section 13 lists assumptions the human may not have reviewed yet — if your
   implementation depends heavily on one of these being correct, surface it.
```