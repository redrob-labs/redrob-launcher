# redrob-launcher:// links

Web pages (Redrob Chat, the website) open Redrob Launcher on one app with a `redrob-launcher://`
link. The older `redrob://` spelling still works and is still registered, but pages link with
`redrob-launcher://` first: `redrob` is not a scheme only the launcher has claimed. Redrob Browser
0.2 to 0.4 registered it on Windows as its own launch scheme (Chromium's direct-launch scheme, now
`redrob-browser`), and that claim survives an update, so `redrob://` can open the browser instead.
The examples below use `redrob://`; `redrob-launcher://` takes every form.

| Link | What the launcher does |
|---|---|
| `redrob://open/<app>` | Starts the app if it is installed. If it isn't, shows it with its Install button focused. |
| `redrob://install/<app>` | Shows the app and starts its install (or its update). An app that is up to date is just shown. |

`<app>` is the catalog id (`crates/core/src/catalog.rs`) with or without its `redrob-` prefix:
`cad`, `redrob-cad`, `cowork`, `office`, `canvas`, `shape`, `design`, `film`,
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

`redrob://open/code` (like the Code card's Open) also makes sure Redrob Code's shared daemon runs
with desk on, so Redrob Chat on the web and the phone app can reach this PC: see
[engine.md](engine.md#using-this-pc-from-the-web-and-the-phone).

`redrob-launcher://open/code?engine=1` is Redrob Chat's "turn on Redrob Code": it doesn't start
Code's terminal. It starts the AI engine with desk on (as above) and shows the AI engine page, where
the desk's sign-in and devices are. Code not installed: shown with its Install button, as for open.

## How a link reaches the launcher

- **Linux:** the desktop entry declares
  `MimeType=x-scheme-handler/redrob-launcher;x-scheme-handler/redrob;` and `Exec=… %u`. The
  deb and rpm refresh the desktop database after install. The AppImage's `AppRun` writes the entry
  to `~/.local/share/applications`, keeping `%u`; a tarball install gets one from the launcher
  itself. On every start the launcher makes its entry the default handler with `xdg-mime default
  ai.redrob.launcher.desktop x-scheme-handler/<scheme>` for both (`crates/native/src/url_scheme.rs`).
- **macOS:** `Info.plist` declares the `redrob-launcher` and `redrob` schemes (`CFBundleURLTypes`). macOS delivers the
  link as a `kAEGetURL` Apple Event, which `apps/redrob-launcher/src/url_event.rs` receives.
- **Windows:** `HKCU\Software\Classes\redrob-launcher` and `...\Classes\redrob` (`URL Protocol`,
  `shell\open\command` = `"<exe>" "%1"`), written by the setup and again by the launcher on every
  start when they point elsewhere (portable zip, moved install). Per-user: no admin rights. The
  uninstaller removes them. On every start the launcher also forgets a recorded user choice for
  either scheme (`...\Shell\Associations\UrlAssociations\<scheme>\UserChoice`) that names another
  app, and drops an old Redrob Browser's `Capabilities\URLAssociations` value for them (a
  `RedrobHTML` ProgId): a recorded choice beats the class key.

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
