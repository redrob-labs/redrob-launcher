# 레드롭 런처

레드롭 런처는 모든 레드롭 앱을 한 곳에서 설치하고, 업데이트하고, 로그인하고, 기기를 연결하는 앱입니다.

English: [README.md](./README.md)

> **포크 안내.** 이 저장소는 [storytold/craft-launcher](https://github.com/storytold/craft-launcher) 태그 `v0.3.2`의
> 하드 포크입니다. 어디서 갈라졌고, 무엇을 소유하지 않으며, 상류를 어떻게 흡수하는지는
> [UPSTREAM.ko.md](./UPSTREAM.ko.md)에, 기계가 읽는 핀은 [`upstream-base.json`](./upstream-base.json)에 있습니다.
> 상류 저작권 표기는 [NOTICE](./NOTICE)에 원문 그대로 두고 그 아래에 우리 줄을 추가했습니다.
> 상류의 ArtCraft 상표는 모두 제거했습니다.

## 무엇인가

- 레드롭 앱 전부를 보여 줍니다: Cut, Shape, CAD, Canvas, Design, Studio, Browser, Code, IDE, Cowork,
  Office, Query, Recall.
- 각 앱의 GitHub 릴리스를 읽어 내 OS와 CPU에 맞는 빌드(DMG, MSI, AppImage, .deb, .rpm 등)를 고르고,
  SHA-256으로 검사한 뒤 설치합니다.
- 설치된 앱과 버전을 보여 주고, 새 버전이 나오면 알리고 업데이트합니다.
- 트레이와 메뉴 막대에 머뭅니다. 순수 Rust와 egui로 만들어 Electron도 웹뷰도 없습니다.
- macOS·Windows·Linux·FreeBSD 네이티브, 웹 빌드는 앱 목록과 다운로드 링크를 보여 줍니다.
- 영어와 한국어.

같은 창에서 레드롭 워크스페이스 로그인, 기기 연결, 모델 다운로드를 하는 기능은 만들고 있습니다.

## 소스에서 빌드

```sh
cargo run --release -p redrob-launcher     # 데스크톱 앱
cargo test --workspace                     # 테스트
python3 scripts/check_upstream_boundary.py # 포크 경계 검사
packaging/brand-assets.sh                  # assets/brand/에서 아이콘 다시 만들기(magick, resvg)
```

## 브랜치와 기여

- 기본 브랜치 `develop`, 릴리스 브랜치 `main`. 브랜치 이름은 `feat/ fix/ chore/ docs/ test/ refactor/ perf/ sync/ hotfix/ release/` 접두사를 따르며 `.github/workflows/gitflow.yml`이 검사합니다.
- 상류 동기화는 `sync/upstream-<tag>` 브랜치로, 머지 커밋으로만 합칩니다.
- 조직 공통 규약: [redrob-labs/.github](https://github.com/redrob-labs/.github)의 CONTRIBUTING.ko.md와 FORKS.ko.md.

## 라이선스

MIT 또는 Apache-2.0 중 선택([LICENSE-MIT](./LICENSE-MIT), [LICENSE-APACHE](./LICENSE-APACHE)).
저작권 표기는 [NOTICE](./NOTICE)에 있습니다. 상류 표기 아래에 우리 표기를 추가했습니다:

```
Copyright (c) 2026 ArtCraft Team and the ArtCraft Launcher contributors.
Copyright (c) 2026-present Janghoon Lee (Redrob) and contributors
```

레드롭 이름과 마크는 레드롭의 상표이며 위 라이선스에 포함되지 않습니다. 자산별 귀속은
[ATTRIBUTION.md](./ATTRIBUTION.md)에 있습니다.
