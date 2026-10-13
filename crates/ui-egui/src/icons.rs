//! Bundled images: each Redrob app's icon and the launcher's own, all derived from the design
//! system's files in `assets/brand/` by `packaging/brand-assets.sh`.

use egui::ImageSource;

/// The 128 px icon of a Redrob app.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    Some(match id {
        "redrob-cut" => egui::include_image!("../../../assets/app-icons/redrob-cut.png"),
        "redrob-shape" => egui::include_image!("../../../assets/app-icons/redrob-shape.png"),
        "redrob-cad" => egui::include_image!("../../../assets/app-icons/redrob-cad.png"),
        "redrob-canvas" => egui::include_image!("../../../assets/app-icons/redrob-canvas.png"),
        "redrob-design" => egui::include_image!("../../../assets/app-icons/redrob-design.png"),
        "redrob-studio" => egui::include_image!("../../../assets/app-icons/redrob-studio.png"),
        "redrob-browser" => egui::include_image!("../../../assets/app-icons/redrob-browser.png"),
        "redrob-code" => egui::include_image!("../../../assets/app-icons/redrob-code.png"),
        "redrob-cowork" => egui::include_image!("../../../assets/app-icons/redrob-cowork.png"),
        "redrob-office" => egui::include_image!("../../../assets/app-icons/redrob-office.png"),
        "redrob-query" => egui::include_image!("../../../assets/app-icons/redrob-query.png"),
        "redrob-recall" => egui::include_image!("../../../assets/app-icons/redrob-recall.png"),
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

/// The Redrob tile as supplied: the launcher's app icon.
pub fn redrob_mark() -> ImageSource<'static> {
    egui::include_image!("../../../assets/app-icon/hicolor/128x128/apps/ai.redrob.launcher.png")
}
