# The shared Redrob account file

한국어: [shared-account.ko.md](./shared-account.ko.md)

Redrob Launcher signs a computer in to a Redrob workspace **once**, and every Redrob app on that
computer can use the same workspace without its own sign-in. This page is the contract between the
launcher, which writes the file, and the apps, which read it.

## How the key is obtained

The launcher runs the console's device authorization grant (RFC 8628) under the product id
`launcher`: it asks `POST /device/authorize` for a code, opens `https://console.redrob.ai/connect`
with the code filled in, the person approves it there, and `POST /device/token` hands back a
workspace API key named **Redrob Launcher**. Nobody copies a key by hand. The console base is
`https://console.redrob.ai/api/backend/v1` (`REDROB_CONSOLE_API` overrides it for staging).

## Where the file is

| Platform | Path |
|---|---|
| Linux, FreeBSD | `$XDG_CONFIG_HOME/redrob-shared/account.json` (`~/.config/redrob-shared/account.json`) |
| macOS | `~/Library/Application Support/ai.redrob.shared/account.json` |
| Windows | `%APPDATA%\Redrob\Shared\account.json` |

`REDROB_SHARED_DIR`, when set and not empty, replaces the directory on every platform (tests,
portable setups).

## What is in it

```json
{
  "apiKey": "rk_…",
  "apiKeyName": "Redrob Launcher",
  "accountName": "Acme",
  "apiBaseUrl": "https://console.redrob.ai/api/backend/v1"
}
```

The shape is the one Redrob CAD and Redrob Cut already use for their own `account.json`, so an app
can read either file with the same code. Only `apiKey` is secret. `apiBaseUrl` is where the key is
spent; an app sends it as a bearer token there.

## Rules for readers

1. **Read, never write.** Only the launcher writes this file. An app that needs a sign-in sends the
   person to the launcher (or runs its own device flow under its own product id and keeps that key
   in its own file).
2. **Read it when you need it**, not once at startup: the person may sign in or out while the app is
   open. A missing file, unreadable JSON or an empty `apiKey` all mean "not signed in".
3. **Never log, display or copy the key.** Show `accountName` and `apiKeyName` only.
4. **The app's own key wins.** If an app keeps its own key (its own `account.json`, `REDROB_API_KEY`
   exported in a terminal), that one is used; the shared file is the fallback.

## How the launcher writes it

The file is written readable by the signed-in user alone (mode `0600` on Unix; the per-user profile
folder on Windows), through a temporary file renamed over the old one, so a reader never sees half a
file. Signing out in the launcher deletes the file; the key stays valid on the console until it is
revoked there (Settings, API keys).

## Apps that keep their own key today

Until an app reads the shared file, the launcher's account dialog can **Connect** it: the launcher
writes the same key into the app's own file. It knows these:

| App | File | Format |
|---|---|---|
| Redrob CAD | `account.json` in `%APPDATA%\Redrob CAD`, `~/Library/Application Support/ai.redrob.cad`, `~/.config/redrob-cad` (`REDROB_CAD_CONFIG_DIR`) | as above |
| Redrob Cut | `account.json` in `%APPDATA%\Redrob Cut`, `~/Library/Application Support/ai.redrob.cut`, `~/.config/redrob-cut` (`REDROB_CUT_CONFIG_DIR`) | as above |
| Redrob Code | `$XDG_DATA_HOME/redrob/auth.json` (`~/.local/share/redrob/auth.json`) on every platform | `"redrob": {"type": "api", "key": "rk_…"}`; the file's other providers are kept |

Every other app shows **Not supported yet** until it reads the shared file or the launcher learns its
own. The code is `crates/native/src/account.rs` (`connector`).
