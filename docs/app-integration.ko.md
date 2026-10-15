# 런처 로그인을 쓰는 앱

English: [app-integration.md](./app-integration.md)

레드롭 런처는 컴퓨터를 한 번 로그인시키고 키를 공용 계정 파일에 둡니다([shared-account.ko.md](./shared-account.ko.md)).
이 문서는 앱마다 그 파일을 쓰기까지 얼마나 남았는지 적습니다. "기기 인증"은 앱이 이미 자기 제품 ID로
console.redrob.ai에서 자기 키를 받는다는 뜻입니다(콘솔의 `DEVICE_PRODUCTS`). 이런 앱은 작은 변경 하나면 됩니다.

| 앱 | 지금 자기 로그인 | 공용 파일 읽기 | 런처가 키를 넣어 줄 수 있음 | 남은 일 |
|---|---|---|---|---|
| Redrob CAD | 기기 인증(`cad`), `account.json` | 예. v0.0.3에 포함(redrob-cad#32) | 예 | 없음 |
| Redrob Film | 기기 인증(`cut`), `account.json` | 예. v0.0.1에 포함(redrob-film#6) | 예 | 없음 |
| Redrob Code | `REDROB_API_KEY`, 자격 저장소, `auth.json` | 머지됨(redrob-code#88). `ConsoleKey.resolve`의 네 번째 출처, `source: "launcher"`로 보고 | 예(`auth.json`, 다른 제공자 유지) | 릴리스 |
| Redrob Shape | 기기 인증(`shape`) | 머지됨(redrob-shape#38) | 아니요 | 릴리스 |
| Redrob Browser | 기기 인증(`redrob-browser`) | 머지됨(redrob-browser#110) | 아니요 | 릴리스 |
| Redrob Cowork | 기기 인증(`work`), 키는 내장 Redrob Code 엔진이 보관 | 엔진을 통해(redrob-code#88). "Redrob Launcher로 로그인됨" 표시(redrob-cowork#207) | 아니요(엔진의 `auth.json`은 Redrob Code 행) | 새 엔진과 함께 릴리스 |
| Redrob Office | 기기 인증(`office`) | 머지됨(redrob-office#154) | 아니요 | 릴리스 |
| Redrob Design | 기기 인증(`design`) | 머지됨(redrob-design#92) | 아니요 | 릴리스 |
| Redrob Canvas | `REDROB_API_KEY`, 저장된 `console-key`, 기기 인증 | 머지됨(redrob-canvas#160) | 아니요 | 릴리스 |
| Redrob Query | `REDROB_API_KEY`, OS 키링 | 머지됨(redrob-query#55) | 아니요 | 릴리스 |
| Redrob Recall | OS 키링, 붙여 넣은 키 | 머지됨(redrob-recall#49) | 아니요 | 릴리스 |

런처 계정 창은 이 표를 `crates/core/src/account.rs`의 `SHARED_READERS`로 읽습니다. 설치된 버전이 표의
릴리스 이상이면 "이 로그인을 씀", 머지됐지만 릴리스 전이면 "다음 버전부터 이 로그인을 씁니다", IDE는
"로그인이 필요 없습니다"로 보입니다. 앱이 릴리스되면 그 표에 버전을 적거나, 더 좋게는 릴리스에
[`redrob-launcher.json`](./release-manifest.ko.md)을 실어 보냅니다. 런처는 표보다 그 파일을 믿습니다.

공용 파일을 읽는 앱의 규칙은 [shared-account.ko.md](./shared-account.ko.md)의 네 가지입니다. 읽기만 하고 쓰지
않기, 키가 필요할 때 읽기, 키를 기록하거나 보여 주지 않기, 앱 자신의 키가 먼저. Redrob CAD와 Redrob Film의
변경이 기준입니다. 각 `device.rs`의 `shared_account_path()`와 `KeySource::current()`, 그리고 공용 키를 쓸 때
연결 해제 대신 보이는 "Signed in with Redrob Launcher"입니다.

## 릴리스 자산 이름 (2026-10-13 점검)

런처는 자산 이름이 `<stem>-<version>-<os>-<arch>[-portable].<ext>` 꼴일 때만 설치합니다
(`crates/core/src/platform.rs`의 `parse_asset_name`). 앱별 최신 릴리스:

| 앱 | 최신 | 자산 예 | 설치 가능 | 그 레포에서 고칠 것 |
|---|---|---|---|---|
| Redrob CAD | v0.0.3 | `redrob-cad-0.0.3-linux-x86_64.tar.gz` | 예, 전 플랫폼 | 없음 |
| Redrob Film | v0.0.1 | `redrob-film-0.0.1-linux-x86_64.tar.gz` | 예, 전 플랫폼 | `filmcraft-web-0.0.1.zip` 이름(설치 대상은 아님) |
| Redrob Canvas | v0.5.6 | `redrob-canvas-0.5.6-linux-x86_64.tar.gz` | Linux, macOS. Windows는 `.zip`만 | `-setup.exe`의 접미사를 파서가 안 읽음 |
| Redrob Shape | v0.1.0 | `redrob-shape-linux-x64-0.1.0.tar.xz` | 아니요 | 버전이 OS 앞에; `x64` -> `x86_64`; `.tar.xz` -> `.tar.gz` |
| Redrob Browser | v0.1.8 | `redrob-browser-macos-arm64-0.1.8.zip`, `redrob-browser-stable_0.1.8+155…_amd64.deb` | 아니요 | 버전이 OS 앞에; deb 이름은 패키징 고유 |
| Redrob Code | v0.5.0 | `redrob-linux-x64.tar.gz` | 아니요 | 버전 없음; stem이 `redrob-code`가 아니라 `redrob`; `x64` -> `x86_64` |
| Redrob Cowork | v0.3.0 | `redrob-linux-x86_64-0.3.0.AppImage` | 아니요 | 버전이 arch 뒤에; stem `redrob`; `mac` -> `macos` |
| Redrob Office | v0.9.0 | `Redrob-0.9.0-arm64.dmg`, `Redrob-0.9.0.AppImage` | 아니요 | electron-builder 기본값: stem `Redrob`, OS 없음, Linux는 arch 없음 |
| Redrob Design | v0.15.0 | `Redrob.Design_0.15.0_aarch64.dmg` | 아니요 | Tauri 기본값 `Redrob.Design_<ver>_<arch>`; Linux·Windows 자산 없음 |
| Redrob Query | v0.2.0 | `Redrob.Query_0.2.0_amd64.AppImage` | 아니요 | Tauri 기본값, Design과 같음 |
| Redrob Recall | v0.2.0 | `Redrob.Recall_0.2.0_amd64.AppImage` | 아니요 | Tauri 기본값, Design과 같음 |

메우는 길은 둘입니다. 레포마다 릴리스 워크플로에서 업로드 전에 이름을 바꾸는 단계 하나(패키저 고유
이름은 사본으로 남김), 또는 런처가 Tauri·electron-builder 표기를 배우는 것. 앞은 모든 앱에 규칙 하나,
뒤는 더 빨리 나감. 둘 다 아직 안 했습니다.
