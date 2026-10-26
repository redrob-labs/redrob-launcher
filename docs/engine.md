# The shared Redrob Code engine

Every Redrob app on a computer runs its AI through one Redrob Code daemon. The engine holds the
Redrob key, picks the model provider behind each model id, masks personal details before a request
leaves the machine, and counts which apps are attached. The design of record is the engine's own
`docs/LOCAL-ENGINE-API.md`, section 3 ("One daemon, not one per product").

## What the launcher does

The launcher is where a person sees that daemon. The sidebar's **AI engine** row (and the tray's
**AI engine** item) opens the AI engine page with the state below. The row itself shows a dot and
a short word: Ready, In use, Busy, Standby (stopped, its normal resting state: the first app that
needs it starts it) or Not installed. The user-facing name is "AI engine" on purpose: the people
using Redrob apps need to know what it is for, not that it is Redrob Code. The page:

| State | Shown | Buttons |
| --- | --- | --- |
| No `redrob` binary | Not installed | Install Redrob Code (the suite install, same as the app card) |
| Binary, no daemon answering | Not running | Start |
| Daemon up, apps attached | Running · version · url · the apps, by name | Stop, Restart |
| Daemon up, nobody attached | Running · "stops on its own in about N min" | Stop, Restart |

Start, stop and restart go through the engine's own `redrob service start --json`, `redrob
service stop`; the launcher never signals a pid itself. Apps reconnect on their own after a stop or
restart: each re-reads the daemon's record when a request fails.

## Keeping it current

The engine is updated by the launcher, not by the apps: each app only bundles a fallback sidecar
for a machine with no engine at all, and the launcher owns the installed copy every app shares.
The release check that finds a newer Redrob Code release drives `ui-egui/src/engine_update.rs`:

| Daemon | What happens |
| --- | --- |
| not running | the new version is installed right away |
| running, nobody attached | `service stop`, install, `service start` — the daemon comes back on the new binary |
| running, apps attached | nothing yet; the AI engine page shows **Update available**, which apps hold it back, and **Update now** |

**Update now** stops the daemon even with apps attached (they reconnect on their own, as after
any restart). A version that failed to install is not retried on its own; **Update now** retries
it. The setting *Keep the Redrob Code engine up to date on its own* (on by default) turns the
automatic part off; the page's **Update now** still works with it off. Swapping the binary under
a running daemon is never done: on Windows the running exe is locked, and on every platform the
daemon would keep serving the old version until something restarted it.

## Using this PC from the web and the phone

Redrob Chat on the web and the phone app drive this daemon through the desk relay (the desk relay
v2 contract, `docs/desk-relay.md` in redrob-mobile). The daemon holds the relay connection, the
sign-in and the trusted devices; the launcher keeps it running and is where a person signs in and
allows a device once (`crates/native/src/desk.rs`, `crates/ui-egui/src/desk.rs`).

- At launcher start with Redrob Code installed, on `redrob://open/code` and on the Code card's
  Open: `redrob service start --json --idle-exit 0`, then `POST /desk/enable {"enabled":true}`.
- While the daemon runs: `GET /desk/status` every 3 s.
- Not signed in: the Code card's line says *Use this PC from the web and phone · Sign in*; it and
  the AI engine page's **Sign in** send `POST /desk/login` and open `verificationUriComplete`
  (https only) in the default browser.
- A pending device: one OS notification, the window comes forward, and a prompt shows its name,
  kind and 6-digit code with **Allow** (`POST /desk/pending/<code>/approve`) and **Deny**
  (`.../deny`). The device shows the same code.
- The AI engine page lists the connected devices (name, kind, date) with **Disconnect**
  (`POST /desk/devices/<id>/revoke`), and **Sign out** (`POST /desk/logout`).

A daemon with no `/desk` routes (an older Redrob Code) answers 404; the page then says to update
Redrob Code. Codes (six digits) and device ids (`[A-Za-z0-9_-]`, at most 128) are checked before
they go into a path.

## The record

`$HOME/.redrob/run/server.json` (mode 0600; `REDROB_RUN_DIR` overrides the folder):

```json
{ "id": "…", "version": "0.6.0", "url": "http://127.0.0.1:4096", "port": 4096, "pid": 8412, "secret": "…" }
```

`secret` is the Basic password (username `redrob`), drawn fresh on every daemon start. It is the
auth boundary: the file is per-user, and nothing in the launcher logs or displays it.

## `redrob-engine-client`

The crate lives in the engine's repository, at `crates/engine-client` of
[redrob-code](https://github.com/redrob-labs/redrob-code), and is pinned here by engine tag
(`tag = "v0.6.3"` in the workspace `Cargo.toml`) so the launcher and the engine it drives agree on
the record format. It is the Rust half of that contract, for every Rust Redrob app (Shape, Canvas)
as well as this launcher:

- `read_record()`, `current()` (the record, if its daemon answers `/api/health`)
- `clients(&record)` — who is attached, how long idle, when the daemon exits
- `connect_or_start(idle_exit_minutes)` — the daemon, found or started through `redrob service start --json`
- `stop()` — through `redrob service stop`
- `authorization(&secret)`, `openai_base(&record)` — what an app needs to call
  `<url>/v1/chat/completions` with the engine's OpenAI-compatible route
- `redrob_bin()`, `installed()` — the engine's binary, by its resolution order
  (`REDROB_CODE_BIN`, `~/.redrob/bin/redrob`, PATH)

The crate speaks plain HTTP to `127.0.0.1` only and depends on nothing but serde, ureq and dirs.
The JavaScript twin is `@redrob-labs/sdk/daemon` in redrob-code.
