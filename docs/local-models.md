# Local models: what the launcher puts on the computer, and how an app uses it

The launcher is the one place a person downloads models and starts a local model server. The
apps never download weights or start runtimes themselves; they read one file and, when it names
nothing for the task they need, they use the Redrob Console as before. Nothing in this contract
is required for an app to work.

## The catalogue

`assets/models/models.json` (version 2). One entry per model, pinned by commit or tag URL,
SHA-256 and byte size for every file:

| Field | Meaning |
| --- | --- |
| `id` | the model's folder name (`qwen3.8-27b-q4`) |
| `app` | the app that reads it (`redrob-film`), or `redrob` for a model every app may use |
| `task` | what it does: `chat`, `code`, `vision`, `embedding`, `text-to-image`, `image-edit`, `inpaint`, `background-removal`, `upscale`, `segmentation`, `vectorize`, `stt`, `tts`, `vad`, `stem-separation`, `image-to-3d`, `ocr` |
| `runtime` | `files` (the app loads the files itself), `llama-server` (the launcher serves it), `llama-server-prism` (served by PrismML's llama.cpp fork; shown only when the person turns that setting on) |
| `requires` | `memory_gb` and `backends` (`cuda`, `metal`, `vulkan`, `cpu`): what the computer needs |
| `serve` | for served models: the GGUF file, and the vision projector if there is one |
| `files` | `name`, `url`, `sha256`, `size` |

The runtimes themselves are pinned the same way in `assets/models/runtimes.json`: one llama.cpp
build per platform and backend, downloaded on the first start of a served model.

The launcher measures the computer (GPU and its memory, or a Mac's unified memory; RAM; NPU) and
shows one verdict per model: **Runs here**, **Runs slowly** (CPU, or spilling into RAM), **Won't
run here**, or **Not measured**. The verdict never blocks a download.

## Where the files go

| `app` | Folder |
| --- | --- |
| `redrob-film` | Redrob Film's own models folder (`<data dir>/models`), as before |
| `redrob`, `redrob-canvas`, `redrob-shape`, `redrob-code` | `~/.redrob/models/<id>/` (`REDROB_MODELS_DIR` overrides the folder) |

A file is in place only once its SHA-256 matched; a half-downloaded file has a `.part` suffix
and is never something an app should open.

## The registry an app reads

`~/.redrob/models/installed.json` (same `REDROB_MODELS_DIR` override). Rewritten whole, atomically,
after every download, delete, start and stop:

```json
{
  "version": 1,
  "backend": "cuda",
  "models": [
    { "id": "lama", "app": "redrob-canvas", "task": "inpaint", "runtime": "files",
      "path": "/home/me/.redrob/models/lama", "files": ["big-lama.pt"] },
    { "id": "qwen3.8-27b-q4", "app": "redrob", "task": "chat", "runtime": "llama-server",
      "path": "/home/me/.redrob/models/qwen3.8-27b-q4", "files": ["Qwen3.8-27B-Q4_K_M.gguf"],
      "served": { "url": "http://127.0.0.1:4100", "port": 4100, "pid": 8412 } }
  ]
}
```

- `backend` is what this computer uses; an app that has a CUDA and a CPU code path picks by it.
- Only models whose every file is present are listed.
- `served` is present only while the launcher's server for that model is up. It is an
  OpenAI-compatible endpoint on loopback with no auth: `/v1/chat/completions`, `/v1/models`,
  `/v1/embeddings`. The `model` to send is the entry's `id`.

The rule for an app, in one line: **look up the task; if there is an entry, use its `path` (or
its `served.url`); if there is none, call the Console.** Re-read the file when a request fails:
the person may have deleted the model or stopped the server since.

## Redrob Code

A served model is also written into the engine's config (`$XDG_CONFIG_HOME/redrob/redrob.json`,
`REDROB_CONFIG_DIR` overrides) as a provider the engine already knows how to accept: a local URL
on `@ai-sdk/openai-compatible`, id `redrob-local-<model id>`, removed again when the server
stops. Everything else in that file is left untouched; a file that is not valid JSON is not
touched at all. An app that talks to the shared engine therefore gets the local model as
`redrob-local-<id>/<id>` with no change of its own.

## Why one folder and one file

Each app once had its own models folder and its own download code, so the same weights were
fetched twice and verified differently. One folder, one pinned catalogue and one registry mean
one download, one checksum, and one place a person can see what is on their disk and delete it.
