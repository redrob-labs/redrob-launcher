# 런처 로그인을 쓰는 앱

English: [app-integration.md](./app-integration.md)

레드롭 런처는 컴퓨터를 한 번 로그인시키고 키를 공용 계정 파일에 둡니다([shared-account.ko.md](./shared-account.ko.md)).
이 문서는 앱마다 그 파일을 쓰기까지 얼마나 남았는지 적습니다. "기기 인증"은 앱이 이미 자기 제품 ID로
console.redrob.ai에서 자기 키를 받는다는 뜻입니다(콘솔의 `DEVICE_PRODUCTS`). 이런 앱은 작은 변경 하나면 됩니다.

| 앱 | 지금 자기 로그인 | 공용 파일 읽기 | 런처가 키를 넣어 줄 수 있음 | 남은 일 |
|---|---|---|---|---|
| Redrob CAD | 기기 인증(`cad`), `account.json` | 예. redrob-cad#32 다음 릴리스부터 | 예 | 릴리스 |
| Redrob Cut | 기기 인증(`cut`), `account.json` | 예. redrob-cut#6 다음 릴리스부터 | 예 | 릴리스 |
| Redrob Code | `REDROB_API_KEY`, 자격 저장소, `auth.json` | 아니요 | 예(`auth.json`, 다른 제공자 유지) | `ConsoleKey.resolve`에서 공용 파일을 네 번째 출처로 읽기 |
| Redrob Shape | 기기 인증(`shape`) | 아니요 | 아니요 | 자기 키 옆에서 공용 파일 읽기 |
| Redrob Browser | 기기 인증(`redrob-browser`) | 아니요 | 아니요 | 자기 키 옆에서 공용 파일 읽기 |
| Redrob Cowork | 기기 인증(`cowork`, `work`) | 아니요 | 아니요 | 자기 키 옆에서 공용 파일 읽기 |
| Redrob Office | 기기 인증(`office`) | 아니요 | 아니요 | 자기 키 옆에서 공용 파일 읽기 |
| Redrob Design | 기기 인증(`design`) | 아니요 | 아니요 | 자기 키 옆에서 공용 파일 읽기 |
| Redrob Canvas | 확인 안 함 | 아니요 | 아니요 | 키를 쓰는지, 어디에 두는지 확인 |
| Redrob IDE | 확인 안 함 | 아니요 | 아니요 | 키를 쓰는지, 어디에 두는지 확인 |
| Redrob Query | 확인 안 함 | 아니요 | 아니요 | 키를 쓰는지, 어디에 두는지 확인 |
| Redrob Recall | 확인 안 함 | 아니요 | 아니요 | 키를 쓰는지, 어디에 두는지 확인 |
| Redrob Studio | 없음. 모든 모델을 로컬에서 실행 | 필요 없음 | 필요 없음 | 없음 |

공용 파일을 읽는 앱의 규칙은 [shared-account.ko.md](./shared-account.ko.md)의 네 가지입니다. 읽기만 하고 쓰지
않기, 키가 필요할 때 읽기, 키를 기록하거나 보여 주지 않기, 앱 자신의 키가 먼저. Redrob CAD와 Redrob Cut의
변경이 기준입니다. 각 `device.rs`의 `shared_account_path()`와 `KeySource::current()`, 그리고 공용 키를 쓸 때
연결 해제 대신 보이는 "Signed in with Redrob Launcher"입니다.
