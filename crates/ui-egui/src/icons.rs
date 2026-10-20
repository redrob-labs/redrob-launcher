//! Bundled images: each Redrob app's icon and the launcher's own, all derived from the design
//! system's files in `assets/brand/` by `packaging/brand-assets.sh`. An app's icon is replaced by
//! the one its latest release ships (`Event::Icon`, from the release manifest) once the backend
//! has it, so the catalog shows the product as it looks today, not as it looked at build time.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use egui::ImageSource;

/// One fetched icon: a hash of the PNG bytes (it goes into the image URI, so egui reloads the
/// image when the bytes change) and the bytes.
type Fetched = (u64, Arc<[u8]>);

/// Icons received from the backend, by app id.
static FETCHED: Mutex<Option<HashMap<String, Fetched>>> = Mutex::new(None);

/// Shows `png` as `id`'s icon from now on. Returns whether that is a change.
pub fn set_fetched(id: &str, png: Vec<u8>) -> bool {
    let hash = {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        png.hash(&mut h);
        h.finish()
    };
    let mut map = FETCHED.lock().unwrap_or_else(PoisonError::into_inner);
    let map = map.get_or_insert_with(HashMap::new);
    if map.get(id).is_some_and(|(h, _)| *h == hash) {
        return false;
    }
    map.insert(id.to_string(), (hash, Arc::from(png)));
    true
}

fn fetched(id: &str) -> Option<ImageSource<'static>> {
    let map = FETCHED.lock().unwrap_or_else(PoisonError::into_inner);
    let (hash, bytes) = map.as_ref()?.get(id)?;
    Some(ImageSource::Bytes {
        uri: format!("bytes://redrob-launcher/icons/{id}-{hash:016x}.png").into(),
        bytes: egui::load::Bytes::Shared(bytes.clone()),
    })
}

/// The 128 px icon of a Redrob app: the one its latest release ships when the backend has fetched
/// it, else the bundled one.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    if let Some(icon) = fetched(id) {
        return Some(icon);
    }
    Some(match id {
        "redrob-film" => egui::include_image!("../../../assets/app-icons/redrob-film.png"),
        "redrob-shape" => egui::include_image!("../../../assets/app-icons/redrob-shape.png"),
        "redrob-cad" => egui::include_image!("../../../assets/app-icons/redrob-cad.png"),
        "redrob-canvas" => egui::include_image!("../../../assets/app-icons/redrob-canvas.png"),
        "redrob-design" => egui::include_image!("../../../assets/app-icons/redrob-design.png"),
        "redrob-browser" => egui::include_image!("../../../assets/app-icons/redrob-browser.png"),
        "redrob-code" => egui::include_image!("../../../assets/app-icons/redrob-code.png"),
        "redrob-cowork" => egui::include_image!("../../../assets/app-icons/redrob-cowork.png"),
        "redrob-office" => egui::include_image!("../../../assets/app-icons/redrob-office.png"),
        _ => return None,
    })
}

/// The Redrob lockup, the variant for the ground: dark wordmark on light, white wordmark on dark.
/// Never tinted (design system `Logos/README.md`).
pub fn redrob_logo(dark_background: bool) -> ImageSource<'static> {
    if dark_background {
        egui::include_image!("../../../assets/app-icon/redrob-logo-dark.png")
    } else {
        egui::include_image!("../../../assets/app-icon/redrob-logo-light.png")
    }
}

/// The lockup's width / height (both variants are 320 × 64).
pub const REDROB_LOGO_ASPECT: f32 = 320.0 / 64.0;

/// The Redrob group icon (2026) as supplied: the launcher's app icon.
pub fn redrob_mark() -> ImageSource<'static> {
    egui::include_image!("../../../assets/app-icon/hicolor/128x128/apps/ai.redrob.launcher.png")
}
