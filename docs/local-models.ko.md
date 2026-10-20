# 로컬 모델: 런처가 컴퓨터에 두는 것과 앱이 쓰는 법

모델을 내려받고 로컬 모델 서버를 켜는 곳은 런처 하나입니다. 앱은 가중치를 직접 받거나 런타임을
직접 띄우지 않습니다. 파일 하나를 읽고, 필요한 작업에 해당하는 항목이 없으면 지금처럼 Redrob
Console을 씁니다. 이 계약의 어떤 것도 앱이 동작하는 데 필수는 아닙니다.

## 카탈로그

`assets/models/models.json` (version 2). 모델마다 항목 하나, 파일마다 커밋/태그 URL과 SHA-256,
바이트 크기를 고정합니다.

| 필드 | 뜻 |
| --- | --- |
| `id` | 모델 폴더 이름 (`qwen3.8-27b-q4`) |
| `app` | 읽는 앱 (`redrob-film`), 모든 앱이 쓰는 모델이면 `redrob` |
| `task` | 하는 일: `chat`, `code`, `vision`, `embedding`, `text-to-image`, `image-edit`, `inpaint`, `background-removal`, `upscale`, `segmentation`, `vectorize`, `stt`, `tts`, `vad`, `stem-separation`, `image-to-3d`, `ocr` |
| `runtime` | `files`(앱이 직접 로드), `llama-server`(런처가 서빙), `llama-server-prism`(PrismML llama.cpp 포크로 서빙, 설정을 켠 사람에게만 보임) |
| `requires` | `memory_gb`와 `backends`(`cuda`, `metal`, `vulkan`, `cpu`): 컴퓨터에 필요한 것 |
| `serve` | 서빙 모델의 GGUF 파일과, 있으면 비전 프로젝터 |
| `files` | `name`, `url`, `sha256`, `size` |

런타임도 `assets/models/runtimes.json`에 같은 방식으로 고정합니다. 플랫폼·백엔드별 llama.cpp 빌드
하나씩이고, 서빙 모델을 처음 시작할 때 내려받습니다.

런처는 컴퓨터를 측정해(GPU와 그 메모리 또는 Mac의 통합 메모리, RAM, NPU) 모델마다 판정 하나를
보여 줍니다. **여기서 됨**, **느리게 됨**(CPU 또는 RAM으로 넘침), **여기서 안 됨**, **측정 안 됨**.
판정이 내려받기를 막지는 않습니다.

## 파일이 가는 곳

| `app` | 폴더 |
| --- | --- |
| `redrob-film` | Redrob Film 자체 모델 폴더(`<데이터 폴더>/models`), 이전과 같음 |
| `redrob`, `redrob-canvas`, `redrob-shape`, `redrob-code` | `~/.redrob/models/<id>/` (`REDROB_MODELS_DIR`로 바꿀 수 있음) |

파일은 SHA-256이 맞은 뒤에만 제자리에 놓입니다. 받다 만 파일은 `.part`가 붙어 있고 앱이 열 것이
아닙니다.

## 앱이 읽는 레지스트리

`~/.redrob/models/installed.json` (같은 `REDROB_MODELS_DIR`). 내려받기·삭제·시작·중지마다 통째로,
원자적으로 다시 씁니다.

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

- `backend`는 이 컴퓨터가 쓰는 백엔드입니다. CUDA 경로와 CPU 경로가 둘 다 있는 앱은 이것으로 고릅니다.
- 모든 파일이 있는 모델만 목록에 듭니다.
- `served`는 런처의 서버가 떠 있는 동안만 있습니다. 인증 없는 루프백 OpenAI 호환 엔드포인트입니다:
  `/v1/chat/completions`, `/v1/models`, `/v1/embeddings`. 보낼 `model` 값은 항목의 `id`입니다.

앱의 규칙은 한 줄입니다. **작업(task)으로 찾는다. 항목이 있으면 `path`(또는 `served.url`)를 쓴다.
없으면 Console을 부른다.** 요청이 실패하면 파일을 다시 읽습니다. 그 사이 사람이 모델을 지우거나
서버를 멈췄을 수 있습니다.

## Redrob Code

서빙 모델은 엔진 설정(`$XDG_CONFIG_HOME/redrob/redrob.json`, `REDROB_CONFIG_DIR`로 바꿀 수 있음)에도
provider로 들어갑니다. 엔진이 이미 받아들이는 모양, 즉 `@ai-sdk/openai-compatible` 위의 로컬 URL이고,
id는 `redrob-local-<모델 id>`, 서버가 멈추면 지워집니다. 그 파일의 다른 내용은 건드리지 않고, JSON이
깨진 파일은 아예 건드리지 않습니다. 공유 엔진을 쓰는 앱은 아무 변경 없이 `redrob-local-<id>/<id>`로
로컬 모델을 씁니다.

## 왜 폴더 하나, 파일 하나인가

앱마다 모델 폴더와 내려받기 코드를 따로 두면 같은 가중치를 두 번 받고 검증도 제각각이 됩니다. 폴더
하나, 고정된 카탈로그 하나, 레지스트리 하나면 내려받기도 한 번, 체크섬도 한 번이고, 사람이 디스크에
무엇이 있는지 보고 지울 곳도 한 곳입니다.
