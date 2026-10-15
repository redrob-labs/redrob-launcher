# The shared language file

One language for the whole suite: the person picks it once, in the Windows setup wizard's first
page or in the launcher's settings, and the wizard, the launcher and every Redrob app speak it.
The launcher writes one file; the apps read it.

## How the choice travels

```
Windows setup wizard, page 1  -->  HKCU\Software\Redrob\Redrob Launcher\Language  (en | ko)
                                              |
                                              v  first run
Redrob Launcher settings  -----------------> language.json  (shared dir)
                                              |
                                              v  at startup
Redrob Canvas, Film, Shape, Code, ...  read it and draw in that language
```

- The wizard's language page defaults to the previous install's choice, else Korean on a Korean
  Windows, else English. The rest of the wizard follows it at once.
- The launcher adopts the wizard's answer on its first run (when it has no settings of its own
  yet). Later the launcher's Settings dialog is where the language is changed, for the launcher and
  for every app, or per app under "Per-app language".
- The macOS disk image and the Linux packages have no wizard. There the launcher follows the
  system until a language is chosen in its settings; the file is written either way.

## Where the file is

Next to the shared account file (`docs/shared-account.md`):

| Platform | Path |
|---|---|
| Linux | `$XDG_CONFIG_HOME/redrob-shared/language.json` (`~/.config/redrob-shared/language.json`) |
| macOS | `~/Library/Application Support/ai.redrob.shared/language.json` |
| Windows | `%APPDATA%\Redrob\Shared\language.json` |

`REDROB_SHARED_DIR`, when set and not empty, replaces the directory on every platform.

## What is in it

```json
{
  "language": "ko",
  "chosen": true,
  "apps": {
    "redrob-canvas": "en"
  }
}
```

| Field | Meaning |
|---|---|
| `language` | The language for every Redrob app: a lowercase code the launcher supports (`en`, `ko`, ...). Never `auto`: the launcher resolves the system language before writing. |
| `chosen` | `true` when a person picked it (wizard or settings); `false` when it is what the launcher worked out from the system. |
| `apps` | Per-app overrides, app id (`redrob-canvas`, `redrob-film`, ...) to language code. May be empty or missing. |

## Rules for readers

1. **Read, never write.** Only the launcher writes this file. An app's own language setting, if it
   has one, lives in the app's own config.
2. **Order of precedence** for an app `<id>`: the app's own explicit setting, if the person set one
   in the app; else `apps["<id>"]`; else `language`; else the system language. An app that
   detects the system language itself may treat `"chosen": false` as "no preference" and keep
   its own detection.
3. **Read it at startup**, and again when the window regains focus if that is cheap; a missing
   file, unreadable JSON or an unknown code all mean "follow the system".
4. **Unknown code, fall back.** An app that has no translation for the code draws in English.

## How the launcher writes it

Through a temporary file renamed over the old one, so a reader never sees half a file; only when
the content changes (startup, a change in Settings). It is not deleted on uninstall: the choice
survives a reinstall, like the account.
