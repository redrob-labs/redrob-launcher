# 레드롭 공용 계정 파일

English: [shared-account.md](./shared-account.md)

레드롭 런처는 컴퓨터 한 대를 레드롭 워크스페이스에 **한 번** 로그인시키고, 그 컴퓨터의 모든 레드롭 앱은 따로
로그인하지 않고 같은 워크스페이스를 씁니다. 이 문서는 파일을 쓰는 런처와 파일을 읽는 앱 사이의 약속입니다.

## 키를 받는 방법

런처는 콘솔의 기기 인증(RFC 8628)을 제품 ID `launcher`로 실행합니다. `POST /device/authorize`로 코드를 받고,
코드가 채워진 `https://console.redrob.ai/connect`를 열고, 사람이 거기서 승인하면 `POST /device/token`이 이름이
**Redrob Launcher**인 워크스페이스 API 키를 돌려줍니다. 키를 손으로 복사하는 사람은 없습니다. 콘솔 주소는
`https://console.redrob.ai/api/backend/v1`이고, 스테이징에서는 `REDROB_CONSOLE_API`로 바꿉니다.

## 파일 위치

| 플랫폼 | 경로 |
|---|---|
| Linux, FreeBSD | `$XDG_CONFIG_HOME/redrob-shared/account.json` (`~/.config/redrob-shared/account.json`) |
| macOS | `~/Library/Application Support/ai.redrob.shared/account.json` |
| Windows | `%APPDATA%\Redrob\Shared\account.json` |

`REDROB_SHARED_DIR`가 비어 있지 않으면 모든 플랫폼에서 이 폴더를 대신 씁니다(테스트, 포터블 설치).

## 내용

```json
{
  "apiKey": "rk_…",
  "apiKeyName": "Redrob Launcher",
  "accountName": "Acme",
  "apiBaseUrl": "https://console.redrob.ai/api/backend/v1"
}
```

Redrob CAD와 Redrob Cut이 이미 자기 `account.json`에 쓰는 모양과 같아서, 앱은 같은 코드로 두 파일을 다 읽을
수 있습니다. 비밀은 `apiKey` 하나입니다. `apiBaseUrl`은 키를 쓰는 곳이고, 앱은 거기에 bearer 토큰으로 보냅니다.

## 읽는 앱의 규칙

1. **읽기만 하고 쓰지 않습니다.** 이 파일은 런처만 씁니다. 로그인이 필요한 앱은 사람을 런처로 보내거나, 자기
   제품 ID로 직접 기기 인증을 하고 그 키는 자기 파일에 둡니다.
2. **필요할 때 읽습니다.** 시작할 때 한 번만 읽지 않습니다. 앱이 열려 있는 동안 로그인하거나 로그아웃할 수
   있습니다. 파일이 없거나, JSON이 깨졌거나, `apiKey`가 비었으면 모두 "로그인 안 됨"입니다.
3. **키를 기록하거나, 보여 주거나, 복사하지 않습니다.** `accountName`과 `apiKeyName`만 보여 줍니다.
4. **앱 자신의 키가 먼저입니다.** 앱이 자기 키(자기 `account.json`, 터미널에서 내보낸 `REDROB_API_KEY`)를
   갖고 있으면 그것을 쓰고, 공용 파일은 그다음입니다.

## 런처가 쓰는 방법

파일은 로그인한 사용자만 읽을 수 있게(Unix는 `0600`, Windows는 사용자 프로필 폴더) 임시 파일로 쓴 뒤 기존
파일 위로 이름을 바꿉니다. 그래서 읽는 쪽은 반쯤 쓰인 파일을 보지 않습니다. 런처에서 로그아웃하면 파일을
지웁니다. 키 자체는 콘솔에서 폐기할 때까지(설정, API 키) 유효합니다.

## 지금 자기 키를 따로 두는 앱

앱이 공용 파일을 읽기 전까지는 런처의 계정 창에서 **연결**할 수 있습니다. 런처가 같은 키를 그 앱의 파일에
써 줍니다. 런처가 아는 앱은 다음과 같습니다.

| 앱 | 파일 | 형식 |
|---|---|---|
| Redrob CAD | `%APPDATA%\Redrob CAD`, `~/Library/Application Support/ai.redrob.cad`, `~/.config/redrob-cad`의 `account.json` (`REDROB_CAD_CONFIG_DIR`) | 위와 같음 |
| Redrob Cut | `%APPDATA%\Redrob Cut`, `~/Library/Application Support/ai.redrob.cut`, `~/.config/redrob-cut`의 `account.json` (`REDROB_CUT_CONFIG_DIR`) | 위와 같음 |
| Redrob Code | 모든 플랫폼에서 `$XDG_DATA_HOME/redrob/auth.json` (`~/.local/share/redrob/auth.json`) | `"redrob": {"type": "api", "key": "rk_…"}`. 파일의 다른 제공자 항목은 그대로 둠 |

다른 앱은 공용 파일을 읽거나 런처가 그 앱의 파일을 알게 될 때까지 **아직 지원 안 됨**으로 보입니다. 코드는
`crates/native/src/account.rs`(`connector`)에 있습니다. 앱별 현황은 [app-integration.ko.md](./app-integration.ko.md)에 있습니다.
