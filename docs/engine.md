# The shared Redrob Code engine

Every Redrob app on a computer runs its AI through one Redrob Code daemon. The engine holds the
Redrob key, picks the model provider behind each model id, masks personal details before a request
leaves the machine, and counts which apps are attached. The design of record is the engine's own
`docs/LOCAL-ENGINE-API.md`, section 3 ("One daemon, not one per product").

## What the launcher does

The launcher is where a person sees that daemon. The header's **Engine** link (and the tray's
**Redrob Code engine** item) opens a dialog with:

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
| running, nobody attached | `service stop`, install, `service start` -- the daemon comes back on the new binary |
| running, apps attached | nothing yet; the dialog shows **Update available**, which apps hold it back, and **Update now** |

**Update now** stops the daemon even with apps attached (they reconnect on their own, as after
any restart). A version that failed to install is not retried on its own; **Update now** retries
it. The setting *Keep the Redrob Code engine up to date on its own* (on by default) turns the
automatic part off; the dialog's **Update now** still works with it off. Swapping the binary under
a running daemon is never done: on Windows the running exe is locked, and on every platform the
daemon would keep serving the old version until something restarted it.

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
the record format. It is the Rust half of that contract, for every Rust Redrob app (Shape, Canvas,
Query, Recall) as well as this launcher:

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
