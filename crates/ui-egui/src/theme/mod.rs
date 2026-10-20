// ADDED-LINE-RATIONALE 71b01dd86ab18be2:1: Supports `Tokens` while preserving the nearby constraint: Primary action hover fill.
// ADDED-LINE-RATIONALE abd600b85f0a8df5:1: Supports `Tokens` while preserving the nearby constraint: Keyboard focus ring.
// ADDED-LINE-RATIONALE b1d4d37f8f5f54ec:1: Supports `Tokens` while preserving the nearby constraint: Subtle brand-tinted surface.
// ADDED-LINE-RATIONALE de817fcab24fff1c:1: Supports `Tokens` while preserving the nearby constraint: Text and glyphs on brand and status fills.
// ADDED-LINE-RATIONALE f5784c7ee7a09d14:1: Supports `platform_caption` while preserving the nearby constraint: Caption colours fixed by the platform contract rather than the active appearance.
// ADDED-LINE-RATIONALE f3b40bcc0e0961f1:1: Gives `platform_caption` direct access to `super / Color32 / generated` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 4b74204365555f07:1: Supports `platform_caption` while preserving the nearby constraint: Redrob Cowork's fixed title-bar symbol colour in both appearances.
// ADDED-LINE-RATIONALE 5f853ff3761adc55:1: Supports `platform_caption` while preserving the nearby constraint: Windows' standard close-button hover, as used by Electron.
// ADDED-LINE-RATIONALE ae76b112339cf949:5: Bounds the `platform_caption` implementation here so `CLOSE_HOVER / Color32 / hex` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c974d889a3e057e4:1: Keeps `primary_hover / hex / generated` in `platform_caption`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE c589295c79e1d960:1: Keeps `focus / hex / generated` in `platform_caption`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE a9db2246e741bc04:1: Keeps `brand_subtle / hex / generated` in `platform_caption`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE d707c0f8b7e1985c:1: Keeps `on_brand / hex / generated` in `platform_caption`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 25cee0903bd9d1b8:1: Supports `raised` while preserving the nearby constraint: (#f7f8f9 / #131519).
// ADDED-LINE-RATIONALE 8dee55efee34bfc1:1: Delegates `raised`'s `mix / bg / sunken` work to `mix` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:5: Bounds the `raised` implementation here so `mix / bg / sunken` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 91c28afd9a1b7c99:1: Supports `primary_hover` while preserving the nearby constraint: launcher's other brand roles.
// ADDED-LINE-RATIONALE d60698e319b79559:1: Makes `primary_hover` an explicit part of `primary_hover` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:6: Bounds the `primary_hover` implementation here so `primary_hover` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0a0c05121982b68f:1: Supports `brand_subtle` while preserving the nearby constraint: `surface-brand-subtle`, emitted directly to keep native and Rust controls identical.
// ADDED-LINE-RATIONALE 67ef73c1d3f3b52d:1: Makes `brand_subtle` an explicit part of `brand_subtle` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:7: Bounds the `brand_subtle` implementation here so `brand_subtle` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 576b991a4c7b76a8:1: Supports `selection` while preserving the nearby constraint: (#dde3ff / #1c3292).
// ADDED-LINE-RATIONALE f9213b7826d62a21:1: Delegates `selection`'s `mix / bg / accent` work to `mix` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:8: Bounds the `selection` implementation here so `mix / bg / accent` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b8028a9bb2ce90c2:1: Supports `focus` while preserving the nearby constraint: `focus-ring`, emitted directly because each appearance has an explicit DS value.
// ADDED-LINE-RATIONALE 4bc68c9dea7f0a86:1: Makes `focus` an explicit part of `focus` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:9: Bounds the `focus` implementation here so `focus` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE de00a9c500605e87:1: Supports `on_brand` while preserving the nearby constraint: thumb.
// ADDED-LINE-RATIONALE 46f07f1cdab1d631:1: Makes `on_brand` an explicit part of `on_brand` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:10: Bounds the `on_brand` implementation here so `on_brand` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2410c2dbaebe6704:1: Supports `tooltip` while preserving the nearby constraint: #f8f9fb with #0a0b0c): (fill, text).
// ADDED-LINE-RATIONALE 402cac9d8f92a483:1: Makes `dark / ink / invert_bg` an explicit part of `tooltip` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:11: Bounds the `tooltip` implementation here so `dark / ink / invert_bg` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 43335c3203082aab:1: Supports `hover_row` while preserving the nearby constraint: below the current row's full sunken fill (DS AppShell nav hover).
// ADDED-LINE-RATIONALE 8deaddefee31be02:1: Delegates `hover_row`'s `mix / bg / sunken` work to `mix` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:12: Bounds the `hover_row` implementation here so `mix / bg / sunken` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4629baa1535b23e8:1: Supports `card_edge` while preserving the nearby constraint: Redrob Cowork's floating card edge: `border-subtle` at 55% (dark: white at 7%).
// ADDED-LINE-RATIONALE 2b9df85e1ce06bea:1: Prevents `card_edge` from applying its side effect outside the `dark / tint / Color32` state.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:13: Bounds the `card_edge` implementation here so `dark / tint / Color32` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b2bed8f1d7d919ed:1: Supports `card_highlight` while preserving the nearby constraint: The floating card's 1 px top highlight (`inset 0 1px 0`): white at 40% (dark 7%).
// ADDED-LINE-RATIONALE f20243b5f9cffe20:1: Delegates `card_highlight`'s `tint / Color32 / WHITE` work to `tint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:14: Bounds the `card_highlight` implementation here so `tint / Color32 / WHITE` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a4204694d91eeac4:1: Supports `scrim` while preserving the nearby constraint: `overlay-scrim` behind a modal: redrob-black at 55% (dark: black at 66%).
// ADDED-LINE-RATIONALE 7be32b0eb0b9e2cf:1: Prevents `scrim` from applying its side effect outside the `dark / tint / Color32` state.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:15: Bounds the `scrim` implementation here so `dark / tint / Color32` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 9e00aa1eb6f6c7ea:1: Supports `alert_edge` while preserving the nearby constraint: A brand Alert's edge (DS Alert: blue-2 light): the derived selection blue.
// ADDED-LINE-RATIONALE 84aae8a0ef641237:1: Delegates `alert_edge`'s `selection` work to `self.selection` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:16: Bounds the `alert_edge` implementation here so `selection` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7411b624ba936090:1: Supports `bad_hover` while preserving the nearby constraint: Danger button hover (DS: `status-danger` at `brightness(0.92)`).
// ADDED-LINE-RATIONALE 194d6fa3c22c8c75:1: Delegates `bad_hover`'s `scale / bad` work to `scale` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:17: Bounds the `bad_hover` implementation here so `scale / bad` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e3153567971fa386:1: Supports `subtle` while preserving the nearby constraint: light / step 5 dark), approximated as the colour 12% (dark 18%) over the base.
// ADDED-LINE-RATIONALE f2cabafc94a438a3:1: Delegates `subtle`'s `mix / bg / c` work to `mix` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:18: Bounds the `subtle` implementation here so `mix / bg / c` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3fd48a19a4de8afa:1: Supports `mix` while preserving the nearby constraint: `a` moved toward `b` by `f` (0-1), opaque: a colour between two roles.
// ADDED-LINE-RATIONALE a9f27ae69daea910:1: Freezes `f` for `mix` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 14718a51060c158f:1: Freezes `ch` for `mix` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f296b1514bfecbc0:1: Delegates `mix`'s `Color32 / from_rgb / ch` work to `Color32::from_rgb` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 73c97b494e4e6e3f:1: Supports `scale` while preserving the nearby constraint: `c` with its red, green and blue scaled by `k` (CSS `brightness(k)`), alpha kept.
// ADDED-LINE-RATIONALE 65fd346ba82ae2d6:1: Freezes `ch` for `scale` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4f145bf8a9f6c089:1: Delegates `scale`'s `Color32 / from_rgba_premultiplied / ch` work to `Color32::from_rgba_premultiplied` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ae76b112339cf949:9: Bounds the `scale` implementation here so `Color32 / from_rgba_premultiplied / ch` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f14a04d4d8d39e84:1: Supports `radius` while preserving the nearby constraint: Corner radii (DS `radius-*`), in points.
// ADDED-LINE-RATIONALE 047976f735ca4594:1: Supports `radius` while preserving the nearby constraint: `radius-xs`: checkbox box, chart containers.
// ADDED-LINE-RATIONALE e7bbe7c76f3a1651:1: Supports `radius` while preserving the nearby constraint: `radius-sm` / `field-radius`: text fields, nav rows, menu items, tooltips.
// ADDED-LINE-RATIONALE a9f35941bb068dd4:1: Supports `radius` while preserving the nearby constraint: `radius-md` / `control-radius`: buttons, icon buttons, menus, toasts.
// ADDED-LINE-RATIONALE 6425ae9f4562afff:1: Supports `radius` while preserving the nearby constraint: `radius-lg` / `card-radius`: cards, tiles, select lists, table wraps.
// ADDED-LINE-RATIONALE fe2f2b4ca22e0faa:1: Supports `radius` while preserving the nearby constraint: `radius-xl`: modals, drawers, the floating sidebar card.
// ADDED-LINE-RATIONALE 115cebb8ab359fc6:1: Supports `radius` while preserving the nearby constraint: `radius-full`: pills (badges, switches, segmented controls, progress tracks).
// ADDED-LINE-RATIONALE ae76b112339cf949:10: Bounds the `radius` implementation here so `PILL / u8` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 9ed949c63b9a4cce:1: Supports `space` while preserving the nearby constraint: The space scale (DS `space-*`), in points.
// ADDED-LINE-RATIONALE a4f317569f68d92f:1: Supports `space` while preserving the nearby constraint: `space-1`.
// ADDED-LINE-RATIONALE 6306db3ee5e22caa:1: Supports `space` while preserving the nearby constraint: `space-2`.
// ADDED-LINE-RATIONALE 55fcf19b058e9658:1: Supports `space` while preserving the nearby constraint: `space-3`.
// ADDED-LINE-RATIONALE 5e1f2a993a9690cf:1: Supports `space` while preserving the nearby constraint: `space-4`.
// ADDED-LINE-RATIONALE f403455ae9e09f2d:1: Supports `space` while preserving the nearby constraint: `space-5`; also `card-padding`.
// ADDED-LINE-RATIONALE 59b9bb92d07f8362:1: Supports `space` while preserving the nearby constraint: `space-6`; also `section-gap` and the work area's padding.
// ADDED-LINE-RATIONALE 023bdfb6e7f46fed:1: Supports `space` while preserving the nearby constraint: `space-8`.
// ADDED-LINE-RATIONALE 51d5ca089e88e1f9:1: Supports `space` while preserving the nearby constraint: `space-12`.
// ADDED-LINE-RATIONALE ae76b112339cf949:11: Bounds the `space` implementation here so `S12 / f32` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1dd31a94db83b461:1: Supports `control` while preserving the nearby constraint: Control heights (DS `control-h-*`): buttons, inputs, selects.
// ADDED-LINE-RATIONALE e1b140a6e5b9c8c4:1: Supports `control` while preserving the nearby constraint: `control-h-sm`: small buttons and icon buttons.
// ADDED-LINE-RATIONALE 0fb1c9ea5e911650:1: Supports `control` while preserving the nearby constraint: `control-h-md`: the default.
// ADDED-LINE-RATIONALE 5227f625584aa12c:1: Supports `control` while preserving the nearby constraint: `control-h-lg`: the one prominent action.
// ADDED-LINE-RATIONALE cfe0589dce1f8c29:1: Supports `text` while preserving the nearby constraint: [`semibold`], meta in [`medium`].
// ADDED-LINE-RATIONALE 21321ec22c2a1301:1: Supports `text` while preserving the nearby constraint: `body` (15/24, 400): prose.
// ADDED-LINE-RATIONALE 099a7d426c2bc67f:1: Supports `text` while preserving the nearby constraint: The size egui's Body text style keeps (DS controls, menus and selects are 14).
// ADDED-LINE-RATIONALE f6cd47f149976908:1: Supports `text` while preserving the nearby constraint: `body-sm` (13/20, 400): dense and secondary text.
// ADDED-LINE-RATIONALE b3258a716fd8ae3d:1: Supports `text` while preserving the nearby constraint: `label` (13/18, 600): form labels, buttons, tabs.
// ADDED-LINE-RATIONALE 72259c983c429635:1: Supports `text` while preserving the nearby constraint: `meta` (12/18, 500): the line above a title; sentence case, never upper case.
// ADDED-LINE-RATIONALE 2179cddadb46dc77:1: Supports `text` while preserving the nearby constraint: `caption` (11/16, 400): helper text.
// ADDED-LINE-RATIONALE 8fdc9abfbc8b2fbc:1: Supports `text` while preserving the nearby constraint: `title-4` (17/24, 600): card and dialog titles.
// ADDED-LINE-RATIONALE e6531b2d775acd1c:1: Supports `text` while preserving the nearby constraint: `title-3` (21/28, 600): modal, drawer and empty-state titles.
// ADDED-LINE-RATIONALE 49cedd38efc4604f:1: Supports `text` while preserving the nearby constraint: `title-2` (27/34, 700): major sections.
// ADDED-LINE-RATIONALE ae76b112339cf949:13: Bounds the `text` implementation here so `TITLE_2 / f32` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 93851d2b2ab9d07c:1: Supports `motion` while preserving the nearby constraint: Motion durations (DS `duration-*`), in seconds, and the DS easing curves.
// ADDED-LINE-RATIONALE 328fe6f7335de972:1: Supports `motion` while preserving the nearby constraint: `duration-instant` (80 ms): hover and press feedback.
// ADDED-LINE-RATIONALE 8121e8e9494e17b0:1: Supports `motion` while preserving the nearby constraint: `duration-fast` (140 ms): the default state change (colour, border, opacity).
// ADDED-LINE-RATIONALE babf32fa46f18fbe:1: Supports `motion` while preserving the nearby constraint: `duration-base` (220 ms): small surfaces in and out, the switch thumb, chevrons.
// ADDED-LINE-RATIONALE 61e161689b3f457f:1: Supports `motion` while preserving the nearby constraint: `duration-slow` (320 ms): drawers, modals, expanding panels, progress width.
// ADDED-LINE-RATIONALE 06498fb0aefffc13:1: Supports `motion` while preserving the nearby constraint: The DS easing curves (CSS `cubic-bezier`).
// ADDED-LINE-RATIONALE 454f950e3385729e:1: Supports `Easing` while preserving the nearby constraint: [derive(Clone, Copy, Debug, PartialEq)].
// ADDED-LINE-RATIONALE 144a85f2c0bb08a9:1: Supports `Easing` while preserving the nearby constraint: `ease-standard` (0.2, 0, 0, 1): state changes.
// ADDED-LINE-RATIONALE c089f067a93e79f8:1: Supports `Easing` while preserving the nearby constraint: `ease-entrance` (0.05, 0.7, 0.1, 1): arrivals.
// ADDED-LINE-RATIONALE 4a65c2dfe003e54e:1: Supports `Easing` while preserving the nearby constraint: `ease-exit` (0.3, 0, 0.8, 0.15): departures, faster than arrivals.
// ADDED-LINE-RATIONALE 16ede566cf6b1177:1: Supports `Easing` while preserving the nearby constraint: `ease-settle` (0.34, 1.4, 0.64, 1): a small overshoot for switches and chevrons.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:19: Bounds the `Easing` implementation here so `Settle` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f3c4eb3870260070:1: Makes `Easing` an explicit part of `Easing` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE cb41f939a66b7918:1: Supports `points` while preserving the nearby constraint: The curve's control points (x1, y1, x2, y2).
// ADDED-LINE-RATIONALE 27fac785f7ac226a:1: Prevents `points` from applying its side effect outside the `points / f32` state.
// ADDED-LINE-RATIONALE 2d3b946d79ee3a9d:1: Freezes `Self` for `points` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e134a8e970421b8f:1: Freezes `Self` for `points` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dacbc23d0be89e20:1: Freezes `Self` for `points` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bcac3cc902eef679:1: Freezes `Self` for `points` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fb29eac8c54c2b29:1: Bounds the `points` implementation here so `Self / Settle` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3ef658bd789f1809:3: Bounds the `points` implementation here so `Self / Settle` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE bf7d236e0ecb5222:1: Supports `apply` while preserving the nearby constraint: Progress at `t` (0-1 of the duration): may leave 0-1 for [`Easing::Settle`].
// ADDED-LINE-RATIONALE 74f37a8190243b9b:1: Freezes `let` for `apply` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3500d05af0497cdd:1: Delegates `apply`'s `cubic_bezier / x1 / y1` work to `cubic_bezier` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3ef658bd789f1809:4: Bounds the `apply` implementation here so `cubic_bezier / x1 / y1` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:20: Bounds the `apply` implementation here so `cubic_bezier / x1 / y1` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f8c11e53628f891c:1: Supports `cubic_bezier` while preserving the nearby constraint: parameter by bisection (x is monotonic for x1, x2 in 0-1), then returns its y.
// ADDED-LINE-RATIONALE 1393818afedd2380:1: Freezes `x` for `cubic_bezier` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 78a646744947a1e3:1: Freezes `at` for `cubic_bezier` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE facd1407a98902f6:1: Freezes `r` for `cubic_bezier` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b73bfad8d43b45cd:1: Makes `r / r / s` an explicit part of `cubic_bezier` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 9bb0fbf3f6561cf6:1: Bounds the `cubic_bezier` implementation here so `r / r / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE cc9fd4d996b1ebb0:1: Freezes `let` for `cubic_bezier` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 819f880a17d22580:1: Applies `cubic_bezier` consistently to every `_ / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE cc5fbed90b344c1a:1: Freezes `mid` for `cubic_bezier` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dff42770b6ecb38f:1: Prevents `cubic_bezier` from applying its side effect outside the `at / x1 / x2` state.
// ADDED-LINE-RATIONALE b02324a29c31ef7f:1: Freezes `lo` for `cubic_bezier` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ea2b200bf2d01547:1: Makes `lo / mid` an explicit part of `cubic_bezier` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 2fe0df99f637aa55:1: Freezes `hi` for `cubic_bezier` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fb29eac8c54c2b29:2: Bounds the `cubic_bezier` implementation here so `hi / mid` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3ef658bd789f1809:5: Bounds the `cubic_bezier` implementation here so `at / y1 / y2` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE cb55a523f0f3428a:1: Delegates `cubic_bezier`'s `at / y1 / y2` work to `at` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:21: Bounds the `cubic_bezier` implementation here so `at / y1 / y2` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 90a94ef058610a20:1: Supports `duration` while preserving the nearby constraint: honours reduced motion).
// ADDED-LINE-RATIONALE 793a5d6357af64cc:1: Freezes `if` for `duration` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:22: Bounds the `duration` implementation here so `ctx / global_style / animation_time` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ae76b112339cf949:14: Bounds the `duration` implementation here so `ctx / global_style / animation_time` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE eef0854d86cdf65e:1: Supports `ds_shadow` while preserving the nearby constraint: redrob-black, dark ones black).
// ADDED-LINE-RATIONALE b5d13d241d9e9871:1: Freezes `color` for `ds_shadow` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 29bfb12ab071aa2c:1: Makes `egui / epaint / Shadow` an explicit part of `ds_shadow` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE ae76b112339cf949:15: Bounds the `ds_shadow` implementation here so `egui / epaint / Shadow` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 93d369ac12d88da4:1: Supports `shadow_sm` while preserving the nearby constraint: `shadow-sm` (0 1px 2px, .08 / dark .6): resting inputs, the switch thumb, a selected segment.
// ADDED-LINE-RATIONALE 6ca9dd41c54d872a:1: Delegates `shadow_sm`'s `ds_shadow / t` work to `ds_shadow` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ae76b112339cf949:16: Bounds the `shadow_sm` implementation here so `ds_shadow / t` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8f05e26cda890589:1: Supports `shadow_md` while preserving the nearby constraint: `shadow-md` (0 4px 12px, .10 / dark .7): menus, popovers, tooltips, a hovered card.
// ADDED-LINE-RATIONALE 5516ce46357a3e5a:1: Delegates `shadow_md`'s `ds_shadow / t` work to `ds_shadow` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ae76b112339cf949:17: Bounds the `shadow_md` implementation here so `ds_shadow / t` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 06dc10af50f0868d:1: Supports `shadow_lg` while preserving the nearby constraint: `shadow-lg` (0 16px 40px, .14 / dark .8): modals, drawers, toasts, select lists.
// ADDED-LINE-RATIONALE c9e706345fc02ead:1: Delegates `shadow_lg`'s `ds_shadow / t` work to `ds_shadow` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ae76b112339cf949:18: Bounds the `shadow_lg` implementation here so `ds_shadow / t` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE afaec367f1ee2d06:1: Supports `shadow_floating` while preserving the nearby constraint: negative spread, so draw it under the card's rect shrunk by [`FLOATING_SHADOW_INSET`].
// ADDED-LINE-RATIONALE 1a872c83f81f995d:1: Delegates `shadow_floating`'s `ds_shadow / t` work to `ds_shadow` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ae76b112339cf949:19: Bounds the `shadow_floating` implementation here so `ds_shadow / t` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2dade59422d72755:1: Supports `shadow_floating` while preserving the nearby constraint: The negative spread of [`shadow_floating`]: shrink the card's rect by this before casting it.
// ADDED-LINE-RATIONALE d5c3ddf15f955c21:1: Delegates `shadow`'s `shadow_lg / t` work to `shadow_lg` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 083a07229f1f4f22:1: Supports `visuals` while preserving the nearby constraint: DS text fields sit on the base surface with a strong outline (Field control).
// ADDED-LINE-RATIONALE e6546af76c7d64e3:1: Freezes `v` for `visuals` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f694be0e8658b36b:1: Freezes `v` for `visuals` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fbac9354b4ebe1ad:1: Freezes `v` for `visuals` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 69f64e421da40d48:1: Freezes `v` for `visuals` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1cd758080d0b9c96:1: Freezes `v` for `visuals` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fa9a262a1bf6dd48:1: Freezes `v` for `visuals` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0ec6d65fd8806933:1: Supports `visuals` while preserving the nearby constraint: egui outlines a focused text field with the selection stroke: DS Field focus border.
// ADDED-LINE-RATIONALE 866919644e3bdd50:1: Freezes `v` for `visuals` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 755b31b9ac5954fe:1: Keeps `v / widgets / inactive` in `visuals`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE aef7bf479d803840:1: Keeps `v / widgets / hovered` in `visuals`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 66633024da625c86:1: Supports `visuals` while preserving the nearby constraint: Pressed and focused fields: the brand blue outline (DS Field focus).
// ADDED-LINE-RATIONALE 57c48db20400f700:1: Keeps `v / widgets / open` in `visuals`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 6e75aca8622ecb67:1: Supports `visuals` while preserving the nearby constraint: field radius (our buttons draw their own 8).
// ADDED-LINE-RATIONALE e65479a9a513572d:1: Supports `apply` while preserving the nearby constraint: `interact_size`.
// ADDED-LINE-RATIONALE 88fc4c074c734cd1:1: Supports `apply` while preserving the nearby constraint: DS Menu list: 8 px of padding.
// ADDED-LINE-RATIONALE dbd3558570536d23:1: Freezes `style` for `apply` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 89fe432babbe473e:1: Keeps `TextStyle / Heading / FontId` in `apply`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 7c45a72158dc44b4:1: Keeps `TextStyle / Body / FontId` in `apply`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 2a07692b657942a0:1: Keeps `TextStyle / Button / FontId` in `apply`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 4153dd78f804f47b:1: Keeps `TextStyle / Small / FontId` in `apply`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE bf9b137ddd15d31f:1: Keeps `TextStyle / Monospace / FontId` in `apply`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 4395592e72d58794:1: Supports `apply` while preserving the nearby constraint: egui fall through to Wanted Sans instead of silently selecting Pretendard's Korean glyphs.
// ADDED-LINE-RATIONALE ab8ca3cc61323530:1: Freezes `pub` for `apply` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4a297efb427bbc78:1: Freezes `pub` for `apply` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d448659376c3d413:1: Supports `apply` while preserving the nearby constraint: Height of buttons, inputs and filter cells: the DS's default control (`control-h-md`).
// ADDED-LINE-RATIONALE ee0ce41f88871644:1: Keeps `Pretendard / PRETENDARD_REGULAR / FontFamily` in `install_fonts`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 0b8b323bf6459b29:1: Keeps `Pretendard / Medium / PRETENDARD_MEDIUM` in `install_fonts`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE e38f27578663fb67:1: Keeps `Pretendard / SemiBold / PRETENDARD_SEMIBOLD` in `install_fonts`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE c288fac906a4b510:1: Keeps `Pretendard / SemiBold / PRETENDARD_SEMIBOLD` in `install_fonts`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE ecb47e4fdfccc958:1: Keeps `Pretendard / PRETENDARD_REGULAR / FontFamily` in `install_fonts`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE fb519266f2f5cf21:1: Keeps `Pretendard / Medium / PRETENDARD_MEDIUM` in `install_fonts`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 4882f24271cfe499:1: Keeps `Pretendard / SemiBold / PRETENDARD_SEMIBOLD` in `install_fonts`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 5f5af3811cc6e4b7:1: Supports `tint` while preserving the nearby constraint: Mixes `c` into the background at `alpha` (0â€“1): tinted washes.
// ADDED-LINE-RATIONALE d822dd6677507ea6:1: Freezes `a` for `tint` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6a272336fa39da67:1: Delegates `tint`'s `Color32 / from_rgba_unmultiplied / c` work to `Color32::from_rgba_unmultiplied` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ae76b112339cf949:32: Bounds the `tint` implementation here so `Color32 / from_rgba_unmultiplied / c` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6b4208216661712b:1: Isolates `rgb` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE 39e5d6f404494cd9:1: Delegates `rgb`'s `Color32 / from_rgb / c` work to `Color32::from_rgb` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ae76b112339cf949:33: Bounds the `rgb` implementation here so `Color32 / from_rgb / c` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c4d011e6b40e5d78:1: Declares the `cfg / test` contract on `rgb` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE a2c8d6ba33a69a2a:1: Supports `tests` while preserving the nearby constraint: [cfg(test)].
// ADDED-LINE-RATIONALE d4ce7f81f6c8d21e:1: Gives `tests` direct access to `super / motion / Easing` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE b191d7bf119418e1:1: Gives `tests` direct access to `super` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE df89b2c4e002a407:1: Declares the `test` contract on `tests` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 0b0c665629ff18ad:1: Supports `easing_curves_start_at_0_end_at_1_and_settle_overshoots` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 14165e59f483e312:1: Applies `easing_curves_start_at_0_end_at_1_and_settle_overshoots` consistently to every `e / in / Easing` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 72157b52fae89342:1: Makes `easing_curves_start_at_0_end_at_1_and_settle_overshoots` fail visibly when `assert / e / apply` violates the behavior this path promises.
// ADDED-LINE-RATIONALE ad2b4ee47f76a52c:1: Makes `easing_curves_start_at_0_end_at_1_and_settle_overshoots` fail visibly when `assert / e / apply` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 3ef658bd789f1809:7: Bounds the `easing_curves_start_at_0_end_at_1_and_settle_overshoots` implementation here so `assert / e / apply` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3bf56a7085810868:1: Supports `easing_curves_start_at_0_end_at_1_and_settle_overshoots` while preserving the nearby constraint: Standard is fast out of the gate; exit starts slowly; settle passes 1 before it lands.
// ADDED-LINE-RATIONALE 94824ad525b7e0a5:1: Makes `easing_curves_start_at_0_end_at_1_and_settle_overshoots` fail visibly when `assert / Easing / Exit` violates the behavior this path promises.
// ADDED-LINE-RATIONALE ecd2b0fb93e0dd6b:1: Freezes `assert` for `easing_curves_start_at_0_end_at_1_and_settle_overshoots` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:30: Bounds the `easing_curves_start_at_0_end_at_1_and_settle_overshoots` implementation here so `assert / map / i` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE df89b2c4e002a407:2: Declares the `test` contract on `easing_curves_start_at_0_end_at_1_and_settle_overshoots` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE b39d34b78d45cff2:1: Supports `derived_roles_are_close_to_the_design_system` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE dda26e10509caa0f:1: Freezes `near` for `derived_roles_are_close_to_the_design_system` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 19adbdb336e525fc:1: Freezes `let` for `derived_roles_are_close_to_the_design_system` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE feb881dc20662ce7:1: Freezes `c` for `derived_roles_are_close_to_the_design_system` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9bb0fbf3f6561cf6:2: Bounds the `derived_roles_are_close_to_the_design_system` implementation here so `c / r / abs_diff` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0db5ba5383bd74d0:1: Supports `derived_roles_are_close_to_the_design_system` while preserving the nearby constraint: surface-raised.
// ADDED-LINE-RATIONALE 99322d3db8176792:1: Makes `derived_roles_are_close_to_the_design_system` fail visibly when `assert / near / Tokens` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 030e56fdda7c0c91:1: Supports `derived_roles_are_close_to_the_design_system` while preserving the nearby constraint: selection (light).
// ADDED-LINE-RATIONALE 9f4e4b92ea79cb69:31: Bounds the `derived_roles_are_close_to_the_design_system` implementation here so `assert / near / Tokens` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ae76b112339cf949:34: Bounds the `derived_roles_are_close_to_the_design_system` implementation here so `assert / near / Tokens` cannot leak into the adjacent declaration or control path.
//! Colours, type and shapes from the Redrob design system: its colour roles (`@redrob-labs/ui`
//! `tokens.json`, generated into `generated.rs` by `cargo xtask tokens`),
//! Pretendard for Latin and Wanted Sans for Korean, hairline rules. Widgets read
//! [`Tokens::get`]; nothing hard-codes a colour.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, FontTweak, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeChoice {
    /// Follow the system's light or dark setting.
    #[default]
    System,
    Dark,
    Light,
}

impl ThemeChoice {
    pub const ALL: [Self; 3] = [Self::System, Self::Dark, Self::Light];

    /// The English key translated at draw time into the current UI language
    /// (the shell sets it per frame via `i18n::set_current`). Same-crate lookup,
    /// pure and wasm-safe; no new dependency. Call sites stay unchanged.
    pub fn label(self) -> &'static str {
        match self {
            Self::System => crate::i18n::t("Match system"),
            Self::Dark => crate::i18n::t("Dark"),
            Self::Light => crate::i18n::t("Light"),
        }
    }
}

/// Colour tokens for one appearance: the website's (`globals.css`), by the same names.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub dark: bool,
    /// Page and card surface.
    pub bg: Color32,
    /// Badges, wells and row hovers.
    pub sunken: Color32,
    /// Body text.
    pub ink: Color32,
    /// Headings.
    pub ink_strong: Color32,
    /// Secondary text.
    pub muted: Color32,
    /// Labels and tertiary text.
    pub faint: Color32,
    /// Hairlines.
    pub line: Color32,
    /// Outlined buttons and stronger rules.
    pub line_strong: Color32,
    /// The brand blue, for surfaces.
    pub accent: Color32,
    /// The brand blue, for text and icons.
    pub accent_ink: Color32,
    /// Primary action hover fill.
    pub primary_hover: Color32,
    /// Keyboard focus ring.
    pub focus: Color32,
    /// Subtle brand-tinted surface.
    pub brand_subtle: Color32,
    /// Text and glyphs on brand and status fills.
    pub on_brand: Color32,
    /// Primary buttons and hover inversion.
    pub invert_bg: Color32,
    pub invert_fg: Color32,
    pub good: Color32,
    pub warn: Color32,
    pub bad: Color32,
}

const fn hex(rgb: u32) -> Color32 {
    let [_, r, g, b] = rgb.to_be_bytes();
    Color32::from_rgb(r, g, b)
}

/// The colour values, generated from the design-system package by `cargo xtask tokens`.
pub mod generated;

/// Caption colours fixed by the platform contract rather than the active appearance.
pub mod platform_caption {
    use super::{Color32, generated, hex};

    /// Redrob Cowork's fixed title-bar symbol colour in both appearances.
    pub const SYMBOL: Color32 = hex(generated::platform_caption::SYMBOL);
    /// Windows' standard close-button hover, as used by Electron.
    pub const CLOSE_HOVER: Color32 = hex(generated::platform_caption::CLOSE_HOVER);
}

/// The design-system package release the colours come from (`@redrob-labs/ui x.y.z`).
pub const TOKENS_SOURCE: &str = generated::SOURCE;

macro_rules! from_generated {
    ($m:ident, $dark:expr) => {
        Self {
            dark: $dark,
            bg: hex(generated::$m::BG),
            sunken: hex(generated::$m::SUNKEN),
            ink: hex(generated::$m::INK),
            ink_strong: hex(generated::$m::INK_STRONG),
            muted: hex(generated::$m::MUTED),
            faint: hex(generated::$m::FAINT),
            line: hex(generated::$m::LINE),
            line_strong: hex(generated::$m::LINE_STRONG),
            accent: hex(generated::$m::ACCENT),
            accent_ink: hex(generated::$m::ACCENT_INK),
            primary_hover: hex(generated::$m::PRIMARY_HOVER),
            focus: hex(generated::$m::FOCUS),
            brand_subtle: hex(generated::$m::BRAND_SUBTLE),
            on_brand: hex(generated::$m::ON_BRAND),
            invert_bg: hex(generated::$m::INVERT_BG),
            invert_fg: hex(generated::$m::INVERT_FG),
            good: hex(generated::$m::GOOD),
            warn: hex(generated::$m::WARN),
            bad: hex(generated::$m::BAD),
        }
    };
}

impl Tokens {
    // The values are the Redrob design system's (`@redrob-labs/ui`, `tokens.json`), bound to these
    // roles by xtask/src/tokens.rs and written to `generated.rs`. Nothing here is typed by hand.
    pub const DARK: Self = from_generated!(dark, true);

    pub const LIGHT: Self = from_generated!(light, false);

    pub fn get(ctx: &egui::Context) -> Self {
        if ctx.theme() == egui::Theme::Dark { Self::DARK } else { Self::LIGHT }
    }

    pub fn hairline(&self) -> Stroke {
        Stroke::new(1.0, self.line)
    }

    /// An app's text colour (`--app-ink`): readable on this appearance's background.
    pub fn app_ink(&self, app: &redrob_launcher_core::CraftApp) -> Color32 {
        rgb(if self.dark { app.ink_dark } else { app.ink_light })
    }

    // Exact DS roles below are emitted directly; the remaining appearance helpers document any
    // deliberate derivation from generated roles.

    /// `surface-raised` (#f8f9fb / #141719): halfway from the base to the sunken surface
    /// (#f7f8f9 / #131519).
    pub fn raised(&self) -> Color32 {
        mix(self.bg, self.sunken, 0.5)
    }

    /// `action-primary-hover`, emitted directly because its dark value is not a blend of the
    /// launcher's other brand roles.
    pub fn primary_hover(&self) -> Color32 {
        self.primary_hover
    }

    /// `surface-brand-subtle`, emitted directly to keep native and Rust controls identical.
    pub fn brand_subtle(&self) -> Color32 {
        self.brand_subtle
    }

    /// `selection` background (#d9e6ff / #09209c): the brand blue 16% (dark 55%) over the base
    /// (#dde3ff / #1c3292).
    pub fn selection(&self) -> Color32 {
        mix(self.bg, self.accent, if self.dark { 0.55 } else { 0.16 })
    }

    /// `focus-ring`, emitted directly because each appearance has an explicit DS value.
    pub fn focus(&self) -> Color32 {
        self.focus
    }

    /// `ink-on-brand`: text and glyphs on a brand, danger or product-colour fill, and the switch
    /// thumb.
    pub fn on_brand(&self) -> Color32 {
        self.on_brand
    }

    /// The tooltip bubble (DS Tooltip: `redrob-black` with white text; dark inverts to
    /// #f8f9fb with #0a0b0c): (fill, text).
    pub fn tooltip(&self) -> (Color32, Color32) {
        (if self.dark { self.ink } else { self.invert_bg }, self.invert_fg)
    }

    /// A row's hover in a list on the base surface: `surface-sunken` 60% over the base, a step
    /// below the current row's full sunken fill (DS AppShell nav hover).
    pub fn hover_row(&self) -> Color32 {
        mix(self.bg, self.sunken, 0.6)
    }

    /// Redrob Cowork's floating card edge: `border-subtle` at 55% (dark: white at 7%).
    pub fn card_edge(&self) -> Color32 {
        if self.dark { tint(Color32::WHITE, 0.07) } else { tint(self.line, 0.55) }
    }

    /// The floating card's 1 px top highlight (`inset 0 1px 0`): white at 40% (dark 7%).
    pub fn card_highlight(&self) -> Color32 {
        tint(Color32::WHITE, if self.dark { 0.07 } else { 0.40 })
    }

    /// `overlay-scrim` behind a modal: redrob-black at 55% (dark: black at 66%).
    pub fn scrim(&self) -> Color32 {
        if self.dark { tint(Color32::BLACK, 0.66) } else { tint(self.invert_bg, 0.55) }
    }

    /// A brand Alert's edge (DS Alert: blue-2 light): the derived selection blue.
    pub fn alert_edge(&self) -> Color32 {
        self.selection()
    }

    /// Danger button hover (DS: `status-danger` at `brightness(0.92)`).
    pub fn bad_hover(&self) -> Color32 {
        scale(self.bad, 0.92)
    }

    /// A subtle badge's fill for a status colour (DS Badge `subtle`: the accent family's step 1
    /// light / step 5 dark), approximated as the colour 12% (dark 18%) over the base.
    pub fn subtle(&self, c: Color32) -> Color32 {
        mix(self.bg, c, if self.dark { 0.18 } else { 0.12 })
    }
}

/// `a` moved toward `b` by `f` (0-1), opaque: a colour between two roles.
pub fn mix(a: Color32, b: Color32, f: f32) -> Color32 {
    let f = f.clamp(0.0, 1.0);
    let ch = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * f).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(ch(a.r(), b.r()), ch(a.g(), b.g()), ch(a.b(), b.b()))
}

/// `c` with its red, green and blue scaled by `k` (CSS `brightness(k)`), alpha kept.
pub fn scale(c: Color32, k: f32) -> Color32 {
    let ch = |x: u8| (x as f32 * k).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgba_premultiplied(ch(c.r()), ch(c.g()), ch(c.b()), c.a())
}

/// Corner radii (DS `radius-*`), in points.
pub mod radius {
    /// `radius-xs`: checkbox box, chart containers.
    pub const XS: u8 = 4;
    /// `radius-sm` / `field-radius`: text fields, nav rows, menu items, tooltips.
    pub const SM: u8 = 6;
    /// `radius-md` / `control-radius`: buttons, icon buttons, menus, toasts.
    pub const MD: u8 = 8;
    /// `radius-lg` / `card-radius`: cards, tiles, select lists, table wraps.
    pub const LG: u8 = 12;
    /// `radius-xl`: modals, drawers, the floating sidebar card.
    pub const XL: u8 = 16;
    /// `radius-full`: pills (badges, switches, segmented controls, progress tracks).
    pub const PILL: u8 = 255;
}

/// The space scale (DS `space-*`), in points.
pub mod space {
    /// `space-1`.
    pub const S1: f32 = 4.0;
    /// `space-2`.
    pub const S2: f32 = 8.0;
    /// `space-3`.
    pub const S3: f32 = 12.0;
    /// `space-4`.
    pub const S4: f32 = 16.0;
    /// `space-5`; also `card-padding`.
    pub const S5: f32 = 20.0;
    /// `space-6`; also `section-gap` and the work area's padding.
    pub const S6: f32 = 24.0;
    /// `space-8`.
    pub const S8: f32 = 32.0;
    /// `space-12`.
    pub const S12: f32 = 48.0;
}

/// Control heights (DS `control-h-*`): buttons, inputs, selects.
pub mod control {
    /// `control-h-sm`: small buttons and icon buttons.
    pub const SM: f32 = 32.0;
    /// `control-h-md`: the default.
    pub const MD: f32 = 40.0;
    /// `control-h-lg`: the one prominent action.
    pub const LG: f32 = 48.0;
}

/// Type sizes (DS text styles), in points. Weights come from the family: label/title in
/// [`semibold`], meta in [`medium`].
pub mod text {
    /// `body` (15/24, 400): prose.
    pub const BODY: f32 = 15.0;
    /// The size egui's Body text style keeps (DS controls, menus and selects are 14).
    pub const CONTROL: f32 = 14.0;
    /// `body-sm` (13/20, 400): dense and secondary text.
    pub const BODY_SM: f32 = 13.0;
    /// `label` (13/18, 600): form labels, buttons, tabs.
    pub const LABEL: f32 = 13.0;
    /// `meta` (12/18, 500): the line above a title; sentence case, never upper case.
    pub const META: f32 = 12.0;
    /// `caption` (11/16, 400): helper text.
    pub const CAPTION: f32 = 11.0;
    /// `title-4` (17/24, 600): card and dialog titles.
    pub const TITLE_4: f32 = 17.0;
    /// `title-3` (21/28, 600): modal, drawer and empty-state titles.
    pub const TITLE_3: f32 = 21.0;
    /// `title-2` (27/34, 700): major sections.
    pub const TITLE_2: f32 = 27.0;
}

/// Motion durations (DS `duration-*`), in seconds, and the DS easing curves.
pub mod motion {
    /// `duration-instant` (80 ms): hover and press feedback.
    pub const INSTANT: f32 = 0.080;
    /// `duration-fast` (140 ms): the default state change (colour, border, opacity).
    pub const FAST: f32 = 0.140;
    /// `duration-base` (220 ms): small surfaces in and out, the switch thumb, chevrons.
    pub const BASE: f32 = 0.220;
    /// `duration-slow` (320 ms): drawers, modals, expanding panels, progress width.
    pub const SLOW: f32 = 0.320;

    /// The DS easing curves (CSS `cubic-bezier`).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Easing {
        /// `ease-standard` (0.2, 0, 0, 1): state changes.
        Standard,
        /// `ease-entrance` (0.05, 0.7, 0.1, 1): arrivals.
        Entrance,
        /// `ease-exit` (0.3, 0, 0.8, 0.15): departures, faster than arrivals.
        Exit,
        /// `ease-settle` (0.34, 1.4, 0.64, 1): a small overshoot for switches and chevrons.
        Settle,
    }

    impl Easing {
        /// The curve's control points (x1, y1, x2, y2).
        pub const fn points(self) -> [f32; 4] {
            match self {
                Self::Standard => [0.2, 0.0, 0.0, 1.0],
                Self::Entrance => [0.05, 0.7, 0.1, 1.0],
                Self::Exit => [0.3, 0.0, 0.8, 0.15],
                Self::Settle => [0.34, 1.4, 0.64, 1.0],
            }
        }

        /// Progress at `t` (0-1 of the duration): may leave 0-1 for [`Easing::Settle`].
        pub fn apply(self, t: f32) -> f32 {
            let [x1, y1, x2, y2] = self.points();
            cubic_bezier(x1, y1, x2, y2, t)
        }
    }

    /// CSS `cubic-bezier(x1, y1, x2, y2)` at time `x` (0-1): solves the curve's x for its
    /// parameter by bisection (x is monotonic for x1, x2 in 0-1), then returns its y.
    pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
        let x = x.clamp(0.0, 1.0);
        let at = |a: f32, b: f32, s: f32| {
            let r = 1.0 - s;
            3.0 * r * r * s * a + 3.0 * r * s * s * b + s * s * s
        };
        let (mut lo, mut hi) = (0.0_f32, 1.0_f32);
        for _ in 0..24 {
            let mid = (lo + hi) / 2.0;
            if at(x1, x2, mid) < x {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        at(y1, y2, (lo + hi) / 2.0)
    }

    /// `secs`, or 0 when motion is off (egui's `animation_time` set to 0, which is how the shell
    /// honours reduced motion).
    pub fn duration(ctx: &egui::Context, secs: f32) -> f32 {
        if ctx.global_style().animation_time <= 0.0 { 0.0 } else { secs }
    }
}

/// A DS shadow: `y` offset, `blur`, the colour's alpha light / dark (light shadows are
/// redrob-black, dark ones black).
fn ds_shadow(t: &Tokens, y: i8, blur: u8, light: f32, dark: f32) -> egui::epaint::Shadow {
    let color = if t.dark { tint(Color32::BLACK, dark) } else { tint(t.invert_bg, light) };
    egui::epaint::Shadow { offset: [0, y], blur, spread: 0, color }
}

/// `shadow-sm` (0 1px 2px, .08 / dark .6): resting inputs, the switch thumb, a selected segment.
pub fn shadow_sm(t: &Tokens) -> egui::epaint::Shadow {
    ds_shadow(t, 1, 2, 0.08, 0.6)
}

/// `shadow-md` (0 4px 12px, .10 / dark .7): menus, popovers, tooltips, a hovered card.
pub fn shadow_md(t: &Tokens) -> egui::epaint::Shadow {
    ds_shadow(t, 4, 12, 0.10, 0.7)
}

/// `shadow-lg` (0 16px 40px, .14 / dark .8): modals, drawers, toasts, select lists.
pub fn shadow_lg(t: &Tokens) -> egui::epaint::Shadow {
    ds_shadow(t, 16, 40, 0.14, 0.8)
}

/// Redrob Cowork's floating sidebar card shadow (0 12px 32px -16px, .30; dark .45). egui shadows have no
/// negative spread, so draw it under the card's rect shrunk by [`FLOATING_SHADOW_INSET`].
pub fn shadow_floating(t: &Tokens) -> egui::epaint::Shadow {
    ds_shadow(t, 12, 32, 0.30, 0.45)
}

/// The negative spread of [`shadow_floating`]: shrink the card's rect by this before casting it.
pub const FLOATING_SHADOW_INSET: f32 = 16.0;

/// The shadow under dialogs and toasts: the DS's `shadow-lg`.
pub fn shadow(t: &Tokens) -> egui::epaint::Shadow {
    shadow_lg(t)
}

fn visuals(t: &Tokens) -> Visuals {
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.bg;
    v.window_fill = t.bg;
    v.extreme_bg_color = t.sunken;
    // DS text fields sit on the base surface with a strong outline (Field control).
    v.text_edit_bg_color = Some(t.bg);
    v.faint_bg_color = t.sunken;
    v.code_bg_color = t.sunken;
    v.window_stroke = Stroke::new(1.0, t.line);
    v.window_corner_radius = CornerRadius::same(radius::XL);
    v.menu_corner_radius = CornerRadius::same(radius::MD);
    v.window_shadow = shadow_lg(t);
    v.popup_shadow = shadow_md(t);
    v.override_text_color = Some(t.ink);
    v.hyperlink_color = t.accent_ink;
    v.selection.bg_fill = t.selection();
    // egui outlines a focused text field with the selection stroke: DS Field focus border.
    v.selection.stroke = Stroke::new(1.0, t.focus());
    v.text_cursor.stroke = Stroke::new(2.0, t.focus());
    v.warn_fg_color = t.warn;
    v.error_fg_color = t.bad;
    for (w, fill, stroke, fg) in [
        (&mut v.widgets.noninteractive, t.bg, t.line, t.muted),
        (&mut v.widgets.inactive, t.bg, t.line_strong, t.ink),
        (&mut v.widgets.hovered, t.sunken, t.muted, t.ink_strong),
        // Pressed and focused fields: the brand blue outline (DS Field focus).
        (&mut v.widgets.active, t.sunken, t.accent, t.ink_strong),
        (&mut v.widgets.open, t.sunken, t.accent, t.ink_strong),
    ] {
        w.bg_fill = fill;
        w.weak_bg_fill = fill;
        w.bg_stroke = Stroke::new(1.0, stroke);
        // egui's own controls here are text fields, selects, menu rows and checkboxes: the DS's
        // field radius (our buttons draw their own 8).
        w.corner_radius = CornerRadius::same(radius::SM);
        w.fg_stroke = Stroke::new(1.0, fg);
        w.expansion = 0.0;
    }
    v
}

/// Applies both appearances (egui switches between them with the theme preference).
pub fn apply(ctx: &egui::Context, choice: ThemeChoice) {
    ctx.set_visuals_of(egui::Theme::Dark, visuals(&Tokens::DARK));
    ctx.set_visuals_of(egui::Theme::Light, visuals(&Tokens::LIGHT));
    ctx.set_theme(match choice {
        ThemeChoice::System => egui::ThemePreference::System,
        ThemeChoice::Dark => egui::ThemePreference::Dark,
        ThemeChoice::Light => egui::ThemePreference::Light,
    });
    ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        // Between a checkbox (or radio button) and its label.
        style.spacing.icon_spacing = 10.0;
        // DS Button md pads 16 px either side; egui's padding also sizes ComboBoxes and menu rows,
        // so it stays at 12 and [`crate::widgets::button`] pads its own. The height comes from
        // `interact_size`.
        style.spacing.button_padding = egui::vec2(space::S3, 6.0);
        style.spacing.interact_size.y = CONTROL_H;
        // DS Menu list: 8 px of padding.
        style.spacing.menu_margin = egui::Margin::same(space::S2 as i8);
        style.spacing.window_margin = egui::Margin::same(space::S1 as i8);
        // Body stays 14 (DS controls, menus and selects), not the DS's 15 prose size: the
        // launcher's text is almost all control text, and 15 overflows the fixed table columns.
        style.text_styles = [
            (TextStyle::Heading, FontId::new(text::TITLE_3, display())),
            (TextStyle::Body, FontId::new(text::CONTROL, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(text::CONTROL, semibold())),
            (TextStyle::Small, FontId::new(text::META, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(text::BODY_SM, FontFamily::Monospace)),
        ]
        .into();
    });
}

/// The committed Latin-only Pretendard subsets. Removing Hangul from these primary faces makes
/// egui fall through to Wanted Sans instead of silently selecting Pretendard's Korean glyphs.
pub(crate) const PRETENDARD_REGULAR: &[u8] = include_bytes!("../../../../assets/fonts/Pretendard-Regular.ttf");
pub(crate) const PRETENDARD_MEDIUM: &[u8] = include_bytes!("../../../../assets/fonts/Pretendard-Medium.ttf");
pub(crate) const PRETENDARD_SEMIBOLD: &[u8] = include_bytes!("../../../../assets/fonts/Pretendard-SemiBold.ttf");

/// The Korean system face of the Redrob design system (Wanted Sans Regular, SIL OFL 1.1; licence
/// beside the file). The primary Pretendard subsets carry no Hangul, so Korean falls to this face.
pub const KOREAN_FONT: &[u8] = include_bytes!("../../../../assets/fonts/WantedSans-Regular.ttf");

/// Korean system text is set at 96% of the Latin size (design system, `35-korean.md`).
const KOREAN_SCALE: f32 = 0.96;

/// Height of buttons, inputs and filter cells: the DS's default control (`control-h-md`).
pub const CONTROL_H: f32 = control::MD;

/// Labels: the design system's meta line, 12 px Pretendard 500, sentence case, no extra tracking.
pub const HUD_SIZE: f32 = 12.0;
pub const HUD_SPACING: f32 = 0.0;
/// Buttons: the design system's label, Pretendard 600, no extra tracking.
pub const BUTTON_SPACING: f32 = 0.0;

/// Pretendard 500.
pub fn medium() -> FontFamily {
    FontFamily::Name("medium".into())
}

/// Pretendard 600.
pub fn semibold() -> FontFamily {
    FontFamily::Name("semibold".into())
}

/// Headings and app names: Pretendard 600 (the design system's card and dialog title).
pub fn display() -> FontFamily {
    FontFamily::Name("display".into())
}

/// Labels and badges: Pretendard 500 (the design system's meta line). The system uses one
/// typeface; a monospace face is for code only.
pub fn mono() -> FontFamily {
    FontFamily::Name("mono".into())
}

/// Buttons and app number tabs: Pretendard 600 (the design system's label).
pub fn mono_bold() -> FontFamily {
    FontFamily::Name("mono-bold".into())
}

/// A font registered under `name`, and the family it sets.
type Face<'a> = (&'a str, &'static [u8], FontFamily);

/// The Redrob design system's generated Latin-only Pretendard faces and its Korean Wanted Sans
/// face behind them in every family. The subset boundary is intentional: egui chooses the first
/// face claiming a glyph, so retaining Pretendard's source Hangul cmap would bypass Wanted Sans.
pub fn install_fonts(ctx: &egui::Context, cjk: bool) {
    let mut fonts = FontDefinitions::default();
    // egui's own fonts stay behind ours, for symbols and emoji ours don't have.
    let fallback: Vec<String> = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    fonts
        .font_data
        .insert("WantedSans".to_owned(), Arc::new(FontData::from_static(KOREAN_FONT).tweak(FontTweak { scale: KOREAN_SCALE, ..Default::default() })));
    let faces: [Face<'_>; 7] = [
        ("Pretendard", PRETENDARD_REGULAR, FontFamily::Proportional),
        ("Pretendard-Medium", PRETENDARD_MEDIUM, medium()),
        ("Pretendard-SemiBold", PRETENDARD_SEMIBOLD, semibold()),
        ("Pretendard-SemiBold", PRETENDARD_SEMIBOLD, display()),
        ("Pretendard", PRETENDARD_REGULAR, FontFamily::Monospace),
        ("Pretendard-Medium", PRETENDARD_MEDIUM, mono()),
        ("Pretendard-SemiBold", PRETENDARD_SEMIBOLD, mono_bold()),
    ];
    for (name, bytes, family) in faces {
        fonts.font_data.entry(name.to_owned()).or_insert_with(|| Arc::new(FontData::from_static(bytes)));
        let mut stack = vec![name.to_owned(), "WantedSans".to_owned()];
        stack.extend(fallback.iter().cloned());
        fonts.families.insert(family, stack);
    }
    // Chinese, behind everything else, from the system: only when asked for, as the font is large.
    if cjk && let Some(font) = cjk_font() {
        fonts.font_data.insert("CJK".to_owned(), font);
        for stack in fonts.families.values_mut() {
            stack.push("CJK".to_owned());
        }
    }
    ctx.set_fonts(fonts);
}

/// The system's Simplified Chinese font, read once on the first call (tens of MB). Desktop only:
/// the web build has none, so Chinese can't be drawn there.
pub fn cjk_font() -> Option<Arc<FontData>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        CJK_FONT.get_or_init(|| crate::cjk_font::load().map(Arc::new)).clone()
    }
    #[cfg(target_arch = "wasm32")]
    {
        None
    }
}

/// The system's Chinese font once [`cjk_font`] has looked for it.
#[cfg(not(target_arch = "wasm32"))]
static CJK_FONT: std::sync::OnceLock<Option<Arc<FontData>>> = std::sync::OnceLock::new();

/// Has the system's Chinese font already been loaded? Never loads it.
pub fn cjk_font_loaded() -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        CJK_FONT.get().is_some_and(Option::is_some)
    }
    #[cfg(target_arch = "wasm32")]
    {
        false
    }
}

/// Can Chinese be drawn? Loads the system font on the first call ([`cjk_font`]).
pub fn cjk_font_available() -> bool {
    cjk_font().is_some()
}

/// Mixes `c` into the background at `alpha` (0â€“1): tinted washes.
pub fn tint(c: Color32, alpha: f32) -> Color32 {
    let a = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}

pub fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

#[cfg(test)]
mod tests {
    use super::motion::Easing;
    use super::*;

    #[test]
    fn easing_curves_start_at_0_end_at_1_and_settle_overshoots() {
        for e in [Easing::Standard, Easing::Entrance, Easing::Exit, Easing::Settle] {
            assert!(e.apply(0.0).abs() < 1e-3, "{e:?}");
            assert!((e.apply(1.0) - 1.0).abs() < 1e-3, "{e:?}");
        }
        // Standard is fast out of the gate; exit starts slowly; settle passes 1 before it lands.
        assert!(Easing::Standard.apply(0.5) > 0.7);
        assert!(Easing::Exit.apply(0.3) < 0.1);
        assert!((0..=100).map(|i| Easing::Settle.apply(i as f32 / 100.0)).any(|y| y > 1.02));
    }

    #[test]
    fn derived_roles_are_close_to_the_design_system() {
        let near = |c: Color32, rgb: u32| {
            let [_, r, g, b] = rgb.to_be_bytes();
            c.r().abs_diff(r) <= 12 && c.g().abs_diff(g) <= 12 && c.b().abs_diff(b) <= 12
        };
        // surface-raised
        assert!(near(Tokens::LIGHT.raised(), 0xf8f9fb));
        assert!(near(Tokens::DARK.raised(), 0x141719));
        // selection (light)
        assert!(near(Tokens::LIGHT.selection(), 0xd9e6ff));
    }
}
