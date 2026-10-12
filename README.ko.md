# 레드롭 런처

레드롭 런처는 모든 레드롭 앱을 한 곳에서 설치하고, 업데이트하고, 로그인하고, 기기를 연결하는 앱입니다.

English: [README.md](./README.md)

> **포크 안내.** 이 저장소는 [storytold/craft-launcher](https://github.com/storytold/craft-launcher) 태그 `v0.3.2`의
> 하드 포크입니다. 어디서 갈라졌고, 무엇을 소유하지 않으며, 상류를 어떻게 흡수하는지는
> [UPSTREAM.ko.md](./UPSTREAM.ko.md)에, 기계가 읽는 핀은 [`upstream-base.json`](./upstream-base.json)에 있습니다.
> 상류 저작권 표기는 [NOTICE](./NOTICE)에 원문 그대로 두고 그 아래에 우리 줄을 추가했습니다.
> `docs/brand/` 아래 ArtCraft 이름과 로고, `assets/app-icon/` 아래 앱 아이콘은 상류의 상표이며 교체 중입니다.

## 무엇인가

- 순수 Rust와 egui로 만든 작은 네이티브 앱. Electron도 웹뷰도 없습니다.
- 각 앱의 GitHub 릴리스를 읽어 내 OS와 CPU에 맞는 빌드(DMG, MSI, AppImage, .deb, .rpm 등)를 고르고, SHA-256으로 검사한 뒤 설치합니다.
- 설치된 앱과 버전을 보여 주고, 새 버전이 나오면 알립니다. 트레이와 메뉴 막대에서 바로 열 수 있습니다.
- macOS·Windows·Linux·FreeBSD 네이티브, 브라우저는 WebAssembly.

## 시작하기

```sh
cargo run --release -p artcraft-launcher   # 데스크톱 앱
cargo test --workspace                     # 테스트
python3 scripts/check_upstream_boundary.py # 포크 경계 검사
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

자산별 귀속은 [ATTRIBUTION.md](./ATTRIBUTION.md)에 있습니다.
