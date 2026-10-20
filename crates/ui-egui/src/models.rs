// ADDED-LINE-RATIONALE cab953c6fb708693:1: Supports `models` while preserving the nearby constraint: The page's reading measure: the same column as the Engine, Account and Settings pages.
// ADDED-LINE-RATIONALE dabccdd5420765eb:1: Supports `models` while preserving the nearby constraint: Width of an installing model's progress bar.
// ADDED-LINE-RATIONALE 62d355e2f71fd5a1:1: Supports `models_nav` while preserving the nearby constraint: blue dot while one is coming in; the hover says what the page is for.
// ADDED-LINE-RATIONALE 203c9a0d38b716b8:1: Freezes `meta` for `models_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 199f61d040f8481d:1: Freezes `hint` for `models_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2b5de20f5ca47ebf:1: Prevents `models_nav` from applying its side effect outside the `busy / tl / A` state.
// ADDED-LINE-RATIONALE 0c9cd358e364ac59:1: Freezes `if` for `models_nav` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2d1ec9fc426af78f:1: Delegates `models_nav`'s `set_page / Page / Models` work to `self.set_page` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9e541d9e56a9000d:1: Gives `device_card` direct access to `theme / space` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 2cd8d55906d5bac0:1: Delegates `device_card`'s `widgets / card_frame / t` work to `widgets::card_frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fd80508b5aa6306b:1: Freezes `ui` for `device_card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3cb0420ee2b4c07b:1: Freezes `ui` for `device_card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f810479424a90dad:1: Delegates `device_card`'s `ui / label / RichText` work to `ui.label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cfda65db1b3dd3d9:1: Delegates `device_card`'s `ui / with_layout / egui` work to `ui.with_layout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a2dca29e73f7d65f:1: Freezes `remeasure` for `device_card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9e2d112c6d15b5e8:1: Supports `device_card` while preserving the nearby constraint: Body: the processor line, then the facts that decide what runs, on one meta line.
// ADDED-LINE-RATIONALE 32885e775181e8e4:1: Prevents `device_card` from applying its side effect outside the `device / cpu / is_empty` state.
// ADDED-LINE-RATIONALE d9cb372e7b6a8f1e:1: Delegates `device_card`'s `line / push / device` work to `line.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a29a5f29457b829f:1: Bounds the `device_card` implementation here so `line / push / device` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d8876d429c2fefbe:1: Prevents `device_card` from applying its side effect outside the `device / cores` state.
// ADDED-LINE-RATIONALE 2ae43ccf35fb7027:1: Delegates `device_card`'s `line / push / fmt` work to `line.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a29a5f29457b829f:2: Bounds the `device_card` implementation here so `line / push / fmt` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1c917d751bb87eea:1: Prevents `device_card` from applying its side effect outside the `device / ram_mb` state.
// ADDED-LINE-RATIONALE f24b411a1d5f45b1:1: Delegates `device_card`'s `line / push / fmt` work to `line.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a29a5f29457b829f:3: Bounds the `device_card` implementation here so `line / push / fmt` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a43744394e79866f:1: Freezes `if` for `device_card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 39148c65452b6c96:1: Delegates `device_card`'s `line / push / fmt` work to `line.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a29a5f29457b829f:4: Bounds the `device_card` implementation here so `line / push / fmt` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 96db6f8596ea0d71:1: Delegates `device_card`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 691390def8bd43c0:1: Prevents `device_card` from applying its side effect outside the `line / is_empty` state.
// ADDED-LINE-RATIONALE 52f666badf8342b3:1: Delegates `device_card`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3eeca346858cd110:1: Delegates `device_card`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a29a5f29457b829f:5: Bounds the `device_card` implementation here so `ui / add_space / space` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6d3101011ce32f1e:1: Delegates `device_card`'s `widgets / tight_wrapped / ui` work to `widgets::tight_wrapped` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6e3439f99ae09848:1: Delegates `device_card`'s `widgets / hud / ui` work to `widgets::hud` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ebc3f5bf3d71f28e:1: Freezes `if` for `device_card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c2be502697645ba7:1: Delegates `device_card`'s `widgets / meta_sep / ui` work to `widgets::meta_sep` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c12b8b6150cc739e:1: Delegates `device_card`'s `widgets / hud / ui` work to `widgets::hud` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5ac97c5052c36d9f:1: Bounds the `device_card` implementation here so `widgets / hud / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 37107038c5d21e21:1: Freezes `if` for `device_card` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c2be502697645ba7:2: Delegates `device_card`'s `widgets / meta_sep / ui` work to `widgets::meta_sep` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1dfffd56557e8a89:1: Delegates `device_card`'s `widgets / hud / ui` work to `widgets::hud` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5ac97c5052c36d9f:2: Bounds the `device_card` implementation here so `widgets / hud / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 69a41c2fab1bae9b:2: Makes `widgets / hud / ui` an explicit part of `device_card` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 0534388f480f4368:1: Supports `model_row` while preserving the nearby constraint: edge, centred on the row, the primary last. Returns what a click asked for.
// ADDED-LINE-RATIONALE 9e541d9e56a9000d:2: Gives `model_row` direct access to `theme / space` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE bfc948853f467250:1: Gives `model_row` direct access to `widgets / Size` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 90541e64b69e7199:1: Freezes `m` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1160a7a7854605b4:1: Supports `model_row` while preserving the nearby constraint: The primary goes first: in the right-to-left column it ends up on the trailing edge.
// ADDED-LINE-RATIONALE d4066a82b7fde7f9:1: Prevents `model_row` from applying its side effect outside the `v / state` state.
// ADDED-LINE-RATIONALE 375ae7cb1609195a:1: Freezes `ModelState` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 34bfec210aa5bea6:1: Freezes `ModelState` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6dd5d7e425c07246:1: Freezes `label` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a0240047ae39d55d:1: Delegates `model_row`'s `actions / push / label` work to `actions.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 00e04b4166e6a5b7:1: Delegates `model_row`'s `actions / push / tl` work to `actions.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a29a5f29457b829f:6: Bounds the `model_row` implementation here so `actions / push / tl` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3fc7d6d58c6a855d:1: Freezes `ModelState` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d1ef0ed9e31db83e:1: Freezes `ModelState` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c6b99b7d276a929b:1: Prevents `model_row` from applying its side effect outside the `m / runtime / served` state.
// ADDED-LINE-RATIONALE eba3a6094fbd7bc8:1: Prevents `model_row` from applying its side effect outside the `v / serving` state.
// ADDED-LINE-RATIONALE 67a589cd0f321eda:1: Freezes `Serving` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e26259d43ac20f4c:1: Freezes `Serving` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5b3a0c6089a8a856:1: Freezes `label` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e197de76259c1975:1: Delegates `model_row`'s `actions / push / label` work to `actions.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a03f72373546d39f:1: Bounds the `model_row` implementation here so `actions / push / label` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a890ee055013889f:1: Bounds the `model_row` implementation here so `actions / push / label` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5ac97c5052c36d9f:3: Bounds the `model_row` implementation here so `actions / push / tl` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 00e04b4166e6a5b7:2: Delegates `model_row`'s `actions / push / tl` work to `actions.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a29a5f29457b829f:7: Bounds the `model_row` implementation here so `actions / push / tl` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE bc0f97160cea9d08:1: Freezes `ModelState` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 88898998d490c79f:2: Bounds the `model_row` implementation here so `ModelState / NoFolder` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3d8e326ebdee3b52:1: Freezes `gap` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5bd8ac6cf743a79a:1: Freezes `actions_w` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cc5a1f9d0d150367:1: Delegates `model_row`'s `gap / actions / len` work to `actions.len` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 71563a500065e5a2:1: Freezes `ask` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e8d21ce565e599d7:1: Delegates `model_row`'s `ui / horizontal / ui` work to `ui.horizontal` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7443cb2599619cd1:1: Freezes `room` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e4567c494f1f1e45:1: Freezes `content_w` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0a5dcd0328c23af9:1: Delegates `model_row`'s `ui / vertical / ui` work to `ui.vertical` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2d9f6dbefc7ea37c:1: Delegates `model_row`'s `ui / set_width / content_w` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bc613d2d06f4916b:1: Freezes `ui` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e3f3bd51fd89e202:1: Freezes `headline` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 70ef2da8cc28fc3a:1: Delegates `model_row`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3eeca346858cd110:2: Delegates `model_row`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7629dcd29ae09a1e:1: Supports `model_row` while preserving the nearby constraint: The details, small: the model's name, its size, which app, the licence.
// ADDED-LINE-RATIONALE c4d84f3e7f6ac652:1: Delegates `model_row`'s `widgets / hud / ui` work to `widgets::hud` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c2be502697645ba7:3: Delegates `model_row`'s `widgets / meta_sep / ui` work to `widgets::meta_sep` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 32f265b2bcf8d774:1: Delegates `model_row`'s `widgets / hud / ui` work to `widgets::hud` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c2be502697645ba7:4: Delegates `model_row`'s `widgets / meta_sep / ui` work to `widgets::meta_sep` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 14a2b0e185aba718:1: Delegates `model_row`'s `widgets / hud / ui` work to `widgets::hud` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c2be502697645ba7:5: Delegates `model_row`'s `widgets / meta_sep / ui` work to `widgets::meta_sep` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c9890c8e0dd660c7:1: Delegates `model_row`'s `widgets / hud_link / ui` work to `widgets::hud_link` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 52f982c76701519b:3: Makes `widgets / hud_link / ui` an explicit part of `model_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 58ab9e4694105671:1: Delegates `model_row`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9a318e5dbbb744a7:1: Supports `model_row` while preserving the nearby constraint: The verdict for this computer first, so it always starts at the content edge.
// ADDED-LINE-RATIONALE 1fc4d7d8c67b80cd:1: Freezes `need` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 22c1b4ea3ad648bd:1: Keeps `tl / Needs / about` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 81482f0710bff3b3:1: Makes `tl / Needs / about` an explicit part of `model_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 2f76af4e0cd847d6:1: Keeps `gb / format / f64` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 1fe4c8e570c7ce66:1: Keeps `kind / cpu / tl` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 8160260710d47649:1: Keeps `kind / cpu / tl` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 071b587ca17d7ae8:1: Bounds the `model_row` implementation here so `ModelState / Downloading / done` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 275bd109a6e1efab:1: Freezes `if` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5570d7d53e7d0ae8:1: Supports `model_row` while preserving the nearby constraint: Top-aligned, so the chip sits on the caption's line, the bar under them.
// ADDED-LINE-RATIONALE a47835e890561aa1:1: Delegates `model_row`'s `ui / allocate_ui_with_layout / line` work to `ui.allocate_ui_with_layout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 72bb88e5be6ba63b:1: Delegates `model_row`'s `widgets / badge_sized / ui` work to `widgets::badge_sized` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0789ddd37779e02b:1: Freezes `fraction` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e5e14bfee3489891:1: Freezes `caption` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dc12baea997f6954:1: Keeps `tl / Installing / done` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE ea6d0bea6ce4b22e:1: Keeps `done / widgets / human_bytes` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 91f22fcf86c3cce8:1: Bounds the `model_row` implementation here so `done / widgets / human_bytes` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 862ef4ea14af7733:1: Delegates `model_row`'s `widgets / progress / ui` work to `widgets::progress` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c0ea300236c6a49b:1: Makes `widgets / progress / ui` an explicit part of `model_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 0e7ab6bd35142299:1: Makes `widgets / tight_row / ui` an explicit part of `model_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 00ca05c944b27d39:1: Delegates `model_row`'s `widgets / tight_row / ui` work to `widgets::tight_row` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 72bb88e5be6ba63b:2: Delegates `model_row`'s `widgets / badge_sized / ui` work to `widgets::badge_sized` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e794cece4be83e46:1: Freezes `let` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1ad1d9292e39bae3:1: Keeps `ModelState / Installed / Serving` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 1ce7ffdc51a0695a:1: Keeps `ModelState / Installed / Serving` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 6d2dd78e043e96db:1: Keeps `ModelState / Installed / Serving` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 2cbd14bfd4073b37:1: Keeps `ModelState / NotDownloaded / _` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 5feff091b9d0d8e2:1: Makes `ModelState / Partial / done` an explicit part of `model_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE d2afd1ccd85aeaa1:1: Delegates `model_row`'s `fmt` work to `fmt` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d8380b468f3c7097:1: Keeps `tl / done / of` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 32e3b9428767b42e:1: Keeps `done / widgets / human_bytes` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE a2718c9ff6884f9d:1: Keeps `done / widgets / human_bytes` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 06706a753c94b5e4:1: Keeps `Tone / Warn` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE d56703758ba7be9d:1: Keeps `Tone / Warn` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE b73b9255af1c3b00:1: Keeps `ModelState / Downloading / _` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 38b66cbc6c8cf610:1: Keeps `ModelState / Installed / Serving` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE d442e361a10488e6:1: Keeps `ModelState / Failed / e` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 26e704983474deb0:1: Keeps `ModelState / NoFolder / _` in `model_row`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 92a2b7cf87599fac:1: Bounds the `model_row` implementation here so `ModelState / NoFolder / _` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4a45fb9500e37842:1: Freezes `r` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3263d02beca3a429:1: Freezes `if` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c47733872acb5d67:1: Delegates `model_row`'s `r / on_hover_text / folder` work to `r.on_hover_text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a03f72373546d39f:2: Bounds the `model_row` implementation here so `r / on_hover_text / folder` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c0ea300236c6a49b:2: Makes `r / on_hover_text / folder` an explicit part of `model_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 5ac97c5052c36d9f:4: Bounds the `model_row` implementation here so `actions / is_empty` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 69a41c2fab1bae9b:3: Makes `actions / is_empty` an explicit part of `model_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 02bcde7d96fdbe23:1: Prevents `model_row` from applying its side effect outside the `actions / is_empty` state.
// ADDED-LINE-RATIONALE cfda65db1b3dd3d9:2: Delegates `model_row`'s `ui / with_layout / egui` work to `ui.with_layout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2c212e453f460193:1: Applies `model_row` consistently to every `label / kind / what` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 353ef545c959145d:1: Prevents `model_row` from applying its side effect outside the `widgets / button_sized / ui` state.
// ADDED-LINE-RATIONALE d9f0e274407ea0a3:1: Freezes `ask` for `model_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a03f72373546d39f:3: Bounds the `model_row` implementation here so `ask / what / m` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a890ee055013889f:2: Bounds the `model_row` implementation here so `ask / what / m` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 52f982c76701519b:4: Makes `ask / what / m` an explicit part of `model_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE a29a5f29457b829f:8: Bounds the `model_row` implementation here so `ask` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE eceff217fbb2bb9b:2: Makes `ask` an explicit part of `model_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9ae4f6185e6e5c09:1: Makes `ask` an explicit part of `model_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 5dca140fe5883c9f:8: Bounds the `model_row` implementation here so `ask` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 698e6e089ebd01c5:1: Delegates `models_page`'s `widgets / page_scroll / ui` work to `widgets::page_scroll` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1093a9035aca7b0d:1: Gives `models_page` direct access to `theme / space` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE efa263e638866773:1: Delegates `models_page`'s `ui / set_width / ui` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fd80508b5aa6306b:2: Supports `models_page` while preserving the nearby constraint: The rhythm below is the DS's 4 px grid, set step by step.
// ADDED-LINE-RATIONALE afea40dcaf45e59d:1: Supports `models_page` while preserving the nearby constraint: The title is in the page's top bar.
// ADDED-LINE-RATIONALE 7c8f099addb4a6ea:1: Delegates `models_page`'s `egui / Label / new` work to `egui::Label::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 71b0dbc0d70aa1f2:1: Delegates `models_page`'s `RichText / new / tl` work to `RichText::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ae4375b9edc8b200:1: Makes `Install / what / the` an explicit part of `models_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 811c8107109b441e:1: Makes `Install / what / the` an explicit part of `models_page` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 4b9d4b800fdfd969:1: Delegates `models_page`'s `size / theme / text` work to `size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d4dc83aec3acd8d5:1: Keeps `color / t / muted` in `models_page`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 5ac9485052c31543:1: Bounds the `models_page` implementation here so `color / t / muted` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 42e9fab24d279baf:1: Keeps `wrap` in `models_page`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE c721cf21104929e8:1: Bounds the `models_page` implementation here so `wrap` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ba919b85ab71a80d:1: Delegates `models_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a9084185a173715b:1: Delegates `models_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 22cd48d5cbf84128:1: Delegates `models_page`'s `widgets / switch / ui` work to `widgets::switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 17b2652292b06888:1: Freezes `ui` for `models_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d717a5a345b192f6:1: Supports `models_page` while preserving the nearby constraint: card (DS: N items at once are a list on hairlines, not a grid of identical cards).
// ADDED-LINE-RATIONALE c64075f73803db13:1: Freezes `cpu` for `models_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5169cb5ee64c04e9:1: Freezes `for` for `models_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 63faa2f52ad8a817:1: Freezes `task` for `models_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7c61ca46a897f10d:1: Delegates `models_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3f6f487358037e12:1: Delegates `models_page`'s `ui / label / RichText` work to `ui.label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3abf1b9405f5bccc:1: Freezes `if` for `models_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 576a65f122d82a10:1: Delegates `models_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2a9a894bd5bda00e:1: Delegates `models_page`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5118fd4690156912:1: Delegates `models_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d73efd0f05153669:1: Delegates `models_page`'s `widgets / card_frame / t` work to `widgets::card_frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1a8808194395826b:1: Freezes `ui` for `models_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8d5e5e31630942ba:1: Applies `models_page` consistently to every `i / v / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 5c5dd910f70d46e3:1: Prevents `models_page` from applying its side effect outside the `i` state.
// ADDED-LINE-RATIONALE 617a3ff69ed0c00e:1: Delegates `models_page`'s `widgets / row_rule / ui` work to `widgets::row_rule` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a03f72373546d39f:4: Bounds the `models_page` implementation here so `widgets / row_rule / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE cfd7699715ee7c5b:1: Delegates `models_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1292af1414222025:1: Freezes `if` for `models_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 088de24dd82335b9:1: Freezes `ask` for `models_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a03f72373546d39f:5: Bounds the `models_page` implementation here so `ask / a` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE cfd7699715ee7c5b:2: Delegates `models_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a890ee055013889f:4: Bounds the `models_page` implementation here so `ui / add_space / space` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ba919b85ab71a80d:2: Delegates `models_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cad46d85b45a3507:1: Delegates `models_page`'s `ui / add_space / space` work to `ui.add_space` so existing state and error handling remain centralized.
//! The Models page: what this computer is, and every model the apps can use on it, grouped by
//! what it is for (subtitles, pictures, chat...), since that is how a person who has never heard
//! of a model chooses one. Each row leads with a plain sentence and the verdict for this computer;
//! the model's name, size and app come second, small. The person can install, pause, resume or
//! delete a model and, for one the launcher serves, start and stop the server. The backend does
//! the work ([`ModelBackend`]); this only shows states and sends requests.

use egui::RichText;
use redrob_launcher_core::device::{Device, Fit};
use redrob_launcher_core::models::{ModelState, ModelView, Runtime, RuntimeState, SHARED_APP, Serving};

use crate::app::{LauncherUi, Page};
use crate::i18n::{fmt, msg};
use crate::model::Tone;
use crate::theme::{self, Tokens};
use crate::widgets::{self, Kind};

/// The page's reading measure: the same column as the Engine, Account and Settings pages.
const MEASURE: f32 = 720.0;

/// Width of an installing model's progress bar.
const PROGRESS_W: f32 = 240.0;

/// What one row asks the backend to do.
enum Ask {
    Download,
    Pause,
    Delete,
    Start,
    Stop,
    Remeasure,
}

/// The verdict chip's words and tone.
pub(crate) fn fit_label(fit: Fit) -> (String, Tone) {
    match fit {
        Fit::Runs => (tl!("Runs here").into(), Tone::Good),
        Fit::Slow => (tl!("Runs slowly").into(), Tone::Warn),
        Fit::WontRun => (tl!("Won't run here").into(), Tone::Neutral),
        Fit::Unknown => (tl!("Not measured").into(), Tone::Faint),
    }
}

/// The task label shown on a row (`chat` -> "Chat"). English keys; the catalog translates.
pub(crate) fn task_label(task: &str) -> String {
    match task {
        "chat" => tl!("Chat").to_string(),
        "code" => tl!("Coding").to_string(),
        "vision" => tl!("Chat and vision").to_string(),
        "embedding" => tl!("Search embeddings").to_string(),
        "text-to-image" => tl!("Text to image").to_string(),
        "image-edit" => tl!("Image editing").to_string(),
        "inpaint" => tl!("Inpainting").to_string(),
        "background-removal" => tl!("Background removal").to_string(),
        "upscale" => tl!("Upscaling").to_string(),
        "segmentation" => tl!("Selection").to_string(),
        "vectorize" => tl!("Vectorizing").to_string(),
        "stt" => tl!("Speech to text").to_string(),
        "tts" => tl!("Text to speech").to_string(),
        "vad" => tl!("Voice detection").to_string(),
        "stem-separation" => tl!("Stem separation").to_string(),
        "image-to-3d" => tl!("Image to 3D").to_string(),
        "ocr" => tl!("Text recognition").to_string(),
        other => other.to_string(),
    }
}

/// The purposes in the order the page lists them: what most people come for first.
const PURPOSES: [&str; 17] = [
    "chat",
    "code",
    "vision",
    "stt",
    "tts",
    "text-to-image",
    "image-edit",
    "inpaint",
    "background-removal",
    "upscale",
    "segmentation",
    "vectorize",
    "stem-separation",
    "vad",
    "image-to-3d",
    "ocr",
    "embedding",
];

/// One plain sentence under a purpose's heading: what these models do for the person.
fn purpose_blurb(task: &str) -> Option<String> {
    Some(match task {
        "chat" => tl!("Talk to an assistant inside the apps, without sending anything off this computer.").to_string(),
        "code" => tl!("An assistant that writes and fixes code in Redrob Code.").to_string(),
        "vision" => tl!("An assistant that can also look at your images.").to_string(),
        "stt" => tl!("Turns speech in your videos into subtitles and text.").to_string(),
        "tts" => tl!("Reads text aloud as narration.").to_string(),
        "text-to-image" => tl!("Makes pictures from a description.").to_string(),
        "image-edit" => tl!("Changes a picture the way you describe.").to_string(),
        "inpaint" => tl!("Removes things from photos and fills the gap.").to_string(),
        "background-removal" => tl!("Cuts the background out of a photo.").to_string(),
        "upscale" => tl!("Makes small images bigger and sharper.").to_string(),
        "segmentation" => tl!("Selects an object in a picture with one click.").to_string(),
        "stem-separation" => tl!("Splits music into vocals, drums, bass and the rest.").to_string(),
        "vad" => tl!("Finds where people speak in a recording.").to_string(),
        "image-to-3d" => tl!("Turns a photo into a 3D model.").to_string(),
        "ocr" => tl!("Reads the text in images and scans so Redrob Recall can search it.").to_string(),
        "embedding" => tl!("Lets Redrob Recall search your files by meaning.").to_string(),
        _ => return None,
    })
}

fn purpose_rank(task: &str) -> usize {
    PURPOSES.iter().position(|p| *p == task).unwrap_or(PURPOSES.len())
}

fn group_name(app: &str) -> String {
    if app == SHARED_APP {
        tl!("Every app").to_string()
    } else {
        redrob_launcher_core::CraftApp::by_id(app).map_or_else(|| app.to_string(), |a| a.name.to_string())
    }
}

/// Rows in display order: by purpose, and within one, what runs here first, then smaller first.
fn ordered(views: Vec<ModelView>) -> Vec<ModelView> {
    let mut v = views;
    v.sort_by_key(|x| (purpose_rank(&x.model.task), fit_rank(x.fit), x.model.size()));
    v
}

fn fit_rank(fit: Fit) -> u8 {
    match fit {
        Fit::Runs => 0,
        Fit::Slow => 1,
        Fit::Unknown => 2,
        Fit::WontRun => 3,
    }
}

impl LauncherUi {
    /// The sidebar's On-device AI row (the models page), on backends that can download models. A
    /// blue dot while one is coming in; the hover says what the page is for.
    pub(crate) fn models_nav(&mut self, ui: &mut egui::Ui, rail: bool) {
        let Some(models) = self.backend.models() else { return };
        let views = models.models();
        let busy = views.iter().any(|v| matches!(v.state, ModelState::Downloading { .. }) || v.serving == Serving::Starting);
        let meta = if busy { widgets::NavMeta::Dot(Tokens::get(ui.ctx()).accent) } else { widgets::NavMeta::None };
        let hint =
            if busy { tl!("A model is downloading or starting") } else { tl!("AI models that run on this computer. What you give them stays here.") };
        if widgets::nav_row(ui, crate::glyphs::Icon::Models, tl!("On-device AI"), hint, meta, self.page == Page::Models, rail).clicked() {
            self.set_page(Page::Models);
        }
    }

    fn device_card(ui: &mut egui::Ui, t: &Tokens, device: &Device, disk_free: Option<u64>) -> bool {
        use theme::space;
        let mut remeasure = false;
        widgets::card_frame(t).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 0.0;
            // Head: what this is and its GPU, Measure again on the trailing edge, centred on them.
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = space::S1;
                    widgets::hud(ui, tl!("This computer"), t.muted);
                    let gpu = match &device.gpu {
                        Some(g) => match g.memory_mb {
                            Some(mb) if g.unified => {
                                fmt(tl!("{name}, {gb} GB unified memory"), &[("name", &g.name), ("gb", &(mb / 1024).to_string())])
                            }
                            Some(mb) => fmt(tl!("{name}, {gb} GB"), &[("name", &g.name), ("gb", &mb.div_ceil(1024).to_string())]),
                            None if g.unified => {
                                fmt(tl!("{name}, {gb} GB unified memory"), &[("name", &g.name), ("gb", &(device.ram_mb / 1024).to_string())])
                            }
                            None => fmt(tl!("{name} (memory not measured)"), &[("name", &g.name)]),
                        },
                        None if device.gpu_probed => tl!("No GPU found; models run on the CPU").to_string(),
                        None => tl!("GPU not measured yet").to_string(),
                    };
                    ui.label(RichText::new(gpu).size(theme::text::CONTROL).family(theme::medium()).color(t.ink_strong));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    remeasure = widgets::button_sized(ui, tl!("Measure again"), Kind::Secondary, widgets::Size::Sm, 0.0).clicked();
                });
            });
            // Body: the processor line, then the facts that decide what runs, on one meta line.
            let mut line = Vec::new();
            if !device.cpu.is_empty() {
                line.push(device.cpu.clone());
            }
            if device.cores > 0 {
                line.push(fmt(tl!("{n} cores"), &[("n", &device.cores.to_string())]));
            }
            if device.ram_mb > 0 {
                line.push(fmt(tl!("{gb} GB RAM"), &[("gb", &(device.ram_mb / 1024).to_string())]));
            }
            if let Some(npu) = &device.npu {
                line.push(fmt(tl!("NPU: {name}"), &[("name", &npu.name)]));
            }
            ui.add_space(space::S2);
            if !line.is_empty() {
                ui.add(egui::Label::new(RichText::new(line.join(" · ")).size(theme::text::BODY_SM).color(t.muted)).wrap());
                ui.add_space(space::S1);
            }
            widgets::tight_wrapped(ui, |ui| {
                widgets::hud(ui, &fmt(tl!("Backend: {backend}"), &[("backend", device.backend().id())]), t.faint);
                if let Some(gb) = device.runs_up_to_gb().filter(|gb| *gb > 0) {
                    widgets::meta_sep(ui);
                    widgets::hud(ui, &fmt(tl!("Runs models up to about {gb} GB"), &[("gb", &gb.to_string())]), t.faint);
                }
                if let Some(free) = disk_free {
                    widgets::meta_sep(ui);
                    widgets::hud(ui, &fmt(tl!("{free} free for models"), &[("free", &widgets::human_bytes(free))]), t.faint);
                }
            });
        });
        remeasure
    }

    fn runtime_line(ui: &mut egui::Ui, t: &Tokens, runtime: Runtime, state: &RuntimeState) {
        let name = match runtime {
            Runtime::LlamaServerPrism => "llama-server (PrismML)",
            _ => "llama-server",
        };
        let text = match state {
            RuntimeState::Unavailable => fmt(tl!("No {runtime} build for this computer"), &[("runtime", name)]),
            RuntimeState::NotInstalled => fmt(tl!("{runtime} is installed on first start"), &[("runtime", name)]),
            RuntimeState::Downloading { done, total } if *total > 0 => fmt(
                tl!("Installing {runtime}: {done} of {total}"),
                &[("runtime", name), ("done", &widgets::human_bytes(*done)), ("total", &widgets::human_bytes(*total))],
            ),
            RuntimeState::Downloading { .. } => fmt(tl!("Installing {runtime}…"), &[("runtime", name)]),
            RuntimeState::Installed => fmt(tl!("{runtime} ready"), &[("runtime", name)]),
            RuntimeState::Failed(e) => fmt(tl!("{runtime}: {e}"), &[("runtime", name), ("e", &msg(e))]),
        };
        let color = if matches!(state, RuntimeState::Failed(_)) { t.warn } else { t.faint };
        widgets::hud(ui, &text, color);
    }

    /// One model on its section's hairline list: what it gives the person, the details small, then
    /// the verdict for this computer and its state; its actions in one column on the trailing
    /// edge, centred on the row, the primary last. Returns what a click asked for.
    fn model_row(ui: &mut egui::Ui, t: &Tokens, cpu: bool, v: &ModelView) -> Option<(Ask, String, String)> {
        use theme::space;
        use widgets::Size;
        let m = &v.model;
        // The primary goes first: in the right-to-left column it ends up on the trailing edge.
        let mut actions: Vec<(&str, Kind, Ask)> = Vec::new();
        match &v.state {
            ModelState::NotDownloaded => actions.push((tl!("Install"), Kind::Primary, Ask::Download)),
            ModelState::Partial { .. } | ModelState::Failed(_) => {
                let label = if matches!(v.state, ModelState::Failed(_)) { tl!("Try again") } else { tl!("Resume") };
                actions.push((label, Kind::Primary, Ask::Download));
                actions.push((tl!("Delete"), Kind::Secondary, Ask::Delete));
            }
            ModelState::Downloading { .. } => actions.push((tl!("Pause"), Kind::Secondary, Ask::Pause)),
            ModelState::Installed => {
                if m.runtime.served() {
                    match &v.serving {
                        Serving::Running { .. } | Serving::Starting => actions.push((tl!("Stop"), Kind::Secondary, Ask::Stop)),
                        Serving::No | Serving::Failed(_) => {
                            let label = if matches!(v.serving, Serving::Failed(_)) { tl!("Try again") } else { tl!("Start") };
                            actions.push((label, Kind::Primary, Ask::Start));
                        }
                    }
                }
                actions.push((tl!("Delete"), Kind::Secondary, Ask::Delete));
            }
            ModelState::NoFolder => {}
        }
        let gap = ui.spacing().item_spacing.x;
        let actions_w = actions.iter().map(|(label, _, _)| widgets::button_width(ui, label, Size::Sm)).sum::<f32>()
            + gap * actions.len().saturating_sub(1) as f32;
        let mut ask = None;
        ui.horizontal(|ui| {
            let room = if actions.is_empty() { 0.0 } else { actions_w + space::S6 };
            let content_w = (ui.available_width() - room).max(0.0);
            ui.vertical(|ui| {
                ui.set_width(content_w);
                ui.spacing_mut().item_spacing.y = 0.0;
                let headline = if m.summary.is_empty() { m.name.clone() } else { crate::i18n::t(&m.summary).to_string() };
                ui.add(egui::Label::new(RichText::new(headline).size(theme::text::CONTROL).family(theme::medium()).color(t.ink_strong)).wrap());
                ui.add_space(space::S1);
                // The details, small: the model's name, its size, which app, the licence.
                widgets::tight_wrapped(ui, |ui| {
                    widgets::hud(ui, &m.name, t.faint).on_hover_text(crate::i18n::t(&m.description));
                    widgets::meta_sep(ui);
                    widgets::hud(ui, &widgets::human_bytes(m.size()), t.faint);
                    widgets::meta_sep(ui);
                    widgets::hud(ui, &fmt(tl!("for {app}"), &[("app", &group_name(&m.app))]), t.faint);
                    widgets::meta_sep(ui);
                    widgets::hud_link(ui, tl!("Licence and source"), &m.source).on_hover_text(&m.license);
                });
                ui.add_space(space::S2);
                // The verdict for this computer first, so it always starts at the content edge.
                let (fit_text, tone) = fit_label(v.fit);
                let need = fmt(
                    tl!("Needs about {gb} GB of {kind}"),
                    &[
                        ("gb", &format!("{:.0}", f64::from(m.requires.memory_mb) / 1024.0)),
                        ("kind", if cpu { tl!("RAM") } else { tl!("GPU memory") }),
                    ],
                );
                if let ModelState::Downloading { done, total } = &v.state {
                    // While installing, the bar and its caption say so: no second "Installing" chip.
                    // Top-aligned, so the chip sits on the caption's line, the bar under them.
                    let line = egui::vec2(ui.available_width(), theme::control::MD);
                    ui.allocate_ui_with_layout(line, egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        widgets::badge_sized(ui, &fit_text, widgets::tone_color(t, tone), Size::Sm).on_hover_text(need);
                        let fraction = (*total > 0).then(|| *done as f32 / *total as f32);
                        let caption = fmt(
                            tl!("Installing {done} of {total}"),
                            &[("done", &widgets::human_bytes(*done)), ("total", &widgets::human_bytes(*total))],
                        );
                        widgets::progress(ui, fraction, &caption, t.accent, PROGRESS_W.min(ui.available_width()));
                    });
                } else {
                    widgets::tight_row(ui, |ui| {
                        widgets::badge_sized(ui, &fit_text, widgets::tone_color(t, tone), Size::Sm).on_hover_text(need);
                        let (text, tone): (String, Tone) = match (&v.state, &v.serving) {
                            (ModelState::Installed, Serving::Running { url }) => (fmt(tl!("Serving at {url}"), &[("url", url)]), Tone::Good),
                            (ModelState::Installed, Serving::Starting) => (tl!("Starting…").into(), Tone::Info),
                            (ModelState::Installed, Serving::Failed(e)) => (msg(e), Tone::Warn),
                            (ModelState::NotDownloaded, _) => (tl!("Not installed").into(), Tone::Neutral),
                            (ModelState::Partial { done, total }, _) => (
                                fmt(
                                    tl!("{done} of {total} installed"),
                                    &[("done", &widgets::human_bytes(*done)), ("total", &widgets::human_bytes(*total))],
                                ),
                                Tone::Warn,
                            ),
                            (ModelState::Downloading { .. }, _) => (tl!("Installing").into(), Tone::Info),
                            (ModelState::Installed, Serving::No) => (tl!("Installed").into(), Tone::Good),
                            (ModelState::Failed(e), _) => (msg(e), Tone::Warn),
                            (ModelState::NoFolder, _) => (tl!("Not available on this system").into(), Tone::Faint),
                        };
                        let r = widgets::badge_sized(ui, &text, widgets::tone_color(t, tone), Size::Sm);
                        if let Some(folder) = &v.folder {
                            r.on_hover_text(folder.as_str());
                        }
                    });
                }
            });
            if !actions.is_empty() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    for (label, kind, what) in actions {
                        if widgets::button_sized(ui, label, kind, Size::Sm, 0.0).clicked() {
                            ask = Some((what, m.app.clone(), m.id.clone()));
                        }
                    }
                });
            }
        });
        ask
    }

    /// The page. Fills the central panel; the header's Apps tab leads back.
    pub(crate) fn models_page(&mut self, ui: &mut egui::Ui) {
        let t = Tokens::get(ui.ctx());
        let only_runnable = self.settings.models_only_runnable;
        let prism = self.settings.prism_runtime;
        let Some(models) = self.backend.models() else { return };
        let device = models.device();
        let disk_free = models.disk_free();
        let all = models.models();
        let views = ordered(all.into_iter().filter(|v| prism || v.model.runtime != Runtime::LlamaServerPrism).collect());
        let hidden = views.iter().filter(|v| only_runnable && v.fit == Fit::WontRun).count();
        let runtimes: Vec<(Runtime, RuntimeState)> = [Runtime::LlamaServer, Runtime::LlamaServerPrism]
            .into_iter()
            .filter(|r| views.iter().any(|v| v.model.runtime == *r))
            .map(|r| (r, models.runtime(r)))
            .collect();
        let mut ask: Option<(Ask, String, String)> = None;
        let mut toggle = only_runnable;

        widgets::page_scroll(ui, "models", |ui| {
            use theme::space;
            ui.set_width(ui.available_width().min(MEASURE));
            // The rhythm below is the DS's 4 px grid, set step by step.
            ui.spacing_mut().item_spacing.y = 0.0;
            // The title is in the page's top bar.
            ui.add(
                egui::Label::new(
                    RichText::new(tl!(
                        "Install what the apps need to work offline on this computer. Anything missing is done on Redrob's servers instead."
                    ))
                    .size(theme::text::BODY_SM)
                    .color(t.muted),
                )
                .wrap(),
            );
            ui.add_space(space::S6);
            if Self::device_card(ui, &t, &device, disk_free) {
                ask = Some((Ask::Remeasure, String::new(), String::new()));
            }
            ui.add_space(space::S4);
            // The filter toolbar: the switch and what it hides, the runtimes on the trailing edge.
            ui.horizontal(|ui| {
                widgets::switch(ui, &mut toggle, tl!("Only show what runs on this computer"));
                if hidden > 0 {
                    widgets::hud(ui, &crate::i18n::tn(hidden as u64, "{n} model hidden", "{n} models hidden"), t.faint);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = space::S3;
                    for (r, st) in &runtimes {
                        Self::runtime_line(ui, &t, *r, st);
                    }
                });
            });

            // One section per purpose: its heading, then its models as rows on hairlines in one
            // card (DS: N items at once are a list on hairlines, not a grid of identical cards).
            let shown: Vec<&ModelView> = views.iter().filter(|v| !(only_runnable && v.fit == Fit::WontRun)).collect();
            let cpu = device.backend() == redrob_launcher_core::models::Backend::Cpu;
            for group in shown.chunk_by(|a, b| a.model.task == b.model.task) {
                let task = &group[0].model.task;
                ui.add_space(space::S6);
                ui.label(RichText::new(task_label(task)).size(theme::text::BODY).family(theme::semibold()).color(t.ink_strong));
                if let Some(blurb) = purpose_blurb(task) {
                    ui.add_space(space::S1);
                    ui.add(egui::Label::new(RichText::new(blurb).size(theme::text::BODY_SM).color(t.muted)).wrap());
                }
                ui.add_space(space::S3);
                widgets::card_frame(&t).inner_margin(egui::Margin::symmetric(space::S5 as i8, 0)).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.spacing_mut().item_spacing.y = 0.0;
                    for (i, v) in group.iter().enumerate() {
                        if i > 0 {
                            widgets::row_rule(ui, 0.0);
                        }
                        ui.add_space(space::S4);
                        if let Some(a) = Self::model_row(ui, &t, cpu, v) {
                            ask = Some(a);
                        }
                        ui.add_space(space::S4);
                    }
                });
            }
            ui.add_space(space::S6);
            widgets::hud(ui, tl!("Every file is checked against its official copy before an app uses it."), t.faint);
            ui.add_space(space::S8);
        });

        if toggle != only_runnable {
            self.settings.models_only_runnable = toggle;
        }
        let live = views.iter().any(|v| matches!(v.state, ModelState::Downloading { .. }) || v.serving == Serving::Starting)
            || runtimes.iter().any(|(_, s)| matches!(s, RuntimeState::Downloading { .. }));
        if live {
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
        }
        if let Some((what, app, id)) = ask {
            let ctx = ui.ctx().clone();
            if let Some(models) = self.backend.models() {
                match what {
                    Ask::Download => models.download(&app, &id),
                    Ask::Pause => models.pause(&app, &id),
                    Ask::Start => models.start(&app, &id),
                    Ask::Stop => models.stop(&app, &id),
                    Ask::Remeasure => models.remeasure(),
                    Ask::Delete => {
                        if let Err(e) = models.delete(&app, &id) {
                            self.toast(&ctx, msg(&e), true);
                        }
                    }
                }
            }
        }
    }
}
