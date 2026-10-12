# Which apps use the launcher's sign-in

한국어: [app-integration.ko.md](./app-integration.ko.md)

Redrob Launcher signs a computer in once and keeps the key in the shared account file
([shared-account.md](./shared-account.md)). This page tracks, app by app, how far each one is from
using it. "Device flow" means the app already asks console.redrob.ai for its own key under its own
product id (the console's `DEVICE_PRODUCTS`); those apps are one small change away.

| App | Its own sign-in today | Reads the shared file | Launcher can hand it the key | What is left |
|---|---|---|---|---|
| Redrob CAD | Device flow (`cad`), `account.json` | Yes, from the release after redrob-cad#32 | Yes | Release it |
| Redrob Cut | Device flow (`cut`), `account.json` | Yes, from the release after redrob-cut#6 | Yes | Release it |
| Redrob Code | `REDROB_API_KEY`, its credential store, `auth.json` | No | Yes (`auth.json`, other providers kept) | Read the shared file in `ConsoleKey.resolve` as a fourth origin |
| Redrob Shape | Device flow (`shape`) | No | No | Read the shared file beside its own key |
| Redrob Browser | Device flow (`redrob-browser`) | No | No | Read the shared file beside its own key |
| Redrob Cowork | Device flow (`cowork`, `work`) | No | No | Read the shared file beside its own key |
| Redrob Office | Device flow (`office`) | No | No | Read the shared file beside its own key |
| Redrob Design | Device flow (`design`) | No | No | Read the shared file beside its own key |
| Redrob Canvas | Not checked | No | No | Find where it keeps a key, if it uses one |
| Redrob IDE | Not checked | No | No | Find where it keeps a key, if it uses one |
| Redrob Query | Not checked | No | No | Find where it keeps a key, if it uses one |
| Redrob Recall | Not checked | No | No | Find where it keeps a key, if it uses one |
| Redrob Studio | None: runs every model locally | Not needed | Not needed | Nothing |

For an app that reads the shared file, the rules are the four in
[shared-account.md](./shared-account.md#rules-for-readers): read, never write; read it when the key is
needed; never log or show the key; the app's own key wins. Redrob CAD's and Redrob Cut's change is the
reference: `shared_account_path()` and `KeySource::current()` in their `device.rs`, and "Signed in
with Redrob Launcher" in place of Disconnect when the shared key is the one in use.
