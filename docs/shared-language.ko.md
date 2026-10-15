# 공유 언어 파일

스위트 전체가 언어 하나를 씁니다. 사람은 Windows 설치 마법사 첫 페이지나 런처 설정에서 한 번
고르고, 마법사·런처·모든 Redrob 앱이 그 언어로 말합니다. 런처가 파일 하나를 쓰고, 앱은 읽습니다.

## 선택이 전달되는 길

```
Windows 설치 마법사 1페이지  -->  HKCU\Software\Redrob\Redrob Launcher\Language  (en | ko)
                                              |
                                              v  첫 실행
Redrob Launcher 설정  --------------------> language.json  (공유 디렉터리)
                                              |
                                              v  시작할 때
Redrob Canvas, Film, Shape, Code, ...  이 파일을 읽어 그 언어로 그림
```

- 마법사 언어 페이지의 기본값은 이전 설치의 선택, 없으면 한국어 Windows에서는 한국어, 그 외는
  영어입니다. 마법사의 나머지 페이지가 바로 그 언어로 바뀝니다.
- 런처는 첫 실행(자기 설정이 아직 없을 때)에 마법사의 답을 가져옵니다. 그 뒤로는 런처 설정
  대화상자에서 바꿉니다. 런처와 모든 앱의 언어, 또는 "앱별 언어"에서 앱 하나만.
- macOS 디스크 이미지와 Linux 패키지에는 마법사가 없습니다. 거기서는 설정에서 언어를 고르기 전까지
  런처가 시스템 언어를 따르고, 파일은 어느 쪽이든 씁니다.

## 파일 위치

공유 계정 파일(`docs/shared-account.ko.md`) 옆:

| 플랫폼 | 경로 |
|---|---|
| Linux | `$XDG_CONFIG_HOME/redrob-shared/language.json` (`~/.config/redrob-shared/language.json`) |
| macOS | `~/Library/Application Support/ai.redrob.shared/language.json` |
| Windows | `%APPDATA%\Redrob\Shared\language.json` |

`REDROB_SHARED_DIR`가 비어 있지 않게 설정돼 있으면 모든 플랫폼에서 그 디렉터리를 씁니다.

## 내용

```json
{
  "language": "ko",
  "chosen": true,
  "apps": {
    "redrob-canvas": "en"
  }
}
```

| 필드 | 뜻 |
|---|---|
| `language` | 모든 Redrob 앱의 언어. 런처가 지원하는 소문자 코드(`en`, `ko`, ...). `auto`는 절대 없음: 런처가 시스템 언어를 풀어서 씁니다. |
| `chosen` | 사람이 고른 것(마법사 또는 설정)이면 `true`, 런처가 시스템에서 알아낸 것이면 `false`. |
| `apps` | 앱별 덮어쓰기. 앱 id(`redrob-canvas`, `redrob-film`, ...) -> 언어 코드. 비어 있거나 없을 수 있음. |

## 읽는 쪽 규칙

1. **읽기만, 쓰지 않음.** 이 파일은 런처만 씁니다. 앱 자체 언어 설정이 있다면 앱 자기 설정에 둡니다.
2. **우선순위**, 앱 `<id>`에 대해: 사람이 앱 안에서 직접 정한 설정; 없으면 `apps["<id>"]`; 없으면
   `language`; 없으면 시스템 언어. 시스템 언어를 스스로 감지하는 앱은 `"chosen": false`를
   "선호 없음"으로 보고 자기 감지를 유지해도 됩니다.
3. **시작할 때 읽고**, 비용이 싸면 창이 포커스를 되찾을 때 다시 읽습니다. 파일 없음, 깨진 JSON, 모르는
   코드는 전부 "시스템 따름"입니다.
4. **모르는 코드면 영어로.** 그 코드의 번역이 없는 앱은 영어로 그립니다.

## 런처가 쓰는 방식

임시 파일을 써서 옛 파일 위로 이름을 바꾸므로 읽는 쪽이 반쪽 파일을 보지 않습니다. 내용이 바뀔 때만
씁니다(시작, 설정 변경). 제거할 때 지우지 않습니다. 계정처럼 재설치 뒤에도 선택이 남습니다.
