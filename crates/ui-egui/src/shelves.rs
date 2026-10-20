// ADDED-LINE-RATIONALE 902b6586d4bfef58:1: Supports `shelves` while preserving the nearby constraint: ! the backend still know every app, so a hidden app is still detected, checked and updated.
// ADDED-LINE-RATIONALE 80fad0767d4bbd37:1: Supports `shelves` while preserving the nearby constraint: A shelf on the Apps page.
// ADDED-LINE-RATIONALE 886a1beae25140c1:1: Supports `Shelf` while preserving the nearby constraint: [derive(Clone, Copy, Debug, PartialEq, Eq)].
// ADDED-LINE-RATIONALE 1bb7bede4e88320a:1: Supports `Shelf` while preserving the nearby constraint: Redrob Code: the AI engine every other app runs its AI through.
// ADDED-LINE-RATIONALE e4513d7ed39138a3:1: Supports `Shelf` while preserving the nearby constraint: Apps for everyday work.
// ADDED-LINE-RATIONALE 3279b52734feb3db:1: Supports `Shelf` while preserving the nearby constraint: Apps that make things.
// ADDED-LINE-RATIONALE fa6d3f9b41cf1377:1: Bounds the `Shelf` implementation here so `Creative` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ab64582d8dd3b4af:1: Makes `Shelf` an explicit part of `Shelf` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 47ad3993ee36da39:1: Supports `Shelf` while preserving the nearby constraint: The shelves, top to bottom.
// ADDED-LINE-RATIONALE 09f9afcbc73e62b7:1: Supports `placed` while preserving the nearby constraint: listed here still shows: it goes to the end of the shelf its group maps to ([`shelf_of`]).
// ADDED-LINE-RATIONALE 53f106689126b1e4:1: Prevents `placed` from applying its side effect outside the `placed / str` state.
// ADDED-LINE-RATIONALE 8e7752610b513451:1: Freezes `Shelf` for `placed` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6179dfda98c71bb2:1: Freezes `Shelf` for `placed` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bf4cd2228e4d9c16:1: Freezes `Shelf` for `placed` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e7b2def072969377:1: Bounds the `placed` implementation here so `Shelf / Creative / redrob` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e750bd8f8a14d377:1: Bounds the `placed` implementation here so `Shelf / Creative / redrob` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE fa6d3f9b41cf1377:2: Bounds the `placed` implementation here so `Apps / kept / off` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 45ffe4ee05b51ab8:1: Supports `placed` while preserving the nearby constraint: update, open or remove it).
// ADDED-LINE-RATIONALE fa0970ecd8eb554d:1: Supports `shown` while preserving the nearby constraint: Whether an app shows on the page and in its counts.
// ADDED-LINE-RATIONALE 5fb943d33ee4527a:1: Delegates `shown`'s `installed / HIDDEN_UNLESS_INSTALLED / contains` work to `HIDDEN_UNLESS_INSTALLED.contains` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fa6d3f9b41cf1377:3: Bounds the `shown` implementation here so `installed / HIDDEN_UNLESS_INSTALLED / contains` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 946f3ef35e11fe82:1: Supports `shelf_of` while preserving the nearby constraint: The shelf an app sits on: where it is placed, else by its catalog group.
// ADDED-LINE-RATIONALE 99d01e53213796e6:1: Delegates `shelf_of`'s `Shelf / ALL / into_iter` work to `Shelf::ALL.into_iter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 653b1429de8bef80:1: Freezes `AppGroup` for `shelf_of` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3a893338a9d3d58d:1: Freezes `AppGroup` for `shelf_of` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 230578e7a16328ba:1: Makes `AppGroup / Work / Shelf` an explicit part of `shelf_of` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE fa6d3f9b41cf1377:4: Bounds the `shelf_of` implementation here so `AppGroup / Work / Shelf` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b2989f2b7b697218:1: Supports `page_order` while preserving the nearby constraint: then any others on that shelf in catalog order. Hidden apps are included; [`shown`] decides.
// ADDED-LINE-RATIONALE ea4d5a285907fef9:1: Freezes `out` for `page_order` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7719934c91b14368:1: Applies `page_order` consistently to every `shelf / in / Shelf` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 8de1ac694413e654:1: Freezes `out` for `page_order` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6acca13f14ae324a:1: Freezes `out` for `page_order` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e750bd8f8a14d377:2: Bounds the `page_order` implementation here so `out / extend / SUITE` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 83201d97954abc14:1: Makes `out` an explicit part of `page_order` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE fa6d3f9b41cf1377:5: Bounds the `page_order` implementation here so `out` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE be30f14ef2afd682:1: Declares the `cfg / test` contract on `page_order` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE d47452a4926bcba8:1: Supports `tests` while preserving the nearby constraint: [cfg(test)].
// ADDED-LINE-RATIONALE 7182ac246d8c7e9b:1: Gives `tests` direct access to `super` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 42a46ac8c8b15359:1: Declares the `test` contract on `tests` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE c7c5a332e55c79b4:1: Supports `page_order_is_the_three_shelves_and_keeps_every_catalog_app` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE b82626b9caccf046:1: Freezes `order` for `page_order_is_the_three_shelves_and_keeps_every_catalog_app` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5240e502b4e3aa91:1: Supports `page_order_is_the_three_shelves_and_keeps_every_catalog_app` while preserving the nearby constraint: As a fresh install sees it (nothing installed, so Query and Recall are filtered out).
// ADDED-LINE-RATIONALE 66941f41aa618e6c:1: Makes `page_order_is_the_three_shelves_and_keeps_every_catalog_app` fail visibly when `assert_eq` violates the behavior this path promises.
// ADDED-LINE-RATIONALE ce14f92fe90cafa8:1: Keeps `fresh` in `page_order_is_the_three_shelves_and_keeps_every_catalog_app`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 980488f02d1419b1:1: Declares the `fresh` contract on `page_order_is_the_three_shelves_and_keeps_every_catalog_app` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE c6230b2065c95cac:1: Keeps `redrob / code` in `page_order_is_the_three_shelves_and_keeps_every_catalog_app`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 7898f5ce5c1aa8c6:1: Keeps `redrob / cowork` in `page_order_is_the_three_shelves_and_keeps_every_catalog_app`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f5ba9f200f93ae29:1: Keeps `redrob / office` in `page_order_is_the_three_shelves_and_keeps_every_catalog_app`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 12dbf698d8fed187:1: Keeps `redrob / browser` in `page_order_is_the_three_shelves_and_keeps_every_catalog_app`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 005e2871fcc0b61f:1: Keeps `redrob / design` in `page_order_is_the_three_shelves_and_keeps_every_catalog_app`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 0f30a7f087a87fe1:1: Keeps `redrob / canvas` in `page_order_is_the_three_shelves_and_keeps_every_catalog_app`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 2f8e3f5fe750538b:1: Keeps `redrob / film` in `page_order_is_the_three_shelves_and_keeps_every_catalog_app`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE bd194ad67eb56252:1: Keeps `redrob / shape` in `page_order_is_the_three_shelves_and_keeps_every_catalog_app`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE e9df4303551dd7b5:1: Makes `redrob / cad` an explicit part of `page_order_is_the_three_shelves_and_keeps_every_catalog_app` because the shared egui interface renders and responds consistently on desktop and web.
// ADDED-LINE-RATIONALE 98048af02d141d17:1: Bounds the `page_order_is_the_three_shelves_and_keeps_every_catalog_app` implementation here so `redrob / cad` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4c869692b6b80be0:1: Bounds the `page_order_is_the_three_shelves_and_keeps_every_catalog_app` implementation here so `Nothing / the / catalog` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 65521d1a22542a10:1: Supports `page_order_is_the_three_shelves_and_keeps_every_catalog_app` while preserving the nearby constraint: Nothing the catalog knows is dropped (the hidden ones are only filtered by `shown`).
// ADDED-LINE-RATIONALE 68cc0cc83772f3da:1: Applies `page_order_is_the_three_shelves_and_keeps_every_catalog_app` consistently to every `app / in / SUITE` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 710cae8a01bb024f:1: Makes `page_order_is_the_three_shelves_and_keeps_every_catalog_app` fail visibly when `assert / order / contains` violates the behavior this path promises.
// ADDED-LINE-RATIONALE e7b2def072969377:2: Bounds the `page_order_is_the_three_shelves_and_keeps_every_catalog_app` implementation here so `assert / order / contains` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e750bd8f8a14d377:3: Bounds the `page_order_is_the_three_shelves_and_keeps_every_catalog_app` implementation here so `assert / order / contains` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 42a46ac8c8b15359:2: Declares the `test` contract on `page_order_is_the_three_shelves_and_keeps_every_catalog_app` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE d11269882e41f4d9:1: Supports `query_and_recall_show_only_when_installed` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE e0ebbb1484913c71:1: Applies `query_and_recall_show_only_when_installed` consistently to every `id / in / redrob` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 05b9e342a5617513:1: Makes `query_and_recall_show_only_when_installed` fail visibly when `assert / shown / id` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 2aa6ee851f45a0b1:1: Makes `query_and_recall_show_only_when_installed` fail visibly when `assert / shown / id` violates the behavior this path promises.
// ADDED-LINE-RATIONALE e7b2def072969377:3: Bounds the `query_and_recall_show_only_when_installed` implementation here so `assert / shown / id` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 06663560feff8436:1: Makes `query_and_recall_show_only_when_installed` fail visibly when `assert / shown / redrob` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 7982efea4f5d5031:1: Supports `query_and_recall_show_only_when_installed` while preserving the nearby constraint: Installed, they sit on the work shelf (their catalog group).
// ADDED-LINE-RATIONALE 6b5d038693033648:1: Makes `query_and_recall_show_only_when_installed` fail visibly when `assert_eq / shelf_of / query` violates the behavior this path promises.
// ADDED-LINE-RATIONALE e750bd8f8a14d377:4: Bounds the `query_and_recall_show_only_when_installed` implementation here so `assert_eq / shelf_of / query` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE fa6d3f9b41cf1377:6: Bounds the `query_and_recall_show_only_when_installed` implementation here so `assert_eq / shelf_of / query` cannot leak into the adjacent declaration or control path.
//! How the apps are laid out on the Apps page: which shelf each sits on, in what order, and which
//! stay out of sight. This is presentation only: the catalog (`redrob_launcher_core::SUITE`) and
//! the backend still know every app, so a hidden app is still detected, checked and updated.

use redrob_launcher_core::{AppGroup, CraftApp, SUITE};

/// A shelf on the Apps page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Shelf {
    /// Redrob Code: the AI engine every other app runs its AI through.
    Engine,
    /// Apps for everyday work.
    Work,
    /// Apps that make things.
    Creative,
}

impl Shelf {
    /// The shelves, top to bottom.
    pub(crate) const ALL: [Shelf; 3] = [Shelf::Engine, Shelf::Work, Shelf::Creative];

    /// The apps placed on this shelf, in order. An app the catalog gains later and that is not
    /// listed here still shows: it goes to the end of the shelf its group maps to ([`shelf_of`]).
    fn placed(self) -> &'static [&'static str] {
        match self {
            Shelf::Engine => &["redrob-code"],
            Shelf::Work => &["redrob-cowork", "redrob-office", "redrob-browser"],
            Shelf::Creative => &["redrob-design", "redrob-canvas", "redrob-film", "redrob-shape", "redrob-cad"],
        }
    }
}

/// Apps kept off the page unless they are already installed (so someone who has one can still
/// update, open or remove it).
pub(crate) const HIDDEN_UNLESS_INSTALLED: &[&str] = &["redrob-query", "redrob-recall"];

/// Whether an app shows on the page and in its counts.
pub(crate) fn shown(id: &str, installed: bool) -> bool {
    installed || !HIDDEN_UNLESS_INSTALLED.contains(&id)
}

/// The shelf an app sits on: where it is placed, else by its catalog group.
pub(crate) fn shelf_of(app: &CraftApp) -> Shelf {
    Shelf::ALL.into_iter().find(|s| s.placed().contains(&app.id)).unwrap_or(match app.group {
        AppGroup::Create => Shelf::Creative,
        AppGroup::Work => Shelf::Work,
    })
}

/// Every catalog app id in page order: shelf by shelf, placed apps first in their listed order,
/// then any others on that shelf in catalog order. Hidden apps are included; [`shown`] decides.
pub(crate) fn page_order() -> Vec<&'static str> {
    let mut out = Vec::with_capacity(SUITE.len());
    for shelf in Shelf::ALL {
        out.extend(shelf.placed().iter().copied().filter(|id| SUITE.iter().any(|a| a.id == *id)));
        out.extend(SUITE.iter().filter(|a| shelf_of(a) == shelf && !shelf.placed().contains(&a.id)).map(|a| a.id));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_order_is_the_three_shelves_and_keeps_every_catalog_app() {
        let order = page_order();
        // As a fresh install sees it (nothing installed, so Query and Recall are filtered out).
        let fresh: Vec<_> = order.iter().copied().filter(|id| shown(id, false)).collect();
        assert_eq!(
            fresh,
            [
                "redrob-code",
                "redrob-cowork",
                "redrob-office",
                "redrob-browser",
                "redrob-design",
                "redrob-canvas",
                "redrob-film",
                "redrob-shape",
                "redrob-cad"
            ]
        );
        // Nothing the catalog knows is dropped (the hidden ones are only filtered by `shown`).
        assert_eq!(order.len(), SUITE.len());
        for app in &SUITE {
            assert!(order.contains(&app.id), "{} missing from the page order", app.id);
        }
    }

    #[test]
    fn query_and_recall_show_only_when_installed() {
        for id in ["redrob-query", "redrob-recall"] {
            assert!(!shown(id, false), "{id}");
            assert!(shown(id, true), "{id}");
        }
        assert!(shown("redrob-code", false));
        // Installed, they sit on the work shelf (their catalog group).
        let query = CraftApp::by_id("redrob-query").unwrap();
        assert_eq!(shelf_of(query), Shelf::Work);
    }
}
