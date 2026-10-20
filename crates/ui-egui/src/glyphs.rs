// ADDED-LINE-RATIONALE f638cb8a736bd194:1: Supports `glyphs` while preserving the nearby constraint: ! mitres and does not round line ends.
// ADDED-LINE-RATIONALE 1322f00859e8dead:1: Gives `glyphs` direct access to `egui / Color32 / Painter` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE cb31c505d27b261e:1: Supports `glyphs` while preserving the nearby constraint: A drawn icon.
// ADDED-LINE-RATIONALE 7b751d8c702467c9:1: Supports `Icon` while preserving the nearby constraint: [derive(Clone, Copy, Debug, PartialEq, Eq, Hash)].
// ADDED-LINE-RATIONALE 2bd96fe7d8e3a95c:1: Supports `Icon` while preserving the nearby constraint: A gear: Settings.
// ADDED-LINE-RATIONALE bd5a3fe3d2db0eb8:1: Supports `Icon` while preserving the nearby constraint: A circling arrow: check again.
// ADDED-LINE-RATIONALE 32dc756a97791b8f:1: Supports `Icon` while preserving the nearby constraint: A magnifier.
// ADDED-LINE-RATIONALE 76aa1e65b310d570:1: Supports `Icon` while preserving the nearby constraint: A chevron pointing down: a list opens below.
// ADDED-LINE-RATIONALE 4f0320d82d702f87:1: Supports `Icon` while preserving the nearby constraint: A chevron pointing left: back.
// ADDED-LINE-RATIONALE 9d50db5e9cb9ac74:1: Supports `Icon` while preserving the nearby constraint: Three dots: more actions.
// ADDED-LINE-RATIONALE 1b6eaf28b31aa5ed:1: Supports `Icon` while preserving the nearby constraint: A cross: close, dismiss, cancel.
// ADDED-LINE-RATIONALE 0a67b855d009c5e3:1: Supports `Icon` while preserving the nearby constraint: Two bars.
// ADDED-LINE-RATIONALE 29144bc5b0aaa90d:1: Supports `Icon` while preserving the nearby constraint: A triangle.
// ADDED-LINE-RATIONALE ac24c879045d93a1:1: Supports `Icon` while preserving the nearby constraint: A tick.
// ADDED-LINE-RATIONALE 531008e88619513f:1: Supports `Icon` while preserving the nearby constraint: An arrow into a tray.
// ADDED-LINE-RATIONALE a84e1b5459e2c5fa:1: Supports `Icon` while preserving the nearby constraint: An arrow leaving a box: opens in the browser.
// ADDED-LINE-RATIONALE d06d3a20c8ffe803:1: Supports `Icon` while preserving the nearby constraint: Four squares: the apps.
// ADDED-LINE-RATIONALE 3073271701cadab5:1: Supports `Icon` while preserving the nearby constraint: A chip: local models.
// ADDED-LINE-RATIONALE abc45566dcda7745:1: Supports `Icon` while preserving the nearby constraint: A bolt: the engine.
// ADDED-LINE-RATIONALE ad5b00da04b25de0:1: Supports `Icon` while preserving the nearby constraint: A person: the account.
// ADDED-LINE-RATIONALE 615bbcd35bcc8367:1: Supports `Icon` while preserving the nearby constraint: The light appearance.
// ADDED-LINE-RATIONALE 2d70465e5d901ed4:1: Supports `Icon` while preserving the nearby constraint: The dark appearance.
// ADDED-LINE-RATIONALE 7c8f4216290c929f:1: Supports `Icon` while preserving the nearby constraint: A screen: match the system.
// ADDED-LINE-RATIONALE 0bb23f8596c65b48:1: Bounds the `Icon` implementation here so `Monitor` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6df53545cd393993:1: Declares the `cfg / test` contract on `Icon` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE fcb43438f02e8ce1:1: Supports `Icon` while preserving the nearby constraint: [cfg(test)].
// ADDED-LINE-RATIONALE de9943e68857b58e:1: Supports `Icon` while preserving the nearby constraint: Every icon, for the bounds regression below.
// ADDED-LINE-RATIONALE 5499d7e9aeead232:1: Keeps `Self / Settings` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f22053369310e282:1: Keeps `Self / Refresh` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 3752d971099375e1:1: Keeps `Self / Search` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE fe7354f78699523a:1: Keeps `Self / ChevronDown` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE ce1a66f938224081:1: Keeps `Self / ChevronLeft` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE c8df2eb0b2e2162a:1: Keeps `Self / More` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 7f45aa363fca8d93:1: Keeps `Self / Close` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 8b0f404ddc71fab1:1: Keeps `Self / Pause` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 36a181ed9113ab4b:1: Keeps `Self / Play` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE c8850a1ea8debf4f:1: Keeps `Self / Check` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE de1fb55ebc29ce7d:1: Keeps `Self / Download` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE a2b5588daebe1e90:1: Keeps `Self / ExternalLink` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 9e0998cc35b9dced:1: Keeps `Self / Apps` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE e1719fd2b686eceb:1: Keeps `Self / Models` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE ee0899bba9b9341f:1: Keeps `Self / Engine` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE b58fb84aafe4d2ba:1: Keeps `Self / Account` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 442a0ff969af9359:1: Keeps `Self / Sun` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE b03c81b13537a766:1: Keeps `Self / Moon` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 4dfc2cd51c1da031:1: Keeps `Self / Monitor` in `Icon`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 78e74849c2b53e59:1: Bounds the `Icon` implementation here so `Self / Monitor` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0bb23f8596c65b48:2: Bounds the `Icon` implementation here so `Self / Monitor` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3e6570d8711b98b1:1: Supports `Grid` while preserving the nearby constraint: Maps the 24-unit grid onto a square.
// ADDED-LINE-RATIONALE e0e684c125b56657:1: Keeps `min / Pos2` in `Grid`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f746631d4a766ac8:1: Keeps `unit / f32` in `Grid`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE b2a648bb227287c1:1: Keeps `stroke / Stroke` in `Grid`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 46820e68acbb2844:1: Keeps `color / Color32` in `Grid`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 0bb23f8596c65b48:3: Bounds the `Grid` implementation here so `color / Color32` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2d2abb30682cdd18:1: Makes `Grid` an explicit part of `Grid` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE ad1b4fdf5cd79e3f:1: Isolates `p` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE 119d81681a43093a:1: Delegates `p`'s `min / vec2 / x` work to `vec2` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:1: Bounds the `p` implementation here so `min / vec2 / x` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6b04a7cd72f61e8e:1: Isolates `line` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE bac0c7b3964f6fa5:1: Delegates `line`'s `out / push / Shape` work to `out.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:2: Bounds the `line` implementation here so `out / push / Shape` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5934a158fae3c554:1: Isolates `closed` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE de8b4802761765e6:1: Delegates `closed`'s `out / push / Shape` work to `out.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:3: Bounds the `closed` implementation here so `out / push / Shape` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE adebef089bc2ae2e:1: Isolates `rect` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE c812402d753d7e9b:1: Delegates `rect`'s `closed / out / x0` work to `self.closed` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:4: Bounds the `rect` implementation here so `closed / out / x0` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2189bbeaaaf86b93:1: Isolates `fill` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE ab6511fc00c18ffe:1: Delegates `fill`'s `out / push / Shape` work to `out.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:5: Bounds the `fill` implementation here so `out / push / Shape` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6f48d6e458efabe2:1: Isolates `circle` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE 52fbdefda99ac3c4:1: Delegates `circle`'s `out / push / Shape` work to `out.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:6: Bounds the `circle` implementation here so `out / push / Shape` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 86f0d686a559d399:1: Isolates `dot` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE d131ea8d2a8e4757:1: Delegates `dot`'s `out / push / Shape` work to `out.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:7: Bounds the `dot` implementation here so `out / push / Shape` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 95ee0bd5e0d77e46:1: Supports `arc` while preserving the nearby constraint: sweeps clockwise on screen).
// ADDED-LINE-RATIONALE 1566caa7c3e99713:1: Freezes `n` for `arc` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2bdd157acb9a8149:1: Makes `n` an explicit part of `arc` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 396203ce2f98e9a5:1: Delegates `arc`'s `map / i` work to `map` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 14a120e1819ba171:1: Freezes `a` for `arc` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 235f3216dfab1d84:1: Delegates `arc`'s `cx / r / a` work to `a.cos` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 278d35bbe1a44563:1: Makes `cx / r / a` an explicit part of `arc` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE a96e2dd78438f802:1: Delegates `arc`'s `collect` work to `collect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:8: Bounds the `arc` implementation here so `collect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0bb23f8596c65b48:4: Bounds the `arc` implementation here so `collect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d0ba573536f54685:1: Supports `sweep_via` while preserving the nearby constraint: The signed sweep from `from` to `to` that passes through `through` (`want` true) or avoids it.
// ADDED-LINE-RATIONALE 5d8e45a018b0d3a8:1: Freezes `ccw` for `sweep_via` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6b6665601b55cf6b:1: Freezes `contains` for `sweep_via` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 192733739b606263:1: Freezes `if` for `sweep_via` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0bb23f8596c65b48:5: Bounds the `sweep_via` implementation here so `contains / want / ccw` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4134c1a6401d2203:1: Supports `paint` while preserving the nearby constraint: Paints `icon` filling the square centred in `rect`, in `color`.
// ADDED-LINE-RATIONALE f387147344044845:1: Delegates `paint`'s `painter / extend / shapes` work to `painter.extend` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0bb23f8596c65b48:6: Bounds the `paint` implementation here so `painter / extend / shapes` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a9223f8592bbe7ea:1: Supports `shapes` while preserving the nearby constraint: The shapes that draw `icon` in the square centred in `rect`.
// ADDED-LINE-RATIONALE ca2b3cc473b8f613:1: Freezes `side` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 69d241aa63e80efa:1: Freezes `unit` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 30c7e9ea34614639:1: Freezes `square` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE efcf5d1c068c5bea:1: Freezes `g` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 39a74c9ee684f8be:1: Freezes `s` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1662abfb0527d7a2:1: Prevents `shapes` from applying its side effect outside the `icon` state.
// ADDED-LINE-RATIONALE e0655aee4f2f7ecd:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5f8504fe04cdc42e:1: Supports `shapes` while preserving the nearby constraint: Eight square-cut teeth round a ring, a hole in the middle.
// ADDED-LINE-RATIONALE a4aed099e72426bd:1: Applies `shapes` consistently to every `k / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE cd78bff83435f8f3:1: Freezes `base` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 33df007cc58cf74d:1: Freezes `deg` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d43560885a68fb8a:1: Applies `shapes` consistently to every `r / da / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE a104f1aac98f90e4:1: Delegates `shapes`'s `pts / push / r` work to `pts.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b2d5538a6ca07488:1: Bounds the `shapes` implementation here so `pts / push / r` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 996dcf886071f5b8:1: Bounds the `shapes` implementation here so `g / closed / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7e3f6d7fc547693a:1: Delegates `shapes`'s `g / closed / s` work to `g.closed` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 583bc7ee9ef07d69:1: Delegates `shapes`'s `g / circle / s` work to `g.circle` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e911a02005f771e8:1: Bounds the `shapes` implementation here so `g / circle / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1d3e33d13b774097:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 609b6dc498a92be2:1: Supports `shapes` while preserving the nearby constraint: Most of a circle, open at the top right, with an arrowhead where it ends.
// ADDED-LINE-RATIONALE 50bc37f8f5f7a245:1: Freezes `pts` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 224ee0efd9fd9594:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8e76d5894831f1d5:1: Freezes `end` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dc3c2893d39e4786:1: Freezes `let` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b2b940989d0c950b:1: Supports `shapes` while preserving the nearby constraint: The tangent at the end, in the direction of travel.
// ADDED-LINE-RATIONALE 2555aff3ddd253e0:1: Freezes `head` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 747c118ad020a02f:1: Freezes `back` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3eb5355ea7bc82cb:1: Freezes `let` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a0afba82a9c5ef73:1: Makes `ex / head / tx` an explicit part of `shapes` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 278d27bbe1a42d99:1: Bounds the `shapes` implementation here so `ex / head / tx` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2841c7a6e4619cd0:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e911a02005f771e8:2: Bounds the `shapes` implementation here so `g / line / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7e2e4fa0ebcc341e:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a775f6c4a8eb0e39:1: Delegates `shapes`'s `g / circle / s` work to `g.circle` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 10bf5df406b985b3:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e911a02005f771e8:3: Bounds the `shapes` implementation here so `g / line / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 306b40905688e86a:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5bf42b2ed23fc8ac:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f2540a2c7de2ebe9:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7d31d7b34af8f111:1: Applies `shapes` consistently to every `x / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE cc5eac7a740b355f:1: Delegates `shapes`'s `g / dot / s` work to `g.dot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 996dcf886071f5b8:2: Bounds the `shapes` implementation here so `g / dot / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e911a02005f771e8:4: Bounds the `shapes` implementation here so `Icon / Close` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 71c971a79108fcde:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7bf11eed01bcd33f:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 915dba635b3fb5c1:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e911a02005f771e8:5: Bounds the `shapes` implementation here so `g / line / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a06bb9c693f34360:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9fd7a0f9201a5706:1: Delegates `shapes`'s `g / fill / s` work to `g.fill` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d2cedf786154df70:1: Delegates `shapes`'s `g / fill / s` work to `g.fill` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e911a02005f771e8:6: Bounds the `shapes` implementation here so `g / fill / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 87b7110ec920035c:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3e741c3c14bc20c7:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a3b61988fc806cca:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5b6dd15d7b3dfbbf:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bd75aeea9fce3d76:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c2009f44cba4bd5a:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e911a02005f771e8:7: Bounds the `shapes` implementation here so `g / line / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 82bba80afbef9adf:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d9542440e85cb9e6:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a70787438256f878:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bee6a45f8ef30ba0:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e911a02005f771e8:8: Bounds the `shapes` implementation here so `g / line / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 52534173eaad8f4a:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 749ab2f859adabdc:1: Applies `shapes` consistently to every `x / y / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE e0194d740b980e5d:1: Delegates `shapes`'s `g / rect / s` work to `g.rect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 996dcf886071f5b8:3: Bounds the `shapes` implementation here so `g / rect / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e911a02005f771e8:9: Bounds the `shapes` implementation here so `Icon / Models` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 21065474db2433d8:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 45e1382e4a40538d:1: Delegates `shapes`'s `g / rect / s` work to `g.rect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 20410d98bb5eed6a:1: Applies `shapes` consistently to every `v / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 9dbdcb5901426dc3:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 03213b61afd50914:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b11e11723c45aabb:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c55c6ce72aa8dfb6:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 996dcf886071f5b8:4: Bounds the `shapes` implementation here so `g / line / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e911a02005f771e8:10: Bounds the `shapes` implementation here so `Icon / Engine / g` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 19ea805c5a2e5ce8:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 30463eb8488915db:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b89510b82025450b:1: Delegates `shapes`'s `g / circle / s` work to `g.circle` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 946abb646bf4f25b:1: Freezes `shoulders` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9d5fdd009a7188e4:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e911a02005f771e8:11: Bounds the `shapes` implementation here so `g / line / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4ade955600589134:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5b94e6008b1b0050:1: Delegates `shapes`'s `g / circle / s` work to `g.circle` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a4aed099e72426bd:2: Applies `shapes` consistently to every `k / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 66ba96bef033c0bd:1: Freezes `a` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f250bd43c3224d3f:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 996dcf886071f5b8:5: Bounds the `shapes` implementation here so `g / line / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e911a02005f771e8:12: Bounds the `shapes` implementation here so `Icon / Moon` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 23cb43fc649553b1:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7c78ab81a22c6cc7:1: Supports `shapes` while preserving the nearby constraint: A disc with a second one bitten out of its top right.
// ADDED-LINE-RATIONALE 8c739b6847a285f9:1: Freezes `let` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 82e74cd74cd40e9b:1: Freezes `d` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 74cdef876f2ba804:1: Freezes `a` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a4c2fa28454cfdc2:1: Freezes `h` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 82c0ba85369155df:1: Freezes `let` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ca7ddf9ae6070295:1: Freezes `i1` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bc15769032a85ae8:1: Freezes `i2` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4c450a8fe1d4c019:1: Freezes `ang` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE de269ecb8ccf6d7e:1: Freezes `towards_c2` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5112fff0d3f93244:1: Freezes `let` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b2c4e9116e4c6071:1: Freezes `pts` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2c597d842c539734:1: Freezes `let` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8788f835e2c5dd7e:1: Freezes `back` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE eeca7c549d71ed4e:1: Delegates `shapes`'s `pts / extend / back` work to `pts.extend` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ac506cc28815d77d:1: Delegates `shapes`'s `pts / pop` work to `pts.pop` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7e3f6d7fc547693a:2: Delegates `shapes`'s `g / closed / s` work to `g.closed` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e911a02005f771e8:13: Bounds the `shapes` implementation here so `g / closed / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a97451456a5a5464:1: Freezes `Icon` for `shapes` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2e6e18f0827b2c84:1: Delegates `shapes`'s `g / rect / s` work to `g.rect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c0e97a30d570b26:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 052b16e286b1a130:1: Delegates `shapes`'s `g / line / s` work to `g.line` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e911a02005f771e8:14: Bounds the `shapes` implementation here so `g / line / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:9: Bounds the `shapes` implementation here so `s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8c6787cbcae48832:1: Makes `s` an explicit part of `shapes` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 0bb23f8596c65b48:7: Bounds the `shapes` implementation here so `s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 12a7d8d56b8853a6:1: Supports `loader` while preserving the nearby constraint: (1080 ms, 110 ms apart). No spinning ring. `rect` is the loader's box; repaints while shown.
// ADDED-LINE-RATIONALE 66ba5f25f5895db6:1: Freezes `h` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ba65de63ad567815:1: Freezes `w` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2ce1782c0e2dabf4:1: Freezes `gap` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 28490ba3b98191a5:1: Freezes `total` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2f9228b58647b94d:1: Freezes `left` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c59997ac4f792edc:1: Freezes `base` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 449b4a410b4180f6:1: Supports `loader` while preserving the nearby constraint: skewX(-40deg): the top leans right by tan(40) of the height.
// ADDED-LINE-RATIONALE 15ec08e51c3e8447:1: Applies `loader` consistently to every `i / k / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE f573e61805890381:1: Freezes `bh` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1f26908ef5723be4:1: Freezes `bottom` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a4b969a010537391:1: Freezes `top` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2b8e827e964d0327:1: Freezes `x` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e4dd2da66bbe0ed0:1: Freezes `phase` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8086d5d61c9b8cbb:1: Supports `loader` while preserving the nearby constraint: Keyframes: rest at 0 and from 62%, active 16-40%.
// ADDED-LINE-RATIONALE 4531ff9350c3348c:1: Freezes `p` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cc1e7a2163e0a5c6:1: Freezes `p` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 28602f2edcb51052:1: Freezes `p` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 742e6467546d5089:1: Freezes `_` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0064e96a23766989:1: Bounds the `loader` implementation here so `_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e940d23b024ee927:1: Freezes `c` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e15f0bfba24fe15b:1: Freezes `cy` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 71e7973ce8571821:1: Freezes `dx` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 59dd680c451e942a:1: Freezes `pts` for `loader` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7a4bc6decb1a16a1:1: Delegates `loader`'s `painter / add / Shape` work to `painter.add` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:10: Bounds the `loader` implementation here so `painter / add / Shape` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0bb23f8596c65b48:8: Bounds the `loader` implementation here so `painter / add / Shape` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6df53545cd393993:2: Declares the `cfg / test` contract on `loader` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 5239197f40f9d06b:1: Supports `tests` while preserving the nearby constraint: [cfg(test)].
// ADDED-LINE-RATIONALE 1ab688a8450874fc:1: Gives `tests` direct access to `super` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 8f4ae5ba272d9936:1: Supports `tests` while preserving the nearby constraint: does not match itself.
// ADDED-LINE-RATIONALE 458b5fb56b6f14cf:1: Supports `production_ui_draws_no_spinner` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 15f587b7a502f80a:1: Freezes `sources` for `production_ui_draws_no_spinner` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7f9dbf368c04a53e:1: Keeps `account / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE d613734ebe72b1ca:1: Keeps `app / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 220d9a0ba697fb40:1: Keeps `channels / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE fa0da99cf39445f2:1: Keeps `dialogs / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 29423c0bdc9305f8:1: Keeps `engine / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 8d856ed506c5ed20:1: Keeps `glyphs / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 4e1623b68ee06e5c:1: Keeps `models / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 364141b501482ad2:1: Keeps `notes / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE a0680fee80154ab0:1: Keeps `shell / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 0f5b7b9a612d7b00:1: Keeps `views / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 9ef1d2812d4fe94a:1: Keeps `widgets / rs / include_str` in `production_ui_draws_no_spinner`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE fff8696a231a7129:1: Bounds the `production_ui_draws_no_spinner` implementation here so `widgets / rs / include_str` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7f0216e7824a4a4b:1: Freezes `needles` for `production_ui_draws_no_spinner` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5458714690d9701a:1: Applies `production_ui_draws_no_spinner` consistently to every `file / source / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE ce7dad1800e84a59:1: Applies `production_ui_draws_no_spinner` consistently to every `needle / in / needles` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 464ad661d29e3989:1: Makes `production_ui_draws_no_spinner` fail visibly when `assert / source / contains` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 996dcf886071f5b8:6: Bounds the `production_ui_draws_no_spinner` implementation here so `assert / source / contains` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e911a02005f771e8:15: Bounds the `production_ui_draws_no_spinner` implementation here so `assert / source / contains` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:11: Bounds the `production_ui_draws_no_spinner` implementation here so `test` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8f4ae5ba272d9936:2: Declares the `test` contract on `production_ui_draws_no_spinner` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 058c0580ac53b932:1: Supports `every_icon_draws_inside_its_box` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE ef7fd3f0863a2306:1: Freezes `rect` for `every_icon_draws_inside_its_box` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 24e79fbb7a617cf9:1: Applies `every_icon_draws_inside_its_box` consistently to every `icon / in / Icon` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 33867213117ebc55:1: Freezes `shapes` for `every_icon_draws_inside_its_box` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c862f78d89c7ad0e:1: Makes `every_icon_draws_inside_its_box` fail visibly when `assert / shapes / is_empty` violates the behavior this path promises.
// ADDED-LINE-RATIONALE fa7ebe07bf636f5f:1: Applies `every_icon_draws_inside_its_box` consistently to every `s / in / shapes` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 3a7fb499cf944e10:1: Freezes `b` for `every_icon_draws_inside_its_box` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f26632c38a883890:1: Makes `every_icon_draws_inside_its_box` fail visibly when `assert / rect / expand` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 996dcf886071f5b8:7: Bounds the `every_icon_draws_inside_its_box` implementation here so `assert / rect / expand` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e911a02005f771e8:16: Bounds the `every_icon_draws_inside_its_box` implementation here so `assert / rect / expand` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8c6789cbcae48b98:12: Bounds the `every_icon_draws_inside_its_box` implementation here so `assert / rect / expand` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0bb23f8596c65b48:9: Bounds the `every_icon_draws_inside_its_box` implementation here so `assert / rect / expand` cannot leak into the adjacent declaration or control path.
//! The design system's line icons, drawn with egui shapes so no font has to carry them.
//!
//! Every icon is laid out on the DS's 24-unit grid (`src/icons` in `@redrob-labs/ui`) and drawn
//! with its 2-unit stroke, scaled to the size asked for (never thinner than 1 px). Strokes end
//! square and corners are sharp, the DS's "butt caps, miter joins": egui joins polylines with
//! mitres and does not round line ends.

use std::f32::consts::{PI, TAU};

use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, pos2, vec2};

/// A drawn icon.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Icon {
    /// A gear: Settings.
    Settings,
    /// A circling arrow: check again.
    Refresh,
    /// A magnifier.
    Search,
    /// A chevron pointing down: a list opens below.
    ChevronDown,
    /// A chevron pointing left: back.
    ChevronLeft,
    /// Three dots: more actions.
    More,
    /// A cross: close, dismiss, cancel.
    Close,
    /// Two bars.
    Pause,
    /// A triangle.
    Play,
    /// A tick.
    Check,
    /// An arrow into a tray.
    Download,
    /// An arrow leaving a box: opens in the browser.
    ExternalLink,
    /// Four squares: the apps.
    Apps,
    /// A chip: local models.
    Models,
    /// A bolt: the engine.
    Engine,
    /// A person: the account.
    Account,
    /// The light appearance.
    Sun,
    /// The dark appearance.
    Moon,
    /// A screen: match the system.
    Monitor,
}

#[cfg(test)]
impl Icon {
    /// Every icon, for the bounds regression below.
    const ALL: [Self; 19] = [
        Self::Settings,
        Self::Refresh,
        Self::Search,
        Self::ChevronDown,
        Self::ChevronLeft,
        Self::More,
        Self::Close,
        Self::Pause,
        Self::Play,
        Self::Check,
        Self::Download,
        Self::ExternalLink,
        Self::Apps,
        Self::Models,
        Self::Engine,
        Self::Account,
        Self::Sun,
        Self::Moon,
        Self::Monitor,
    ];
}

/// Maps the 24-unit grid onto a square.
struct Grid {
    min: Pos2,
    unit: f32,
    stroke: Stroke,
    color: Color32,
}

impl Grid {
    fn p(&self, x: f32, y: f32) -> Pos2 {
        self.min + vec2(x, y) * self.unit
    }

    fn line(&self, out: &mut Vec<Shape>, pts: &[(f32, f32)]) {
        out.push(Shape::line(pts.iter().map(|&(x, y)| self.p(x, y)).collect(), self.stroke));
    }

    fn closed(&self, out: &mut Vec<Shape>, pts: &[(f32, f32)]) {
        out.push(Shape::closed_line(pts.iter().map(|&(x, y)| self.p(x, y)).collect(), self.stroke));
    }

    fn rect(&self, out: &mut Vec<Shape>, (x0, y0): (f32, f32), (x1, y1): (f32, f32)) {
        self.closed(out, &[(x0, y0), (x1, y0), (x1, y1), (x0, y1)]);
    }

    fn fill(&self, out: &mut Vec<Shape>, pts: &[(f32, f32)]) {
        out.push(Shape::convex_polygon(pts.iter().map(|&(x, y)| self.p(x, y)).collect(), self.color, Stroke::NONE));
    }

    fn circle(&self, out: &mut Vec<Shape>, (x, y): (f32, f32), r: f32) {
        out.push(Shape::circle_stroke(self.p(x, y), r * self.unit, self.stroke));
    }

    fn dot(&self, out: &mut Vec<Shape>, (x, y): (f32, f32), r: f32) {
        out.push(Shape::circle_filled(self.p(x, y), r * self.unit, self.color));
    }

    /// Points along a circle from angle `a0` sweeping `sweep` radians (y down, so positive
    /// sweeps clockwise on screen).
    fn arc(&self, (cx, cy): (f32, f32), r: f32, a0: f32, sweep: f32) -> Vec<(f32, f32)> {
        let n = ((sweep.abs() / TAU) * 48.0).ceil().max(2.0) as usize;
        (0..=n)
            .map(|i| {
                let a = a0 + sweep * i as f32 / n as f32;
                (cx + r * a.cos(), cy + r * a.sin())
            })
            .collect()
    }
}

/// The signed sweep from `from` to `to` that passes through `through` (`want` true) or avoids it.
fn sweep_via(from: f32, to: f32, through: f32, want: bool) -> f32 {
    let ccw = (to - from).rem_euclid(TAU);
    let contains = (through - from).rem_euclid(TAU) < ccw;
    if contains == want { ccw } else { ccw - TAU }
}

/// Paints `icon` filling the square centred in `rect`, in `color`.
pub(crate) fn paint(painter: &Painter, rect: Rect, icon: Icon, color: Color32) {
    painter.extend(shapes(rect, icon, color));
}

/// The shapes that draw `icon` in the square centred in `rect`.
fn shapes(rect: Rect, icon: Icon, color: Color32) -> Vec<Shape> {
    let side = rect.width().min(rect.height());
    let unit = side / 24.0;
    let square = Rect::from_center_size(rect.center(), vec2(side, side));
    let g = Grid { min: square.min, unit, stroke: Stroke::new((2.0 * unit).max(1.0), color), color };
    let mut s = Vec::new();
    match icon {
        Icon::Settings => {
            // Eight square-cut teeth round a ring, a hole in the middle.
            let mut pts = Vec::with_capacity(32);
            for k in 0..8 {
                let base = k as f32 * PI / 4.0;
                let deg = PI / 180.0;
                for (r, da) in [(7.0, -16.5 * deg), (9.5, -9.0 * deg), (9.5, 9.0 * deg), (7.0, 16.5 * deg)] {
                    pts.push((12.0 + r * (base + da).cos(), 12.0 + r * (base + da).sin()));
                }
            }
            g.closed(&mut s, &pts);
            g.circle(&mut s, (12.0, 12.0), 3.0);
        }
        Icon::Refresh => {
            // Most of a circle, open at the top right, with an arrowhead where it ends.
            let (a0, sweep) = (-PI / 4.0 + 0.5, TAU - 1.0);
            let pts = g.arc((12.0, 12.0), 7.5, a0, sweep);
            g.line(&mut s, &pts);
            let end = a0 + sweep;
            let (ex, ey) = (12.0 + 7.5 * end.cos(), 12.0 + 7.5 * end.sin());
            // The tangent at the end, in the direction of travel.
            let (tx, ty) = (-end.sin(), end.cos());
            let head = 3.5;
            let back = |turn: f32| {
                let (c, sn) = (turn.cos(), turn.sin());
                (ex - head * (tx * c - ty * sn), ey - head * (tx * sn + ty * c))
            };
            g.line(&mut s, &[back(PI / 4.0), (ex, ey), back(-PI / 4.0)]);
        }
        Icon::Search => {
            g.circle(&mut s, (10.5, 10.5), 6.5);
            g.line(&mut s, &[(15.2, 15.2), (20.0, 20.0)]);
        }
        Icon::ChevronDown => g.line(&mut s, &[(6.0, 9.0), (12.0, 15.0), (18.0, 9.0)]),
        Icon::ChevronLeft => g.line(&mut s, &[(15.0, 6.0), (9.0, 12.0), (15.0, 18.0)]),
        Icon::More => {
            for x in [5.5, 12.0, 18.5] {
                g.dot(&mut s, (x, 12.0), 1.75);
            }
        }
        Icon::Close => {
            g.line(&mut s, &[(6.0, 6.0), (18.0, 18.0)]);
            g.line(&mut s, &[(18.0, 6.0), (6.0, 18.0)]);
        }
        Icon::Pause => {
            g.fill(&mut s, &[(7.0, 5.0), (10.0, 5.0), (10.0, 19.0), (7.0, 19.0)]);
            g.fill(&mut s, &[(14.0, 5.0), (17.0, 5.0), (17.0, 19.0), (14.0, 19.0)]);
        }
        Icon::Play => g.fill(&mut s, &[(8.0, 5.0), (19.0, 12.0), (8.0, 19.0)]),
        Icon::Check => g.line(&mut s, &[(5.0, 12.5), (10.0, 17.5), (19.0, 7.0)]),
        Icon::Download => {
            g.line(&mut s, &[(12.0, 4.0), (12.0, 15.0)]);
            g.line(&mut s, &[(7.0, 10.0), (12.0, 15.0), (17.0, 10.0)]);
            g.line(&mut s, &[(5.0, 20.0), (19.0, 20.0)]);
        }
        Icon::ExternalLink => {
            g.line(&mut s, &[(18.0, 13.0), (18.0, 19.0), (5.0, 19.0), (5.0, 6.0), (11.0, 6.0)]);
            g.line(&mut s, &[(10.0, 14.0), (19.0, 5.0)]);
            g.line(&mut s, &[(13.0, 5.0), (19.0, 5.0), (19.0, 11.0)]);
        }
        Icon::Apps => {
            for (x, y) in [(4.0, 4.0), (14.0, 4.0), (4.0, 14.0), (14.0, 14.0)] {
                g.rect(&mut s, (x, y), (x + 6.0, y + 6.0));
            }
        }
        Icon::Models => {
            g.rect(&mut s, (7.0, 7.0), (17.0, 17.0));
            for v in [10.0, 14.0] {
                g.line(&mut s, &[(v, 3.0), (v, 7.0)]);
                g.line(&mut s, &[(v, 17.0), (v, 21.0)]);
                g.line(&mut s, &[(3.0, v), (7.0, v)]);
                g.line(&mut s, &[(17.0, v), (21.0, v)]);
            }
        }
        Icon::Engine => g.closed(&mut s, &[(13.0, 3.0), (5.0, 14.0), (11.0, 14.0), (10.0, 21.0), (19.0, 10.0), (13.0, 10.0)]),
        Icon::Account => {
            g.circle(&mut s, (12.0, 8.0), 4.0);
            let shoulders = g.arc((12.0, 21.5), 7.5, PI, PI);
            g.line(&mut s, &shoulders);
        }
        Icon::Sun => {
            g.circle(&mut s, (12.0, 12.0), 4.0);
            for k in 0..8 {
                let a = k as f32 * PI / 4.0;
                g.line(&mut s, &[(12.0 + 7.0 * a.cos(), 12.0 + 7.0 * a.sin()), (12.0 + 9.5 * a.cos(), 12.0 + 9.5 * a.sin())]);
            }
        }
        Icon::Moon => {
            // A disc with a second one bitten out of its top right.
            let (c1, r1, c2, r2) = ((12.0_f32, 12.0_f32), 8.0_f32, (17.0_f32, 8.0_f32), 7.0_f32);
            let (dx, dy) = (c2.0 - c1.0, c2.1 - c1.1);
            let d = (dx * dx + dy * dy).sqrt();
            let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
            let h = (r1 * r1 - a * a).max(0.0).sqrt();
            let (px, py) = (c1.0 + a * dx / d, c1.1 + a * dy / d);
            let i1 = (px - h * dy / d, py + h * dx / d);
            let i2 = (px + h * dy / d, py - h * dx / d);
            let ang = |c: (f32, f32), p: (f32, f32)| (p.1 - c.1).atan2(p.0 - c.0);
            let towards_c2 = dy.atan2(dx);
            let (b1, b2) = (ang(c1, i1), ang(c1, i2));
            let mut pts = g.arc(c1, r1, b1, sweep_via(b1, b2, towards_c2, false));
            let (e2, e1) = (ang(c2, i2), ang(c2, i1));
            let back = g.arc(c2, r2, e2, sweep_via(e2, e1, towards_c2 + PI, true));
            pts.extend(back.into_iter().skip(1));
            pts.pop();
            g.closed(&mut s, &pts);
        }
        Icon::Monitor => {
            g.rect(&mut s, (3.0, 4.0), (21.0, 16.0));
            g.line(&mut s, &[(12.0, 16.0), (12.0, 20.0)]);
            g.line(&mut s, &[(8.0, 20.0), (16.0, 20.0)]);
        }
    }
    s
}

/// The DS Loader: three bars raked at 40 degrees, each passing from `rest` to `active` in turn
/// (1080 ms, 110 ms apart). No spinning ring. `rect` is the loader's box; repaints while shown.
pub(crate) fn loader(painter: &Painter, rect: Rect, rest: Color32, active: Color32, time: f64) {
    let h = rect.height().min(rect.width());
    let w = (0.19 * h).max(2.0);
    let gap = 0.2 * h;
    let total = 3.0 * w + 2.0 * gap;
    let left = rect.center().x - total / 2.0;
    let base = rect.center().y + h / 2.0;
    // skewX(-40deg): the top leans right by tan(40) of the height.
    let lean = (40.0_f32).to_radians().tan();
    for (i, k) in [0.6_f32, 1.0, 0.6].into_iter().enumerate() {
        let bh = h * k;
        let bottom = base - (h - bh) / 2.0;
        let top = bottom - bh;
        let x = left + i as f32 * (w + gap);
        let phase = ((time - i as f64 * 0.110) / 1.080).rem_euclid(1.0) as f32;
        // Keyframes: rest at 0 and from 62%, active 16-40%.
        let on = match phase {
            p if p < 0.16 => p / 0.16,
            p if p < 0.40 => 1.0,
            p if p < 0.62 => 1.0 - (p - 0.40) / 0.22,
            _ => 0.0,
        };
        let c = crate::theme::mix(rest, active, on);
        let cy = rect.center().y;
        let dx = |y: f32| (cy - y) * lean;
        let pts = vec![pos2(x + dx(bottom), bottom), pos2(x + w + dx(bottom), bottom), pos2(x + w + dx(top), top), pos2(x + dx(top), top)];
        painter.add(Shape::convex_polygon(pts, c, Stroke::NONE));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The DS has no spinning ring: every "working" indicator in the UI is the raked Loader
    /// ([`loader`], `widgets::loader_inline`). The needles are built at run time so this file
    /// does not match itself.
    #[test]
    fn production_ui_draws_no_spinner() {
        let sources = [
            ("account.rs", include_str!("account.rs")),
            ("app.rs", include_str!("app.rs")),
            ("channels.rs", include_str!("channels.rs")),
            ("dialogs.rs", include_str!("dialogs.rs")),
            ("engine.rs", include_str!("engine.rs")),
            ("glyphs.rs", include_str!("glyphs.rs")),
            ("models.rs", include_str!("models.rs")),
            ("notes.rs", include_str!("notes.rs")),
            ("shell.rs", include_str!("shell.rs")),
            ("views.rs", include_str!("views.rs")),
            ("widgets.rs", include_str!("widgets.rs")),
        ];
        let needles = [format!(".{}()", "spinner"), format!("{}::", "Spinner")];
        for (file, source) in sources {
            for needle in &needles {
                assert!(!source.contains(needle.as_str()), "{file} draws a spinner ({needle}); use widgets::loader_inline");
            }
        }
    }

    #[test]
    fn every_icon_draws_inside_its_box() {
        let rect = Rect::from_min_size(pos2(100.0, 100.0), vec2(24.0, 24.0));
        for icon in Icon::ALL {
            let shapes = shapes(rect, icon, Color32::WHITE);
            assert!(!shapes.is_empty(), "{icon:?} draws nothing");
            for s in shapes {
                let b = s.visual_bounding_rect();
                assert!(rect.expand(2.0).contains_rect(b), "{icon:?} leaves its box: {b:?}");
            }
        }
    }
}
