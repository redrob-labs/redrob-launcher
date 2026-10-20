// ADDED-LINE-RATIONALE d967a2b7672a423e:1: Gives `widgets` direct access to `crate / glyphs / Icon` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE d566b3fb7d262cc0:1: Gives `widgets` direct access to `crate / theme / Tokens` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 6ddcf8a938b336c0:1: Supports `shelf_heading` while preserving the nearby constraint: SemiBold face rather than carrying a separate multi-megabyte 700 face for three labels.
// ADDED-LINE-RATIONALE 18843d98619f54f6:1: Freezes `size` for `shelf_heading` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 44a4ead85affcbb6:1: Delegates `shelf_heading`'s `ui / add` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f4fe202ef53c6652:1: Delegates `shelf_heading`'s `egui / Label / new` work to `egui::Label::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ef147b2b26abd9f1:1: Delegates `shelf_heading`'s `selectable` work to `selectable` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 107cd942377a3214:1: Keeps `truncate` in `shelf_heading`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE fe63d6ece9199774:1: Bounds the `shelf_heading` implementation here so `truncate` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 36f6464bf44d178d:1: Supports `loader_inline` while preserving the nearby constraint: halfway lit, and ask for no repaints.
// ADDED-LINE-RATIONALE 923c69e8508f5ddf:2: Freezes `t` for `loader_inline` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3ae69535d522bc8d:1: Freezes `let` for `loader_inline` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b12311e24f457e67:1: Delegates `loader_inline`'s `paint_loader / ui / rect` work to `paint_loader` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE da50407db57f7e8f:1: Delegates `loader_inline`'s `resp / widget_info / egui` work to `resp.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 57dae3898f1c1f1b:1: Supports `paint_loader` while preserving the nearby constraint: Paints the DS Loader into `rect`, animated unless motion is off.
// ADDED-LINE-RATIONALE 79a87a86a31fecde:1: Freezes `if` for `paint_loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 07848a6a9a9d552d:1: Freezes `still` for `paint_loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0ea41cb19b291d2c:1: Delegates `paint_loader`'s `crate / glyphs / loader` work to `crate::glyphs::loader` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f8539a7ca2b090d0:1: Makes `crate / glyphs / loader` an explicit part of `paint_loader` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE c990bcbe0b4d6eb2:1: Delegates `paint_loader`'s `crate / glyphs / loader` work to `crate::glyphs::loader` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1eb994e6023b3a64:1: Delegates `paint_loader`'s `ui / ctx / request_repaint` work to `ui.ctx` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:2: Bounds the `paint_loader` implementation here so `ui / ctx / request_repaint` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE cd3c5644b537d7de:1: Supports `card_frame` while preserving the nearby constraint: A DS Card on a page: surface-raised, a border-subtle hairline, radius 12, padding 20.
// ADDED-LINE-RATIONALE a0b04e83bb0dad8d:1: Delegates `card_frame`'s `egui / Frame / new` work to `egui::Frame::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c5265201145869b0:1: Delegates `card_frame`'s `fill / t / raised` work to `fill` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dca4d3328db02fbf:1: Delegates `card_frame`'s `stroke / t / hairline` work to `stroke` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 101fe9cc51b545c0:1: Delegates `card_frame`'s `corner_radius / CornerRadius / same` work to `corner_radius` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e34a3aed32ce34ba:1: Delegates `card_frame`'s `inner_margin / egui / Margin` work to `inner_margin` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:11: Bounds the `card_frame` implementation here so `inner_margin / egui / Margin` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7093e902eab27dda:1: Supports `row_rule` while preserving the nearby constraint: A full-width hairline between rows inside a card, with `gap` above and below.
// ADDED-LINE-RATIONALE caf4febc02073bff:1: Delegates `row_rule`'s `ui / add_space / gap` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b4c893fb22b0dad4:1: Freezes `let` for `row_rule` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0b0596150ad27b10:1: Delegates `row_rule`'s `ui / painter / hline` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE caf4febc02073bff:2: Delegates `row_rule`'s `ui / add_space / gap` work to `ui.add_space` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8b584d7e09f93e3e:1: Supports `alert_frame` while preserving the nearby constraint: For things that stay until they are dealt with (an update, an announcement).
// ADDED-LINE-RATIONALE a0b04e83bb0dad8d:2: Delegates `alert_frame`'s `egui / Frame / new` work to `egui::Frame::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5c3ac5b6a0b20a71:1: Delegates `alert_frame`'s `fill / t / brand_subtle` work to `fill` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1c0642b6b46c7f0f:1: Delegates `alert_frame`'s `stroke / Stroke / new` work to `stroke` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 218ccac3ceccfd7c:1: Delegates `alert_frame`'s `corner_radius / CornerRadius / same` work to `corner_radius` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0e66eb2520239865:1: Delegates `alert_frame`'s `inner_margin / egui / Margin` work to `inner_margin` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:13: Bounds the `alert_frame` implementation here so `inner_margin / egui / Margin` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 080bac8810909276:1: Supports `alert_frame_dense` while preserving the nearby constraint: 32 px control the bar is 44 px tall (was about 66).
// ADDED-LINE-RATIONALE 838fb209ccdc6775:1: Delegates `alert_frame_dense`'s `alert_frame / t / inner_margin` work to `alert_frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:14: Bounds the `alert_frame_dense` implementation here so `alert_frame / t / inner_margin` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 24448b2cb3a723f9:1: Supports `alert_frame_dense` while preserving the nearby constraint: a custom title bar, and otherwise hugging the edge (the central panel's right margin).
// ADDED-LINE-RATIONALE 8b8d3adce68fcfe5:1: Supports `alert_frame_dense` while preserving the nearby constraint: window's edge (DS page padding) with the bar over that padding, not over the content.
// ADDED-LINE-RATIONALE a71f3b55f144b07d:1: Supports `page_scroll` while preserving the nearby constraint: page with the page's own style (only the bar is restyled).
// ADDED-LINE-RATIONALE f35e3f8c638b83e7:1: Freezes `page_style` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a378da18bf178a38:1: Delegates `page_scroll`'s `ui / scope / ui` work to `ui.scope` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1022098f63b68548:1: Freezes `style` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ad6e902bbb6a3575:1: Freezes `scroll` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b2d5caf3699c5182:1: Freezes `scroll` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a55aba91db2f7ff3:1: Supports `page_scroll` while preserving the nearby constraint: The same width hovered or not: the thumb fades in, it doesn't grow.
// ADDED-LINE-RATIONALE 5eaae28fc35b0519:1: Freezes `scroll` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4211ff451052b27b:1: Freezes `scroll` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cd397c05c519cb9e:1: Freezes `scroll` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2de27ac0ca1ca565:1: Supports `page_scroll` while preserving the nearby constraint: The thumb takes the widgets' foreground colours set below; no track behind it.
// ADDED-LINE-RATIONALE 0af7bea75dd5bbf0:1: Freezes `scroll` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3ceabe6c4b19eee7:1: Freezes `scroll` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0588f9dfc5f49eef:1: Freezes `scroll` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 86a800a897ac23ea:1: Supports `page_scroll` while preserving the nearby constraint: Hidden until the pointer is over the page, like Cowork's.
// ADDED-LINE-RATIONALE bd7fe753a074aa98:1: Freezes `scroll` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bd646101d744c760:1: Freezes `scroll` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5e347ded066faca1:1: Freezes `style` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 23a5f74232d4699c:1: Supports `page_scroll` while preserving the nearby constraint: The thumb: border-strong at rest, a step darker hovered and dragged, fully round.
// ADDED-LINE-RATIONALE 2feb654d07f4b400:1: Keeps `style / visuals / widgets` in `page_scroll`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f9e5c78ae594d760:1: Keeps `style / visuals / widgets` in `page_scroll`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE ad966e1115fe70ee:1: Keeps `style / visuals / widgets` in `page_scroll`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 0c0313dba5890079:1: Makes `style / visuals / widgets` an explicit part of `page_scroll` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE ab9ad80f2a5c6cbf:1: Freezes `w` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 19f045649f4c75f8:1: Freezes `w` for `page_scroll` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:1: Bounds the `page_scroll` implementation here so `w / corner_radius / CornerRadius` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8623596d655cea5e:1: Delegates `page_scroll`'s `egui / ScrollArea / vertical` work to `egui::ScrollArea::vertical` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4c3fd7bd127a9232:1: Delegates `page_scroll`'s `id_salt / id_salt` work to `id_salt` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE acfd12a670048804:1: Delegates `page_scroll`'s `auto_shrink` work to `auto_shrink` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0b52fbcf06af45d4:1: Delegates `page_scroll`'s `content_margin / egui / Margin` work to `content_margin` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dadfa4de034a1a15:1: Delegates `page_scroll`'s `show / ui / ui` work to `show` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6be3d09f15f157a8:1: Delegates `page_scroll`'s `ui / set_style / page_style` work to `ui.set_style` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c43fa12c79520205:1: Delegates `page_scroll`'s `add / ui` work to `add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7616fc659e2da703:1: Makes `add / ui` an explicit part of `page_scroll` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE a5411690c41a7593:1: Makes `inner` an explicit part of `page_scroll` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 5dd3099016fb22e3:1: Makes `inner` an explicit part of `page_scroll` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE c6fe34784c14f1f3:1: Makes `inner` an explicit part of `page_scroll` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 137904f9a2de7fb7:1: Supports `WindowButton` while preserving the nearby constraint: Width of a caption button: Windows' own (Redrob Cowork keeps the native 46 px ones).
// ADDED-LINE-RATIONALE 274c15c648984ef0:1: Supports `window_buttons` while preserving the nearby constraint: (0x1A, 0x33 of 0xFF).
// ADDED-LINE-RATIONALE 553a21d0a0ec50da:1: Freezes `CAPTION_PRESSED_ALPHA` for `window_buttons` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6819aca0ac4a0cb3:1: Supports `window_buttons` while preserving the nearby constraint: Close pressed: the red at Electron's 0x98.
// ADDED-LINE-RATIONALE ead45e443efa6cc9:1: Freezes `let` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f73c42ca43c7eb61:1: Freezes `maximized` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a1c2e7f01db619c9:1: Freezes `edge` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f2b594fe8ce5b177:1: Freezes `right` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fcc43d268c617905:1: Freezes `hit` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9dd90e700ab92c45:1: Freezes `pressed` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1a1e3de4210842ef:1: Freezes `hovered` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a79212c57dec0854:1: Prevents `window_button` from applying its side effect outside the `matches / button / WindowButton` state.
// ADDED-LINE-RATIONALE 2f753bfd152938c7:1: Delegates `window_button`'s `crate / caption / publish_maximize` work to `crate::caption::publish_maximize` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e67d5f767b79ba12:1: Freezes `os` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b6fadfb75ad34a73:1: Freezes `hovered` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 56fe6f2ae9ff4adf:1: Freezes `pressed` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fe6402ece919e238:8: Bounds the `window_button` implementation here so `pressed / os / pressed` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 42bac8542ce94b47:1: Freezes `k` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5c656d9f03025fe9:1: Supports `window_button` while preserving the nearby constraint: on a dark one), not with the symbol colour.
// ADDED-LINE-RATIONALE 079542f6742fa737:1: Freezes `symbol` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0eb77aec65e57a19:1: Freezes `close_hover` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1d09cd69e53abb7f:1: Freezes `let` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dbd8f81969e03a61:1: Freezes `WindowButton` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c133085cc48e2271:1: Freezes `WindowButton` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fd4a1d3eb6ef7369:1: Freezes `_` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f9cc22b26a8cbf7b:1: Freezes `_` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a22c0b96b19fc7f6:1: Prevents `window_button` from applying its side effect outside the `k / pressed` state.
// ADDED-LINE-RATIONALE aa16a4079e66e854:1: Delegates `window_button`'s `p / rect_filled / rect` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:9: Bounds the `window_button` implementation here so `p / rect_filled / rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7fc3908235ba891f:1: Supports `window_button` while preserving the nearby constraint: button's centre (measured against Cowork at 150%: 2 px down, 1 px right).
// ADDED-LINE-RATIONALE 96dc65da264e365a:1: Freezes `glyph_rect` for `window_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 65ed9513be74e795:1: Delegates `window_button`'s `caption_glyph / p / glyph_rect` work to `caption_glyph` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE de54463df2b7d6a0:1: Delegates `window_button`'s `resp / widget_info / egui` work to `resp.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bf83c682a6443871:1: Delegates `window_button`'s `tip / resp / label` work to `tip` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:19: Bounds the `window_button` implementation here so `tip / resp / label` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6367447d0222ae30:1: Supports `window_button` while preserving the nearby constraint: Scale on close's stroke so egui's feathered diagonals carry the ink of Chromium's.
// ADDED-LINE-RATIONALE 7f4efd0fce816243:1: Supports `caption_glyph` while preserving the nearby constraint: (10% when white) halo.
// ADDED-LINE-RATIONALE 5ef0478ea3edaebf:1: Gives `caption_glyph` direct access to `egui / pos2` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 2877b677bde86558:1: Freezes `let` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b18c4c4f98ed58e0:1: Freezes `let` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9a8680aa8b902fce:1: Freezes `size` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 35fb5bafd88e6e6e:1: Freezes `let` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 36f8418d99b8c798:1: Freezes `w` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b8ec6d24c0f1ab01:1: Freezes `pt` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f222d7dd3604f100:1: Freezes `stroke` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 54e1c8b23ec47768:1: Freezes `radius` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d64ab939aee3915d:1: Freezes `half` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0a0f880b07bd5c4e:1: Supports `caption_glyph` while preserving the nearby constraint: because the tessellator feathers a filled rect's edges.
// ADDED-LINE-RATIONALE 5506b707fd0dd173:1: Freezes `top` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bb231874c027997a:1: Freezes `mesh` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0ef1042a31ec8e5d:1: Delegates `caption_glyph`'s `mesh / add_colored_rect / Rect` work to `mesh.add_colored_rect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b51db6176a912f4c:1: Delegates `caption_glyph`'s `p / add / egui` work to `p.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5c4507f488eb74b0:1: Freezes `r` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3d9a7b7f8122db59:1: Delegates `caption_glyph`'s `p / add / egui` work to `p.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 75eab4fed7bb867c:1: Freezes `sep` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e4449e668d950b1f:1: Supports `caption_glyph` while preserving the nearby constraint: Front square: the stroke-inset box, shrunk at the top and right for the back one.
// ADDED-LINE-RATIONALE 58132f14b3d7ef26:1: Delegates `caption_glyph`'s `p / add / egui` work to `p.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b736a8c64613a2eb:1: Supports `caption_glyph` while preserving the nearby constraint: sep - half) covers it, so only its top and right edges show.
// ADDED-LINE-RATIONALE 7b436c32feee46a5:1: Freezes `g` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b65ddb1384ab1607:1: Freezes `grown` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c4fce1280465c009:1: Freezes `path` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7e2cc8e24ac9a648:1: Freezes `outside` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d4a4513f779358b4:1: Supports `caption_glyph` while preserving the nearby constraint: The visible run is contiguous on the closed path: start it right after a hidden point.
// ADDED-LINE-RATIONALE 7471a1449be62a1d:1: Freezes `n` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 510e808c97ba0917:1: Freezes `run` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d94a1de735562ef9:1: Delegates `caption_glyph`'s `p / add / egui` work to `p.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6a3050f21c05fd58:1: Bounds the `caption_glyph` implementation here so `p / add / egui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE bfa1597224de4509:1: Freezes `halo` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8a41ff9977384dae:1: Supports `caption_glyph` while preserving the nearby constraint: same width: measured against Cowork at 150%, 0.68 of the stroke matches its weight.
// ADDED-LINE-RATIONALE f329de2b135382d4:1: Freezes `clip` for `caption_glyph` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 10ac55dfb1cd7ebe:1: Delegates `caption_glyph`'s `clip / line_segment / pt` work to `clip.line_segment` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 44a1766848bb8a0e:1: Delegates `caption_glyph`'s `clip / line_segment / pt` work to `clip.line_segment` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:20: Bounds the `caption_glyph` implementation here so `Points / along / a` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE af80a977cf3c6456:1: Supports `rounded_rect_path` while preserving the nearby constraint: densely enough (about 0.25 px apart) to be clipped point by point.
// ADDED-LINE-RATIONALE f8ca72faa0ff53b3:1: Gives `rounded_rect_path` direct access to `std / f32 / consts` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE b66e5a198f1bebe8:1: Freezes `let` for `rounded_rect_path` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9b007d123b019965:1: Freezes `let` for `rounded_rect_path` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 20bf33ea75c13bcb:1: Freezes `out` for `rounded_rect_path` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 13824efb2cab8997:1: Freezes `edge` for `rounded_rect_path` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2ec3094032c27c28:1: Freezes `steps` for `rounded_rect_path` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4bc5d37c5e427b44:1: Applies `rounded_rect_path` consistently to every `i / in / steps` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 507aa4a3cdaa8bed:1: Freezes `f` for `rounded_rect_path` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c4b1c9372aeef499:1: Delegates `rounded_rect_path`'s `out / push / to` work to `out.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:6: Bounds the `rounded_rect_path` implementation here so `out / push / to` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5dd2fb9016fb0b19:4: Bounds the `rounded_rect_path` implementation here so `arc / c / f32` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 02d7e7f142b7bc56:1: Freezes `arc` for `rounded_rect_path` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 134dfbd03097583b:1: Freezes `steps` for `rounded_rect_path` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4bc5d37c5e427b44:2: Applies `rounded_rect_path` consistently to every `i / in / steps` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 0317fabc54182f1e:1: Freezes `a` for `rounded_rect_path` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a5e44686fa278544:1: Delegates `rounded_rect_path`'s `out / push / c` work to `out.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:7: Bounds the `rounded_rect_path` implementation here so `out / push / c` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5dd2fb9016fb0b19:5: Bounds the `rounded_rect_path` implementation here so `arc / l / tl` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c8f4a815275be435:1: Delegates `rounded_rect_path`'s `arc / l / tl` work to `arc` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d3289e1ddb062ad4:1: Delegates `rounded_rect_path`'s `edge / l / tl` work to `edge` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 22aed85ffdf841a6:1: Delegates `rounded_rect_path`'s `arc / rr / tr` work to `arc` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bb74547ae5e523b2:1: Delegates `rounded_rect_path`'s `edge / rr / t` work to `edge` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0cd05d44d64f62ac:1: Delegates `rounded_rect_path`'s `arc / rr / br` work to `arc` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d85ed92309f84684:1: Delegates `rounded_rect_path`'s `edge / rr / br` work to `edge` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 39b65a1d1625bb57:1: Delegates `rounded_rect_path`'s `arc / l / bl` work to `arc` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0857138ece5b0c42:1: Delegates `rounded_rect_path`'s `edge / l / b` work to `edge` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 04eab4d76545da8f:1: Makes `out` an explicit part of `rounded_rect_path` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE ac06cb53bc9f054b:1: Delegates `badge`'s `badge_sized / ui / text` work to `badge_sized` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:24: Bounds the `badge` implementation here so `badge_sized / ui / text` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 676901d6095e22be:1: Supports `badge_sized` while preserving the nearby constraint: [`badge`] at the DS's sm (20 px, 11 px text) or md (24 px, 12 px text) size.
// ADDED-LINE-RATIONALE a2ffa4c018f8c246:1: Freezes `let` for `badge_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 72f6544eeb406665:1: Freezes `if` for `badge_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5528ec50a4d1e4f6:1: Freezes `galley` for `badge_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 296efaf6d7e8c87f:1: Freezes `let` for `badge_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e9b66dcfd8bf6161:1: Delegates `badge_sized`'s `ui / painter / rect_filled` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5e7d1f3672e2c804:1: Supports `Kind` while preserving the nearby constraint: No fill, `ink-brand` text; `surface-brand-subtle` on hover.
// ADDED-LINE-RATIONALE d8bfb596a327c193:1: Supports `Kind` while preserving the nearby constraint: that still reads as one, for chrome (Cowork's title bar carries no filled button).
// ADDED-LINE-RATIONALE e3b3134b8888d668:26: Bounds the `Kind` implementation here so `Tinted` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2086039342ebf327:1: Supports `Kind` while preserving the nearby constraint: The design system's control sizes.
// ADDED-LINE-RATIONALE 74cf6a914973ec5f:1: Supports `Size` while preserving the nearby constraint: [derive(Clone, Copy, Debug, Default, PartialEq, Eq)].
// ADDED-LINE-RATIONALE 41a418d6656d1f33:1: Supports `Size` while preserving the nearby constraint: 32 tall, 12 px padding, 13 px label.
// ADDED-LINE-RATIONALE 844fe5325409e0c3:1: Supports `Size` while preserving the nearby constraint: 40 tall, 16 px padding, 14 px label: the default.
// ADDED-LINE-RATIONALE dffcf8d6bfcdf178:1: Supports `Size` while preserving the nearby constraint: [default].
// ADDED-LINE-RATIONALE e7c2c1d6c3f47c68:1: Supports `Size` while preserving the nearby constraint: 48 tall, 20 px padding, 15 px label.
// ADDED-LINE-RATIONALE e3b3134b8888d668:27: Bounds the `Size` implementation here so `Lg` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2a8650e4bb1853ed:1: Makes `Size` an explicit part of `Size` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 7fccac0cbb35774a:1: Supports `metrics` while preserving the nearby constraint: (height, side padding, label size).
// ADDED-LINE-RATIONALE 5a220a6cb0e20045:1: Prevents `metrics` from applying its side effect outside the `metrics / f32 / f32` state.
// ADDED-LINE-RATIONALE 0b28dbd911dc836c:1: Freezes `Self` for `metrics` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 70ee10ab43a0a9b2:1: Freezes `Self` for `metrics` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6356ea9f0e07e9b6:1: Freezes `Self` for `metrics` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:9: Bounds the `metrics` implementation here so `Self / Lg / theme` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE fe6402ece919e238:14: Bounds the `metrics` implementation here so `Self / Lg / theme` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e3b3134b8888d668:28: Bounds the `metrics` implementation here so `DS / disabled / controls` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d665323bc5648515:1: Supports `metrics` while preserving the nearby constraint: DS disabled controls: the whole control at 45%.
// ADDED-LINE-RATIONALE 820caf4e6856f549:1: Supports `focus_ring` while preserving the nearby constraint: Drawn only when focus arrived from the keyboard (CSS `:focus-visible`): a click draws none.
// ADDED-LINE-RATIONALE 465093906085216c:1: Freezes `id` for `focus_ring` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 37b15d661a4ea191:1: Prevents `focus_ring` from applying its side effect outside the `resp / gained_focus` state.
// ADDED-LINE-RATIONALE 533bbcacc9c58a77:1: Freezes `by_pointer` for `focus_ring` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1ce1790425fd8b02:1: Delegates `focus_ring`'s `ui / data_mut / d` work to `ui.data_mut` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:15: Bounds the `focus_ring` implementation here so `ui / data_mut / d` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 52feac4ba3e1dd9a:1: Freezes `keyboard` for `focus_ring` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 204725fbe1bdea26:1: Prevents `focus_ring` from applying its side effect outside the `resp / has_focus / keyboard` state.
// ADDED-LINE-RATIONALE f8fefa26d66cd16f:1: Freezes `t` for `focus_ring` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f2fd1816a61d5b5e:1: Freezes `r` for `focus_ring` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 14d3908e2cee58b3:1: Delegates `focus_ring`'s `ui / painter / rect_stroke` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:16: Bounds the `focus_ring` implementation here so `ui / painter / rect_stroke` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3d5f7e0c72f64970:1: Supports `tip` while preserving the nearby constraint: our controls draw theirs with this.
// ADDED-LINE-RATIONALE 24985d6cf1a349cc:1: Freezes `t` for `tip` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d9633db3e16a3766:1: Freezes `let` for `tip` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d5e7e7d0a0a436ed:1: Freezes `tooltip` for `tip` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 14e83d8cd01d6239:1: Freezes `tooltip` for `tip` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c43ae32110b2017d:1: Delegates `tip`'s `egui / Frame / new` work to `egui::Frame::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d0c7df6a5ace71a4:1: Delegates `tip`'s `fill / fill` work to `fill` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2a765a4f0b66d73d:1: Delegates `tip`'s `corner_radius / CornerRadius / same` work to `corner_radius` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ad3e1b8172224504:1: Delegates `tip`'s `inner_margin / egui / Margin` work to `inner_margin` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3c09a870fa8a46d8:1: Keeps `shadow / theme / shadow_md` in `tip`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 5d3d8390167c1b3d:2: Bounds the `tip` implementation here so `shadow / theme / shadow_md` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE bcc66bd175b65ab2:1: Delegates `tip`'s `tooltip / show / ui` work to `tooltip.show` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d1a7ee147dc12716:1: Delegates `tip`'s `ui / add / egui` work to `ui.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 68bc17d70cbc3508:1: Makes `ui / add / egui` an explicit part of `tip` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 7aba7864cc1b751b:5: Makes `resp` an explicit part of `tip` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE e3b3134b8888d668:30: Bounds the `tip` implementation here so `resp` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE bb8224d71fc7229f:1: Delegates `button`'s `button_sized / ui / text` work to `button_sized` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:31: Bounds the `button` implementation here so `button_sized / ui / text` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7854f70af6c3ff11:1: Supports `button_sized` while preserving the nearby constraint: A DS button: radius 8, a 600-weight label, `size` tall, at least `width` wide.
// ADDED-LINE-RATIONALE a6a22925d007348e:1: Freezes `let` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1e2e1ee1c2720b45:1: Freezes `galley` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 268a83a39bc97782:1: Freezes `let` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2676dfc4bdb88996:1: Freezes `hovered` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a8938bc4160418d9:1: Freezes `let` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 522f7e1b20a0c819:1: Freezes `Kind` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 47323bde2214d9ca:1: Freezes `Kind` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8610430305d52780:1: Freezes `Kind` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 847b3969f646d8e3:1: Freezes `Kind` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 97d188326dc3cb5d:1: Freezes `Kind` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 42bac8542ce94b47:2: Supports `button_sized` while preserving the nearby constraint: The fill changes over the DS's 140 ms standard state change.
// ADDED-LINE-RATIONALE 70236710e40b37e8:1: Freezes `k` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 90c09c7510125605:1: Freezes `fill` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b5f87c7a08dd928c:1: Freezes `fade` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4b1bdbe9936db7a6:1: Freezes `radius` for `button_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4c9414917c7d0458:1: Delegates `button_sized`'s `ui / painter / rect` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7f6a331f691c4b5e:1: Delegates `button_sized`'s `focus_ring / ui / resp` work to `focus_ring` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 89b9d630ee0244be:1: Prevents `button_sized` from applying its side effect outside the `enabled / resp / on_hover_cursor` state.
// ADDED-LINE-RATIONALE e3b3134b8888d668:32: Bounds the `button_sized` implementation here so `enabled / resp / on_hover_cursor` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0ea9b540fbb5c2b1:1: Supports `button_width` while preserving the nearby constraint: The width [`button_sized`] gives `text` at `size` (with no minimum width).
// ADDED-LINE-RATIONALE 3f3d0ddaf5761b70:1: Freezes `let` for `button_width` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ae1fbdc21708e0df:1: Delegates `button_width`'s `caps / ui / text` work to `caps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:33: Bounds the `button_width` implementation here so `caps / ui / text` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 490b454ccef5d651:1: Supports `search_field` while preserving the nearby constraint: survives a change of page. Returns the text field's response.
// ADDED-LINE-RATIONALE 923c69e8508f5ddf:9: Freezes `t` for `search_field` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f09e618844aa54ea:1: Freezes `GLYPH` for `search_field` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4c0aec3dbf41d565:1: Freezes `PAD` for `search_field` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f8c0a165e1ddaade:1: Freezes `let` for `search_field` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a0c0112105b61f7d:1: Freezes `id` for `search_field` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3e67cf64011ad1e9:1: Freezes `strong` for `search_field` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a9c30473f4a1a7c8:2: Supports `search_field` while preserving the nearby constraint: Under the text field: drawn first.
// ADDED-LINE-RATIONALE caf141cebfb195c7:1: Delegates `search_field`'s `p / rect` work to `p.rect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c79ab26711e8fb51:1: Keeps `rect` in `search_field`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE b39408e641ba2f03:1: Keeps `CornerRadius / same / theme` in `search_field`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE b05f82e51714eabc:1: Keeps `t / bg` in `search_field`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE b3e6c3951dc8b7f4:1: Keeps `Stroke / new / strong` in `search_field`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 8d726d8543d2174f:2: Keeps `egui / StrokeKind / Inside` in `search_field`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 5d3d8390167c1b3d:3: Bounds the `search_field` implementation here so `egui / StrokeKind / Inside` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a1814f27782f2a06:1: Freezes `glyph` for `search_field` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f86608fcacb6e691:1: Delegates `search_field`'s `crate / glyphs / paint` work to `crate::glyphs::paint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5aa204913311d50e:1: Freezes `field` for `search_field` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4759818e6688c00c:1: Supports `search_field` while preserving the nearby constraint: A child, not `ui.put`: putting would move a right-to-left parent's cursor back to the field.
// ADDED-LINE-RATIONALE 98d2b8e43b568038:1: Freezes `resp` for `search_field` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a03ef980f953c0e4:1: Delegates `search_field`'s `egui / TextEdit / singleline` work to `egui::TextEdit::singleline` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5e24d0c09bd83e90:1: Delegates `search_field`'s `id / id` work to `id` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 64b6b54ba818f64e:1: Delegates `search_field`'s `frame / egui / Frame` work to `frame` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 78095307e1999cb9:1: Delegates `search_field`'s `hint_text / RichText / new` work to `hint_text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1120776e3c03bc5a:1: Delegates `search_field`'s `font / egui / FontId` work to `font` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 13886505b60a2313:1: Delegates `search_field`'s `text_color / t / ink` work to `text_color` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bf3ca07a13fa9fde:1: Delegates `search_field`'s `vertical_align / egui / Align` work to `vertical_align` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 66b56c337636f3f5:1: Delegates `search_field`'s `margin / vec2` work to `margin` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e375cd51608321f2:1: Keeps `desired_width / field / width` in `search_field`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 5d3d8390167c1b3d:4: Bounds the `search_field` implementation here so `desired_width / field / width` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 559aa69212c72cdc:1: Supports `search_field` while preserving the nearby constraint: The hint is only a placeholder to screen readers; the field's name is the hint's words.
// ADDED-LINE-RATIONALE e308ae40b800da04:1: Supports `search_field` while preserving the nearby constraint: The glyph and padding focus the field too.
// ADDED-LINE-RATIONALE 717638913c8644c3:1: Delegates `search_field`'s `ui / memory_mut / m` work to `ui.memory_mut` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:17: Bounds the `search_field` implementation here so `ui / memory_mut / m` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c874117469834ef3:1: Delegates `search_field`'s `resp / on_hover_cursor / egui` work to `resp.on_hover_cursor` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:34: Bounds the `search_field` implementation here so `resp / on_hover_cursor / egui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 081435917728818b:1: Supports `logo_button` while preserving the nearby constraint: never the mark. `label` is the tooltip and the name screen readers read.
// ADDED-LINE-RATIONALE 923c69e8508f5ddf:10: Freezes `t` for `logo_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8b1d18fab286d423:1: Freezes `let` for `logo_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2a569a39dd93f18e:1: Freezes `hovered` for `logo_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 42bac8542ce94b47:3: Freezes `k` for `logo_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 88dd8be1a95b8a06:1: Prevents `logo_button` from applying its side effect outside the `k` state.
// ADDED-LINE-RATIONALE f9b001ee1351ec6d:1: Delegates `logo_button`'s `ui / painter / rect_filled` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:18: Bounds the `logo_button` implementation here so `ui / painter / rect_filled` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2fbb848c35574efa:1: Delegates `logo_button`'s `egui / Image / new` work to `egui::Image::new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE de54463df2b7d6a0:2: Delegates `logo_button`'s `resp / widget_info / egui` work to `resp.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7f6a331f691c4b5e:2: Delegates `logo_button`'s `focus_ring / ui / resp` work to `focus_ring` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 37887562030daf81:1: Delegates `logo_button`'s `tip / resp / label` work to `tip` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:35: Bounds the `logo_button` implementation here so `tip / resp / label` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 768b8a37f914db25:1: Supports `icon_button` while preserving the nearby constraint: name screen readers read.
// ADDED-LINE-RATIONALE 5ade01040d535d67:1: Delegates `icon_button`'s `icon_button_busy / ui / icon` work to `icon_button_busy` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:36: Bounds the `icon_button` implementation here so `icon_button_busy / ui / icon` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ccecf0bd3f012148:1: Supports `icon_button_busy` while preserving the nearby constraint: [`icon_button`] that, while `busy`, shows the DS loader instead of its icon and can't be clicked.
// ADDED-LINE-RATIONALE 923c69e8508f5ddf:11: Freezes `t` for `icon_button_busy` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ab6acf25c9aa840b:1: Freezes `let` for `icon_button_busy` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3659e8cd9bc04e92:1: Freezes `enabled` for `icon_button_busy` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2676dfc4bdb88996:2: Freezes `hovered` for `icon_button_busy` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 42bac8542ce94b47:4: Freezes `k` for `icon_button_busy` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a9c30473f4a1a7c8:3: Freezes `p` for `icon_button_busy` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 88dd8be1a95b8a06:2: Prevents `icon_button_busy` from applying its side effect outside the `k` state.
// ADDED-LINE-RATIONALE 7dce5842bd338356:1: Delegates `icon_button_busy`'s `p / rect_filled / rect` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:19: Bounds the `icon_button_busy` implementation here so `p / rect_filled / rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b7c7e49ef2aebf9c:1: Supports `icon_button_busy` while preserving the nearby constraint: DS: glyphs 16 px in a 32 control, 20 px in a 40.
// ADDED-LINE-RATIONALE ac48eb83f10b483f:1: Freezes `glyph_rect` for `icon_button_busy` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0f20d15a19ba05ea:1: Prevents `icon_button_busy` from applying its side effect outside the `busy` state.
// ADDED-LINE-RATIONALE 34605432097ae83b:1: Delegates `icon_button_busy`'s `paint_loader / ui / Rect` work to `paint_loader` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f8539a7ca2b090d0:3: Makes `paint_loader / ui / Rect` an explicit part of `icon_button_busy` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE c6bcd8c4b9e564dc:1: Freezes `ink` for `icon_button_busy` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 92314e621f8127b4:1: Delegates `icon_button_busy`'s `crate / glyphs / paint` work to `crate::glyphs::paint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:20: Bounds the `icon_button_busy` implementation here so `crate / glyphs / paint` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 28300a5c538c929f:1: Delegates `icon_button_busy`'s `resp / widget_info / egui` work to `resp.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7f6a331f691c4b5e:3: Delegates `icon_button_busy`'s `focus_ring / ui / resp` work to `focus_ring` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 85bc7c748d4b1b3e:1: Freezes `resp` for `icon_button_busy` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d83b1e62d3b8c8af:1: Freezes `PAD` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2b1f986ee3321626:1: Freezes `SEG_H` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 917b6c3ca0f13ab3:1: Freezes `SEG_PAD` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d0058b8ad77cf5ed:1: Freezes `label_galley` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3640a71a2bf0745e:1: Supports `cells` while preserving the nearby constraint: Measure first: the track is drawn under the segments.
// ADDED-LINE-RATIONALE 02bf3d81a3ca3748:1: Delegates `cells`'s `iter` work to `iter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7f9a49b81db0b2a8:1: Delegates `cells`'s `map / _ / cell` work to `map` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 00642cebab36fe80:1: Freezes `label` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8a45ef357e997109:1: Freezes `count` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 815b5543dae3a1c3:1: Makes `label / count / SEG_PAD` an explicit part of `cells` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 81718cf97103a313:1: Makes `label / count / SEG_PAD` an explicit part of `cells` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE ca638a4141902c8b:1: Delegates `cells`'s `collect` work to `collect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a5c5f95f9a1a1a3d:1: Freezes `total` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ec571b25cc11abe3:1: Freezes `let` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d74e4ea25392b93b:1: Freezes `pill` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0693de92dfa14266:1: Delegates `cells`'s `ui / painter / rect_filled` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a5a58de1122ffa8b:1: Freezes `x` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 60a065eed8e3694a:1: Applies `cells` consistently to every `opt / cell / w` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 5109e6876a1609f3:1: Freezes `on` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5b25078d6ad67445:1: Freezes `rect` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6fcb96abc1ccaadf:1: Freezes `x` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f300f3defd9614f5:1: Delegates `cells`'s `Rect / from_min_size / egui` work to `Rect::from_min_size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 057025bb17e7a400:1: Makes `Rect / from_min_size / egui` an explicit part of `cells` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE b0425a68b50b337e:1: Freezes `r` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 90eb9289214ce379:1: Freezes `x` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6a304ff21c05fba5:1: Makes `r` an explicit part of `cells` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 81719ef97103c1a9:1: Bounds the `cells` implementation here so `r` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5b57c7a2482f284f:1: Freezes `resp` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9f784c2822070a6b:1: Freezes `hovered` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8852a7bb5121b47e:1: Prevents `cells` from applying its side effect outside the `on` state.
// ADDED-LINE-RATIONALE a1dd8e8817556cdb:1: Delegates `cells`'s `p / add / theme` work to `p.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 10d626b741cf8a8e:1: Delegates `cells`'s `p / rect_filled / rect` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:10: Bounds the `cells` implementation here so `p / rect_filled / rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 880e1ac1fdf175fb:1: Freezes `ink` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b5929bfe29ddea7a:1: Freezes `label` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a601ea15c17f7464:1: Freezes `label_w` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 957659587d76185b:1: Freezes `count_w` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7b4de13f2ab237fa:1: Freezes `text_rect` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ad3caff36a40cacf:1: Delegates `cells`'s `paint_left / ui / text_rect` work to `paint_left` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 90e183a222e4581a:1: Freezes `if` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 427f2042877b9d25:1: Freezes `c` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3309a89c31255a09:1: Delegates `cells`'s `paint_left / ui / text_rect` work to `paint_left` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:11: Bounds the `cells` implementation here so `paint_left / ui / text_rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE dd36d917f66c7625:1: Freezes `name` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 32605db437f7b966:1: Freezes `Some` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d83363490c508188:1: Freezes `None` for `cells` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 81719ef97103c1a9:2: Bounds the `cells` implementation here so `cell / label / to_string` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 33d08da4e65b5226:1: Delegates `cells`'s `resp / widget_info / egui` work to `resp.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f0976ebf9fe7c70:1: Delegates `cells`'s `focus_ring / ui / resp` work to `focus_ring` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fd27df410432af71:1: Prevents `cells` from applying its side effect outside the `resp / on_hover_text / cell` state.
// ADDED-LINE-RATIONALE 5d1b980d204d4e21:1: Makes `value / opt` an explicit part of `cells` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE d2edc042965ddf75:1: Supports `cells` while preserving the nearby constraint: value = *opt;.
// ADDED-LINE-RATIONALE d55f1bb3de3c6f38:1: Supports `switch` while preserving the nearby constraint: with Save. Returns the response, `changed()` when flipped.
// ADDED-LINE-RATIONALE 923c69e8508f5ddf:13: Freezes `t` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1e65a6f08e03d386:2: Freezes `enabled` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 78185c6ac6dd84b5:1: Freezes `galley` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE be72c72c74343467:1: Keeps `label / to_owned` in `switch`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 226241ba9b4767bf:1: Keeps `egui / FontId / new` in `switch`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE c61cac93bac4547b:1: Keeps `t / ink` in `switch`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE a167ac8571c2b9de:1: Keeps `ui / available_width` in `switch`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 5d3d8390167c1b3d:5: Bounds the `switch` implementation here so `ui / available_width` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 431bdcc17395945b:1: Freezes `let` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 759ffc46d8a28aaa:1: Freezes `height` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 62f2256a7e16f0b6:1: Freezes `let` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a13d42ae84d8b518:1: Prevents `switch` from applying its side effect outside the `resp / clicked / enabled` state.
// ADDED-LINE-RATIONALE 57efc5809915ef08:1: Makes `on / on` an explicit part of `switch` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 0571ec53e5746e6b:1: Supports `switch` while preserving the nearby constraint: on = !*on;.
// ADDED-LINE-RATIONALE fe6402ece919e238:22: Bounds the `switch` implementation here so `resp / mark_changed` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 17988a8ef87ab0c4:1: Freezes `k` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 78200efeb4c7b5bb:1: Freezes `travel` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5ef1fedfabec68e9:1: Freezes `track` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9e1d6f09715f0a16:1: Freezes `fade` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a9c30473f4a1a7c8:4: Freezes `p` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d74e4ea25392b93b:2: Freezes `pill` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dbd8d66b20ab14e6:1: Delegates `switch`'s `p / rect_filled / track` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 585c97b938f030a0:1: Freezes `inset` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d3f5e44de6d371b7:1: Freezes `x` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9c43a51e1eb1b1c2:1: Freezes `thumb_rect` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 647ee0db94e7564c:1: Delegates `switch`'s `p / add / theme` work to `p.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 322ba29cb91bd5ed:1: Delegates `switch`'s `p / rect_filled / thumb_rect` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3527654691eae269:1: Freezes `text_pos` for `switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 040200f5872c89f2:1: Delegates `switch`'s `p / galley / text_pos` work to `p.galley` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c06283f3526b85cf:1: Delegates `switch`'s `resp / widget_info / egui` work to `resp.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7fc17154337b2c40:1: Delegates `switch`'s `focus_ring / ui / resp` work to `focus_ring` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 89b9d630ee0244be:3: Prevents `switch` from applying its side effect outside the `enabled / resp / on_hover_cursor` state.
// ADDED-LINE-RATIONALE e3b3134b8888d668:40: Bounds the `switch` implementation here so `enabled / resp / on_hover_cursor` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d74e4ea25392b93b:3: Freezes `pill` for `progress` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 35e4e39c714cbead:1: Freezes `track` for `progress` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e85dc667d031f22c:1: Delegates `progress`'s `p / rect_filled / track` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 67c1004ef73605f2:1: Freezes `w` for `progress` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a3f47d599d70f93a:1: Prevents `progress` from applying its side effect outside the `w` state.
// ADDED-LINE-RATIONALE 40b435dd82e5a8c8:1: Delegates `progress`'s `p / rect_filled / Rect` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6a3050f21c05fd58:2: Bounds the `progress` implementation here so `p / rect_filled / Rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:13: Bounds the `progress` implementation here so `theme / motion / duration` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 082e11c00a262d2f:1: Freezes `None` for `progress` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8f75821ec8aa50f2:1: Delegates `progress`'s `p / rect_filled / track` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c4a282c58c529462:1: Freezes `s` for `progress` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1c23aee6b2af9c52:1: Supports `progress` while preserving the nearby constraint: ease-in-out.
// ADDED-LINE-RATIONALE 66a2a31d660a8f5e:1: Freezes `left` for `progress` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4cc46af11f7e7ecf:1: Freezes `seg` for `progress` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2fccbfbfcc4d0003:1: Prevents `progress` from applying its side effect outside the `seg / width` state.
// ADDED-LINE-RATIONALE 5177d8a73a3a3bd3:1: Delegates `progress`'s `p / rect_filled / seg` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6a3050f21c05fd58:3: Bounds the `progress` implementation here so `p / rect_filled / seg` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE af699574124bdd04:1: Delegates `progress`'s `p / text` work to `p.text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 062fcc1af1996b26:1: Keeps `egui / pos2 / rect` in `progress`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 7594c29aee3fe3a7:1: Keeps `egui / Align2 / LEFT_CENTER` in `progress`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 03aabac1f585ba4b:1: Keeps `caption` in `progress`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 8da2e972c80af650:1: Keeps `egui / FontId / new` in `progress`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 893581922ffb7b86:1: Keeps `t / muted` in `progress`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 5d3d8390167c1b3d:6: Bounds the `progress` implementation here so `t / muted` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2dde0151d35607db:1: Freezes `resp` for `more_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 348cf0d7e5c0b253:1: Delegates `more_menu`'s `egui / Popup / menu` work to `egui::Popup::menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1bc40f1ae245d45c:1: Supports `more_menu` while preserving the nearby constraint: DS menu items are about 36 px.
// ADDED-LINE-RATIONALE 36ee41e29e2d630c:1: Supports `heading` while preserving the nearby constraint: What a sidebar row shows at its right end.
// ADDED-LINE-RATIONALE 46b76ae7f74f01aa:1: Supports `NavMeta` while preserving the nearby constraint: [derive(Clone, Copy, Debug, PartialEq)].
// ADDED-LINE-RATIONALE 9e16e5ca323ddf05:1: Keeps `NavMeta / a` in `NavMeta`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 65d12b798b8c48e9:1: Supports `NavMeta` while preserving the nearby constraint: A count in this colour (DS nav meta: 12 px, right aligned); nothing when zero.
// ADDED-LINE-RATIONALE 738306278452d9a7:1: Supports `NavMeta` while preserving the nearby constraint: A 7 px status dot (DS TaskStatus).
// ADDED-LINE-RATIONALE 694d6d0f51922306:1: Supports `NavMeta` while preserving the nearby constraint: folded rail keeps only the dot.
// ADDED-LINE-RATIONALE e3b3134b8888d668:46: Bounds the `NavMeta` implementation here so `State / a / str` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 88822cfb3ac0279e:1: Supports `NavMeta` while preserving the nearby constraint: Height of a sidebar row (Redrob Cowork: 7 px of padding round a 24 px line).
// ADDED-LINE-RATIONALE db85448475674ff5:1: Supports `nav_row` while preserving the nearby constraint: shows, the label becomes its tooltip and a meta becomes a dot on the icon.
// ADDED-LINE-RATIONALE 923c69e8508f5ddf:16: Freezes `t` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 04816529f142083d:1: Freezes `let` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ee839d09c39387bb:1: Freezes `hovered` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a9c30473f4a1a7c8:6: Freezes `p` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 839f78f7f7721103:1: Freezes `radius` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 40ebb960dbdc4586:1: Prevents `nav_row` from applying its side effect outside the `current` state.
// ADDED-LINE-RATIONALE 9f7da97dd95ea97c:1: Delegates `nav_row`'s `p / rect_filled / rect` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 32af4cc6c31e6146:1: Makes `hovered` an explicit part of `nav_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE d00a2121abc52eba:1: Delegates `nav_row`'s `p / rect_filled / rect` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:25: Bounds the `nav_row` implementation here so `p / rect_filled / rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ef0f8f3bcd1b63fe:1: Freezes `icon_ink` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 568434ec7e6f4a6e:1: Makes `t / accent_ink` an explicit part of `nav_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 32af4cc6c31e6146:2: Makes `hovered` an explicit part of `nav_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 000bbae544b22e35:1: Makes `t / ink` an explicit part of `nav_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE f8539a7ca2b090d0:5: Makes `t / ink` an explicit part of `nav_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 5cd96707f3986947:1: Makes `t / faint` an explicit part of `nav_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 5dd2fb9016fb0b19:9: Bounds the `nav_row` implementation here so `t / faint` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a0638535c8415e17:1: Freezes `ink` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 18cea618672831b1:1: Freezes `pad` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 99ffcd64a4db0bd9:1: Freezes `icon_rect` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 59e7015c2984b6c9:1: Delegates `nav_row`'s `Rect / from_center_size / rect` work to `Rect::from_center_size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f8539a7ca2b090d0:6: Makes `Rect / from_center_size / rect` an explicit part of `nav_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE cd8a22b72c28167d:1: Delegates `nav_row`'s `Rect / from_min_size / egui` work to `Rect::from_min_size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5dd2fb9016fb0b19:10: Bounds the `nav_row` implementation here so `Rect / from_min_size / egui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0c242493ee702188:1: Delegates `nav_row`'s `crate / glyphs / paint` work to `crate::glyphs::paint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6de27b78d66cb13b:1: Prevents `nav_row` from applying its side effect outside the `rail` state.
// ADDED-LINE-RATIONALE c190d031989524c5:1: Freezes `dot` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 71a67fe187205e6f:1: Freezes `NavMeta` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b0195948110740d6:1: Freezes `NavMeta` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE acc4c71eeeccff8b:1: Freezes `_` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 81719ef97103c1a9:3: Bounds the `nav_row` implementation here so `_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f7119365084fdf6a:1: Freezes `if` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 28a7106d0996376e:1: Delegates `nav_row`'s `p / circle_filled / icon_rect` work to `p.circle_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:16: Bounds the `nav_row` implementation here so `p / circle_filled / icon_rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f8539a7ca2b090d0:7: Makes `right / rect / right` an explicit part of `nav_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 8a21250973dd69c1:1: Freezes `right` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 828f5238b2dd67f2:1: Prevents `nav_row` from applying its side effect outside the `meta` state.
// ADDED-LINE-RATIONALE da0ebcab350e27df:1: Freezes `NavMeta` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 59af4bdfb99c3c34:1: Freezes `g` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f50916fde5c250d6:1: Freezes `w` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 410a6b6217e6d690:1: Delegates `nav_row`'s `paint_left / ui / Rect` work to `paint_left` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 244e3dfde8b8f7b9:1: Freezes `right` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6a3050f21c05fd58:4: Bounds the `nav_row` implementation here so `right / w / pad` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 175073bb2b8aaae4:1: Freezes `NavMeta` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2bc24ce1c34577c8:1: Delegates `nav_row`'s `p / circle_filled / egui` work to `p.circle_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4c13bdf0ce7668a3:1: Freezes `right` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6a3050f21c05fd58:5: Bounds the `nav_row` implementation here so `right / pad` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8fd042e276c8560a:1: Freezes `NavMeta` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2bc24ce1c34577c8:2: Supports `nav_row` while preserving the nearby constraint: The dot at the row's edge, where the other rows keep theirs; the word before it.
// ADDED-LINE-RATIONALE b9fcf1d0b1f37e92:1: Freezes `right` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 26371ac49db6f5ae:1: Freezes `g` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f50916fde5c250d6:2: Freezes `w` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b1d40ddcebcbfe5d:1: Delegates `nav_row`'s `paint_left / ui / Rect` work to `paint_left` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 244e3dfde8b8f7b9:2: Freezes `right` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6a3050f21c05fd58:6: Bounds the `nav_row` implementation here so `right / w / pad` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7d0c9c9a9c5b587f:1: Freezes `_` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:17: Bounds the `nav_row` implementation here so `_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4cd7dace018c5ea4:1: Freezes `text_left` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0838b9e8f6eac244:1: Freezes `family` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c52ba1073cb099d3:1: Freezes `format` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 50a05d0673029551:1: Freezes `job` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2d2c1b8ffb1dd71e:1: Freezes `job` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3c4da0975fbaedc7:1: Freezes `g` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a352997baef2b4a7:1: Delegates `nav_row`'s `paint_left / ui / Rect` work to `paint_left` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:26: Bounds the `nav_row` implementation here so `paint_left / ui / Rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b5f52cad47fce0ed:1: Supports `nav_row` while preserving the nearby constraint: A state word is part of what the row says, so screen readers hear it too ("AI engine: Standby").
// ADDED-LINE-RATIONALE 33379ab5e6d74d86:1: Freezes `NavMeta` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a7c4b6973da641c5:1: Freezes `_` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5dd2fb9016fb0b19:11: Bounds the `nav_row` implementation here so `_ / label / to_owned` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 62f08bb2299ca290:1: Delegates `nav_row`'s `resp / widget_info / egui` work to `resp.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2c93a0b2e19f447b:1: Delegates `nav_row`'s `focus_ring / ui / resp` work to `focus_ring` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 67d084eef23c76bb:1: Freezes `resp` for `nav_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 46424633c50d8e21:1: Delegates `nav_row`'s `tip / resp / label` work to `tip` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1c9ee9554ba0abd9:1: Delegates `nav_row`'s `hint / is_empty` work to `hint.is_empty` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2969e32713df4e8b:1: Makes `resp` an explicit part of `nav_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE f8539a7ca2b090d0:8: Makes `resp` an explicit part of `nav_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE e60e2ebac941b66c:1: Delegates `nav_row`'s `tip / resp / hint` work to `tip` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5dd2fb9016fb0b19:12: Bounds the `nav_row` implementation here so `tip / resp / hint` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7c1524a36cea364f:1: Delegates `nav_row`'s `resp / on_hover_cursor / egui` work to `resp.on_hover_cursor` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:47: Bounds the `nav_row` implementation here so `resp / on_hover_cursor / egui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5d3c5fd5bda81ff8:1: Supports `account_row` while preserving the nearby constraint: ink-faint under it. Screen readers read `name`.
// ADDED-LINE-RATIONALE 923c69e8508f5ddf:17: Freezes `t` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4fa5980f73d914f0:1: Freezes `let` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ee839d09c39387bb:2: Freezes `hovered` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a9c30473f4a1a7c8:7: Freezes `p` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 839f78f7f7721103:2: Freezes `radius` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 40ebb960dbdc4586:2: Prevents `account_row` from applying its side effect outside the `current` state.
// ADDED-LINE-RATIONALE 9f7da97dd95ea97c:2: Delegates `account_row`'s `p / rect_filled / rect` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 32af4cc6c31e6146:3: Makes `hovered` an explicit part of `account_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE d00a2121abc52eba:2: Delegates `account_row`'s `p / rect_filled / rect` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:27: Bounds the `account_row` implementation here so `p / rect_filled / rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4e4e8604adc54554:1: Freezes `size` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 06cfcb9e81c276d9:1: Freezes `centre` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5be10160fd33d362:1: Delegates `account_row`'s `p / circle / centre` work to `p.circle` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b3d7c8535ad46413:1: Prevents `account_row` from applying its side effect outside the `initial` state.
// ADDED-LINE-RATIONALE e8b1c7c43038414b:1: Freezes `Some` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7f8c1580a3ef42ef:1: Freezes `s` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c4928787a558fc6c:1: Delegates `account_row`'s `p / text / centre` work to `p.text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:18: Bounds the `account_row` implementation here so `p / text / centre` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE df903c2bbc179cdb:1: Freezes `None` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fe6402ece919e238:28: Bounds the `account_row` implementation here so `crate / glyphs / paint` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 52254fe5f0d5c69c:1: Prevents `account_row` from applying its side effect outside the `rail` state.
// ADDED-LINE-RATIONALE ef12600cffc4b649:1: Freezes `left` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a6fe1b044b37acec:1: Freezes `width` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 62ebff6ac490125c:1: Freezes `line` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b9910738e4d33c1e:1: Freezes `format` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c4c1595839c1eaca:1: Freezes `job` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 59ef8add8a27ec56:1: Freezes `job` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 37c4f357d4edd14b:1: Delegates `account_row`'s `ui / painter / layout_job` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 81719ef97103c1a9:4: Bounds the `account_row` implementation here so `ui / painter / layout_job` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 940cf46699532a80:1: Freezes `top` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8a29408e88bc25c4:1: Freezes `bottom` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fe8ad8cd9ae183e1:1: Freezes `total` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0e7b5b9ab9a9bcba:1: Freezes `y` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9f559dcccd00e808:1: Freezes `top_h` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ac36e0d80c4336fd:1: Delegates `account_row`'s `ui / painter / galley` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 450b12575d0f4cea:1: Delegates `account_row`'s `ui / painter / galley` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:29: Bounds the `account_row` implementation here so `ui / painter / galley` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 53e76276443d031a:1: Delegates `account_row`'s `resp / widget_info / egui` work to `resp.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2c93a0b2e19f447b:2: Delegates `account_row`'s `focus_ring / ui / resp` work to `focus_ring` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 23294afc6c73aa4c:1: Freezes `resp` for `account_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7c1524a36cea364f:2: Delegates `account_row`'s `resp / on_hover_cursor / egui` work to `resp.on_hover_cursor` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:48: Bounds the `account_row` implementation here so `resp / on_hover_cursor / egui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b0bbc16e0e40ff09:1: Supports `menu_item` while preserving the nearby constraint: version line) is ink-faint and can't be clicked.
// ADDED-LINE-RATIONALE 923c69e8508f5ddf:18: Freezes `t` for `menu_item` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 27062ca7d13eb0f0:1: Freezes `let` for `menu_item` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c6f9193991832682:1: Freezes `hovered` for `menu_item` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a9c30473f4a1a7c8:8: Freezes `p` for `menu_item` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3db756bb0906a1bc:1: Prevents `menu_item` from applying its side effect outside the `hovered` state.
// ADDED-LINE-RATIONALE 4fdb6506609731df:1: Delegates `menu_item`'s `p / rect_filled / rect` work to `p.rect_filled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:30: Bounds the `menu_item` implementation here so `p / rect_filled / rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 18cea618672831b1:2: Freezes `pad` for `menu_item` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4bf47fd19c65f99c:1: Freezes `icon_rect` for `menu_item` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d6e2101846aa0c3a:1: Freezes `if` for `menu_item` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a748089f79c0d050:1: Delegates `menu_item`'s `crate / glyphs / paint` work to `crate::glyphs::paint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe6402ece919e238:31: Bounds the `menu_item` implementation here so `crate / glyphs / paint` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE cbc49c8261180c79:1: Freezes `ink` for `menu_item` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 460590d4beea7332:1: Freezes `galley` for `menu_item` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 02674e1ab47a7384:1: Delegates `menu_item`'s `paint_left / ui / Rect` work to `paint_left` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 28300a5c538c929f:2: Delegates `menu_item`'s `resp / widget_info / egui` work to `resp.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 88c015d9a18c5e97:1: Prevents `menu_item` from applying its side effect outside the `enabled` state.
// ADDED-LINE-RATIONALE 3441667cca943098:1: Stops `menu_item` at the `resp` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE fe6402ece919e238:32: Bounds the `menu_item` implementation here so `resp` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2c93a0b2e19f447b:3: Delegates `menu_item`'s `focus_ring / ui / resp` work to `focus_ring` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7c1524a36cea364f:3: Delegates `menu_item`'s `resp / on_hover_cursor / egui` work to `resp.on_hover_cursor` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:49: Bounds the `menu_item` implementation here so `resp / on_hover_cursor / egui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0d61de15b0e39883:1: Supports `theme_switch` while preserving the nearby constraint: Stacked when `vertical` (the folded sidebar). Returns whether the choice changed.
// ADDED-LINE-RATIONALE 11bac0bbd50a6bc2:1: Gives `theme_switch` direct access to `theme / ThemeChoice` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 923c69e8508f5ddf:19: Freezes `t` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 59a142b846ef3350:1: Supports `theme_switch` while preserving the nearby constraint: targets meet, never overlap) and 36 across it (Cowork's 36 px hit target).
// ADDED-LINE-RATIONALE 2c0a9069dd07924a:1: Freezes `PAD` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 42833e281ebc7315:1: Freezes `GAP` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1b0306921f1341c8:1: Freezes `HIT_ALONG` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6ef8a848d7e95fc7:1: Freezes `HIT_ACROSS` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 789a83dc3ce622ee:1: Freezes `options` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 50867894c72ab59b:1: Freezes `long` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5f4796468c0fb2d5:1: Freezes `short` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d9c30ebbd6a5a24c:1: Freezes `let` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d74e4ea25392b93b:4: Freezes `pill` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 90406b3bd499eb2d:1: Delegates `theme_switch`'s `ui / painter / rect` work to `ui.painter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b9dd919bbd27d815:2: Freezes `changed` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a60fd2e57442fda0:1: Applies `theme_switch` consistently to every `i / choice / icon` item rather than handling only the first match.
// ADDED-LINE-RATIONALE b2442d911ef7f8e0:1: Freezes `off` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 715896daafb37e76:1: Freezes `min` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6070cd13ecab1ff1:1: Freezes `rect` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d6ee34b97a8c2da2:1: Freezes `label` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b8ca7438a2c6f410:1: Freezes `hit` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6c33d2cc81707970:1: Freezes `resp` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f3f90ba99a65134f:1: Freezes `on` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1d2a72f8ed29dd38:2: Freezes `p` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8852a7bb5121b47e:2: Prevents `theme_switch` from applying its side effect outside the `on` state.
// ADDED-LINE-RATIONALE 3a038b9d04bd51b3:1: Delegates `theme_switch`'s `p / rect / rect` work to `p.rect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:19: Bounds the `theme_switch` implementation here so `p / rect / rect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE db832d9e8309d09d:1: Freezes `ink` for `theme_switch` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3fedaa5cf6ef4f78:1: Delegates `theme_switch`'s `crate / glyphs / paint` work to `crate::glyphs::paint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e58c0c2063a41902:1: Delegates `theme_switch`'s `resp / widget_info / egui` work to `resp.widget_info` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f0976ebf9fe7c70:2: Delegates `theme_switch`'s `focus_ring / ui / resp` work to `focus_ring` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2d6fb661a00f4a55:1: Prevents `theme_switch` from applying its side effect outside the `tip / resp / label` state.
// ADDED-LINE-RATIONALE a21ae108d6fabe11:1: Makes `value / choice` an explicit part of `theme_switch` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE d2edc042965ddf75:2: Supports `theme_switch` while preserving the nearby constraint: value = choice;.
// ADDED-LINE-RATIONALE a6e45c0e1beb6908:20: Bounds the `theme_switch` implementation here so `changed` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE fe6402ece919e238:33: Bounds the `theme_switch` implementation here so `changed` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 44e3cac07509068b:2: Makes `changed` an explicit part of `theme_switch` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE e3b3134b8888d668:50: Bounds the `theme_switch` implementation here so `changed` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6b5ba9e852a4226b:1: Supports `tight_row` while preserving the nearby constraint: and below a 12-14 px line; rows of controls keep using `horizontal`.
// ADDED-LINE-RATIONALE a378da18bf178a38:2: Delegates `tight_row`'s `ui / scope / ui` work to `ui.scope` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0149650e12612ddb:1: Freezes `ui` for `tight_row` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 516534bba60f3f25:1: Delegates `tight_row`'s `ui / horizontal / add` work to `ui.horizontal` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5dd3099016fb22e3:2: Makes `ui / horizontal / add` an explicit part of `tight_row` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE e3b3134b8888d668:51: Bounds the `tight_row` implementation here so `ui / horizontal / add` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 27186e774084c2c2:1: Supports `tight_wrapped` while preserving the nearby constraint: [`tight_row`] that wraps onto further lines when it runs out of width.
// ADDED-LINE-RATIONALE a378da18bf178a38:3: Delegates `tight_wrapped`'s `ui / scope / ui` work to `ui.scope` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0149650e12612ddb:2: Freezes `ui` for `tight_wrapped` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f5a0861f3a509157:1: Delegates `tight_wrapped`'s `ui / horizontal_wrapped / add` work to `ui.horizontal_wrapped` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5dd3099016fb22e3:3: Makes `ui / horizontal_wrapped / add` an explicit part of `tight_wrapped` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE e3b3134b8888d668:52: Bounds the `tight_wrapped` implementation here so `ui / horizontal_wrapped / add` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4e273468fc824cde:1: Supports `meta_sep` while preserving the nearby constraint: screen readers skip it.
// ADDED-LINE-RATIONALE 923c69e8508f5ddf:20: Freezes `t` for `meta_sep` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE faca7da38b1dfd28:1: Freezes `galley` for `meta_sep` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 484af86653557935:1: Freezes `let` for `meta_sep` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fa202f1667c899eb:1: Delegates `meta_sep`'s `paint_left / ui / rect` work to `paint_left` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e3b3134b8888d668:53: Bounds the `meta_sep` implementation here so `paint_left / ui / rect` cannot leak into the adjacent declaration or control path.
//! Small building blocks in the Redrob design system's style: labels, square buttons, badges and
//! hairline cell groups.
//!
//! Upper-case text is drawn upper case but announced to screen readers as written.

use std::sync::Arc;

use egui::{Color32, CornerRadius, Galley, Rect, Response, RichText, Sense, Stroke, Ui, vec2};

use crate::glyphs::Icon;
use crate::model::Tone;
use crate::theme::{self, Tokens, tint};

pub fn tone_color(t: &Tokens, tone: Tone) -> Color32 {
    match tone {
        Tone::Good => t.good,
        Tone::Warn => t.warn,
        Tone::Info => t.accent_ink,
        Tone::Neutral => t.muted,
        Tone::Faint => t.faint,
    }
}

/// `text` laid out as written in `family`, `spacing` points between letters. The design system
/// never sets labels in capitals, so the text is not upper-cased.
pub fn caps(ui: &Ui, text: &str, size: f32, family: egui::FontFamily, spacing: f32, color: Color32) -> Arc<Galley> {
    let format = egui::TextFormat { font_id: egui::FontId::new(size, family), color, extra_letter_spacing: spacing, ..Default::default() };
    ui.painter().layout_job(egui::text::LayoutJob::single_section(text.to_owned(), format))
}

/// A label: the design system's meta line (12 px Pretendard 500, sentence case).
pub fn hud_galley(ui: &Ui, text: &str, color: Color32) -> Arc<Galley> {
    caps(ui, text, theme::HUD_SIZE, theme::mono(), theme::HUD_SPACING, color)
}

/// Paints `galley` at the left of `rect`, centred vertically.
fn paint_left(ui: &Ui, rect: Rect, galley: Arc<Galley>, color: Color32) {
    let pos = egui::pos2(rect.left(), rect.center().y - galley.size().y / 2.0);
    ui.painter().galley_with_override_text_color(pos, galley, color);
}

/// Paints `galley` centred in `rect`.
fn paint_centred(ui: &Ui, rect: Rect, galley: Arc<Galley>, color: Color32) {
    ui.painter().galley_with_override_text_color(rect.center() - galley.size() / 2.0, galley, color);
}

/// A mono label.
pub fn hud(ui: &mut Ui, text: &str, color: Color32) -> Response {
    let galley = hud_galley(ui, text, color);
    let (rect, resp) = ui.allocate_exact_size(galley.size(), Sense::hover());
    paint_left(ui, rect, galley, color);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    resp
}

/// A mono label cut to one line with "…" where it is wider than `max_width`; the full text shows
/// on hover and is what screen readers read.
pub fn hud_elided(ui: &mut Ui, text: &str, color: Color32, max_width: f32) -> Response {
    let format = egui::TextFormat {
        font_id: egui::FontId::new(theme::HUD_SIZE, theme::mono()),
        color,
        extra_letter_spacing: theme::HUD_SPACING,
        ..Default::default()
    };
    let mut job = egui::text::LayoutJob::single_section(text.to_owned(), format);
    job.wrap = egui::text::TextWrapping::truncate_at_width(max_width.min(ui.available_width()).max(0.0));
    let galley = ui.painter().layout_job(job);
    let elided = galley.elided;
    let (rect, resp) = ui.allocate_exact_size(galley.size(), Sense::hover());
    paint_left(ui, rect, galley, color);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    if elided { resp.on_hover_text(text) } else { resp }
}

/// A shelf heading on the Apps page, following Redrob Cowork's compact Trending AI treatment:
/// 16 px Pretendard 600, tracked in -0.01em, ink-primary, no tick. It reuses the bundled
/// SemiBold face rather than carrying a separate multi-megabyte 700 face for three labels.
pub fn shelf_heading(ui: &mut Ui, title: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let size = 16.0;
    ui.add(
        egui::Label::new(RichText::new(title).size(size).family(theme::semibold()).color(t.ink).extra_letter_spacing(-0.01 * size))
            .selectable(false)
            .truncate(),
    )
}

/// The DS Loader inline (three bars raked at 40 degrees passing from border-strong to `active`
/// in turn), `size` tall: the launcher's only "working" indicator; no spinning ring. With motion
/// off (egui `animation_time` 0, the shell's reduced-motion setting) the bars stand still,
/// halfway lit, and ask for no repaints.
pub fn loader_inline(ui: &mut Ui, size: f32, active: Color32) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    paint_loader(ui, rect, active, t.line_strong);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::ProgressIndicator, true, tl!("Working…")));
    resp
}

/// Paints the DS Loader into `rect`, animated unless motion is off.
fn paint_loader(ui: &Ui, rect: Rect, active: Color32, rest: Color32) {
    if theme::motion::duration(ui.ctx(), 1.0) == 0.0 {
        let still = theme::mix(rest, active, 0.5);
        crate::glyphs::loader(ui.painter(), rect, still, still, 0.0);
    } else {
        crate::glyphs::loader(ui.painter(), rect, rest, active, ui.input(|i| i.time));
        ui.ctx().request_repaint();
    }
}

/// A DS Card on a page: surface-raised, a border-subtle hairline, radius 12, padding 20.
pub fn card_frame(t: &Tokens) -> egui::Frame {
    egui::Frame::new()
        .fill(t.raised())
        .stroke(t.hairline())
        .corner_radius(CornerRadius::same(theme::radius::LG))
        .inner_margin(egui::Margin::same(theme::space::S5 as i8))
}

/// A full-width hairline between rows inside a card, with `gap` above and below.
pub fn row_rule(ui: &mut Ui, gap: f32) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(gap);
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(r.x_range(), r.center().y, t.hairline());
    ui.add_space(gap);
}

/// The DS Alert, brand tone: surface-brand-subtle with a blue-2 edge, radius 8, padding 16 / 12.
/// For things that stay until they are dealt with (an update, an announcement).
pub fn alert_frame(t: &Tokens) -> egui::Frame {
    egui::Frame::new()
        .fill(t.brand_subtle())
        .stroke(Stroke::new(1.0, t.alert_edge()))
        .corner_radius(CornerRadius::same(theme::radius::MD))
        .inner_margin(egui::Margin::symmetric(16, 12))
}

/// [`alert_frame`] for a one-line alert (the launcher's own update): padding 12 / 6, so with a
/// 32 px control the bar is 44 px tall (was about 66).
pub fn alert_frame_dense(t: &Tokens) -> egui::Frame {
    alert_frame(t).inner_margin(egui::Margin::symmetric(12, 6))
}

/// How far a page's scroll bar sits from the window's right edge: clear of the 5 px resize band of
/// a custom title bar, and otherwise hugging the edge (the central panel's right margin).
pub const PAGE_SCROLL_EDGE: f32 = 6.0;
/// The page's own right padding inside its scroll area, so the content still ends 24 px from the
/// window's edge (DS page padding) with the bar over that padding, not over the content.
pub const PAGE_SCROLL_PAD: f32 = theme::space::S6 - PAGE_SCROLL_EDGE;

/// A page's vertical scroll, like Redrob Cowork's (Base UI ScrollArea): the bar floats over the
/// page's right padding at the window's edge, a round 8 px thumb in border-strong with no track,
/// hidden until the pointer is over the page, stronger when hovered or dragged. `add` draws the
/// page with the page's own style (only the bar is restyled).
pub fn page_scroll<R>(ui: &mut Ui, id_salt: &str, add: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    let page_style = ui.style().clone();
    ui.scope(|ui| {
        let style = ui.style_mut();
        let mut scroll = egui::style::ScrollStyle::floating();
        scroll.bar_width = 8.0;
        // The same width hovered or not: the thumb fades in, it doesn't grow.
        scroll.floating_width = 8.0;
        scroll.floating_allocated_width = 0.0;
        scroll.bar_inner_margin = 0.0;
        scroll.handle_min_length = 24.0;
        // The thumb takes the widgets' foreground colours set below; no track behind it.
        scroll.foreground_color = true;
        scroll.dormant_background_opacity = 0.0;
        scroll.active_background_opacity = 0.0;
        scroll.interact_background_opacity = 0.0;
        // Hidden until the pointer is over the page, like Cowork's.
        scroll.dormant_handle_opacity = 0.0;
        scroll.active_handle_opacity = 1.0;
        scroll.interact_handle_opacity = 1.0;
        style.spacing.scroll = scroll;
        // The thumb: border-strong at rest, a step darker hovered and dragged, fully round.
        for (w, ink) in [
            (&mut style.visuals.widgets.inactive, t.line_strong),
            (&mut style.visuals.widgets.hovered, t.faint),
            (&mut style.visuals.widgets.active, t.muted),
        ] {
            w.fg_stroke.color = ink;
            w.corner_radius = CornerRadius::same(theme::radius::PILL);
        }
        egui::ScrollArea::vertical()
            .id_salt(id_salt)
            .auto_shrink(false)
            .content_margin(egui::Margin { right: PAGE_SCROLL_PAD as i8, ..egui::Margin::ZERO })
            .show(ui, |ui| {
                ui.set_style(page_style);
                add(ui)
            })
            .inner
    })
    .inner
}

/// How far in from the window's edges a custom title bar's window resizes ([`window_edges`]).
/// The title bar's own controls leave this band alone.
const RESIZE_EDGE: f32 = 5.0;

/// Custom title bars: dragging the header's empty space moves the window, and double-clicking it
/// maximizes or restores it. Call before drawing the header, so its controls stay on top.
pub fn title_bar_drag(ui: &mut Ui) {
    let r = ui.max_rect();
    let rect = Rect::from_min_max(r.min + vec2(RESIZE_EDGE, RESIZE_EDGE), r.max - vec2(RESIZE_EDGE, 0.0));
    let resp = ui.interact(rect, ui.id().with("title-bar"), Sense::click_and_drag());
    if resp.drag_started_by(egui::PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    if resp.double_clicked() {
        let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
    }
}

/// A window button in a custom title bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WindowButton {
    Minimize,
    Maximize,
    Restore,
    Close,
}

/// Width of a caption button: Windows' own (Redrob Cowork keeps the native 46 px ones).
const CAPTION_W: f32 = 46.0;

/// Minimize, maximize (or restore) and close, `height` tall, for a right-to-left layout: close
/// ends up at the right.
pub fn window_buttons(ui: &mut Ui, height: f32) {
    let ctx = ui.ctx().clone();
    let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    if window_button(ui, WindowButton::Close, height).clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
    let size = if maximized { WindowButton::Restore } else { WindowButton::Maximize };
    if window_button(ui, size, height).clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
    }
    if window_button(ui, WindowButton::Minimize, height).clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }
}

/// Hover and pressed fills of minimize and maximize: the glyph's colour at Electron's alphas
/// (0x1A, 0x33 of 0xFF).
const CAPTION_HOVER_ALPHA: f32 = 26.0 / 255.0;
const CAPTION_PRESSED_ALPHA: f32 = 51.0 / 255.0;
/// Close pressed: the red at Electron's 0x98.
const CAPTION_CLOSE_PRESSED_ALPHA: f32 = 152.0 / 255.0;

/// One caption button, drawn like Redrob Cowork's (Windows caption buttons over its 48 px bar,
/// Electron's `win_caption_button.cc`): a 10 px glyph in ink-faint (Cowork's symbol colour is
/// Gray 5). Hover fades in a 10% fill of the glyph's colour, press shows 20% at once; close
/// hovers Windows red with a white glyph and presses a lighter red.
///
/// Maximized, the buttons reach the top of the screen and close its right edge, as Windows'
/// do, so flinging the pointer into the corner lands on Close. Restored, the top 5 px stay the
/// window's resize band.
///
/// On Windows the desktop host can hand Maximize to the OS ([`crate::caption`]) so hovering it
/// opens Snap Layouts. The OS then gets the pointer instead of egui, and its hover and press come
/// back through that hand-off.
fn window_button(ui: &mut Ui, button: WindowButton, height: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(CAPTION_W, height), Sense::hover());
    let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false) || i.viewport().fullscreen.unwrap_or(false));
    let edge = if maximized { 0.0 } else { RESIZE_EDGE };
    let right = if button == WindowButton::Close { edge } else { 0.0 };
    let hit = Rect::from_min_max(rect.min + vec2(0.0, edge), rect.max - vec2(right, 0.0));
    let resp = ui.interact(hit, ui.id().with(("window-button", button as u8)), Sense::click());
    let mut pressed = resp.is_pointer_button_down_on() && resp.contains_pointer();
    let mut hovered = resp.hovered() || pressed;
    if matches!(button, WindowButton::Maximize | WindowButton::Restore) {
        crate::caption::publish_maximize(ui.ctx(), hit);
        let os = crate::caption::os_state();
        hovered |= os.hovered;
        pressed |= os.pressed;
    }
    let k = ui.ctx().animate_bool_with_time(resp.id.with("hover"), hovered, theme::motion::duration(ui.ctx(), theme::motion::FAST));
    // Electron fills hover and press with the caption foreground (black on a light frame, white
    // on a dark one), not with the symbol colour.
    let base = t.invert_bg;
    let symbol = theme::platform_caption::SYMBOL;
    let close_hover = theme::platform_caption::CLOSE_HOVER;
    let (fill, ink) = match button {
        WindowButton::Close if pressed => (close_hover.gamma_multiply(CAPTION_CLOSE_PRESSED_ALPHA), t.on_brand()),
        WindowButton::Close => (close_hover.gamma_multiply(k), theme::mix(symbol, t.on_brand(), k)),
        _ if pressed => (base.gamma_multiply(CAPTION_PRESSED_ALPHA), symbol),
        _ => (base.gamma_multiply(CAPTION_HOVER_ALPHA * k), symbol),
    };
    let p = ui.painter();
    if k > 0.0 || pressed {
        p.rect_filled(rect, CornerRadius::ZERO, fill);
    }
    let label = match button {
        WindowButton::Minimize => tl!("Minimize"),
        WindowButton::Maximize => tl!("Maximize"),
        WindowButton::Restore => tl!("Restore"),
        WindowButton::Close => tl!("Close"),
    };
    // Electron lays Cowork's caption buttons out under the window's 1 px top frame border and one
    // pixel further right, so restored its glyphs sit 1 point lower and 1 px right of the
    // button's centre (measured against Cowork at 150%: 2 px down, 1 px right).
    let ppp = ui.ctx().pixels_per_point();
    let glyph_rect = rect.translate(vec2(1.0 / ppp, if maximized { 0.0 } else { 1.0 }));
    caption_glyph(p, glyph_rect, button, ink, ppp);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    tip(resp, label)
}

/// Scale on close's stroke so egui's feathered diagonals carry the ink of Chromium's.
const CLOSE_INK_MATCH: f32 = 0.68;

/// Paints a caption glyph exactly as Redrob Cowork's are painted: Chromium's
/// `Windows11IconPainter` (`windows_icon_painter.cc`, via Electron's `win_caption_button.cc`),
/// worked in physical pixels. A `round(10 * scale)` px square centred in the button, a
/// `round(scale)` px stroke kept inside it; minimize is a crisp line through the middle, maximize
/// a rounded square (radius `floor(1.5 * scale)` px), restore a rounded square with a second one
/// behind it (its top-right radius doubled), close two diagonals clipped to the square with a 5%
/// (10% when white) halo.
fn caption_glyph(p: &egui::Painter, rect: Rect, button: WindowButton, color: Color32, ppp: f32) {
    use egui::pos2;
    let (bl, bt) = ((rect.left() * ppp).round(), (rect.top() * ppp).round());
    let (bw, bh) = ((rect.width() * ppp).round(), (rect.height() * ppp).round());
    let size = (10.0 * ppp).round();
    let (x0, y0) = (bl + ((bw - size) / 2.0).floor(), bt + ((bh - size) / 2.0).floor());
    let w = ppp.round().max(1.0);
    let pt = |x: f32, y: f32| pos2(x / ppp, y / ppp);
    let stroke = |px: f32| Stroke::new(px / ppp, color);
    let radius = (1.5 * ppp).floor();
    let half = w / 2.0;
    match button {
        WindowButton::Minimize => {
            // Not anti-aliased: whole pixel rows centred on the square's middle row. A plain mesh,
            // because the tessellator feathers a filled rect's edges.
            let cy = y0 + (size / 2.0).floor();
            let top = cy - (w / 2.0).ceil();
            let mut mesh = egui::Mesh::default();
            mesh.add_colored_rect(Rect::from_min_max(pt(x0, top), pt(x0 + size, top + w)), color);
            p.add(egui::Shape::mesh(mesh));
        }
        WindowButton::Maximize => {
            let r = [x0 + half, y0 + half, x0 + size - half, y0 + size - half];
            p.add(egui::Shape::closed_line(rounded_rect_path(r, [radius; 4]).into_iter().map(|(x, y)| pt(x, y)).collect(), stroke(w)));
        }
        WindowButton::Restore => {
            let sep = 2.0 * ppp;
            // Front square: the stroke-inset box, shrunk at the top and right for the back one.
            let front = [x0 + half, y0 + half + sep, x0 + size - half - sep, y0 + size - half];
            p.add(egui::Shape::closed_line(rounded_rect_path(front, [radius; 4]).into_iter().map(|(x, y)| pt(x, y)).collect(), stroke(w)));
            // Back square: offset up and right, clipped out where the front (grown by
            // sep - half) covers it, so only its top and right edges show.
            let back = [front[0] + sep, front[1] - sep, front[2] + sep, front[3] - sep];
            let g = sep - half;
            let grown = [front[0] - g, front[1] - g, front[2] + g, front[3] + g];
            let path = rounded_rect_path(back, [radius, 2.0 * radius, radius, radius]);
            let outside = |&(x, y): &(f32, f32)| x < grown[0] || x > grown[2] || y < grown[1] || y > grown[3];
            // The visible run is contiguous on the closed path: start it right after a hidden point.
            if let Some(first_hidden) = path.iter().position(|q| !outside(q)) {
                let n = path.len();
                let run: Vec<_> = (1..=n).map(|i| path[(first_hidden + i) % n]).filter(outside).map(|(x, y)| pt(x, y)).collect();
                p.add(egui::Shape::line(run, stroke(w)));
            }
        }
        WindowButton::Close => {
            let halo = w * if color == Color32::WHITE { 0.1 } else { 0.05 };
            // Chromium's anti-aliased diagonals carry less ink than egui's feathered ones at the
            // same width: measured against Cowork at 150%, 0.68 of the stroke matches its weight.
            let width = (w + halo) * CLOSE_INK_MATCH;
            let clip = p.with_clip_rect(Rect::from_min_max(pt(x0, y0), pt(x0 + size, y0 + size)).intersect(p.clip_rect()));
            clip.line_segment([pt(x0, y0), pt(x0 + size, y0 + size)], stroke(width));
            clip.line_segment([pt(x0 + size, y0), pt(x0, y0 + size)], stroke(width));
        }
    }
}

/// Points along a rounded rectangle `[left, top, right, bottom]` with corner radii
/// `[top-left, top-right, bottom-right, bottom-left]`, clockwise from the top-left corner,
/// densely enough (about 0.25 px apart) to be clipped point by point.
fn rounded_rect_path(r: [f32; 4], radii: [f32; 4]) -> Vec<(f32, f32)> {
    use std::f32::consts::{FRAC_PI_2, PI};
    let [l, t, rr, b] = r;
    let [tl, tr, br, bl] = radii;
    let mut out = Vec::new();
    let edge = |from: (f32, f32), to: (f32, f32), out: &mut Vec<(f32, f32)>| {
        let steps = (((to.0 - from.0).abs() + (to.1 - from.1).abs()) / 0.25).ceil().max(1.0) as usize;
        for i in 0..steps {
            let f = i as f32 / steps as f32;
            out.push((from.0 + (to.0 - from.0) * f, from.1 + (to.1 - from.1) * f));
        }
    };
    let arc = |c: (f32, f32), rad: f32, start: f32, out: &mut Vec<(f32, f32)>| {
        let steps = ((rad * FRAC_PI_2) / 0.25).ceil().max(1.0) as usize;
        for i in 0..steps {
            let a = start + FRAC_PI_2 * i as f32 / steps as f32;
            out.push((c.0 + rad * a.cos(), c.1 + rad * a.sin()));
        }
    };
    arc((l + tl, t + tl), tl, PI, &mut out);
    edge((l + tl, t), (rr - tr, t), &mut out);
    arc((rr - tr, t + tr), tr, -FRAC_PI_2, &mut out);
    edge((rr, t + tr), (rr, b - br), &mut out);
    arc((rr - br, b - br), br, 0.0, &mut out);
    edge((rr - br, b), (l + bl, b), &mut out);
    arc((l + bl, b - bl), bl, FRAC_PI_2, &mut out);
    edge((l, b - bl), (l, t + tl), &mut out);
    out
}

/// Custom title bars: a hairline round the window, and resizing it from its edges and corners.
pub fn window_edges(ctx: &egui::Context) {
    use egui::CursorIcon as C;
    use egui::viewport::ResizeDirection as D;
    let (maximized, fullscreen) = ctx.input(|i| (i.viewport().maximized.unwrap_or(false), i.viewport().fullscreen.unwrap_or(false)));
    if maximized || fullscreen {
        return;
    }
    let t = Tokens::get(ctx);
    let r = ctx.content_rect();
    ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("window-border"))).rect_stroke(
        r,
        CornerRadius::ZERO,
        t.hairline(),
        egui::StrokeKind::Inside,
    );
    let Some(p) = ctx.input(|i| i.pointer.hover_pos()).filter(|p| r.contains(*p)) else { return };
    let near = |d: f32| d < RESIZE_EDGE;
    // A corner takes a little of each edge next to it.
    let corner = |d: f32| d < 3.0 * RESIZE_EDGE;
    let (left, right, top, bottom) = (p.x - r.left(), r.right() - p.x, p.y - r.top(), r.bottom() - p.y);
    let hit = if (near(top) && corner(left)) || (near(left) && corner(top)) {
        Some((D::NorthWest, C::ResizeNorthWest))
    } else if (near(top) && corner(right)) || (near(right) && corner(top)) {
        Some((D::NorthEast, C::ResizeNorthEast))
    } else if (near(bottom) && corner(left)) || (near(left) && corner(bottom)) {
        Some((D::SouthWest, C::ResizeSouthWest))
    } else if (near(bottom) && corner(right)) || (near(right) && corner(bottom)) {
        Some((D::SouthEast, C::ResizeSouthEast))
    } else if near(top) {
        Some((D::North, C::ResizeNorth))
    } else if near(bottom) {
        Some((D::South, C::ResizeSouth))
    } else if near(left) {
        Some((D::West, C::ResizeWest))
    } else if near(right) {
        Some((D::East, C::ResizeEast))
    } else {
        None
    };
    if let Some((direction, cursor)) = hit {
        ctx.set_cursor_icon(cursor);
        if ctx.input(|i| i.pointer.primary_pressed()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(direction));
        }
    }
}

/// A mono link with an ↗: muted, ink on hover. Opens `url` in the browser.
pub fn hud_link(ui: &mut Ui, text: &str, url: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let galley = hud_galley(ui, &format!("{text} ↗"), t.muted);
    let (rect, resp) = ui.allocate_exact_size(galley.size(), Sense::click());
    paint_left(ui, rect, galley, if resp.hovered() { t.ink_strong } else { t.muted });
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, true, text));
    let resp = resp.on_hover_text(url).on_hover_cursor(egui::CursorIcon::PointingHand);
    if resp.clicked() {
        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
    }
    resp
}

/// The DS Badge (subtle tone, md): a 24 px pill, 12 px semibold text in `color` on a light wash
/// of it, no extra letter spacing. Never clickable; state is a word, not only a colour.
pub fn badge(ui: &mut Ui, text: &str, color: Color32) -> Response {
    badge_sized(ui, text, color, Size::Md)
}

/// [`badge`] at the DS's sm (20 px, 11 px text) or md (24 px, 12 px text) size.
pub fn badge_sized(ui: &mut Ui, text: &str, color: Color32, size: Size) -> Response {
    let t = Tokens::get(ui.ctx());
    let (h, pad, font) =
        if size == Size::Sm { (20.0, theme::space::S2, theme::text::CAPTION) } else { (24.0, theme::space::S3 - 2.0, theme::text::META) };
    let galley = caps(ui, text, font, theme::semibold(), 0.0, color);
    let (rect, resp) = ui.allocate_exact_size(vec2(galley.size().x + 2.0 * pad, h), Sense::hover());
    ui.painter().rect_filled(rect, CornerRadius::same(theme::radius::PILL), t.subtle(color));
    paint_centred(ui, rect, galley, color);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    resp
}

/// Which of the design system's button variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// `action-primary` blue with white text; `action-primary-hover` on hover. One per view.
    Primary,
    /// The base surface outlined in `border-strong`; sunken on hover.
    Secondary,
    /// `status-danger` with white text; a little darker on hover.
    Danger,
    /// No fill, `ink-brand` text; `surface-brand-subtle` on hover.
    Ghost,
    /// `surface-brand-subtle` with `ink-brand` text, a touch more blue on hover: a quiet action
    /// that still reads as one, for chrome (Cowork's title bar carries no filled button).
    Tinted,
}

/// The design system's control sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    /// 32 tall, 12 px padding, 13 px label.
    Sm,
    /// 40 tall, 16 px padding, 14 px label: the default.
    #[default]
    Md,
    /// 48 tall, 20 px padding, 15 px label.
    Lg,
}

impl Size {
    /// (height, side padding, label size).
    pub fn metrics(self) -> (f32, f32, f32) {
        match self {
            Self::Sm => (theme::control::SM, theme::space::S3, 13.0),
            Self::Md => (theme::control::MD, theme::space::S4, 14.0),
            Self::Lg => (theme::control::LG, theme::space::S5, 15.0),
        }
    }
}

/// DS disabled controls: the whole control at 45%.
const DISABLED_ALPHA: f32 = 0.45;

/// The DS focus ring: 2 px of `focus-ring`, 2 px outside the control, following its radius.
/// Drawn only when focus arrived from the keyboard (CSS `:focus-visible`): a click draws none.
pub fn focus_ring(ui: &Ui, resp: &Response, radius: u8) {
    let id = resp.id.with("focus-visible");
    if resp.gained_focus() {
        let by_pointer = ui.input(|i| i.pointer.any_pressed() || i.pointer.any_down());
        ui.data_mut(|d| d.insert_temp(id, !by_pointer));
    }
    let keyboard = ui.data(|d| d.get_temp::<bool>(id)).unwrap_or(false);
    if resp.has_focus() && keyboard {
        let t = Tokens::get(ui.ctx());
        let r = CornerRadius::same(radius.saturating_add(3));
        ui.painter().rect_stroke(resp.rect.expand(3.0), r, Stroke::new(2.0, t.focus()), egui::StrokeKind::Middle);
    }
}

/// A tooltip in the DS's style: a `redrob-black` bubble with 12 px white text (inverted in dark),
/// radius 6, `shadow-md`, 8 px from its control. egui's own tooltips share the menus' frame, so
/// our controls draw theirs with this.
pub fn tip(resp: Response, text: &str) -> Response {
    let t = Tokens::get(&resp.ctx);
    let (fill, ink) = t.tooltip();
    let mut tooltip = egui::Tooltip::for_enabled(&resp).gap(theme::space::S2);
    tooltip.popup = tooltip.popup.frame(
        egui::Frame::new()
            .fill(fill)
            .corner_radius(CornerRadius::same(theme::radius::SM))
            .inner_margin(egui::Margin::symmetric(12, 6))
            .shadow(theme::shadow_md(&t)),
    );
    tooltip.show(|ui| {
        ui.add(egui::Label::new(RichText::new(text).size(theme::text::META).color(ink)).wrap_mode(egui::TextWrapMode::Extend));
    });
    resp
}

/// A DS button, medium size, at least `width` wide.
pub fn button(ui: &mut Ui, text: &str, kind: Kind, width: f32) -> Response {
    button_sized(ui, text, kind, Size::Md, width)
}

/// A DS button: radius 8, a 600-weight label, `size` tall, at least `width` wide.
pub fn button_sized(ui: &mut Ui, text: &str, kind: Kind, size: Size, width: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let (height, pad, label) = size.metrics();
    let galley = caps(ui, text, label, theme::mono_bold(), theme::BUTTON_SPACING, t.ink);
    let (rect, resp) = ui.allocate_exact_size(vec2((galley.size().x + 2.0 * pad).max(width), height), Sense::click());
    let enabled = ui.is_enabled();
    let hovered = (resp.hovered() || resp.is_pointer_button_down_on()) && enabled;
    let (rest, hover, stroke, color) = match kind {
        Kind::Primary => (t.accent, t.primary_hover(), Stroke::NONE, t.on_brand()),
        Kind::Danger => (t.bad, t.bad_hover(), Stroke::NONE, t.on_brand()),
        Kind::Secondary => (t.bg, t.sunken, Stroke::new(1.0, t.line_strong), t.ink),
        Kind::Ghost => (Color32::TRANSPARENT, t.brand_subtle(), Stroke::NONE, t.accent_ink),
        Kind::Tinted => (t.brand_subtle(), theme::mix(t.brand_subtle(), t.accent, 0.14), Stroke::NONE, t.accent_ink),
    };
    // The fill changes over the DS's 140 ms standard state change.
    let k = ui.ctx().animate_bool_with_time(resp.id.with("hover"), hovered, theme::motion::duration(ui.ctx(), theme::motion::FAST));
    let k = theme::motion::Easing::Standard.apply(k);
    let fill = if kind == Kind::Ghost { hover.gamma_multiply(k) } else { theme::mix(rest, hover, k) };
    let fade = |c: Color32| if enabled { c } else { c.gamma_multiply(DISABLED_ALPHA) };
    let radius = CornerRadius::same(theme::radius::MD);
    ui.painter().rect(rect, radius, fade(fill), Stroke::new(stroke.width, fade(stroke.color)), egui::StrokeKind::Inside);
    paint_centred(ui, rect, galley, fade(color));
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, text));
    focus_ring(ui, &resp, theme::radius::MD);
    if enabled { resp.on_hover_cursor(egui::CursorIcon::PointingHand) } else { resp }
}

/// The width [`button_sized`] gives `text` at `size` (with no minimum width).
pub fn button_width(ui: &Ui, text: &str, size: Size) -> f32 {
    let (_, pad, label) = size.metrics();
    caps(ui, text, label, theme::mono_bold(), theme::BUTTON_SPACING, Tokens::get(ui.ctx()).ink).size().x + 2.0 * pad
}

/// The DS search input (sm): 32 tall, radius 8, a 1 px border-subtle on the base ground that
/// strengthens on hover and while typing, the search glyph (16 px, ink-muted) at the left and the
/// hint in ink-muted 13 px. One per window: its text field keeps the id `id_salt`, so focus
/// survives a change of page. Returns the text field's response.
pub fn search_field(ui: &mut Ui, text: &mut String, hint: &str, width: f32, id_salt: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    const GLYPH: f32 = 16.0;
    const PAD: f32 = 10.0;
    let (rect, bg) = ui.allocate_exact_size(vec2(width, theme::control::SM), Sense::click());
    let id = egui::Id::new(("search-field", id_salt));
    let strong = ui.memory(|m| m.has_focus(id)) || ui.rect_contains_pointer(rect);
    // Under the text field: drawn first.
    let p = ui.painter();
    p.rect(
        rect,
        CornerRadius::same(theme::radius::MD),
        t.bg,
        Stroke::new(1.0, if strong { t.line_strong } else { t.line }),
        egui::StrokeKind::Inside,
    );
    let glyph = Rect::from_center_size(egui::pos2(rect.left() + PAD + GLYPH / 2.0, rect.center().y), vec2(GLYPH, GLYPH));
    crate::glyphs::paint(p, glyph, Icon::Search, t.muted);
    let field = Rect::from_min_max(egui::pos2(glyph.right() + theme::space::S2, rect.top()), egui::pos2(rect.right() - PAD, rect.bottom()));
    // A child, not `ui.put`: putting would move a right-to-left parent's cursor back to the field.
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(field).layout(egui::Layout::left_to_right(egui::Align::Center)));
    let resp = child.add(
        egui::TextEdit::singleline(text)
            .id(id)
            .frame(egui::Frame::NONE)
            .hint_text(RichText::new(hint).color(t.muted))
            .font(egui::FontId::proportional(theme::text::BODY_SM))
            .text_color(t.ink)
            .vertical_align(egui::Align::Center)
            .margin(vec2(0.0, 0.0))
            .desired_width(field.width()),
    );
    // The hint is only a placeholder to screen readers; the field's name is the hint's words.
    ui.ctx().accesskit_node_builder(resp.id, |node| node.set_label(hint));
    // The glyph and padding focus the field too.
    if bg.clicked() {
        ui.memory_mut(|m| m.request_focus(id));
    }
    resp.on_hover_cursor(egui::CursorIcon::Text)
}

/// The Redrob group logo as a ghost button (Redrob Cowork's group mark): `image` exactly as
/// supplied, 28 px in a 32 px target with radius 8; hover tints the button behind it (8% of ink),
/// never the mark. `label` is the tooltip and the name screen readers read.
pub fn logo_button(ui: &mut Ui, image: egui::ImageSource<'static>, label: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(theme::control::SM, theme::control::SM), Sense::click());
    let hovered = resp.hovered() || resp.is_pointer_button_down_on();
    let k = ui.ctx().animate_bool_with_time(resp.id.with("hover"), hovered, theme::motion::duration(ui.ctx(), theme::motion::FAST));
    if k > 0.0 {
        ui.painter().rect_filled(rect, CornerRadius::same(theme::radius::MD), tint(t.ink, 0.08 * k));
    }
    egui::Image::new(image).alt_text(label).paint_at(ui, Rect::from_center_size(rect.center(), vec2(28.0, 28.0)));
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    focus_ring(ui, &resp, theme::radius::MD);
    tip(resp, label).on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A DS IconButton (ghost): a square `size` (32 or 40) with a drawn icon in `ink-secondary`,
/// sunken with `ink-primary` on hover, radius 8. `label` is mandatory: it is the tooltip and the
/// name screen readers read.
pub fn icon_button(ui: &mut Ui, icon: Icon, label: &str, size: f32) -> Response {
    icon_button_busy(ui, icon, label, size, false)
}

/// [`icon_button`] that, while `busy`, shows the DS loader instead of its icon and can't be clicked.
pub fn icon_button_busy(ui: &mut Ui, icon: Icon, label: &str, size: f32, busy: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), if busy { Sense::hover() } else { Sense::click() });
    let enabled = ui.is_enabled() && !busy;
    let hovered = (resp.hovered() || resp.is_pointer_button_down_on()) && enabled;
    let k = ui.ctx().animate_bool_with_time(resp.id.with("hover"), hovered, theme::motion::duration(ui.ctx(), theme::motion::FAST));
    let p = ui.painter();
    if k > 0.0 {
        p.rect_filled(rect, CornerRadius::same(theme::radius::MD), t.sunken.gamma_multiply(theme::motion::Easing::Standard.apply(k)));
    }
    // DS: glyphs 16 px in a 32 control, 20 px in a 40.
    let glyph = if size >= theme::control::MD { 20.0 } else { 16.0 };
    let glyph_rect = Rect::from_center_size(rect.center(), vec2(glyph, glyph));
    if busy {
        paint_loader(ui, Rect::from_center_size(rect.center(), vec2(14.0, 14.0)), t.accent, t.line_strong);
    } else {
        let ink = if hovered { t.ink } else { t.muted };
        crate::glyphs::paint(p, glyph_rect, icon, if ui.is_enabled() { ink } else { ink.gamma_multiply(DISABLED_ALPHA) });
    }
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, label));
    focus_ring(ui, &resp, theme::radius::MD);
    let resp = tip(resp, label);
    if enabled { resp.on_hover_cursor(egui::CursorIcon::PointingHand) } else { resp }
}

/// One option in a [`cells`] group.
pub struct Cell<'a> {
    pub label: &'a str,
    pub tip: &'a str,
    /// A count after the label, and the colour it takes when it isn't zero.
    pub count: Option<(usize, Option<Color32>)>,
}

/// Mutually exclusive options as the DS segmented control: a sunken pill track (3 px inset)
/// holding 30 px pill segments, 13 px semibold; the chosen one raised with `shadow-sm`.
/// Returns whether the choice changed.
pub fn cells<T: PartialEq + Copy>(ui: &mut Ui, value: &mut T, options: &[(T, Cell<'_>)]) -> bool {
    let t = Tokens::get(ui.ctx());
    let mut changed = false;
    const PAD: f32 = 3.0;
    const SEG_H: f32 = 30.0;
    const SEG_PAD: f32 = 14.0;
    // In a right-to-left parent the row fills from the right: keep the options' order.
    let rtl = ui.layout().prefer_right_to_left();
    let ordered: Vec<&(T, Cell<'_>)> = if rtl { options.iter().rev().collect() } else { options.iter().collect() };
    let label_galley = |ui: &Ui, s: &str, c: Color32| caps(ui, s, theme::text::LABEL, theme::semibold(), 0.0, c);
    // Measure first: the track is drawn under the segments.
    let widths: Vec<f32> = ordered
        .iter()
        .map(|(_, cell)| {
            let label = label_galley(ui, cell.label, t.ink).size().x;
            let count = cell.count.map_or(0.0, |(n, _)| hud_galley(ui, &n.to_string(), t.faint).size().x + 6.0);
            label + count + 2.0 * SEG_PAD
        })
        .collect();
    let total = widths.iter().sum::<f32>() + 2.0 * PAD;
    let (track, _) = ui.allocate_exact_size(vec2(total, SEG_H + 2.0 * PAD), Sense::hover());
    let pill = CornerRadius::same(theme::radius::PILL);
    ui.painter().rect_filled(track, pill, t.sunken);
    let mut x = if rtl { track.right() - PAD } else { track.left() + PAD };
    for ((opt, cell), w) in ordered.into_iter().zip(widths) {
        let on = *value == *opt;
        let rect = if rtl {
            x -= w;
            Rect::from_min_size(egui::pos2(x, track.top() + PAD), vec2(w, SEG_H))
        } else {
            let r = Rect::from_min_size(egui::pos2(x, track.top() + PAD), vec2(w, SEG_H));
            x += w;
            r
        };
        let resp = ui.interact(rect, ui.id().with(("segment", cell.label)), Sense::click());
        let hovered = resp.hovered();
        let p = ui.painter();
        if on {
            p.add(theme::shadow_sm(&t).as_shape(rect, pill));
            p.rect_filled(rect, pill, t.raised());
        }
        let ink = if on || hovered { t.ink } else { t.muted };
        let label = label_galley(ui, cell.label, ink);
        let label_w = label.size().x;
        let count_w = cell.count.map_or(0.0, |(n, _)| hud_galley(ui, &n.to_string(), t.faint).size().x + 6.0);
        let text_rect = Rect::from_center_size(rect.center(), vec2(label_w + count_w, rect.height()));
        paint_left(ui, text_rect, label, ink);
        if let Some((n, highlight)) = cell.count {
            let c = highlight.filter(|_| n > 0).unwrap_or(t.faint);
            paint_left(ui, text_rect.with_min_x(text_rect.left() + label_w + 6.0), hud_galley(ui, &n.to_string(), c), c);
        }
        let name = match cell.count {
            Some((n, _)) => format!("{} {n}", cell.label),
            None => cell.label.to_string(),
        };
        resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, on, &name));
        focus_ring(ui, &resp, theme::radius::PILL);
        if resp.on_hover_text(cell.tip).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() && !on {
            *value = *opt;
            changed = true;
        }
    }
    changed
}

/// The DS Switch (md): a 40 x 24 pill, `border-strong` off and `action-primary` on, a white 20 px
/// thumb with `shadow-sm` that travels in 220 ms with the settle overshoot (instant when motion
/// is off), then `label`. It acts at once: for settings that apply immediately, never in a form
/// with Save. Returns the response, `changed()` when flipped.
pub fn switch(ui: &mut Ui, on: &mut bool, label: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let enabled = ui.is_enabled();
    let galley = ui.painter().layout(
        label.to_owned(),
        egui::FontId::new(theme::text::CONTROL, egui::FontFamily::Proportional),
        t.ink,
        ui.available_width() - 52.0,
    );
    let (track_w, track_h, thumb) = (40.0, 24.0, 20.0);
    let height = galley.size().y.max(track_h).max(theme::control::SM);
    let (rect, mut resp) = ui.allocate_exact_size(vec2(track_w + theme::space::S3 + galley.size().x, height), Sense::click());
    if resp.clicked() && enabled {
        *on = !*on;
        resp.mark_changed();
    }
    let k = ui.ctx().animate_bool_with_time(resp.id.with("switch"), *on, theme::motion::duration(ui.ctx(), theme::motion::BASE));
    let travel = theme::motion::Easing::Settle.apply(k);
    let track = Rect::from_min_size(egui::pos2(rect.left(), rect.center().y - track_h / 2.0), vec2(track_w, track_h));
    let fade = |c: Color32| if enabled { c } else { c.gamma_multiply(0.5) };
    let p = ui.painter();
    let pill = CornerRadius::same(theme::radius::PILL);
    p.rect_filled(track, pill, fade(theme::mix(t.line_strong, t.accent, k)));
    let inset = (track_h - thumb) / 2.0;
    let x = track.left() + inset + thumb / 2.0 + travel * (track_w - thumb - 2.0 * inset);
    let thumb_rect = Rect::from_center_size(egui::pos2(x, track.center().y), vec2(thumb, thumb));
    p.add(theme::shadow_sm(&t).as_shape(thumb_rect, pill));
    p.rect_filled(thumb_rect, pill, fade(t.on_brand()));
    let text_pos = egui::pos2(track.right() + theme::space::S3, rect.center().y - galley.size().y / 2.0);
    p.galley(text_pos, galley, fade(t.ink));
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, enabled, *on, label));
    focus_ring(ui, &resp, theme::radius::PILL);
    if enabled { resp.on_hover_cursor(egui::CursorIcon::PointingHand) } else { resp }
}

/// A progress bar (DS Progress): `caption` above an 8 px pill track on the sunken surface, the
/// bar in `color` (blue, warning when reconnecting). Without a fraction the bar is the DS's
/// indeterminate 40% slide (1.4 s, ease-in-out); with motion off, a full bar at 60%.
pub fn progress(ui: &mut Ui, fraction: Option<f32>, caption: &str, color: Color32, width: f32) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(width, theme::CONTROL_H), Sense::hover());
    let p = ui.painter();
    let pill = CornerRadius::same(theme::radius::PILL);
    let track = Rect::from_min_size(egui::pos2(rect.left(), rect.bottom() - 10.0), vec2(width, 8.0));
    p.rect_filled(track, pill, t.sunken);
    match fraction {
        Some(f) => {
            let w = track.width() * f.clamp(0.0, 1.0);
            if w > 0.0 {
                p.rect_filled(Rect::from_min_size(track.min, vec2(w.max(track.height()), track.height())), pill, color);
            }
        }
        None if theme::motion::duration(ui.ctx(), 1.0) == 0.0 => {
            p.rect_filled(track, pill, color.gamma_multiply(0.6));
        }
        None => {
            let s = (ui.input(|i| i.time) / 1.4).fract() as f32;
            // ease-in-out
            let e = if s < 0.5 { 2.0 * s * s } else { 1.0 - (-2.0 * s + 2.0).powi(2) / 2.0 };
            let left = track.left() + track.width() * (-0.4 + 1.4 * e);
            let seg = Rect::from_min_size(egui::pos2(left, track.top()), vec2(track.width() * 0.4, track.height())).intersect(track);
            if seg.width() > 0.0 {
                p.rect_filled(seg, pill, color);
            }
            ui.ctx().request_repaint();
        }
    }
    p.text(
        egui::pos2(rect.left(), rect.top() + 10.0),
        egui::Align2::LEFT_CENTER,
        caption,
        egui::FontId::new(theme::text::META, theme::medium()),
        t.muted,
    );
    let resp = ui.interact(rect, ui.id().with(("progress", caption)), Sense::hover());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::ProgressIndicator, true, caption));
}

pub fn human_bytes(n: u64) -> String {
    const MB: f64 = 1024.0 * 1024.0;
    let n = n as f64;
    if n >= 1024.0 * MB {
        format!("{:.1} GB", n / (1024.0 * MB))
    } else if n >= MB {
        format!("{:.0} MB", n / MB)
    } else {
        format!("{:.0} KB", (n / 1024.0).max(1.0))
    }
}

/// The "more" icon button that opens an app's menu (DS Menu: 8 px radius list, min 200 wide).
pub fn more_menu<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) {
    let resp = icon_button(ui, Icon::More, tl!("More actions"), theme::control::SM);
    egui::Popup::menu(&resp).gap(theme::space::S1).show(|ui| {
        ui.set_min_width(200.0);
        // DS menu items are about 36 px.
        ui.spacing_mut().interact_size.y = 36.0;
        add_contents(ui)
    });
}

/// An app's name in the display face, in ink: product colour is never text (design system,
/// `20-color.md`).
pub fn wordmark(ui: &mut Ui, app: &redrob_launcher_core::CraftApp, size: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    let format = |color| egui::TextFormat {
        font_id: egui::FontId::new(size, theme::display()),
        color,
        extra_letter_spacing: -0.03 * size,
        ..Default::default()
    };
    job.append(app.name, 0.0, format(t.ink_strong));
    ui.add(egui::Label::new(job).selectable(false))
}

/// A heading in the display face, tracked in like the site's (`tracking-[-0.02em]`).
pub fn heading(text: &str, size: f32, color: Color32) -> RichText {
    RichText::new(text).size(size).family(theme::display()).color(color).extra_letter_spacing(-0.02 * size)
}

/// What a sidebar row shows at its right end.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NavMeta<'a> {
    None,
    /// A count in this colour (DS nav meta: 12 px, right aligned); nothing when zero.
    Count(usize, Color32),
    /// A 7 px status dot (DS TaskStatus).
    Dot(Color32),
    /// A status dot in this colour at the row's edge and a short state word before it in
    /// ink-faint (12 px), for a row whose state is worth reading at a glance ("Standby ●"). The
    /// folded rail keeps only the dot.
    State(&'a str, Color32),
}

/// Height of a sidebar row (Redrob Cowork: 7 px of padding round a 24 px line).
pub const NAV_ROW_H: f32 = 38.0;

/// A sidebar row (DS AppShell nav item, Cowork's numbers): 16 px icon in ink-muted, 13 px label
/// in ink-secondary, radius 6; hover takes a step of the sunken surface; the current row is
/// sunken, semibold ink, its icon ink-brand. No accent bar. In the folded `rail` only the icon
/// shows, the label becomes its tooltip and a meta becomes a dot on the icon.
pub fn nav_row(ui: &mut Ui, icon: Icon, label: &str, hint: &str, meta: NavMeta<'_>, current: bool, rail: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), NAV_ROW_H), Sense::click());
    let hovered = resp.hovered();
    let p = ui.painter();
    let radius = CornerRadius::same(theme::radius::SM);
    if current {
        p.rect_filled(rect, radius, t.sunken);
    } else if hovered {
        p.rect_filled(rect, radius, t.hover_row());
    }
    let icon_ink = if current {
        t.accent_ink
    } else if hovered {
        t.ink
    } else {
        t.faint
    };
    let ink = if current || hovered { t.ink } else { t.muted };
    let pad = theme::space::S2;
    let icon_rect = if rail {
        Rect::from_center_size(rect.center(), vec2(16.0, 16.0))
    } else {
        Rect::from_min_size(egui::pos2(rect.left() + pad, rect.center().y - 8.0), vec2(16.0, 16.0))
    };
    crate::glyphs::paint(p, icon_rect, icon, icon_ink);
    if rail {
        let dot = match meta {
            NavMeta::Dot(c) | NavMeta::State(_, c) => Some(c),
            NavMeta::Count(n, c) if n > 0 => Some(c),
            _ => None,
        };
        if let Some(c) = dot {
            p.circle_filled(icon_rect.right_top() + vec2(1.0, 1.0), 3.5, c);
        }
    } else {
        let mut right = rect.right() - pad;
        match meta {
            NavMeta::Count(n, c) if n > 0 => {
                let g = caps(ui, &n.to_string(), theme::text::META, theme::medium(), 0.0, c);
                let w = g.size().x;
                paint_left(ui, Rect::from_min_max(egui::pos2(right - w, rect.top()), egui::pos2(right, rect.bottom())), g, c);
                right -= w + pad;
            }
            NavMeta::Dot(c) => {
                p.circle_filled(egui::pos2(right - 3.5, rect.center().y), 3.5, c);
                right -= 7.0 + pad;
            }
            NavMeta::State(word, c) => {
                // The dot at the row's edge, where the other rows keep theirs; the word before it.
                p.circle_filled(egui::pos2(right - 3.5, rect.center().y), 3.5, c);
                right -= 7.0 + theme::space::S2;
                let g = caps(ui, word, theme::text::META, egui::FontFamily::Proportional, 0.0, t.faint);
                let w = g.size().x;
                paint_left(ui, Rect::from_min_max(egui::pos2(right - w, rect.top()), egui::pos2(right, rect.bottom())), g, t.faint);
                right -= w + pad;
            }
            _ => {}
        }
        let text_left = icon_rect.right() + pad;
        let family = if current { theme::semibold() } else { egui::FontFamily::Proportional };
        let format = egui::TextFormat { font_id: egui::FontId::new(theme::text::LABEL, family), color: ink, ..Default::default() };
        let mut job = egui::text::LayoutJob::single_section(label.to_owned(), format);
        job.wrap = egui::text::TextWrapping::truncate_at_width((right - text_left).max(0.0));
        let g = ui.painter().layout_job(job);
        paint_left(ui, Rect::from_min_max(egui::pos2(text_left, rect.top()), rect.max), g, ink);
    }
    // A state word is part of what the row says, so screen readers hear it too ("AI engine: Standby").
    let name = match meta {
        NavMeta::State(word, _) => format!("{label}: {word}"),
        _ => label.to_owned(),
    };
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, current, &name));
    focus_ring(ui, &resp, theme::radius::SM);
    let resp = if rail {
        tip(resp, label)
    } else if hint.is_empty() {
        resp
    } else {
        tip(resp, hint)
    };
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// The sidebar's account row (Redrob Cowork's): a 32 px avatar on the sunken surface (the
/// workspace's initial in ink, or a person), the name in 13 px semibold and a 12 px status line in
/// ink-faint under it. Screen readers read `name`.
pub fn account_row(ui: &mut Ui, name: &str, meta: &str, initial: Option<char>, current: bool, rail: bool, hint: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 48.0), Sense::click());
    let hovered = resp.hovered();
    let p = ui.painter();
    let radius = CornerRadius::same(theme::radius::SM);
    if current {
        p.rect_filled(rect, radius, t.sunken);
    } else if hovered {
        p.rect_filled(rect, radius, t.hover_row());
    }
    let size = 32.0;
    let centre = if rail { rect.center() } else { egui::pos2(rect.left() + theme::space::S2 + size / 2.0, rect.center().y) };
    p.circle(centre, size / 2.0, if current || hovered { t.bg } else { t.sunken }, t.hairline());
    match initial {
        Some(c) => {
            let s: String = c.to_uppercase().collect();
            p.text(centre, egui::Align2::CENTER_CENTER, s, egui::FontId::new(theme::text::LABEL, theme::semibold()), t.ink);
        }
        None => crate::glyphs::paint(p, Rect::from_center_size(centre, vec2(16.0, 16.0)), Icon::Account, t.muted),
    }
    if !rail {
        let left = centre.x + size / 2.0 + 10.0;
        let width = (rect.right() - theme::space::S2 - left).max(0.0);
        let line = |text: &str, size: f32, family: egui::FontFamily, color: Color32| {
            let format = egui::TextFormat { font_id: egui::FontId::new(size, family), color, ..Default::default() };
            let mut job = egui::text::LayoutJob::single_section(text.to_owned(), format);
            job.wrap = egui::text::TextWrapping::truncate_at_width(width);
            ui.painter().layout_job(job)
        };
        let top = line(name, theme::text::LABEL, theme::semibold(), t.ink);
        let bottom = line(meta, theme::text::META, egui::FontFamily::Proportional, t.faint);
        let total = top.size().y + bottom.size().y;
        let y = rect.center().y - total / 2.0;
        let top_h = top.size().y;
        ui.painter().galley(egui::pos2(left, y), top, t.ink);
        ui.painter().galley(egui::pos2(left, y + top_h), bottom, t.faint);
    }
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, current, name));
    focus_ring(ui, &resp, theme::radius::SM);
    let resp = if rail { tip(resp, name) } else { tip(resp, hint) };
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A DS Menu item, `width` wide: 36 px, radius 6, a 16 px icon in ink-muted (or its room, so the
/// labels line up) and a 13 px label in ink; hover takes the sunken step. A disabled item (the
/// version line) is ink-faint and can't be clicked.
pub fn menu_item(ui: &mut Ui, icon: Option<Icon>, label: &str, enabled: bool, width: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(width, 36.0), if enabled { Sense::click() } else { Sense::hover() });
    let hovered = enabled && (resp.hovered() || resp.is_pointer_button_down_on());
    let p = ui.painter();
    if hovered {
        p.rect_filled(rect, CornerRadius::same(theme::radius::SM), t.hover_row());
    }
    let pad = theme::space::S2;
    let icon_rect = Rect::from_min_size(egui::pos2(rect.left() + pad, rect.center().y - 8.0), vec2(16.0, 16.0));
    if let Some(icon) = icon {
        crate::glyphs::paint(p, icon_rect, icon, if hovered { t.ink } else { t.muted });
    }
    let ink = if enabled { t.ink } else { t.faint };
    let galley = caps(ui, label, theme::text::LABEL, egui::FontFamily::Proportional, 0.0, ink);
    paint_left(ui, Rect::from_min_max(egui::pos2(icon_rect.right() + pad, rect.top()), rect.max), galley, ink);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, label));
    if !enabled {
        return resp;
    }
    focus_ring(ui, &resp, theme::radius::SM);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// The DS ThemeSwitch: System, Light and Dark as three 28 px round icon options in a pill with a
/// hairline, the chosen one sunken with a hairline ring. Three options, never a two-way toggle.
/// Stacked when `vertical` (the folded sidebar). Returns whether the choice changed.
pub fn theme_switch(ui: &mut Ui, value: &mut theme::ThemeChoice, vertical: bool) -> bool {
    use theme::ThemeChoice;
    let t = Tokens::get(ui.ctx());
    // 28 px circles (DS ThemeSwitch), each with a larger target: 32 along the row (neighbours'
    // targets meet, never overlap) and 36 across it (Cowork's 36 px hit target).
    const OPT: f32 = 28.0;
    const PAD: f32 = 2.0;
    const GAP: f32 = 4.0;
    const HIT_ALONG: f32 = OPT + GAP;
    const HIT_ACROSS: f32 = 36.0;
    let options = [(ThemeChoice::System, Icon::Monitor), (ThemeChoice::Light, Icon::Sun), (ThemeChoice::Dark, Icon::Moon)];
    let long = 3.0 * OPT + 2.0 * GAP + 2.0 * PAD;
    let short = OPT + 2.0 * PAD;
    let (track, _) = ui.allocate_exact_size(if vertical { vec2(short, long) } else { vec2(long, short) }, Sense::hover());
    let pill = CornerRadius::same(theme::radius::PILL);
    ui.painter().rect(track, pill, t.bg, t.hairline(), egui::StrokeKind::Inside);
    let mut changed = false;
    for (i, (choice, icon)) in options.into_iter().enumerate() {
        let off = PAD + i as f32 * (OPT + GAP);
        let min = if vertical { track.min + vec2(PAD, off) } else { track.min + vec2(off, PAD) };
        let rect = Rect::from_min_size(min, vec2(OPT, OPT));
        let label = choice.label();
        let hit = Rect::from_center_size(rect.center(), if vertical { vec2(HIT_ACROSS, HIT_ALONG) } else { vec2(HIT_ALONG, HIT_ACROSS) });
        let resp = ui.interact(hit, ui.id().with(("theme-switch", i)), Sense::click());
        let on = *value == choice;
        let p = ui.painter();
        if on {
            p.rect(rect, pill, t.sunken, t.hairline(), egui::StrokeKind::Inside);
        }
        let ink = if on || resp.hovered() { t.ink } else { t.faint };
        crate::glyphs::paint(p, Rect::from_center_size(rect.center(), vec2(16.0, 16.0)), icon, ink);
        resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::RadioButton, true, on, label));
        focus_ring(ui, &resp, theme::radius::PILL);
        if tip(resp, label).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() && !on {
            *value = choice;
            changed = true;
        }
    }
    changed
}

/// A row of text and badges only as tall as its tallest item. egui's `horizontal` is at least
/// `interact_size.y` tall (40, the DS control height), which leaves about 13 px of air above
/// and below a 12-14 px line; rows of controls keep using `horizontal`.
pub fn tight_row<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
    ui.scope(|ui| {
        ui.spacing_mut().interact_size.y = 0.0;
        ui.horizontal(add).inner
    })
}

/// [`tight_row`] that wraps onto further lines when it runs out of width.
pub fn tight_wrapped<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
    ui.scope(|ui| {
        ui.spacing_mut().interact_size.y = 0.0;
        ui.horizontal_wrapped(add).inner
    })
}

/// The faint "·" between the items of a meta line ([`hud`] items in a row). Decoration only:
/// screen readers skip it.
pub fn meta_sep(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let galley = hud_galley(ui, "·", t.faint);
    let (rect, _) = ui.allocate_exact_size(galley.size(), Sense::hover());
    paint_left(ui, rect, galley, t.faint);
}
