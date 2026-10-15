# The release manifest: `redrob-launcher.json`

한국어: [release-manifest.ko.md](./release-manifest.ko.md)

Redrob Launcher learns what a release of an app can do from one small file the app's release
workflow uploads beside its installers. Nothing in the launcher has to change when an app ships: the
next time the launcher reads that app's latest release, it reads the file too.

## What an app uploads

One asset named exactly `redrob-launcher.json` on every GitHub Release, next to the installers and
`SHA256SUMS.txt`:

```json
{ "schema": 1, "sharedAccount": true }
```

| Field | Meaning |
|---|---|
| `schema` | Always `1` for now. A launcher that does not know the schema ignores the file. |
| `sharedAccount` | `true` when this release reads the launcher's shared account file ([shared-account.md](./shared-account.md)): signing in to the launcher is all it needs. Missing reads as `false`. |

Unknown fields are ignored, so schema 1 can grow. Keep the file small; the launcher reads at most a
few kilobytes of it.

A release workflow step that produces it, before the upload step:

```sh
printf '{ "schema": 1, "sharedAccount": true }\n' > dist/release/redrob-launcher.json
```

Then list it in `SHA256SUMS.txt` like every other asset, so the launcher's quota-free path (which
reads the sums file to learn the asset names) sees it too.

## What the launcher does with it

- Latest release has the file and `sharedAccount: true`: an installed copy at or past that version
  shows **Uses this sign-in**; an older installed copy shows **Update to use this sign-in**.
- Latest release has the file and `sharedAccount: false`: the app is treated as not reading the shared
  file, whatever the launcher's built-in table says.
- No file (releases made before this contract): the launcher falls back to its built-in table
  (`SHARED_READERS` in `crates/core/src/account.rs`), which records the releases known to read the
  file -- Redrob CAD 0.0.3 and Redrob Film 0.0.1 -- and shows **Next release uses this sign-in** for
  apps whose change is merged but not released.
- The app's own key always wins over the launcher's, with or without the file.

The file is fetched only from the release's own download URL
(`https://github.com/redrob-labs/<repo>/releases/download/<tag>/redrob-launcher.json`); anything else
is ignored. It is never written by the launcher.

## Asset names

The same release must name its installers `<stem>-<version>-<os>-<arch>[-portable].<ext>` for the
launcher to offer **Install** (`parse_asset_name` in `crates/core/src/platform.rs`). The audit of each
app's current names is in [app-integration.md](./app-integration.md#release-asset-names-audit-2026-10-13).
