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
/// update, open or remove it). Empty now that the catalog no longer lists Query and Recall.
pub(crate) const HIDDEN_UNLESS_INSTALLED: &[&str] = &[];

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
        // As a fresh install sees it (nothing installed).
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
    fn every_catalog_app_shows_installed_or_not() {
        for app in &SUITE {
            assert!(shown(app.id, false), "{}", app.id);
            assert!(shown(app.id, true), "{}", app.id);
        }
    }
}
