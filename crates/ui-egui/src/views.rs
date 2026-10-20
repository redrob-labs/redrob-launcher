// ADDED-LINE-RATIONALE feca990f29dae812:1: Gives `views` direct access to `redrob_launcher_core / time` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 6aedffd285e15fc5:1: Gives `views` direct access to `crate / glyphs / Icon` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 0eb9600b03ec026e:1: Gives `views` direct access to `crate / settings / Filter` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 38101cf29987224c:1: Gives `views` direct access to `crate / theme / Tokens` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 6d4f6d035d2251cb:1: Supports `Primary` while preserving the nearby constraint: it is wider).
// ADDED-LINE-RATIONALE eeb5feb32d4f8c89:1: Supports `Primary` while preserving the nearby constraint: A tile's height: padding, the icon row, the version line and the actions row.
// ADDED-LINE-RATIONALE 9eaa5747b73f9db2:1: Supports `Primary` while preserving the nearby constraint: The space between tiles (DS card gap in a dense grid: space-4).
// ADDED-LINE-RATIONALE 643f0b48fa7ba895:1: Supports `Primary` while preserving the nearby constraint: DS card padding, dense (space-4).
// ADDED-LINE-RATIONALE bd5d091f28977474:1: Supports `Primary` while preserving the nearby constraint: The app icon on a tile (was 48).
// ADDED-LINE-RATIONALE c1ecc2d215ce5b95:1: Supports `Primary` while preserving the nearby constraint: The icon buttons beside an app's main action (pause, cancel, More actions): DS IconButton sm.
// ADDED-LINE-RATIONALE 3333bdc40b001e4f:1: Supports `Primary` while preserving the nearby constraint: Where the group logo leads (Redrob Cowork's title bar does the same).
// ADDED-LINE-RATIONALE 49bde4f95a5eb558:1: Supports `Primary` while preserving the nearby constraint: narrow window.
// ADDED-LINE-RATIONALE 53e25bec5d50f186:1: Freezes `SEARCH_W` for `Primary` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4bba716e5b7f270f:1: Supports `Primary` while preserving the nearby constraint: Empty title bar left of the controls, so the window can always be dragged.
// ADDED-LINE-RATIONALE ca62213837e9b938:1: Supports `UpdateStatus` while preserving the nearby constraint: The update-check status lines Settings shows (see [`LauncherUi::update_status`]).
// ADDED-LINE-RATIONALE 2f3465abcdb3ae69:1: Supports `UpdateStatus` while preserving the nearby constraint: "Checked 3 min ago · using Redrob API", "Checking for updates…" or "Not checked yet".
// ADDED-LINE-RATIONALE dcd2d228dd4bfbbd:1: Supports `UpdateStatus` while preserving the nearby constraint: GitHub's rate-limit notice, only while the limit applies.
// ADDED-LINE-RATIONALE 4ef4abed5b9d3b99:1: Supports `UpdateStatus` while preserving the nearby constraint: "Installing for Windows x64" and friends.
// ADDED-LINE-RATIONALE 22b886c8dbac3da5:4: Bounds the `UpdateStatus` implementation here so `platform / String` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a74f8a99b7f11d63:1: Supports `UpdateStatus` while preserving the nearby constraint: A run of cards, under an optional meta-line title.
// ADDED-LINE-RATIONALE 235af52a4656cb91:1: Freezes `bar_w` for `header` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 62b2269578fc3dd3:1: Prevents `header` from applying its side effect outside the `bar` state.
// ADDED-LINE-RATIONALE 28c83fac77ab7354:1: Freezes `TitleBar` for `header` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ce1a8f304191eb7f:1: Delegates `header`'s `widgets / window_buttons / ui` work to `widgets::window_buttons` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dc624c1b115daa70:1: Delegates `header`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9c666adf7d50e0b8:1: Freezes `TitleBar` for `header` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b7877488d3117edf:1: Freezes `TitleBar` for `header` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5fdee56e75763a40:1: Delegates `header`'s `chrome_cluster / ui / bar_w` work to `self.chrome_cluster` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a270e30a5403c2cb:1: Supports `chrome_cluster` while preserving the nearby constraint: window (never under [`SEARCH_MIN_W`]), leaving the bar room to drag.
// ADDED-LINE-RATIONALE 184479704da8f4c5:1: Freezes `gap` for `chrome_cluster` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 72747cc8b0f5a39a:1: Freezes `updates` for `chrome_cluster` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 486efdd1f78c21aa:1: Freezes `update` for `chrome_cluster` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 30cb8bc98dd83e8e:1: Freezes `if` for `chrome_cluster` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b4deecd23d42452d:2: Makes `updates / tl / Update` an explicit part of `chrome_cluster` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 8b4047cfa84ed909:1: Freezes `update_w` for `chrome_cluster` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d51a229c6f2fd8f9:1: Freezes `room` for `chrome_cluster` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f51f1cb4deacccf5:1: Freezes `search_w` for `chrome_cluster` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b305c890bf493eae:1: Supports `chrome_cluster` while preserving the nearby constraint: Typing here from another page shows the apps it finds.
// ADDED-LINE-RATIONALE ffc1c12dd89eed83:1: Delegates `chrome_cluster`'s `set_page / Page / Apps` work to `self.set_page` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 989c15d468e521e5:3: Bounds the `chrome_cluster` implementation here so `set_page / Page / Apps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE bbb472e009979e24:1: Delegates `chrome_cluster`'s `ui / add_space / gap` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c0b421763c248b70:1: Prevents `chrome_cluster` from applying its side effect outside the `widgets / logo_button / ui` state.
// ADDED-LINE-RATIONALE 5947b6aa7314cba7:1: Delegates `chrome_cluster`'s `ui / ctx / open_url` work to `ui.ctx` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 989c15d468e521e5:4: Bounds the `chrome_cluster` implementation here so `ui / ctx / open_url` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 9c682cc10a2fdee6:1: Freezes `if` for `chrome_cluster` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 177cb34ceb19fa04:1: Delegates `chrome_cluster`'s `ui / add_space / gap` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f62d6aa6a9be11f0:1: Supports `chrome_cluster` while preserving the nearby constraint: Tinted, not filled: the chrome stays quiet (Cowork's title bar has no filled button).
// ADDED-LINE-RATIONALE b6733d9a3feccb59:1: Delegates `chrome_cluster`'s `update_all` work to `self.update_all` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b232890f187a0805:2: Bounds the `chrome_cluster` implementation here so `update_all` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 989c15d468e521e5:5: Bounds the `chrome_cluster` implementation here so `update_all` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE af58771aeaabb4c5:3: Bounds the `chrome_cluster` implementation here so `Updates / every / installed` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d9f7badcb02ca9b1:1: Supports `update_all` while preserving the nearby constraint: Updates every installed app that has an update and isn't busy.
// ADDED-LINE-RATIONALE cde326a182e09731:1: Freezes `ids` for `update_all` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 08c34429cbfe7984:1: Makes `entries` an explicit part of `update_all` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE df3bf6672b326fcd:1: Delegates `update_all`'s `iter` work to `iter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 68c5c4758ca32624:1: Delegates `update_all`'s `filter / e / e` work to `filter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0cc5d03e0a2c78b3:1: Delegates `update_all`'s `map / e / e` work to `map` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2263c15bb900ed1e:1: Delegates `update_all`'s `collect` work to `collect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 03cccd7c66096e6e:1: Applies `update_all` consistently to every `id / in / ids` item rather than handling only the first match.
// ADDED-LINE-RATIONALE da6c9836d78656a8:1: Delegates `update_all`'s `send / Command / Install` work to `self.send` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 989c15d468e521e5:6: Bounds the `update_all` implementation here so `send / Command / Install` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE af58771aeaabb4c5:4: Bounds the `update_all` implementation here so `send / Command / Install` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 542e2cc5a2525a98:1: Delegates `toolbar`'s `widgets / alert_frame_dense / t` work to `widgets::alert_frame_dense` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e04fcd46f3d506c4:1: Supports `toolbar` while preserving the nearby constraint: A row is at least interact_size tall: 32 here (the Sm buttons), not the page's 40.
// ADDED-LINE-RATIONALE 9a4bc1655ac7bd2d:1: Delegates `toolbar`'s `ui / horizontal / ui` work to `ui.horizontal` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6d0bd7153b6407ea:1: Supports `toolbar` while preserving the nearby constraint: 32 px row (the buttons and the dismiss are Sm).
// ADDED-LINE-RATIONALE 662a00f586e5d1f1:1: Delegates `toolbar`'s `crate / glyphs / paint` work to `crate::glyphs::paint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5e73564bd617c37e:1: Delegates `toolbar`'s `ui / add_space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cba4b687af27861c:1: Delegates `toolbar`'s `ui / label` work to `ui.label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fd112d8a46733772:1: Delegates `toolbar`'s `RichText / new / fmt` work to `RichText::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6818dfd601882d76:1: Delegates `toolbar`'s `family / theme / semibold` work to `family` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 950cda68c3c5fde0:1: Keeps `color / t / ink_strong` in `toolbar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 0b8ce687ad83de56:1: Bounds the `toolbar` implementation here so `color / t / ink_strong` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 27fec754416e4ebb:1: Delegates `toolbar`'s `ui / with_layout / egui` work to `ui.with_layout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 76e7dcb749a9c898:1: Freezes `if` for `toolbar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0d0f9a2a1fc3f1d0:1: Freezes `let` for `toolbar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 78738804d488b496:1: Freezes `JobPhase` for `toolbar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ac9928c9b9d20a94:1: Keeps `done / as / f32` in `toolbar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f5687e66bd91ca6f:1: Delegates `toolbar`'s `fmt` work to `fmt` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2c93adcf3d528ec7:1: Keeps `tl / Downloading / done` in `toolbar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 2bdf1e52ae263180:1: Keeps `done / widgets / human_bytes` in `toolbar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE c16b6b9b6d8d6f9f:1: Keeps `JobPhase / Downloading / JobPhase` in `toolbar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 3be8dfde4764c10f:1: Freezes `JobPhase` for `toolbar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3044a0768a377e3e:1: Freezes `JobPhase` for `toolbar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2aa65e3d8624a247:1: Freezes `JobPhase` for `toolbar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 021d528e4b5aa98b:1: Freezes `JobPhase` for `toolbar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4848879d5316bbda:1: Bounds the `toolbar` implementation here so `JobPhase / Installing / JobPhase` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e675efbd2f0132c5:1: Supports `toolbar` while preserving the nearby constraint: Another app's install can hang on a password prompt: the wait can be called off.
// ADDED-LINE-RATIONALE 149160313897e12e:1: Delegates `toolbar`'s `widgets / icon_button / ui` work to `widgets::icon_button` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 73b055e511bf98ff:1: Delegates `toolbar`'s `on_hover_text / tl / Stop` work to `on_hover_text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e0e4fff2c869503a:1: Delegates `toolbar`'s `clicked` work to `clicked` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a3ec0d4bb0be1953:1: Bounds the `toolbar` implementation here so `clicked` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2cde3fc8b40f6d34:1: Delegates `toolbar`'s `send / Command / Cancel` work to `self.send` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6d8378e875a49b32:1: Delegates `toolbar`'s `widgets / progress / ui` work to `widgets::progress` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e2beafbc3230383d:1: Stops `toolbar` at the `widgets / progress / ui` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 1b9c27340aa34565:1: Bounds the `toolbar` implementation here so `widgets / icon_button / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f3d11b25dda17b37:1: Prevents `toolbar` from applying its side effect outside the `widgets / icon_button / ui` state.
// ADDED-LINE-RATIONALE 531889c32f7d50de:1: Freezes `self` for `toolbar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1b9c27340aa34565:2: Bounds the `toolbar` implementation here so `launcher_update` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a3ca6a4a286b1551:1: Prevents `toolbar` from applying its side effect outside the `caps / self_update` state.
// ADDED-LINE-RATIONALE 2aac7114dca66f30:1: Prevents `toolbar` from applying its side effect outside the `widgets / button_sized / ui` state.
// ADDED-LINE-RATIONALE e01558134f82154f:1: Delegates `toolbar`'s `on_hover_text / tl / Downloads` work to `on_hover_text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9541898d9989f69a:1: Delegates `toolbar`'s `clicked` work to `clicked` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a3ec0d4bb0be1953:2: Bounds the `toolbar` implementation here so `clicked` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 9a7e241499319ae4:1: Delegates `toolbar`'s `send / Command / UpdateLauncher` work to `self.send` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2a9d32bd6d76e1b3:1: Makes `url / platform / and_then` an explicit part of `toolbar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 5678c8fb46faaa8c:1: Freezes `url` for `toolbar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7edc3af159af2326:1: Prevents `toolbar` from applying its side effect outside the `widgets / button_sized / ui` state.
// ADDED-LINE-RATIONALE b0fb527772ab6ea5:1: Delegates `toolbar`'s `ui / ctx / open_url` work to `ui.ctx` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1b9c27340aa34565:3: Bounds the `toolbar` implementation here so `ui / add_space` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 961b015f3c548f22:1: Delegates `toolbar`'s `ui / add_space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f7520af6a2365822:1: Delegates `toolbar`'s `widgets / hud_link / ui` work to `widgets::hud_link` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 23844a133122b74d:2: Makes `ui / add_space / theme` an explicit part of `toolbar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 63fa232a92be598f:1: Delegates `toolbar`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a25e45466b664fab:1: Prevents `toolbar` from applying its side effect outside the `widgets / icon_button_busy / ui` state.
// ADDED-LINE-RATIONALE 10842f16475c9133:1: Delegates `toolbar`'s `check_now` work to `self.check_now` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 23844a133122b74d:3: Makes `check_now` an explicit part of `toolbar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE b4deecd23d42452d:3: Makes `check_now` an explicit part of `toolbar` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE af58771aeaabb4c5:5: Bounds the `toolbar` implementation here so `The / DS / EmptyState` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 784135bdfbdbc852:1: Supports `empty_state` while preserving the nearby constraint: brand-subtle disc with the search glyph, a title, what to relax, and the way back.
// ADDED-LINE-RATIONALE 7d27916cf6469de4:2: Freezes `t` for `empty_state` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e81a1e346b98cbe0:1: Freezes `query` for `empty_state` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bd956cf801c48f09:1: Delegates `empty_state`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2017e4ab56981bff:1: Delegates `empty_state`'s `ui / vertical / ui` work to `ui.vertical` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ecce7447adbc9652:1: Freezes `ui` for `empty_state` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b26fcf6c2bf959be:1: Freezes `let` for `empty_state` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 69da364ff38614e4:1: Delegates `empty_state`'s `ui / painter / circle_filled` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 84ffd485f6966b8d:1: Delegates `empty_state`'s `crate / glyphs / paint` work to `crate::glyphs::paint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b5c377dd26174a32:1: Delegates `empty_state`'s `ui / label / widgets` work to `ui.label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 59fc5345f30b2a6a:1: Freezes `text` for `empty_state` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 11077838ed733091:1: Delegates `empty_state`'s `tl / No / app` work to `tl` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f1aa7c49dcd165d3:1: Makes `tl / No / app` an explicit part of `empty_state` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 931eb8e4931fe9b1:1: Delegates `empty_state`'s `fmt / tl / Nothing` work to `fmt` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 45e722a6975c015a:1: Bounds the `empty_state` implementation here so `fmt / tl / Nothing` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f5b383207534b815:1: Delegates `empty_state`'s `ui / scope / ui` work to `ui.scope` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 365151bf39516e00:1: Delegates `empty_state`'s `ui / set_max_width` work to `ui.set_max_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dca3ebcd75872a0a:1: Delegates `empty_state`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 23844a133122b74d:4: Makes `ui / add / egui` an explicit part of `empty_state` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 50c3273bf7b7548d:1: Delegates `empty_state`'s `ui / horizontal / ui` work to `ui.horizontal` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f8e6a4252bcdb5ad:1: Prevents `empty_state` from applying its side effect outside the `query / is_empty / widgets` state.
// ADDED-LINE-RATIONALE 422e0531a44ef263:1: Delegates `empty_state`'s `search / clear` work to `self.search.clear` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ddb0e335f470f125:3: Bounds the `empty_state` implementation here so `search / clear` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d88848103141e5ea:1: Freezes `if` for `empty_state` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0b70e5b45f16c9ac:1: Freezes `self` for `empty_state` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e1c80fd81f165798:1: Delegates `announcements`'s `widgets / alert_frame / t` work to `widgets::alert_frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f3d11b25dda17b37:2: Prevents `announcements` from applying its side effect outside the `widgets / icon_button / ui` state.
// ADDED-LINE-RATIONALE bc57ed5f7e5ab97d:1: Supports `update_status` while preserving the nearby constraint: are installed (or downloaded) for.
// ADDED-LINE-RATIONALE 38476d71297bf585:1: Freezes `checked` for `update_status` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 22b9b56d06f882d6:1: Freezes `rate_limit` for `update_status` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 40369b2a544d2162:1: Delegates `update_status`'s `fmt / tl / GitHub` work to `fmt` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b9ad32b259dc7573:1: Makes `UpdateStatus / checked / rate_limit` an explicit part of `update_status` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 65c5e0aeaeaa3215:1: Supports `grid` while preserving the nearby constraint: DS ConnectorCard tiles, 16 px apart (no shared hairlines).
// ADDED-LINE-RATIONALE 7956f12c29c12804:1: Freezes `card_w` for `grid` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ac04fffd7193b955:1: Supports `grid` while preserving the nearby constraint: Every app, unsearched: on its shelf, in page order. One voice across the three headings.
// ADDED-LINE-RATIONALE 70df5c8e70379ff1:1: Gives `grid` direct access to `crate / shelves / Shelf` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE bff1031381d288ad:1: Makes `Shelf / ALL` an explicit part of `grid` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE c90dadf8db2282e3:1: Delegates `grid`'s `map / shelf` work to `map` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ffb9bce9147d8daa:1: Freezes `title` for `grid` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 79a54d0203bf61fb:1: Freezes `Shelf` for `grid` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fcc7bcf73cfd7150:1: Freezes `Shelf` for `grid` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 76c7b4e4b24e9946:1: Freezes `Shelf` for `grid` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0a8aee87aca8f61a:1: Bounds the `grid` implementation here so `Shelf / Creative / tl` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 12f17303f2f6c4ee:1: Freezes `list` for `grid` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 67e3dd11f517fdfe:1: Delegates `grid`'s `title / list` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8cc212d63eeae675:1: Freezes `if` for `grid` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 50902fc6efdbaf94:1: Delegates `grid`'s `widgets / shelf_heading / ui` work to `widgets::shelf_heading` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0068d2092a5ec82f:1: Delegates `grid`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bf20bf2d5fe4f272:1: Freezes `height` for `grid` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ce8e0e4cf6704701:1: Freezes `min` for `grid` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7ced1e2aa094788c:1: Delegates `grid`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 43a3734c90f331aa:1: Freezes `status` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 94169254d944e698:1: Supports `card` while preserving the nearby constraint: explains itself on hover.
// ADDED-LINE-RATIONALE 9b4689949b6ce9a3:1: Freezes `let` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f65785e15803c4cd:1: Freezes `card_name` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a9abb36aae586337:1: Delegates `card`'s `card / widget_info / egui` work to `card.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 33e67030fc81a73b:1: Freezes `card` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3585007c2360454b:1: Freezes `Some` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 06e9d6b434b3951d:1: Freezes `None` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 365af5ee3d588c3a:4: Bounds the `card` implementation here so `card` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2b19af793790829c:1: Delegates `card`'s `card / context_menu / ui` work to `card.context_menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1009b2e6536b851e:1: Delegates `card`'s `ui / set_min_width` work to `ui.set_min_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 10d58d80a81c6653:1: Supports `card` while preserving the nearby constraint: DS menu items are about 36 px.
// ADDED-LINE-RATIONALE 03cad1efbc1e28b6:1: Delegates `card`'s `menu_items / ui / entry` work to `self.menu_items` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b4deecd23d42452d:6: Makes `menu_items / ui / entry` an explicit part of `card` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 1c1e60b606c94fee:1: Freezes `k` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f5b73716ce5231c7:1: Freezes `radius` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b107f3a1920d7595:1: Freezes `painter` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9d7abdf37cce75d9:1: Prevents `card` from applying its side effect outside the `k` state.
// ADDED-LINE-RATIONALE 71ed22afe69d188f:1: Freezes `shadow` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 17523ba6dc4072da:1: Freezes `shadow` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bbfec7ddc98f0523:1: Delegates `card`'s `painter / add / shadow` work to `painter.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9e5436b230b9e1eb:1: Delegates `card`'s `painter / rect / rect` work to `painter.rect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b5488a8888c5a5f7:1: Freezes `inner` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e0e8330a472da2b7:1: Freezes `ICON` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 80ae9b5ec1bad283:1: Freezes `let` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 84ce91305beca779:1: Keeps `FontId / new / theme` in `card`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 05df3aed8ba6a499:1: Keeps `FontId / proportional / theme` in `card`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE caa166ec3380795f:1: Keeps `FontId / proportional / theme` in `card`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 457a22a696ff2f7a:1: Bounds the `card` implementation here so `FontId / proportional / theme` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7be6cf75c0d739e6:1: Freezes `GAP` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE de6ee81827cff717:1: Delegates `card`'s `egui / Image / new` work to `egui::Image::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 63fa232a92be598f:2: Delegates `card`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 50b4a409aa8c80ff:1: Delegates `card`'s `widgets / wordmark / ui` work to `widgets::wordmark` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 509ed8e7d00a646f:1: Delegates `card`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 27767c03efc1c47c:1: Supports `card` while preserving the nearby constraint: the tooltip.
// ADDED-LINE-RATIONALE 31f04ddf290f3f0d:1: Freezes `font` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 38c842691a5c9244:1: Freezes `fits` for `card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c6b4113353a762da:1: Prevents `card` from applying its side effect outside the `fits` state.
// ADDED-LINE-RATIONALE 71674ea732d3b31b:1: Delegates `card`'s `ui / label / RichText` work to `ui.label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0cc0c74d542f8333:3: Makes `ui / label / RichText` an explicit part of `card` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE a03ae266bb0edb88:1: Delegates `card`'s `ui / label / RichText` work to `ui.label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 989c15d468e521e5:11: Bounds the `card` implementation here so `ui / label / RichText` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d07cbd6903ac4dcb:1: Supports `actions` while preserving the nearby constraint: the primary action; right-click remains a pointer shortcut to the same commands.
// ADDED-LINE-RATIONALE afa4bf1668588a56:1: Freezes `downloading` for `actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 531031d7a8d6927f:1: Supports `actions` while preserving the nearby constraint: before Pause, Cancel, or More can overlap at compact card widths.
// ADDED-LINE-RATIONALE dc098444fe4feddd:1: Makes `trailing_controls / downloading` an explicit part of `actions` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 0e1b9aa6c5fbe36c:1: Makes `phase / JobPhase / Waiting` an explicit part of `actions` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE d36dc144f973b236:1: Makes `phase / JobPhase / Waiting` an explicit part of `actions` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE f1aa7c49dcd165d3:2: Makes `phase / JobPhase / Waiting` an explicit part of `actions` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE ecc6d24507a0a9c7:1: Makes `bar_w / width / trailing_controls` an explicit part of `actions` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 45e722a6975c015a:3: Bounds the `actions` implementation here so `bar_w / width / trailing_controls` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 32a57e39b054cbe1:1: Freezes `bar_w` for `actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 621a0e0a44b57746:1: Freezes `caption` for `actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE de0d600143d38b19:1: Makes `f / caption / t` an explicit part of `actions` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE e7d1abeac700eabe:1: Freezes `JobPhase` for `actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 158e3b9fd9b8f9ca:1: Freezes `JobPhase` for `actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 25844aef916714f7:1: Freezes `JobPhase` for `actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 467ebec1c227835f:1: Freezes `JobPhase` for `actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0779b73bb53bca46:1: Delegates `actions`'s `widgets / progress / ui` work to `widgets::progress` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2cf2e4c907bc7493:1: Prevents `actions` from applying its side effect outside the `downloading / widgets / icon_button` state.
// ADDED-LINE-RATIONALE 3a2707040d1b6fca:1: Delegates `actions`'s `widgets / icon_button / ui` work to `widgets::icon_button` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8658c58ad003f5ba:1: Delegates `actions`'s `clicked` work to `clicked` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3949550629f40c49:1: Delegates `actions`'s `menu / ui / entry` work to `self.menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9363ceb445d89a40:1: Supports `actions` while preserving the nearby constraint: Resume yields space to both Discard and the keyboard-reachable More menu.
// ADDED-LINE-RATIONALE acd0734c3e841c27:1: Prevents `actions` from applying its side effect outside the `widgets / icon_button / ui` state.
// ADDED-LINE-RATIONALE bed85149b2e8f79c:1: Prevents `actions` from applying its side effect outside the `widgets / button / ui` state.
// ADDED-LINE-RATIONALE 4ab28d9d2dd999df:1: Delegates `menu`'s `widgets / more_menu / ui` work to `widgets::more_menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE af58771aeaabb4c5:13: Bounds the `menu` implementation here so `widgets / more_menu / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ff9b7b1d996fa80e:1: Supports `menu_items` while preserving the nearby constraint: An app's menu: from the … button or a right-click on its card.
// ADDED-LINE-RATIONALE 989c0fd468e517b3:3: Bounds the `menu_items` implementation here so `reveal / caps / reveal_label` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 69e9107795a2bbc3:2: Supports `menu_items` while preserving the nearby constraint: DS Menu: the destructive action last, in status-danger.
// ADDED-LINE-RATIONALE b3fb5ac43eb8947f:1: Delegates `menu_items`'s `ui / separator` work to `ui.separator` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 996ee76f4f36bd11:1: Prevents `menu_items` from applying its side effect outside the `ui / button / RichText` state.
// ADDED-LINE-RATIONALE 16b998741c4772ef:1: Freezes `self` for `menu_items` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fd533bb179d2c5da:5: Delegates `menu_items`'s `ui / close` work to `ui.close` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ddb0e335f470f125:18: Bounds the `menu_items` implementation here so `ui / close` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b232890f187a0805:24: Bounds the `menu_items` implementation here so `ui / close` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 989c15d468e521e5:17: Bounds the `menu_items` implementation here so `ui / close` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7e642f7b92af8d46:1: Delegates `version_line`'s `version_line_dated / e / status` work to `version_line_dated` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 22b886c8dbac3da5:6: Bounds the `version_line` implementation here so `version_line_dated / e / status` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a8e85ccd26738d35:1: Supports `version_line_dated` while preserving the nearby constraint: [`version_line`], with or without the release date.
// ADDED-LINE-RATIONALE 5017a3f6f3b2fd07:1: Freezes `released` for `version_line_dated` so every later calculation in this path observes the same input.
//! The header, toolbar and the grid of cards, laid out like the apps page on
//! the Redrob design system: hairline cells, quiet labels and square controls.

use egui::{Color32, CornerRadius, FontId, Rect, RichText, Sense, Stroke, Ui, vec2};
use redrob_launcher_core::platform::pick_asset;
use redrob_launcher_core::protocol::{Command, JobPhase};
use redrob_launcher_core::status::AppStatus;
use redrob_launcher_core::time;

use crate::app::{Dialog, LauncherUi, Page, TitleBar};
use crate::glyphs::Icon;
use crate::i18n::{current, fmt, t, tn, tr_ctx};
use crate::model::{self, AppEntry};
use crate::settings::Filter;
use crate::theme::{self, Tokens};
use crate::widgets::{self, Cell, Kind};

/// Translate a relative timestamp ("just now", "5 min ago", …) into the current language.
pub(crate) fn ago_text(secs: i64, now: i64) -> String {
    match time::ago_kind(secs, now) {
        time::TimeAgo::JustNow => t("just now").to_string(),
        time::TimeAgo::Minutes(n) => tn(n, "{n} min ago", "{n} min ago"),
        time::TimeAgo::Hours(n) => tn(n, "{n} h ago", "{n} h ago"),
        time::TimeAgo::Yesterday => t("yesterday").to_string(),
        time::TimeAgo::Days(n) => tn(n, "{n} day ago", "{n} days ago"),
        time::TimeAgo::Date(secs) => {
            let (y, m, d) = time::civil_from_unix(secs);
            let month = tr_ctx(current(), "month", time::month_short_en(m));
            fmt(tl!("{month} {day}, {year}"), &[("month", month), ("day", &d.to_string()), ("year", &y.to_string())])
        }
    }
}

/// Translate an absolute date ("Oct 8, 2026" → "8 окт. 2026") into the current language.
pub(crate) fn date_text(secs: i64) -> String {
    let (y, m, d) = time::civil_from_unix(secs);
    let month = tr_ctx(current(), "month", time::month_short_en(m));
    fmt(tl!("{month} {day}, {year}"), &[("month", month), ("day", &d.to_string()), ("year", &y.to_string())])
}

/// What the main button of an app does.
enum Primary {
    Install,
    /// The download is already here; only the install step is left. Carries the button's label.
    Retry(String),
    Update(String),
    Open,
    Download(String),
    Unavailable(&'static str),
}

/// Height of the header bar (the site's nav).
pub(crate) const HEADER_H: f32 = 48.0;

/// Three tiles across the main area beside the sidebar at the default 1180 px window (four once
/// it is wider).
const CARD_MIN_W: f32 = 240.0;
/// A tile's height: padding, the icon row, the version line and the actions row.
const CARD_H: f32 = 172.0;
/// The space between tiles (DS card gap in a dense grid: space-4).
const CARD_GAP: f32 = 16.0;
/// DS card padding, dense (space-4).
const CARD_PAD: f32 = 16.0;
/// The app icon on a tile (was 48).
const CARD_ICON: f32 = 40.0;
/// The icon buttons beside an app's main action (pause, cancel, More actions): DS IconButton sm.
pub(crate) const ACTION_ICON: f32 = theme::control::SM;
/// Room macOS's traffic lights take at the left of the header.
const TRAFFIC_LIGHTS_W: f32 = 72.0;
/// Where the group logo leads (Redrob Cowork's title bar does the same).
const CONSOLE_URL: &str = "https://console.redrob.ai";
/// The search field's width: Cowork's `clamp(260px, 29vw, 416px)`, and the least it shrinks to in a
/// narrow window.
const SEARCH_MIN_W: f32 = 160.0;
const SEARCH_W: std::ops::RangeInclusive<f32> = 260.0..=416.0;
/// Empty title bar left of the controls, so the window can always be dragged.
const DRAG_MIN_W: f32 = 24.0;

/// The update-check status lines Settings shows (see [`LauncherUi::update_status`]).
pub(crate) struct UpdateStatus {
    /// "Checked 3 min ago · using Redrob API", "Checking for updates…" or "Not checked yet".
    pub checked: String,
    /// GitHub's rate-limit notice, only while the limit applies.
    pub rate_limit: Option<String>,
    /// "Installing for Windows x64" and friends.
    pub platform: String,
}

/// A run of cards, under an optional meta-line title.
type Section = (Option<&'static str>, Vec<&'static str>);

impl LauncherUi {
    /// The title bar (Redrob Cowork's): with a custom or macOS title bar the whole bar drags the
    /// window outside its controls. At its right end, right to left: the window buttons (custom
    /// bar), the search, the Redrob group logo (console.redrob.ai) and, with updates, Update. With
    /// `TitleBar::Native` (the OS's bar, or the web) the same cluster is a 48 px row over the page.
    pub(crate) fn header(&mut self, ui: &mut Ui, bar: TitleBar) {
        let h = HEADER_H;
        if bar != TitleBar::Native {
            widgets::title_bar_drag(ui);
        }
        let bar_w = ui.max_rect().width();
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            if bar == TitleBar::MacOs {
                ui.add_space(TRAFFIC_LIGHTS_W);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                match bar {
                    TitleBar::Custom => {
                        widgets::window_buttons(ui, h);
                        ui.add_space(theme::space::S2);
                    }
                    TitleBar::MacOs => ui.add_space(theme::space::S4),
                    TitleBar::Native => {}
                }
                self.chrome_cluster(ui, bar_w);
            });
        });
    }

    /// Search, the group logo and Update all, right to left. The search shrinks first in a narrow
    /// window (never under [`SEARCH_MIN_W`]), leaving the bar room to drag.
    fn chrome_cluster(&mut self, ui: &mut Ui, bar_w: f32) {
        let gap = theme::space::S2;
        let updates = if self.caps.manage_apps { self.count(Filter::Updates) } else { 0 };
        let update = (updates > 0).then(|| {
            if updates == 1 { tl!("Update 1 app").to_string() } else { fmt(tl!("Update all ({count})"), &[("count", &updates.to_string())]) }
        });
        let update_w = update.as_ref().map_or(0.0, |l| widgets::button_width(ui, l, widgets::Size::Sm) + gap);
        let room = ui.available_width() - theme::control::SM - gap - update_w - DRAG_MIN_W;
        let search_w = (bar_w * 0.29).clamp(*SEARCH_W.start(), *SEARCH_W.end()).min(room).max(SEARCH_MIN_W);
        // Typing here from another page shows the apps it finds.
        if widgets::search_field(ui, &mut self.search, tl!("Search apps"), search_w, "apps").changed() && self.page != Page::Apps {
            self.set_page(Page::Apps);
        }
        ui.add_space(gap);
        if widgets::logo_button(ui, crate::icons::redrob_mark(), tl!("Go to console.redrob.ai")).clicked() {
            ui.ctx().open_url(egui::OpenUrl::new_tab(CONSOLE_URL));
        }
        if let Some(label) = update {
            ui.add_space(gap);
            // Tinted, not filled: the chrome stays quiet (Cowork's title bar has no filled button).
            if widgets::button_sized(ui, &label, Kind::Tinted, widgets::Size::Sm, 0.0).clicked() {
                self.update_all();
            }
        }
    }

    /// Updates every installed app that has an update and isn't busy.
    fn update_all(&mut self) {
        let ids: Vec<&str> = self
            .entries
            .iter()
            .filter(|e| e.status(self.platform, true) == AppStatus::UpdateAvailable && e.job.is_none())
            .map(|e| e.app.id)
            .collect();
        for id in ids {
            self.send(Command::Install { app: id.to_string() });
        }
    }

    pub(crate) fn toolbar(&mut self, ui: &mut Ui) {
        let t = Tokens::get(ui.ctx());
        if self.dialog.is_none()
            && let Some(r) = self.launcher_update.clone()
        {
            widgets::alert_frame_dense(&t).show(ui, |ui| {
                // A row is at least interact_size tall: 32 here (the Sm buttons), not the page's 40.
                ui.spacing_mut().interact_size.y = theme::control::SM;
                ui.horizontal(|ui| {
                    // A one-line DS Alert: a 16 px icon in the tone's ink, then the title, all on a
                    // 32 px row (the buttons and the dismiss are Sm).
                    let (icon, _) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
                    crate::glyphs::paint(ui.painter(), icon, Icon::Download, t.accent_ink);
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(fmt(tl!("Redrob Launcher {version} is available."), &[("version", &r.version_label())]))
                            .family(theme::semibold())
                            .color(t.ink_strong),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(phase) = self.launcher_job {
                            let (fraction, caption) = match phase {
                                JobPhase::Downloading { done, total } if total > 0 => (
                                    Some(done as f32 / total as f32),
                                    fmt(
                                        tl!("Downloading {done} of {total}"),
                                        &[("done", &widgets::human_bytes(done)), ("total", &widgets::human_bytes(total))],
                                    ),
                                ),
                                JobPhase::Downloading { .. } | JobPhase::Reconnecting { .. } => (None, tl!("Downloading…").to_string()),
                                JobPhase::Verifying => (None, tl!("Verifying…").to_string()),
                                JobPhase::Waiting => (None, tl!("Waiting for other installs…").to_string()),
                                JobPhase::Installing | JobPhase::Removing => (None, tl!("Installing, then restarting…").to_string()),
                            };
                            // Another app's install can hang on a password prompt: the wait can be called off.
                            if phase == JobPhase::Waiting
                                && widgets::icon_button(ui, crate::glyphs::Icon::Close, tl!("Cancel update"), theme::control::SM)
                                    .on_hover_text(tl!("Stop waiting; the update can be started again later"))
                                    .clicked()
                            {
                                self.send(Command::Cancel { app: redrob_launcher_core::catalog::LAUNCHER_ID.to_string() });
                            }
                            widgets::progress(ui, fraction, &caption, t.accent, 220.0);
                            return;
                        }
                        if widgets::icon_button(ui, crate::glyphs::Icon::Close, tl!("Dismiss"), theme::control::SM).clicked() {
                            self.launcher_update = None;
                        }
                        if self.caps.self_update {
                            if widgets::button_sized(ui, tl!("Update and restart"), Kind::Primary, widgets::Size::Sm, 0.0)
                                .on_hover_text(tl!("Downloads the new version, replaces this one and opens it again"))
                                .clicked()
                            {
                                self.send(Command::UpdateLauncher);
                            }
                        } else {
                            let url = self.platform.and_then(|p| launcher_download(&r, p)).unwrap_or_else(|| r.html_url.clone());
                            if widgets::button_sized(ui, tl!("Download"), Kind::Primary, widgets::Size::Sm, 0.0).clicked() {
                                ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                            }
                        }
                        ui.add_space(8.0);
                        widgets::hud_link(ui, tl!("What's new"), &r.html_url);
                    });
                });
            });
            ui.add_space(theme::space::S3);
        }
        self.announcements(ui);
        ui.horizontal(|ui| {
            let filters: Vec<(Filter, Cell<'_>)> = [
                (Filter::All, tl!("All"), tl!("Every Redrob app"), None),
                // The tab names a group ("installed ones"); the status pill says "Installed".
                (Filter::Installed, tr_ctx(current(), "filter", "Installed"), tl!("Apps on this computer"), None),
                (Filter::Updates, tl!("Updates"), tl!("Installed apps with a newer version"), Some(t.warn)),
                (Filter::NotInstalled, tl!("Not installed"), tl!("Apps you can install"), None),
            ]
            .into_iter()
            .filter(|(f, ..)| self.caps.manage_apps || *f == Filter::All)
            .map(|(f, label, tip, highlight)| (f, Cell { label, tip, count: Some((self.count(f), highlight)) }))
            .collect();
            widgets::cells(ui, &mut self.settings.filter, &filters);
            // Check for updates at the row's right end (where the sort was).
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if widgets::icon_button_busy(ui, Icon::Refresh, tl!("Check for updates"), theme::control::SM, self.checking).clicked() {
                    self.check_now();
                }
            });
        });
    }

    /// The DS EmptyState for a filter or search that matches nothing: left aligned, a 48 px
    /// brand-subtle disc with the search glyph, a title, what to relax, and the way back.
    pub(crate) fn empty_state(&mut self, ui: &mut Ui) {
        let t = Tokens::get(ui.ctx());
        let query = self.search.trim().to_string();
        ui.add_space(theme::space::S12);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = theme::space::S3;
            let (disc, _) = ui.allocate_exact_size(vec2(48.0, 48.0), Sense::hover());
            ui.painter().circle_filled(disc.center(), 24.0, t.brand_subtle());
            crate::glyphs::paint(ui.painter(), Rect::from_center_size(disc.center(), vec2(22.0, 22.0)), Icon::Search, t.accent_ink);
            ui.label(widgets::heading(tl!("No apps match"), theme::text::TITLE_3, t.ink));
            let text = if query.is_empty() {
                tl!("No app is in this group right now. Show every app instead.").to_string()
            } else {
                fmt(tl!("Nothing matches “{query}”. Try a shorter or different word."), &[("query", &query)])
            };
            ui.scope(|ui| {
                ui.set_max_width(420.0);
                ui.add(egui::Label::new(RichText::new(text).size(theme::text::BODY_SM).color(t.muted)).wrap());
            });
            ui.horizontal(|ui| {
                if !query.is_empty() && widgets::button(ui, tl!("Clear search"), Kind::Secondary, 0.0).clicked() {
                    self.search.clear();
                }
                if self.settings.filter != Filter::All && widgets::button(ui, tl!("Show all apps"), Kind::Secondary, 0.0).clicked() {
                    self.settings.filter = Filter::All;
                }
            });
        });
    }

    /// News from the release feed (and app renames), until the user dismisses each.
    fn announcements(&mut self, ui: &mut Ui) {
        let t = Tokens::get(ui.ctx());
        let shown: Vec<redrob_launcher_core::feed::Notice> =
            self.announcements.iter().filter(|n| !self.settings.dismissed_notices.contains(&n.id)).take(3).cloned().collect();
        for notice in shown {
            widgets::alert_frame(&t).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if widgets::icon_button(ui, crate::glyphs::Icon::Close, tl!("Dismiss"), theme::control::SM).clicked() {
                            self.settings.dismissed_notices.insert(notice.id.clone());
                        }
                        if let Some(link) = &notice.link {
                            widgets::hud_link(ui, tl!("Learn more"), link);
                        }
                        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                            ui.spacing_mut().item_spacing.y = 2.0;
                            if !notice.title.is_empty() {
                                ui.label(RichText::new(&notice.title).family(theme::semibold()).color(t.ink_strong));
                            }
                            if !notice.body.is_empty() {
                                ui.add(egui::Label::new(RichText::new(&notice.body).color(t.muted)).wrap());
                            }
                        });
                    });
                });
            });
            ui.add_space(8.0);
        }
    }

    /// What the footer used to say, now shown in Settings > Updates: when the catalog was last
    /// checked and from where, GitHub's rate-limit notice while it applies, and the platform apps
    /// are installed (or downloaded) for.
    pub(crate) fn update_status(&self) -> UpdateStatus {
        let now = self.backend.now();
        let checked = if self.checking {
            tl!("Checking for updates…").to_string()
        } else if let Some(at) = self.settings.last_check {
            let ago = ago_text(at, now);
            match self.sources_label() {
                Some(using) => fmt(tl!("Checked {ago} · using {sources}"), &[("ago", &ago), ("sources", &using)]),
                None => fmt(tl!("Checked {ago}"), &[("ago", &ago)]),
            }
        } else {
            tl!("Not checked yet").to_string()
        };
        let rate_limit = self.rate_limited_until.filter(|u| *u > now).map(|until| {
            let min = ((until - now) / 60).max(1).to_string();
            fmt(tl!("GitHub's hourly limit was reached; full details return in {min} min"), &[("min", &min)])
        });
        let platform = match self.platform {
            Some(p) if self.caps.manage_apps => fmt(tl!("Installing for {platform}"), &[("platform", &p.label())]),
            Some(p) => fmt(tl!("Downloads for {platform}"), &[("platform", &p.label())]),
            None => tl!("No downloads for this system").into(),
        };
        UpdateStatus { checked, rate_limit, platform }
    }

    // ---- grid ----------------------------------------------------------------------------

    pub(crate) fn grid(&mut self, ui: &mut Ui, ids: &[&'static str]) {
        let width = ui.available_width();
        // DS ConnectorCard tiles, 16 px apart (no shared hairlines).
        let cols = (((width + CARD_GAP) / (CARD_MIN_W + CARD_GAP)).floor() as usize).max(1);
        let card_w = (width - (cols as f32 - 1.0) * CARD_GAP) / cols as f32;
        // Every app, unsearched: on its shelf, in page order. One voice across the three headings.
        let grouped = self.settings.filter == Filter::All && self.search.trim().is_empty();
        let sections: Vec<Section> = if grouped {
            use crate::shelves::{Shelf, shelf_of};
            Shelf::ALL
                .iter()
                .map(|shelf| {
                    let title = match shelf {
                        Shelf::Engine => tl!("For your AI engine"),
                        Shelf::Work => tl!("For your everyday work"),
                        Shelf::Creative => tl!("For your creative work"),
                    };
                    let list = ids.iter().copied().filter(|id| self.entry(id).is_some_and(|e| shelf_of(e.app) == *shelf)).collect();
                    (Some(title), list)
                })
                .collect()
        } else {
            vec![(None, ids.to_vec())]
        };
        let t = Tokens::get(ui.ctx());
        for (title, list) in sections {
            if list.is_empty() {
                continue;
            }
            if let Some(title) = title {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    widgets::shelf_heading(ui, title);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        widgets::hud(ui, &tn(list.len() as u64, "{n} app", "{n} apps"), t.faint);
                    });
                });
                ui.add_space(theme::space::S3);
            }
            let rows = list.len().div_ceil(cols);
            let height = rows as f32 * (CARD_H + CARD_GAP) - CARD_GAP;
            let (area, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
            for (i, id) in list.iter().enumerate() {
                let (row, col) = (i / cols, i % cols);
                let min = area.min + vec2(col as f32 * (card_w + CARD_GAP), row as f32 * (CARD_H + CARD_GAP));
                self.card(ui, id, Rect::from_min_size(min, vec2(card_w, CARD_H)));
            }
            ui.add_space(theme::space::S6);
        }
    }

    /// One app as a DS ConnectorCard: raised, radius 12, a hairline that strengthens with a
    /// shadow-md on hover (140 ms); the icon exactly as supplied (no shadow, border or tint) beside
    /// the name, tagline and kind; the version; the actions. No status chip and no links: the
    /// button says what can be done, and every card exposes its menu through both the named …
    /// button and right-click. The product's colour is only its icon.
    fn card(&mut self, ui: &mut Ui, id: &str, rect: Rect) {
        let Some(entry) = self.entry(id).cloned() else { return };
        let t = Tokens::get(ui.ctx());
        let app = entry.app;
        let status = entry.status(self.platform, self.caps.manage_apps);
        // The card itself, under its controls (they are added after it, so they win the pointer):
        // right-click opens the app's menu. With no chip on screen, the card is named for screen
        // readers by its app and status ("Redrob Film: Available to install"), and a failed check
        // explains itself on hover.
        let card = ui.interact(rect, ui.id().with(("card", id)), Sense::click());
        let (status_text, _) = model::entry_status_text(&entry, &status, self.caps.manage_apps);
        let card_name = fmt(tl!("{name}: {status}"), &[("name", app.name), ("status", &status_text)]);
        card.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, true, card_name.clone()));
        let card = match entry.failure() {
            Some(f) => card.on_hover_text(fmt(tl!("Couldn't check for updates: {f}"), &[("f", f)])),
            None => card,
        };
        card.context_menu(|ui| {
            ui.set_min_width(200.0);
            // DS menu items are about 36 px.
            ui.spacing_mut().interact_size.y = 36.0;
            self.menu_items(ui, &entry, &status);
        });
        let hovered = ui.rect_contains_pointer(rect);
        let k = ui.ctx().animate_bool_with_time(ui.id().with(("card-hover", id)), hovered, theme::motion::duration(ui.ctx(), theme::motion::FAST));
        let radius = CornerRadius::same(theme::radius::LG);
        let painter = ui.painter_at(rect.expand(24.0));
        if k > 0.0 {
            let mut shadow = theme::shadow_md(&t);
            shadow.color = shadow.color.gamma_multiply(k);
            painter.add(shadow.as_shape(rect, radius));
        }
        painter.rect(rect, radius, t.raised(), Stroke::new(1.0, theme::mix(t.line, t.line_strong, k)), egui::StrokeKind::Inside);

        let inner = rect.shrink(CARD_PAD);
        // A detached child: the card's contents never move the next card along.
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner).layout(egui::Layout::top_down(egui::Align::Min)));
        child.set_clip_rect(rect.intersect(ui.clip_rect()));
        let ui = &mut child;
        ui.horizontal(|ui| {
            const ICON: f32 = CARD_ICON;
            let [name, tagline, like] = [
                FontId::new(theme::text::BODY, theme::display()),
                FontId::proportional(theme::text::BODY_SM),
                FontId::proportional(theme::text::META),
            ];
            const GAP: f32 = 2.0;
            // The slot is as tall as the text beside it when that is taller, so the two share a centre.
            let block = lines_height(ui, &[&name, &tagline, &like], GAP);
            let (slot, icon_response) = ui.allocate_exact_size(vec2(ICON, block.max(ICON)), Sense::hover());
            icon_response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Image, true, fmt(tl!("{name} icon"), &[("name", app.name)])));
            if let Some(icon) = crate::icons::app(app.id) {
                egui::Image::new(icon).paint_at(ui, Rect::from_center_size(slot.center(), vec2(ICON, ICON)));
            }
            ui.add_space(theme::space::S3);
            centered_lines(ui, &[&name, &tagline, &like], GAP, |ui| {
                // The name's row is as tall as its text, not as a button.
                ui.spacing_mut().interact_size.y = 0.0;
                widgets::wordmark(ui, app, theme::text::BODY);
                ui.add(egui::Label::new(RichText::new(tr_ctx(current(), app.id, app.tagline)).font(tagline.clone()).color(t.muted)).truncate());
                ui.add(egui::Label::new(RichText::new(crate::i18n::tr(current(), app.like)).font(like.clone()).color(t.faint)).truncate());
            });
        });
        ui.add_space(theme::space::S3);
        // One line, never cut short: where the dated line doesn't fit the tile, the date moves to
        // the tooltip.
        let full = version_line(&entry, &status);
        let font = FontId::proportional(theme::text::META);
        let fits = ui.fonts_mut(|f| f.layout_no_wrap(full.clone(), font.clone(), t.muted).size().x) <= ui.available_width();
        if fits {
            ui.label(RichText::new(full).font(font).color(t.muted));
        } else {
            ui.label(RichText::new(version_line_dated(&entry, &status, false)).font(font).color(t.muted)).on_hover_text(full);
        }
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            ui.horizontal(|ui| self.actions(ui, &entry, &status, inner.width()));
        });
    }

    // ---- actions -------------------------------------------------------------------------

    fn primary_action(&self, entry: &AppEntry, status: &AppStatus) -> Primary {
        if !self.caps.manage_apps {
            let platform = self.platform;
            let url = entry.release().zip(platform).and_then(|(r, p)| {
                let stems: Vec<&str> = entry.app.stems().collect();
                pick_asset(r, &stems, p, p.download_kinds()).map(|(a, _)| a.url.clone())
            });
            return match (status, url) {
                (AppStatus::NoRelease, _) => Primary::Unavailable(tl!("Coming soon")),
                (AppStatus::Checking, _) => Primary::Unavailable(tl!("Checking…")),
                (_, Some(url)) => Primary::Download(url),
                (_, None) => Primary::Unavailable(tl!("Not available")),
            };
        }
        match status {
            AppStatus::Available if entry.ready => Primary::Retry(tl!("Retry install").into()),
            AppStatus::UpdateAvailable if entry.ready => {
                Primary::Retry(fmt(tl!("Retry update to {version}"), &[("version", &entry.latest_label().unwrap_or_default())]))
            }
            AppStatus::Available => Primary::Install,
            AppStatus::UpdateAvailable => Primary::Update(entry.latest_label().unwrap_or_default()),
            AppStatus::UpToDate | AppStatus::Installed => Primary::Open,
            AppStatus::Checking => Primary::Unavailable(tl!("Checking…")),
            AppStatus::NoRelease => Primary::Unavailable(tl!("Coming soon")),
            AppStatus::Unavailable => Primary::Unavailable(if entry.failure().is_some() { tl!("Offline") } else { tl!("Not available") }),
        }
    }

    /// The main button (or progress) and the named … menu within `width`.
    fn actions(&mut self, ui: &mut Ui, entry: &AppEntry, status: &AppStatus, width: f32) {
        let t = Tokens::get(ui.ctx());
        let id = entry.app.id.to_string();
        let gap = ui.spacing().item_spacing.x;
        // The named menu is always reachable by keyboard, including while an install job replaces
        // the primary action; right-click remains a pointer shortcut to the same commands.
        let main_w = (width - ACTION_ICON - gap).max(120.0);
        if let Some(phase) = entry.job {
            let downloading = matches!(phase, JobPhase::Downloading { .. } | JobPhase::Reconnecting { .. });
            // Reserve the exact number of trailing controls so the progress caption yields space
            // before Pause, Cancel, or More can overlap at compact card widths.
            let trailing_controls = if downloading {
                3.0
            } else if phase == JobPhase::Waiting {
                2.0
            } else {
                1.0
            };
            let bar_w = (width - trailing_controls * ACTION_ICON - trailing_controls * gap).max(72.0);
            let short = bar_w < 200.0;
            let amount = |done: u64, total: u64| {
                if total > 0 {
                    fmt(tl!("{done} of {total}"), &[("done", &widgets::human_bytes(done)), ("total", &widgets::human_bytes(total))])
                } else {
                    widgets::human_bytes(done)
                }
            };
            let (fraction, caption, color) = match phase {
                JobPhase::Downloading { done, total } => {
                    let f = (total > 0).then(|| done as f32 / total as f32);
                    let pct = f.map(|f| format!(" · {:.0}%", f * 100.0)).unwrap_or_default();
                    let caption = if bar_w < 140.0 {
                        match f {
                            Some(f) => {
                                fmt(tl!("{done} of {total}"), &[("done", &format!("{:.0}%", f * 100.0)), ("total", &widgets::human_bytes(total))])
                            }
                            None => widgets::human_bytes(done),
                        }
                    } else if short {
                        format!("{}{pct}", amount(done, total))
                    } else {
                        format!("{} {}{pct}", tl!("Downloading"), amount(done, total))
                    };
                    (f, caption, t.accent)
                }
                JobPhase::Reconnecting { done, total } => {
                    let f = (total > 0).then(|| done as f32 / total as f32);
                    (f, if short { tl!("Reconnecting…").into() } else { format!("{} {}", tl!("Reconnecting…"), amount(done, total)) }, t.warn)
                }
                JobPhase::Verifying => (None, tl!("Verifying checksum…").into(), t.accent),
                JobPhase::Waiting => (None, tl!("Waiting for another app…").into(), t.accent),
                JobPhase::Installing => (None, tl!("Installing…").into(), t.accent),
                JobPhase::Removing => (None, tl!("Removing…").into(), t.accent),
            };
            // A job waiting for the installer hasn't started it yet, so it can still be called off.
            let waiting = phase == JobPhase::Waiting;
            widgets::progress(ui, fraction, &caption, color, bar_w);
            if downloading && widgets::icon_button(ui, crate::glyphs::Icon::Pause, tl!("Pause download"), ACTION_ICON).clicked() {
                self.send(Command::Pause { app: id.clone() });
            }
            if (downloading || waiting)
                && widgets::icon_button(ui, crate::glyphs::Icon::Close, if waiting { tl!("Cancel") } else { tl!("Cancel download") }, ACTION_ICON)
                    .clicked()
            {
                self.send(Command::Cancel { app: id });
            }
            self.menu(ui, entry, status);
            return;
        }
        if let Some(p) = entry.paused
            && self.caps.manage_apps
        {
            let pct = if p.total > 0 { format!(" · {:.0}%", p.done as f32 / p.total as f32 * 100.0) } else { String::new() };
            // Resume yields space to both Discard and the keyboard-reachable More menu.
            let resume_w = (width - 2.0 * (ACTION_ICON + gap)).max(90.0);
            // Narrow: just tl!("Resume"); the tooltip says how far it got.
            let label = match resume_w {
                w if w < 150.0 => tl!("Resume").to_string(),
                w if w < 220.0 => format!("{}{pct}", tl!("Resume")),
                _ => format!("{}{pct}", tl!("Resume download")),
            };
            let resp = widgets::button(ui, &label, Kind::Primary, resume_w);
            let tip = if p.total > 0 {
                fmt(tl!("{done} of {total} downloaded"), &[("done", &widgets::human_bytes(p.done)), ("total", &widgets::human_bytes(p.total))])
            } else {
                fmt(tl!("{done} downloaded"), &[("done", &widgets::human_bytes(p.done))])
            };
            if resp.on_hover_text(tip).clicked() {
                self.send(Command::Install { app: id.clone() });
            }
            if widgets::icon_button(ui, crate::glyphs::Icon::Close, tl!("Discard download"), ACTION_ICON).clicked() {
                self.send(Command::Cancel { app: id.clone() });
            }
            self.menu(ui, entry, status);
            return;
        }
        match self.primary_action(entry, status) {
            Primary::Install => {
                let resp = widgets::button(ui, tl!("Install"), Kind::Primary, main_w);
                // Opened by a `redrob://open` link for an app that isn't installed yet.
                if self.focus_install == Some(entry.app.id) {
                    self.focus_install = None;
                    resp.request_focus();
                    resp.scroll_to_me(None);
                }
                if resp.clicked() {
                    self.send(Command::Install { app: id.clone() });
                }
            }
            Primary::Retry(label) => {
                let tip = tl!("The download is still here. It's checked again, then installed.");
                if widgets::button(ui, &label, Kind::Primary, main_w).on_hover_text(tip).clicked() {
                    self.send(Command::Install { app: id.clone() });
                }
            }
            Primary::Update(v) => {
                if widgets::button(ui, &fmt(tl!("Update to {v}"), &[("v", &v)]), Kind::Primary, main_w).clicked() {
                    self.send(Command::Install { app: id.clone() });
                }
            }
            Primary::Open => {
                if widgets::button(ui, tl!("Open"), Kind::Secondary, main_w).clicked() {
                    self.send(Command::Launch { app: id.clone(), args: Vec::new() });
                }
            }
            Primary::Download(url) => {
                let label = match self.platform {
                    Some(p) => fmt(tl!("Download for {os}"), &[("os", p.os.label())]),
                    None => tl!("Download").into(),
                };
                if widgets::button(ui, &label, Kind::Primary, main_w).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                }
            }
            Primary::Unavailable(label) => {
                ui.add_enabled_ui(false, |ui| widgets::button(ui, label, Kind::Secondary, main_w));
            }
        }
        self.menu(ui, entry, status);
    }

    /// The … button and its menu.
    fn menu(&mut self, ui: &mut Ui, entry: &AppEntry, status: &AppStatus) {
        widgets::more_menu(ui, |ui| self.menu_items(ui, entry, status));
    }

    /// An app's menu: from the … button or a right-click on its card.
    fn menu_items(&mut self, ui: &mut Ui, entry: &AppEntry, status: &AppStatus) {
        let id = entry.app.id.to_string();
        let manage = self.caps.manage_apps;
        let reveal = self.caps.reveal_label;
        {
            if entry.release().is_some() && ui.button(tl!("Release notes")).clicked() {
                self.dialog = Some(Dialog::Notes(id.clone()));
                ui.close();
            }
            // Only where Install would download: an up-to-date app would be reinstalled.
            let can_download_again =
                manage && entry.ready && entry.paused.is_none() && matches!(status, AppStatus::Available | AppStatus::UpdateAvailable);
            if can_download_again && ui.button(tl!("Download again")).clicked() {
                self.send(Command::Cancel { app: id.clone() });
                self.send(Command::Install { app: id.clone() });
                ui.close();
            }
            if manage && status.is_installed() {
                if ui.button(tl!("Open")).clicked() {
                    self.send(Command::Launch { app: id.clone(), args: Vec::new() });
                    ui.close();
                }
                if ui.button(t(reveal)).clicked() {
                    self.send(Command::Reveal { app: id.clone() });
                    ui.close();
                }
                if *status == AppStatus::UpToDate && ui.button(tl!("Reinstall")).clicked() {
                    self.send(Command::Install { app: id.clone() });
                    ui.close();
                }
            }
            if !manage && let (Some(r), Some(p)) = (entry.release(), self.platform) {
                let stems: Vec<&str> = entry.app.stems().collect();
                let options: Vec<(String, String)> = p
                    .download_kinds()
                    .iter()
                    .filter_map(|k| {
                        pick_asset(r, &stems, p, &[*k])
                            .map(|(a, _)| (format!("{} · {}", crate::i18n::t(k.label()), widgets::human_bytes(a.size)), a.url.clone()))
                    })
                    .collect();
                if !options.is_empty() {
                    ui.separator();
                    for (label, url) in options {
                        if ui.button(label).clicked() {
                            ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                            ui.close();
                        }
                    }
                }
            }
            ui.separator();
            let link = |ui: &mut Ui, label: &str, url: String| {
                if ui.button(label).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                    ui.close();
                }
            };
            if let Some(site) = entry.app.website() {
                link(ui, tl!("Website"), site);
            }
            link(ui, tl!("GitHub repository"), entry.app.repo_url());
            link(ui, tl!("All releases"), entry.app.releases_url());
            // DS Menu: the destructive action last, in status-danger.
            if manage && status.is_installed() {
                ui.separator();
                if ui.button(RichText::new(tl!("Uninstall…")).color(Tokens::get(ui.ctx()).bad)).clicked() {
                    self.dialog = Some(Dialog::ConfirmUninstall(id.clone()));
                    ui.close();
                }
            }
        }
    }
}

/// "0.4.1 installed → 0.5.0 released · Oct 8, 2026" and friends.
fn version_line(e: &AppEntry, status: &AppStatus) -> String {
    version_line_dated(e, status, true)
}

/// [`version_line`], with or without the release date.
fn version_line_dated(e: &AppEntry, status: &AppStatus, dated: bool) -> String {
    let released = e.release().and_then(|r| r.published_at).filter(|_| dated).map(|at| format!(" · {}", date_text(at))).unwrap_or_default();
    match (status, e.installed_label(), e.latest_label()) {
        (AppStatus::UpdateAvailable, Some(i), Some(l)) => {
            format!("{}{released}", fmt(tl!("{version} installed → {latest} released"), &[("version", i.as_str()), ("latest", l.as_str())]))
        }
        (AppStatus::UpToDate, Some(i), _) => format!("{}{released}", fmt(tl!("Version {version}"), &[("version", i.as_str())])),
        (_, Some(i), Some(l)) => fmt(tl!("{installed} installed · latest {latest}"), &[("installed", i.as_str()), ("latest", l.as_str())]),
        (_, Some(i), None) => fmt(tl!("Version {version} installed"), &[("version", i.as_str())]),
        (_, None, Some(l)) => format!("{}{released}", fmt(tl!("Version {latest}"), &[("latest", l.as_str())])),
        (AppStatus::NoRelease, None, None) => tl!("No release published yet").to_string(),
        _ => " ".to_string(),
    }
}

/// The launcher's own download for this platform.
pub(crate) fn launcher_download(r: &redrob_launcher_core::Release, p: redrob_launcher_core::Platform) -> Option<String> {
    use redrob_launcher_core::PackageKind;
    let kinds: &[PackageKind] = match p.os {
        redrob_launcher_core::Os::Linux => &[PackageKind::AppImage, PackageKind::Deb],
        _ => p.default_kinds(),
    };
    pick_asset(r, &[redrob_launcher_core::catalog::LAUNCHER_ID], p, kinds).map(|(a, _)| a.url.clone())
}

/// A column of single-line texts, one per font and `gap` apart, centred across the row it is in.
/// `ui.vertical` starts at the top of the row instead: a row only centres what it knows the height of.
/// Each line is measured as an empty galley, which is rounded to the pixel like a label, unlike `Fonts::row_height`.
fn centered_lines(ui: &mut Ui, fonts: &[&FontId], gap: f32, add_contents: impl FnOnce(&mut Ui)) {
    let height = lines_height(ui, fonts, gap);
    ui.allocate_ui_with_layout(vec2(ui.available_width(), height), egui::Layout::top_down(egui::Align::Min), |ui| {
        ui.spacing_mut().item_spacing.y = gap;
        add_contents(ui);
    });
}

/// The height of [`centered_lines`] with these fonts.
fn lines_height(ui: &Ui, fonts: &[&FontId], gap: f32) -> f32 {
    let line = |font: &FontId| ui.fonts_mut(|f| f.layout_no_wrap(String::new(), font.clone(), Color32::PLACEHOLDER)).size().y;
    (fonts.iter().map(|font| line(font) + gap).sum::<f32>() - gap).max(0.0)
}
