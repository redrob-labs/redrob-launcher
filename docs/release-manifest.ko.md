# 릴리스 매니페스트: `redrob-launcher.json`

English: [release-manifest.md](./release-manifest.md)

Redrob Launcher는 앱 릴리스가 무엇을 할 수 있는지, 앱의 릴리스 워크플로가 설치 파일 옆에 올리는 작은
파일 하나로 알아냅니다. 앱이 새로 나와도 런처는 바꿀 것이 없습니다. 다음에 그 앱의 최신 릴리스를 읽을
때 이 파일도 함께 읽습니다.

## 앱이 올리는 것

GitHub Release마다 설치 파일, `SHA256SUMS.txt` 옆에 이름이 정확히 `redrob-launcher.json`인 자산 하나:

```json
{ "schema": 1, "sharedAccount": true }
```

| 필드 | 뜻 |
|---|---|
| `schema` | 지금은 항상 `1`. 모르는 schema를 만난 런처는 파일을 무시합니다. |
| `sharedAccount` | 이 릴리스가 런처의 공용 계정 파일([shared-account.ko.md](./shared-account.ko.md))을 읽으면 `true`. 런처에 로그인한 것만으로 충분하다는 뜻. 없으면 `false`. |

모르는 필드는 무시하므로 schema 1은 자라도 됩니다. 파일은 작게 둡니다. 런처는 몇 KB까지만 읽습니다.

업로드 단계 앞에 두는 릴리스 워크플로 한 줄:

```sh
printf '{ "schema": 1, "sharedAccount": true }\n' > dist/release/redrob-launcher.json
```

그리고 다른 자산처럼 `SHA256SUMS.txt`에 넣습니다. 런처의 할당량 없는 경로는 그 파일로 자산 이름을
알기 때문입니다.

## 런처가 하는 일

- 최신 릴리스에 파일이 있고 `sharedAccount: true`: 설치된 버전이 그 이상이면 **이 로그인을 씀**, 그보다
  오래됐으면 **업데이트하면 이 로그인을 씁니다**.
- 파일이 있고 `sharedAccount: false`: 런처 내장 표가 뭐라 해도 공용 파일을 안 읽는 앱으로 봅니다.
- 파일 없음(이 규약 전 릴리스): 내장 표(`crates/core/src/account.rs`의 `SHARED_READERS`)로 돌아갑니다.
  표에는 읽는 것이 확인된 릴리스(Redrob CAD 0.0.3, Redrob Film 0.0.1)가 있고, 머지만 되고 릴리스 안 된
  앱은 **다음 버전부터 이 로그인을 씁니다**로 보입니다.
- 파일이 있든 없든 앱 자신의 키가 런처 키보다 우선합니다.

파일은 릴리스 자신의 다운로드 주소
(`https://github.com/redrob-labs/<repo>/releases/download/<tag>/redrob-launcher.json`)에서만 받습니다.
다른 곳은 무시합니다. 런처가 이 파일을 쓰는 일은 없습니다.

## 자산 이름

같은 릴리스의 설치 파일 이름이 `<stem>-<version>-<os>-<arch>[-portable].<ext>` 꼴이어야 런처가
**Install**을 보여 줍니다(`crates/core/src/platform.rs`의 `parse_asset_name`). 앱별 현재 이름 점검은
[app-integration.ko.md](./app-integration.ko.md)에 있습니다.
