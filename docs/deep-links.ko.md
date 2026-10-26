# redrob-launcher:// 링크

웹 페이지(레드롭 챗, 웹사이트)는 `redrob-launcher://` 링크로 레드롭 런처를 열어 앱 하나를 보여 줍니다.
예전 `redrob://` 표기도 계속 동작하고 계속 등록하지만, 페이지는 `redrob-launcher://`로 먼저 연결합니다.
`redrob`은 런처만 등록한 스킴이 아니기 때문입니다. Redrob Browser 0.2~0.4가 Windows에서 이 스킴을 자기
실행 스킴(Chromium의 direct-launch 스킴, 지금은 `redrob-browser`)으로 등록했고, 업데이트해도 그 등록이
남아 `redrob://`이 브라우저를 열 수 있습니다. 아래 예시는 `redrob://`로 쓰지만 `redrob-launcher://`도
모든 형식을 받습니다.

| 링크 | 런처가 하는 일 |
|---|---|
| `redrob://open/<app>` | 설치된 앱이면 실행합니다. 설치되지 않았으면 그 앱을 보여 주고 설치 버튼에 포커스를 둡니다. |
| `redrob://install/<app>` | 앱을 보여 주고 설치(또는 업데이트)를 시작합니다. 최신 버전이 설치돼 있으면 보여 주기만 합니다. |

`<app>`은 카탈로그 ID(`crates/core/src/catalog.rs`)이며 `redrob-` 접두사는 있어도 없어도 됩니다:
`cad`, `redrob-cad`, `cowork`, `office`, `canvas`, `shape`, `design`, `film`,
`browser`, `code`. 대소문자는 가리지 않고, 끝의 `/`는 허용되며, 쿼리 문자열과 프래그먼트는 무시합니다.
그 밖의 것(모르는 앱, 다른 동작, 두 번째 경로 조각, `%` 이스케이프, 256자를 넘는 링크)은 런처 홈만
열고 아무것도 하지 않습니다. 파서는 `crates/core/src/deep_link.rs`이고 그 테스트가 명세입니다.

```html
<a href="redrob://open/cad">Redrob CAD 열기</a>
<a href="redrob://install/cowork">Redrob Cowork 설치</a>
```

브라우저는 링크를 런처에 넘기기 전에 사용자에게 묻습니다. 링크의 어떤 것도 셸이나 경로에 닿지 않습니다.
링크는 카탈로그 앱 하나를 가리킬 뿐이고, 런처는 자기 버튼이 하는 일을 합니다.

`redrob://open/code`는 Code 카드의 열기와 마찬가지로 Redrob Code의 공용 데몬이 desk를 켠 채 실행되게
합니다. 그래서 웹의 레드롭 챗과 폰 앱이 이 PC에 연결할 수 있습니다([engine.md](engine.md#using-this-pc-from-the-web-and-the-phone)).

`redrob-launcher://open/code?engine=1`은 레드롭 챗의 "레드롭 코드 켜기"입니다. Code 터미널을 띄우지 않고,
AI 엔진을 desk를 켠 채 시작한 뒤 AI 엔진 페이지(desk 로그인·기기 목록이 있는 곳)를 보여 줍니다. Code가
설치되지 않았으면 open과 같이 설치 버튼을 보여 줍니다.

## 링크가 런처에 닿는 길

- **Linux:** 데스크톱 항목이 `MimeType=x-scheme-handler/redrob-launcher;x-scheme-handler/redrob;`와 `Exec=… %u`를 선언합니다. deb와 rpm은
  설치 뒤 데스크톱 데이터베이스를 갱신합니다. AppImage의 `AppRun`은 `%u`를 남긴 채 항목을
  `~/.local/share/applications`에 쓰고, tarball 설치에는 런처가 직접 항목을 씁니다. 런처는 시작할 때마다
  두 스킴 모두 `xdg-mime default ai.redrob.launcher.desktop x-scheme-handler/<스킴>`으로 자기 항목을 기본 처리기로 둡니다
  (`crates/native/src/url_scheme.rs`).
- **macOS:** `Info.plist`가 `redrob-launcher`, `redrob` 스킴을 선언합니다(`CFBundleURLTypes`). macOS는 링크를 `kAEGetURL`
  Apple Event로 보내고, `apps/redrob-launcher/src/url_event.rs`가 받습니다.
- **Windows:** `HKCU\Software\Classes\redrob-launcher`와 `...\Classes\redrob`(`URL Protocol`,
  `shell\open\command` = `"<exe>" "%1"`). 설치 프로그램이 쓰고, 런처도 시작할 때 다른 곳을 가리키면(포터블
  zip, 옮긴 설치) 다시 씁니다. 사용자 단위라 관리자 권한이 필요 없습니다. 제거 프로그램이 지웁니다. 런처는
  시작할 때마다 두 스킴에 대해 다른 앱을 가리키는 사용자 선택(`...\Shell\Associations\UrlAssociations\<스킴>\UserChoice`)을
  지우고, 옛 Redrob Browser의 `Capabilities\URLAssociations` 값(`RedrobHTML` ProgId)도 지웁니다. 사용자
  선택이 클래스 키보다 우선하기 때문입니다.

개발 빌드(`cargo run`)는 등록하지 않으므로 설치된 런처의 링크를 가로채지 않습니다.

## 런처가 이미 실행 중일 때

OS는 링크를 인자로 두 번째 런처를 띄웁니다(Linux, Windows). 두 번째 런처는 루프백 포트에서 실행 중인
런처를 찾아(`crates/native/src/single_instance.rs`) 평소처럼 인사하고, 이어서
`redrob-launcher:link <url>`을 보냅니다. 실행 중인 런처가 앞으로 나와 링크를 처리하고 두 번째 런처는
끝납니다. 더 새 버전의 두 번째 런처는 전처럼 자리를 넘겨받고 자기 링크를 직접 처리합니다. 링크 이전의
런처는 이 메시지를 무시하고 앞으로 나오기만 합니다. macOS에서는 실행 중인 런처가 Apple Event를 직접
받습니다.

설치와 실행은 런처가 앱의 설치 여부와 최신 릴리스를 알 때까지 기다리므로, 런처를 새로 띄우는 링크도
동작합니다.
