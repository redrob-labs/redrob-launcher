// ADDED-LINE-RATIONALE 84683d9532958dda:1: Gives `dialogs` direct access to `crate / glyphs / Icon` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 2a525f23fdf395cf:1: Isolates `dialogs` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE fb5536ddd63c64fc:1: Freezes `response` for `dialogs` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fd5e7c71a845970e:1: Freezes `close` for `dialogs` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f3520b86e6cdfc87:1: Freezes `Dialog` for `dialogs` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bb1dfc07fd9dde91:1: Freezes `Dialog` for `dialogs` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b62bcd6794b949f0:1: Freezes `Dialog` for `dialogs` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e1c6b5235e02d8a4:1: Makes `close / response / should_close` an explicit part of `dialogs` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE c025b85d7e6263a3:1: Prevents `dialogs` from applying its side effect outside the `close / response / should_close` state.
// ADDED-LINE-RATIONALE 7113fed82f1222ca:1: Supports `modal` while preserving the nearby constraint: [`Self::modal_foot`]).
// ADDED-LINE-RATIONALE fcb178298e94c4ae:1: Freezes `t` for `modal` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7cf6ec361816e827:1: Delegates `modal`'s `egui / Modal / new` work to `egui::Modal::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 48c788073b840591:1: Delegates `modal`'s `egui / Frame / new` work to `egui::Frame::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f456e5757b23ce00:1: Delegates `modal`'s `fill / t / bg` work to `fill` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6ee6a19020349bfb:1: Delegates `modal`'s `stroke / t / hairline` work to `stroke` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c4da80807b12ba87:1: Delegates `modal`'s `corner_radius / egui / CornerRadius` work to `corner_radius` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 78f3acae1dd859f8:1: Delegates `modal`'s `shadow / theme / shadow_lg` work to `shadow` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c18da490a71ac6d3:1: Keeps `inner_margin` in `modal`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 2eb6758c39bd2f28:1: Bounds the `modal` implementation here so `inner_margin` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2847d041c565e974:2: Bounds the `modal` implementation here so `inner_margin` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ea3acf8212ecebde:1: Supports `modal_width` while preserving the nearby constraint: A modal's width: `max` (DS Modal 480), or less in a small window.
// ADDED-LINE-RATIONALE 07d1fbaca6b02927:1: Delegates `modal_width`'s `max / min / ui` work to `max.min` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2847d041c565e974:3: Bounds the `modal_width` implementation here so `max / min / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 22380fed9b8c3989:1: Supports `modal_head` while preserving the nearby constraint: line under it, and a close icon button (sm 32) when `closable`. Returns whether it was pressed.
// ADDED-LINE-RATIONALE 13d2097bc2eee263:1: Freezes `t` for `modal_head` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 09ede75445ffbd29:2: Freezes `close` for `modal_head` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0cc6cd34bb7e6a6a:1: Delegates `modal_head`'s `egui / Frame / new` work to `egui::Frame::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b18d6e2d94362825:1: Delegates `modal_head`'s `ui / set_width / width` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b40b9b5e7da72c02:1: Supports `modal_head` while preserving the nearby constraint: A row, not a free right-to-left layout: the modal's area has no height to centre in.
// ADDED-LINE-RATIONALE 1732bf2c01ef0f66:1: Freezes `if` for `modal_head` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 84fe22d140e9d07a:1: Delegates `modal_head`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9205233e1688ca7c:1: Delegates `modal_head`'s `ui / add_space / theme` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e9eea4333853f164:1: Bounds the `modal_head` implementation here so `ui / add_space / theme` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE da0086675940b089:1: Freezes `text_w` for `modal_head` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2a871da3e82949a4:1: Delegates `modal_head`'s `ui / allocate_ui / vec2` work to `ui.allocate_ui` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b2fec5788e234e43:1: Delegates `modal_head`'s `ui / set_width / text_w` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5eb1da465284a190:1: Delegates `modal_head`'s `ui / vertical / ui` work to `ui.vertical` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e22b0ffda133b7dc:1: Freezes `ui` for `modal_head` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c51be26888eeb6e8:1: Delegates `modal_head`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ab64dac67fbab4b1:1: Prevents `modal_head` from applying its side effect outside the `subtitle / is_empty` state.
// ADDED-LINE-RATIONALE a11f8245b17eeea1:1: Delegates `modal_head`'s `widgets / hud / ui` work to `widgets::hud` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b85a494c478466c4:1: Bounds the `modal_head` implementation here so `widgets / hud / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2e88f04766415134:1: Makes `widgets / hud / ui` an explicit part of `modal_head` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE a143eace57031004:1: Makes `closable` an explicit part of `modal_head` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 779328bb9fe1784a:1: Prevents `modal_head` from applying its side effect outside the `closable` state.
// ADDED-LINE-RATIONALE 37806c3bbb854bcc:1: Freezes `close` for `modal_head` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e9eea4333853f164:2: Bounds the `modal_head` implementation here so `close / widgets / icon_button` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c6c3d9aeccf794d4:1: Makes `close / widgets / icon_button` an explicit part of `modal_head` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE e1c6b5235e02d8a4:2: Makes `close` an explicit part of `modal_head` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 62a3c6e042fba0df:1: Makes `close` an explicit part of `modal_head` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 2847d041c565e974:4: Bounds the `modal_head` implementation here so `close` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 91b02ba3e2fcdff7:1: Supports `modal_body` while preserving the nearby constraint: DS Modal body: padding 12 20 20, 14 px ink-secondary text.
// ADDED-LINE-RATIONALE 04e34ecd296805ff:1: Delegates `modal_body`'s `egui / Frame / new` work to `egui::Frame::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b18d6e2d94362825:2: Delegates `modal_body`'s `ui / set_width / width` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b5856f63b415a79e:1: Delegates `modal_body`'s `add / ui` work to `add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e1c6b5235e02d8a4:3: Makes `add / ui` an explicit part of `modal_body` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 2847d041c565e974:5: Bounds the `modal_body` implementation here so `add / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 24c5c66e2e90b4cb:1: Supports `modal_foot` while preserving the nearby constraint: runs right to left, so the first thing added is the rightmost (the main action).
// ADDED-LINE-RATIONALE 13d2097bc2eee263:2: Freezes `t` for `modal_foot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fedf597ba84e0ab1:1: Freezes `radius` for `modal_foot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 58662e0bace91dcd:1: Freezes `r` for `modal_foot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b2686574e0844b24:1: Delegates `modal_foot`'s `fill / t / raised` work to `fill` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1a2c4ff48a7e5671:1: Delegates `modal_foot`'s `corner_radius / egui / CornerRadius` work to `corner_radius` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7eee6a2b79ef1c54:1: Delegates `modal_foot`'s `outer_margin / egui / Margin` work to `outer_margin` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4f86255c7e5f8a10:1: Delegates `modal_foot`'s `inner_margin / egui / Margin` work to `inner_margin` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b37da01851160b71:1: Delegates `modal_foot`'s `show / ui / ui` work to `show` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE feb354180ce20015:1: Delegates `modal_foot`'s `ui / set_width / width` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ff9317d5bfc5dc77:1: Delegates `modal_foot`'s `ui / horizontal / ui` work to `ui.horizontal` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c6c3d9aeccf794d4:2: Makes `ui / horizontal / ui` an explicit part of `modal_foot` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE a6fdd6d9a2297fa5:1: Delegates `modal_foot`'s `ui / painter / hline` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2847d041c565e974:6: Bounds the `modal_foot` implementation here so `ui / painter / hline` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 92b358c4e80a43e7:1: Supports `body_text` while preserving the nearby constraint: The modal's body text: DS Modal body, 14 px ink-secondary.
// ADDED-LINE-RATIONALE 13d2097bc2eee263:3: Freezes `t` for `body_text` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6c22e09fbc5fac53:1: Delegates `body_text`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2847d041c565e974:7: Bounds the `body_text` implementation here so `ui / add / egui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 03d19b4685078a48:1: Delegates `alert_popup`'s `Self / modal / feed_alert` work to `Self::modal` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e6fa291294fa7b0d:1: Freezes `width` for `alert_popup` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 757a2a36ec5a4ff6:1: Supports `alert_popup` while preserving the nearby constraint: Only OK acknowledges it: no close button, and Escape or a click outside leaves it for later.
// ADDED-LINE-RATIONALE 0a697fba51b883bc:1: Delegates `alert_popup`'s `Self / modal_body / ui` work to `Self::modal_body` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 427294494004c9f2:1: Delegates `alert_popup`'s `egui / ScrollArea / vertical` work to `egui::ScrollArea::vertical` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE aa0326e42f7e5ad8:1: Delegates `alert_popup`'s `Self / modal_foot / ui` work to `Self::modal_foot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 98d5b5b1e7bb3818:1: Supports `notes_dialog` while preserving the nearby constraint: Release notes read better a little wider than the DS's 480.
// ADDED-LINE-RATIONALE 21dea1bf182faf6d:1: Freezes `close` for `notes_dialog` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8ce8d7f848a7d8ac:1: Delegates `notes_dialog`'s `Self / modal_body / ui` work to `Self::modal_body` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 021d0ab9eb65364c:1: Delegates `notes_dialog`'s `egui / ScrollArea / vertical` work to `egui::ScrollArea::vertical` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f3b5e781d84e154d:1: Delegates `notes_dialog`'s `ui / set_width / ui` work to `ui.set_width` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4aa0a383faf9f0fe:1: Prevents `notes_dialog` from applying its side effect outside the `release / notes / is_empty` state.
// ADDED-LINE-RATIONALE 693180b16f75a0e1:1: Delegates `notes_dialog`'s `Self / body_text / ui` work to `Self::body_text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 762152878f2b8d0c:1: Makes `Self / body_text / ui` an explicit part of `notes_dialog` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 7ba8665fdfa3fece:1: Delegates `notes_dialog`'s `crate / notes / show` work to `crate::notes::show` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e9eea4333853f164:4: Bounds the `notes_dialog` implementation here so `crate / notes / show` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c6c3d9aeccf794d4:5: Makes `crate / notes / show` an explicit part of `notes_dialog` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9c74ace553bc4708:1: Delegates `notes_dialog`'s `Self / modal_foot / ui` work to `Self::modal_foot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9d5f70dc8ed7249d:1: Freezes `width` for `launcher_update_dialog` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 92ca457bb052e164:1: Freezes `close` for `launcher_update_dialog` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8ce8d7f848a7d8ac:2: Delegates `launcher_update_dialog`'s `Self / modal_body / ui` work to `Self::modal_body` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7244105fd3d1a5b5:1: Delegates `launcher_update_dialog`'s `Self / body_text / ui` work to `Self::body_text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e1c6b5235e02d8a4:7: Makes `Self / body_text / ui` an explicit part of `launcher_update_dialog` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9c74ace553bc4708:2: Delegates `launcher_update_dialog`'s `Self / modal_foot / ui` work to `Self::modal_foot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 500daeaeb0943c3e:1: Prevents `launcher_update_dialog` from applying its side effect outside the `caps / self_update` state.
// ADDED-LINE-RATIONALE b81e25a773ec81ff:1: Prevents `launcher_update_dialog` from applying its side effect outside the `widgets / button / ui` state.
// ADDED-LINE-RATIONALE ab931df0048b5fe8:1: Delegates `launcher_update_dialog`'s `on_hover_text / tl / Downloads` work to `on_hover_text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 48eba5b5cdd08199:1: Delegates `launcher_update_dialog`'s `clicked` work to `clicked` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e9eea2333853edfe:2: Bounds the `launcher_update_dialog` implementation here so `clicked` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2c118f0d6908f731:1: Delegates `launcher_update_dialog`'s `send / Command / UpdateLauncher` work to `self.send` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dada3a255a611039:1: Freezes `close` for `launcher_update_dialog` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8673f2f70691be9c:1: Makes `url / platform / and_then` an explicit part of `launcher_update_dialog` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE d12a4e17762c6e50:1: Freezes `url` for `launcher_update_dialog` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fef70652d7866d77:1: Prevents `launcher_update_dialog` from applying its side effect outside the `widgets / button / ui` state.
// ADDED-LINE-RATIONALE ab878031fd4d898c:1: Delegates `launcher_update_dialog`'s `ui / ctx / open_url` work to `ui.ctx` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 44b2cff69240e8d4:2: Bounds the `launcher_update_dialog` implementation here so `widgets / button / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2f664385a221c7db:1: Prevents `launcher_update_dialog` from applying its side effect outside the `widgets / button / ui` state.
// ADDED-LINE-RATIONALE c4db2afd8c7db189:1: Freezes `close` for `launcher_update_dialog` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 44b2cff69240e8d4:3: Bounds the `launcher_update_dialog` implementation here so `close` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 72cd58f73a16fcb2:1: Delegates `launcher_update_dialog`'s `ui / with_layout / egui` work to `ui.with_layout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3f5be90e8e01aaf5:1: Delegates `launcher_update_dialog`'s `widgets / hud_link / ui` work to `widgets::hud_link` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9d5f70dc8ed7249d:2: Freezes `width` for `uninstall_dialog` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 53c3088c3cc0f8f5:1: Freezes `close` for `uninstall_dialog` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 11e27f7023e8ecf4:1: Delegates `uninstall_dialog`'s `Self / modal_body / ui` work to `Self::modal_body` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9c74ace553bc4708:3: Delegates `uninstall_dialog`'s `Self / modal_foot / ui` work to `Self::modal_foot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 119f0c4a3c3d0e2c:1: Supports `settings_page` while preserving the nearby constraint: segmented control, Quit at the foot of General. The top bar's back button and Escape leave it.
// ADDED-LINE-RATIONALE c9212b9cfbec3a45:1: Freezes `SettingsTab` for `settings_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fbb0b2db06636949:1: Delegates `settings_general`'s `ui / selectable_value / lang_code` work to `ui.selectable_value` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8d14f8aa376ba0d3:1: Supports `settings_general` while preserving the nearby constraint: limit while it applies, and the platform apps are for.
// ADDED-LINE-RATIONALE 05b4512a97d8c2b1:1: Delegates `settings_general`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dc96d912544a28c7:1: Freezes `if` for `settings_general` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8b953be473d8deca:1: Delegates `settings_general`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2eb6a98c39bd8784:7: Bounds the `settings_general` implementation here so `ui / add / egui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8f95f1de8b988966:1: Delegates `settings_general`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8e9747284e4bc315:1: Delegates `settings_general`'s `ui / add_space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1de35c71ec711737:1: Delegates `settings_general`'s `widgets / switch / ui` work to `widgets::switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 17cc6704fb9914f6:1: Delegates `settings_general`'s `widgets / switch` work to `widgets::switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5bc1b1ce2f94e669:2: Keeps `ui` in `settings_general`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 802a795c1d379696:1: Keeps `settings / engine_auto_update` in `settings_general`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 89cb6f1884eaac31:1: Keeps `tl / Keep / the` in `settings_general`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE fba2c6fa83592979:2: Bounds the `settings_general` implementation here so `tl / Keep / the` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1ab80021e33afba9:1: Delegates `settings_general`'s `widgets / switch / ui` work to `widgets::switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 88269aee5b92cf12:1: Delegates `settings_general`'s `widgets / switch / ui` work to `widgets::switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2ff3f7a5921eb60f:1: Delegates `settings_general`'s `widgets / switch / ui` work to `widgets::switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 43d432b3e31643c5:1: Delegates `settings_general`'s `widgets / switch / ui` work to `widgets::switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4868bde88c653881:1: Delegates `settings_general`'s `widgets / switch / ui` work to `widgets::switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 56306aeed7852fab:1: Delegates `settings_general`'s `widgets / switch / ui` work to `widgets::switch` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6739115d71d98d10:1: Supports `settings_general` while preserving the nearby constraint: Secondary: a settings page has no primary action of its own.
// ADDED-LINE-RATIONALE d1cce5fc524b1e94:4: Prevents `settings_general` from applying its side effect outside the `caps / manage_apps` state.
// ADDED-LINE-RATIONALE ab98277060c9e805:1: Delegates `settings_general`'s `ui / add_space` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dfdc941904e86e2c:1: Prevents `settings_general` from applying its side effect outside the `widgets / button / ui` state.
// ADDED-LINE-RATIONALE f00a91e9173c6770:1: Delegates `settings_general`'s `host / quit` work to `host.quit` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 44b2cff69240e8d4:14: Bounds the `settings_general` implementation here so `host / quit` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2eb6a98c39bd8784:13: Bounds the `settings_general` implementation here so `host / quit` cannot leak into the adjacent declaration or control path.
//! Modal dialogs: release notes, uninstall confirmation, settings.

use crate::i18n::fmt;
use egui::{RichText, vec2};
use redrob_launcher_core::protocol::Command;
use redrob_launcher_core::{Os, PackageKind};

use crate::app::{Dialog, Host, LauncherUi};
use crate::glyphs::Icon;
use crate::settings::{CheckEvery, Settings};
use crate::theme::{self, ThemeChoice, Tokens};
use crate::widgets::{self, Cell, Kind};

impl LauncherUi {
    pub(crate) fn dialogs(&mut self, ctx: &egui::Context) {
        let Some(dialog) = self.dialog.clone() else {
            self.alert_popup(ctx);
            return;
        };
        let mut close = false;
        let response = Self::modal("dialog", ctx).show(ctx, |ui| {
            close = match &dialog {
                Dialog::Notes(id) => self.notes_dialog(ui, id),
                Dialog::ConfirmUninstall(id) => self.uninstall_dialog(ui, id),
                Dialog::LauncherUpdate(r) => self.launcher_update_dialog(ui, r),
            }
        });
        if close || response.should_close() {
            self.dialog = None;
        }
    }

    /// A DS Modal: surface-base, radius 16, a hairline, shadow-lg, on the DS scrim. Its head, body
    /// and footer bring their own padding ([`Self::modal_head`], [`Self::modal_body`],
    /// [`Self::modal_foot`]).
    fn modal(id: &str, ctx: &egui::Context) -> egui::Modal {
        let t = Tokens::get(ctx);
        egui::Modal::new(egui::Id::new(id)).backdrop_color(t.scrim()).frame(
            egui::Frame::new()
                .fill(t.bg)
                .stroke(t.hairline())
                .corner_radius(egui::CornerRadius::same(theme::radius::XL))
                .shadow(theme::shadow_lg(&t))
                .inner_margin(0),
        )
    }

    /// A modal's width: `max` (DS Modal 480), or less in a small window.
    fn modal_width(ui: &egui::Ui, max: f32) -> f32 {
        max.min(ui.ctx().content_rect().width() - 48.0)
    }

    /// DS Modal head (padding 20 20 0): an optional 40 px icon, the title in 21 / 600 with a meta
    /// line under it, and a close icon button (sm 32) when `closable`. Returns whether it was pressed.
    fn modal_head(ui: &mut egui::Ui, width: f32, icon: Option<egui::ImageSource<'static>>, title: &str, subtitle: &str, closable: bool) -> bool {
        let t = Tokens::get(ui.ctx());
        let mut close = false;
        egui::Frame::new().inner_margin(egui::Margin { left: 20, right: 20, top: 20, bottom: 0 }).show(ui, |ui| {
            ui.set_width(width - 40.0);
            // A row, not a free right-to-left layout: the modal's area has no height to centre in.
            ui.horizontal_top(|ui| {
                if let Some(icon) = icon {
                    ui.add(egui::Image::new(icon).fit_to_exact_size(vec2(40.0, 40.0)));
                    ui.add_space(theme::space::S1);
                }
                let text_w = (ui.available_width() - if closable { theme::control::SM + theme::space::S2 } else { 0.0 }).max(0.0);
                ui.allocate_ui(vec2(text_w, 0.0), |ui| {
                    ui.set_width(text_w);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.add(egui::Label::new(widgets::heading(title, theme::text::TITLE_3, t.ink)).wrap());
                        if !subtitle.is_empty() {
                            widgets::hud(ui, subtitle, t.faint);
                        }
                    });
                });
                if closable {
                    close = widgets::icon_button(ui, Icon::Close, tl!("Close"), theme::control::SM).clicked();
                }
            });
        });
        close
    }

    /// DS Modal body: padding 12 20 20, 14 px ink-secondary text.
    fn modal_body(ui: &mut egui::Ui, width: f32, add: impl FnOnce(&mut egui::Ui)) {
        egui::Frame::new().inner_margin(egui::Margin { left: 20, right: 20, top: 12, bottom: 20 }).show(ui, |ui| {
            ui.set_width(width - 40.0);
            add(ui);
        });
    }

    /// DS Modal footer: right aligned, on surface-raised under a hairline, padding 16 20. `add`
    /// runs right to left, so the first thing added is the rightmost (the main action).
    fn modal_foot(ui: &mut egui::Ui, width: f32, add: impl FnOnce(&mut egui::Ui)) {
        let t = Tokens::get(ui.ctx());
        let radius = theme::radius::XL - 1;
        let r = egui::Frame::new()
            .fill(t.raised())
            .corner_radius(egui::CornerRadius { nw: 0, ne: 0, sw: radius, se: radius })
            .outer_margin(egui::Margin { left: 1, right: 1, top: 0, bottom: 1 })
            .inner_margin(egui::Margin::symmetric(19, 16))
            .show(ui, |ui| {
                ui.set_width(width - 40.0);
                ui.horizontal(|ui| ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), add));
            });
        ui.painter().hline(r.response.rect.x_range(), r.response.rect.top(), t.hairline());
    }

    /// The modal's body text: DS Modal body, 14 px ink-secondary.
    fn body_text(ui: &mut egui::Ui, text: &str) {
        let t = Tokens::get(ui.ctx());
        ui.add(egui::Label::new(RichText::new(text).size(theme::text::CONTROL).color(t.muted)).wrap());
    }

    /// The first alert from the release feed the user hasn't acknowledged, as a popup.
    fn alert_popup(&mut self, ctx: &egui::Context) {
        let Some(alert) = self.alerts.iter().find(|a| !self.settings.dismissed_notices.contains(&a.id)).cloned() else { return };
        let mut ok = false;
        Self::modal("feed_alert", ctx).show(ctx, |ui| {
            let width = Self::modal_width(ui, 480.0);
            // Only OK acknowledges it: no close button, and Escape or a click outside leaves it for later.
            Self::modal_head(ui, width, None, &alert.title, "", false);
            Self::modal_body(ui, width, |ui| {
                egui::ScrollArea::vertical().max_height(ctx.content_rect().height() * 0.5).show(ui, |ui| Self::body_text(ui, &alert.body));
            });
            Self::modal_foot(ui, width, |ui| {
                ok = widgets::button(ui, tl!("OK"), Kind::Primary, 90.0).clicked();
                if let Some(link) = &alert.link
                    && widgets::button(ui, tl!("Open link"), Kind::Secondary, 110.0).on_hover_text(link.as_str()).clicked()
                {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(link.clone()));
                }
            });
        });
        if ok {
            self.settings.dismissed_notices.insert(alert.id);
        }
    }

    fn notes_dialog(&mut self, ui: &mut egui::Ui, id: &str) -> bool {
        let Some(entry) = self.entry(id).cloned() else { return true };
        let Some(release) = entry.release().cloned() else { return true };
        // Release notes read better a little wider than the DS's 480.
        let width = Self::modal_width(ui, 560.0);
        let date = release.published_at.map(|d| fmt(tl!("Released {date}"), &[("date", &crate::views::date_text(d))])).unwrap_or_default();
        let close = Self::modal_head(ui, width, crate::icons::app(id), &format!("{} {}", entry.app.name, release.version_label()), &date, true);
        Self::modal_body(ui, width, |ui| {
            egui::ScrollArea::vertical().max_height(ui.ctx().content_rect().height() * 0.6).show(ui, |ui| {
                ui.set_width(ui.available_width());
                if release.notes.is_empty() {
                    Self::body_text(ui, tl!("These notes weren't fetched (GitHub's request limit was reached). They're on the release page."));
                } else {
                    crate::notes::show(ui, &release.notes);
                }
            });
        });
        Self::modal_foot(ui, width, |ui| {
            widgets::hud_link(ui, tl!("Open on GitHub"), &release.html_url);
        });
        close
    }

    fn launcher_update_dialog(&mut self, ui: &mut egui::Ui, r: &redrob_launcher_core::Release) -> bool {
        let width = Self::modal_width(ui, 480.0);
        let title = fmt(tl!("Redrob Launcher {version} is available."), &[("version", &r.version_label())]);
        let mut close = Self::modal_head(ui, width, Some(crate::icons::redrob_mark()), &title, "", true);
        Self::modal_body(ui, width, |ui| {
            Self::body_text(ui, tl!("A new version is available. Would you like to relaunch with the updated version?"))
        });
        Self::modal_foot(ui, width, |ui| {
            if self.caps.self_update {
                if widgets::button(ui, tl!("Update and restart"), Kind::Primary, 0.0)
                    .on_hover_text(tl!("Downloads the new version, replaces this one and opens it again"))
                    .clicked()
                {
                    self.send(Command::UpdateLauncher);
                    close = true;
                }
            } else {
                let url = self.platform.and_then(|p| crate::views::launcher_download(r, p)).unwrap_or_else(|| r.html_url.clone());
                if widgets::button(ui, tl!("Download"), Kind::Primary, 0.0).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                    close = true;
                }
            }
            if widgets::button(ui, tl!("Later"), Kind::Secondary, 80.0).clicked() {
                close = true;
            }
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                widgets::hud_link(ui, tl!("What's new"), &r.html_url);
            });
        });
        close
    }

    fn uninstall_dialog(&mut self, ui: &mut egui::Ui, id: &str) -> bool {
        let Some(entry) = self.entry(id).cloned() else { return true };
        let width = Self::modal_width(ui, 480.0);
        let mut close = Self::modal_head(ui, width, crate::icons::app(id), &fmt(tl!("Uninstall {name}?"), &[("name", entry.app.name)]), "", true);
        let how = match entry.installed.as_ref().map(|i| i.source) {
            Some(redrob_launcher_core::InstallSource::MacApp) => tl!("The app will be moved to the Trash. Your documents and settings stay."),
            Some(redrob_launcher_core::InstallSource::Msi) => {
                tl!("Windows will remove it like any installed program. Your documents and settings stay.")
            }
            Some(
                redrob_launcher_core::InstallSource::Deb | redrob_launcher_core::InstallSource::Rpm | redrob_launcher_core::InstallSource::Pacman,
            ) => tl!("Your system's package manager will remove it, and may ask for your password. Your documents and settings stay."),
            _ => tl!("The app will be removed. Your documents and settings stay."),
        };
        Self::modal_body(ui, width, |ui| Self::body_text(ui, how));
        Self::modal_foot(ui, width, |ui| {
            if widgets::button(ui, tl!("Uninstall"), Kind::Danger, 110.0).clicked() {
                self.send(Command::Uninstall { app: id.to_string() });
                close = true;
            }
            if widgets::button(ui, tl!("Cancel"), Kind::Secondary, 90.0).clicked() {
                close = true;
            }
        });
        close
    }

    /// The Settings page (its title is in the page's top bar): General, Channels and Apps as a
    /// segmented control, Quit at the foot of General. The top bar's back button and Escape leave it.
    pub(crate) fn settings_page(&mut self, ui: &mut egui::Ui, host: &mut dyn Host) {
        // Channels and Apps need the engine's files and daemon: desktop only.
        if self.backend.channels().is_some() {
            use crate::channels::SettingsTab;
            let mut tab = self.settings_tab;
            widgets::cells(
                ui,
                &mut tab,
                &[
                    (SettingsTab::General, Cell { label: tl!("General"), tip: tl!("Language, updates, appearance"), count: None }),
                    (
                        SettingsTab::Channels,
                        Cell { label: tl!("Channels"), tip: tl!("Talk to the engine from Slack, Telegram, Discord or WhatsApp"), count: None },
                    ),
                    (SettingsTab::Apps, Cell { label: tl!("Apps"), tip: tl!("Redrob apps bridged to the engine right now"), count: None }),
                ],
            );
            self.settings_tab = tab;
            ui.add_space(10.0);
            match tab {
                SettingsTab::General => self.settings_general(ui, host),
                SettingsTab::Channels => self.channels_tab(ui),
                SettingsTab::Apps => self.apps_tab(ui),
            }
        } else {
            self.settings_general(ui, host);
        }
    }

    /// The General page of Settings: language, updates, Linux packages, background, appearance.
    fn settings_general(&mut self, ui: &mut egui::Ui, host: &mut dyn Host) {
        let t = Tokens::get(ui.ctx());
        let section = |ui: &mut egui::Ui, s: &str| {
            ui.add_space(14.0);
            widgets::hud(ui, s, t.faint);
            ui.add_space(6.0);
        };
        let row = |ui: &mut egui::Ui, label: &str, add: &mut dyn FnMut(&mut egui::Ui)| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| add(ui));
            });
        };

        let mut lang_code = self.settings.language.clone();
        row(ui, tl!("Language"), &mut |ui| {
            egui::ComboBox::from_id_salt("language").selected_text(Settings::language_label(&lang_code)).width(180.0).show_ui(ui, |ui| {
                ui.selectable_value(&mut lang_code, Settings::LANGUAGE_AUTO.to_string(), Settings::system_language_label());
                for lang in crate::i18n::Lang::all() {
                    ui.selectable_value(&mut lang_code, lang.code().to_string(), Settings::language_label(lang.code()));
                }
            });
        });
        if lang_code != self.settings.language {
            self.settings.language = lang_code;
            crate::i18n::set_current(crate::app::effective_lang(&self.settings.language));
        }
        // One row per installed app: "Same as launcher" or a language of its own. Published for
        // the app in the shared language file; the app reads it, the launcher never restarts it.
        let installed: Vec<&'static redrob_launcher_core::CraftApp> =
            self.entries.iter().filter(|e| e.installed.is_some() && e.app.id != crate::engine_update::ENGINE_APP).map(|e| e.app).collect();
        if !installed.is_empty() {
            ui.add_space(4.0);
            ui.add(egui::Label::new(RichText::new(tl!("Per-app language")).size(12.0).color(t.faint)).wrap());
            for app in installed {
                let mut code = self.settings.app_languages.get(app.id).cloned().unwrap_or_default();
                let label = if code.is_empty() { tl!("Same as launcher").to_string() } else { Settings::language_label(&code) };
                row(ui, app.name, &mut |ui| {
                    egui::ComboBox::from_id_salt(("app-language", app.id)).selected_text(label.clone()).width(180.0).show_ui(ui, |ui| {
                        ui.selectable_value(&mut code, String::new(), tl!("Same as launcher"));
                        for lang in crate::i18n::Lang::all() {
                            ui.selectable_value(&mut code, lang.code().to_string(), Settings::language_label(lang.code()));
                        }
                    });
                });
                if code.is_empty() {
                    self.settings.app_languages.remove(app.id);
                } else {
                    self.settings.app_languages.insert(app.id.to_string(), code);
                }
            }
            ui.add_space(4.0);
        }
        if crate::i18n::Lang::from_pref(&self.settings.language).needs_cjk_font() && !theme::cjk_font_available() {
            // In English on purpose: the language asked for can't be drawn.
            let note = "No Chinese font was found on this system, so the interface stays in English. \
                        Installing one (Noto Sans CJK, or WenQuanYi on Linux) and restarting fixes it.";
            ui.add(egui::Label::new(RichText::new(note).size(12.0).color(t.warn)).wrap());
        }

        section(ui, tl!("Updates"));
        let mut every = self.settings.check_every;
        row(ui, tl!("Check for updates"), &mut |ui| {
            egui::ComboBox::from_id_salt("every").selected_text(every.label()).width(150.0).show_ui(ui, |ui| {
                for e in CheckEvery::ALL {
                    ui.selectable_value(&mut every, e, e.label());
                }
            });
        });
        self.settings.check_every = every;
        // What the Apps page's footer used to say: the last check (and from where), GitHub's
        // limit while it applies, and the platform apps are for.
        let status = self.update_status();
        ui.add(egui::Label::new(RichText::new(&status.checked).size(theme::text::META).color(t.muted)).wrap());
        if let Some(limit) = &status.rate_limit {
            ui.add(egui::Label::new(RichText::new(limit).size(theme::text::META).color(t.warn)).wrap());
        }
        ui.add(egui::Label::new(RichText::new(&status.platform).size(theme::text::META).color(t.faint)).wrap());
        ui.add_space(4.0);
        if self.caps.manage_apps
            && widgets::switch(ui, &mut self.settings.auto_update, tl!("Automatically download and install updates")).changed()
            && self.settings.auto_update
        {
            self.trigger_auto_updates();
        }
        if self.caps.manage_apps {
            widgets::switch(
                ui,
                &mut self.settings.engine_auto_update,
                tl!("Keep the Redrob Code engine up to date on its own (when no app is using it)"),
            );
        }
        widgets::switch(ui, &mut self.settings.notify, tl!("Notify me when an installed app has an update"));
        widgets::switch(ui, &mut self.settings.prereleases, tl!("Include pre-releases (betas and release candidates)"));
        if self.caps.manage_apps {
            widgets::switch(ui, &mut self.settings.prism_runtime, tl!("Offer models that need PrismML's llama.cpp fork (Bonsai 2)"));
        }

        if self.caps.manage_apps && self.platform.is_some_and(|p| p.os == Os::Linux) {
            section(ui, tl!("Installing on Linux"));
            let is_appimage = crate::settings::is_appimage();
            let default_kind = if is_appimage { Some(PackageKind::AppImage) } else { None };
            let mut choice = self.settings.preferred_kinds.first().copied().or(default_kind);
            row(ui, tl!("Install new apps as"), &mut |ui| {
                let default_label = match default_kind {
                    Some(k) => crate::i18n::t(k.label()),
                    None => tl!("System package (recommended)"),
                };
                let label = choice.map_or(default_label, |k| crate::i18n::t(k.label()));
                egui::ComboBox::from_id_salt("kind").selected_text(label).width(210.0).show_ui(ui, |ui| {
                    if !is_appimage {
                        ui.selectable_value(&mut choice, None, tl!("System package (recommended)"));
                    }
                    for k in [PackageKind::Deb, PackageKind::Rpm, PackageKind::AppImage] {
                        ui.selectable_value(&mut choice, Some(k), crate::i18n::t(k.label()));
                    }
                });
            });
            self.settings.preferred_kinds = choice.into_iter().collect();
            ui.label(RichText::new(tl!("Updates always use the format an app was installed with.")).size(12.0).color(t.muted));
        }

        if self.caps.manage_apps {
            section(ui, tl!("Running in the background"));
            if host.has_tray() {
                widgets::switch(ui, &mut self.settings.close_to_tray, tl!("Keep running in the tray when the window is closed"));
            }
            if host.has_desktops() {
                widgets::switch(ui, &mut self.settings.show_on_current_desktop, tl!("Show the window on the current desktop"))
                    .on_hover_text(tl!("When on, opening the launcher from the menu bar brings its window to the desktop you're on. When off, macOS switches you to the desktop the window was on."));
            }
            if let Some(mut on) = host.login_item()
                && widgets::switch(ui, &mut on, tl!("Start Redrob Launcher when I log in")).changed()
                && let Err(e) = host.set_login_item(on)
            {
                let ctx = ui.ctx().clone();
                self.toast(&ctx, fmt(tl!("Couldn't change the login item: {e}"), &[("e", &e)]), true);
            }
        }

        section(ui, tl!("Redrob"));
        ui.label(RichText::new(tl!("News and source code for every Redrob app")).color(t.muted));
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            use redrob_launcher_core::catalog::{GITHUB_URL, WEBSITE_URL};
            // Secondary: a settings page has no primary action of its own.
            if widgets::button(ui, tl!("Website"), Kind::Secondary, 0.0).on_hover_text(WEBSITE_URL).clicked() {
                ui.ctx().open_url(egui::OpenUrl::new_tab(WEBSITE_URL));
            }
            if widgets::button(ui, tl!("GitHub"), Kind::Secondary, 0.0).on_hover_text(GITHUB_URL).clicked() {
                ui.ctx().open_url(egui::OpenUrl::new_tab(GITHUB_URL));
            }
        });

        section(ui, tl!("Appearance"));
        let mut theme_choice = self.settings.theme;
        ui.horizontal(|ui| {
            widgets::cells(ui, &mut theme_choice, &ThemeChoice::ALL.map(|c| (c, Cell { label: c.label(), tip: c.label(), count: None })));
        });
        self.settings.theme = theme_choice;

        if self.caps.manage_apps {
            ui.add_space(24.0);
            if widgets::button(ui, tl!("Quit Redrob Launcher"), Kind::Secondary, 0.0).clicked() {
                host.quit();
            }
        }
    }
}
