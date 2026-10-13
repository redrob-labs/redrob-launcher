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

## The record

`$HOME/.redrob/run/server.json` (mode 0600; `REDROB_RUN_DIR` overrides the folder):

```json
{ "id": "…", "version": "0.6.0", "url": "http://127.0.0.1:4096", "port": 4096, "pid": 8412, "secret": "…" }
```

`secret` is the Basic password (username `redrob`), drawn fresh on every daemon start. It is the
auth boundary: the file is per-user, and nothing in the launcher logs or displays it.

## `redrob-engine-client`

`crates/engine-client` is the Rust half of that contract, for every Rust Redrob app (Shape, Canvas,
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
