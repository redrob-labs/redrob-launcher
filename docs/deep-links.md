# redrob:// links

Web pages (Redrob Chat, the website) open Redrob Launcher on one app with a `redrob://` link.

| Link | What the launcher does |
|---|---|
| `redrob://open/<app>` | Starts the app if it is installed. If it isn't, shows it with its Install button focused. |
| `redrob://install/<app>` | Shows the app and starts its install (or its update). An app that is up to date is just shown. |

`<app>` is the catalog id (`crates/core/src/catalog.rs`) with or without its `redrob-` prefix:
`cad`, `redrob-cad`, `cowork`, `office`, `query`, `canvas`, `recall`, `shape`, `design`, `film`,
`browser`, `code`. Case doesn't matter, a trailing `/` is allowed, and a query string or fragment is
ignored. Anything else (an unknown app, another action, a second path segment, `%`-escapes, a link
longer than 256 characters) opens the launcher's home and does nothing more. The parser is
`crates/core/src/deep_link.rs`; its tests are the spec.

```html
<a href="redrob://open/cad">Open Redrob CAD</a>
<a href="redrob://install/cowork">Install Redrob Cowork</a>
```

The browser asks the person before it hands a link to the launcher. Nothing from the link reaches
a shell or a path: it names one catalog app, and the launcher does what its own buttons do.

## How a link reaches the launcher

- **Linux:** the desktop entry declares `MimeType=x-scheme-handler/redrob;` and `Exec=… %u`. The
  deb and rpm refresh the desktop database after install. The AppImage's `AppRun` writes the entry
  to `~/.local/share/applications`, keeping `%u`; a tarball install gets one from the launcher
  itself. On every start the launcher makes its entry the default handler with `xdg-mime default
  ai.redrob.launcher.desktop x-scheme-handler/redrob` (`crates/native/src/url_scheme.rs`).
- **macOS:** `Info.plist` declares the `redrob` scheme (`CFBundleURLTypes`). macOS delivers the
  link as a `kAEGetURL` Apple Event, which `apps/redrob-launcher/src/url_event.rs` receives.
- **Windows:** `HKCU\Software\Classes\redrob` (`URL Protocol`, `shell\open\command` =
  `"<exe>" "%1"`), written by the setup and again by the launcher on every start when it points
  elsewhere (portable zip, moved install). Per-user: no admin rights. The uninstaller removes it.

Development builds (`cargo run`) never register, so they don't take links from an installed
launcher.

## When the launcher is already running

The OS starts a second launcher with the link as its argument (Linux, Windows). It finds the
running one on its loopback port (`crates/native/src/single_instance.rs`), says hello as usual,
and then sends `redrob-launcher:link <url>`; the running launcher comes forward and acts on it, and
the second one exits. A newer second launcher takes over as before and acts on its own link. A
running launcher from before links ignores the message and just comes forward. On macOS the
running launcher gets the Apple Event itself.

An install or launch waits until the launcher knows whether the app is installed and what its
latest release is, so a link that starts the launcher works too.
