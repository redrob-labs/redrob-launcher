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

/// The icon a modal's head can lead with (an app's, or the launcher's), in points square.
const MODAL_ICON: f32 = 40.0;

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

    /// DS Modal head (padding 20 20 0): an optional 40 px icon, the title in DS title-4 (17 / 600,
    /// the DS's dialog-title size) with a meta line under it, and a close icon button (sm 32) when
    /// `closable`. Returns whether it was pressed.
    ///
    /// * `ui` - the modal's Ui.
    /// * `width` - the modal's width ([`Self::modal_width`]).
    /// * `icon` - the app's or launcher's icon to lead with, if any ([`MODAL_ICON`] square).
    /// * `title` - the translated title.
    /// * `subtitle` - a meta line under the title; empty for none.
    /// * `closable` - whether the head carries a close button.
    fn modal_head(ui: &mut egui::Ui, width: f32, icon: Option<egui::ImageSource<'static>>, title: &str, subtitle: &str, closable: bool) -> bool {
        let t = Tokens::get(ui.ctx());
        let mut close = false;
        egui::Frame::new().inner_margin(egui::Margin { left: 20, right: 20, top: 20, bottom: 0 }).show(ui, |ui| {
            ui.set_width(width - 40.0);
            // A row, not a free right-to-left layout: the modal's area has no height to centre in.
            ui.horizontal_top(|ui| {
                let has_icon = icon.is_some();
                if let Some(icon) = icon {
                    ui.add(egui::Image::new(icon).fit_to_exact_size(vec2(MODAL_ICON, MODAL_ICON)));
                    ui.add_space(theme::space::S1);
                }
                let text_w = (ui.available_width() - if closable { theme::control::SM + theme::space::S2 } else { 0.0 }).max(0.0);
                ui.allocate_ui(vec2(text_w, 0.0), |ui| {
                    ui.set_width(text_w);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        let heading = widgets::heading(title, theme::text::TITLE_4, t.ink);
                        // A lone title is centred on the 40 px icon beside it (the 17 px title is
                        // shorter than the icon); a wrapped one, or one with a meta line, starts at
                        // the icon's top.
                        if has_icon && subtitle.is_empty() {
                            let galley = egui::WidgetText::from(heading.clone()).into_galley(
                                ui,
                                Some(egui::TextWrapMode::Wrap),
                                text_w,
                                egui::FontSelection::Default,
                            );
                            ui.add_space(((MODAL_ICON - galley.size().y) / 2.0).max(0.0));
                        }
                        ui.add(egui::Label::new(heading).wrap());
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

    /// DS Modal footer, on the modal's own ground (no hairline, no raised band), so head, body and
    /// footer read as one surface: right aligned, padding 0 20 20 (the body's 20 px bottom padding
    /// is the gap above it). `add` runs right to left, so the first thing added is the rightmost
    /// (the main action).
    ///
    /// * `ui` - the modal's Ui.
    /// * `width` - the modal's width ([`Self::modal_width`]).
    /// * `add` - adds the footer's controls, right to left.
    fn modal_foot(ui: &mut egui::Ui, width: f32, add: impl FnOnce(&mut egui::Ui)) {
        egui::Frame::new().inner_margin(egui::Margin { left: 20, right: 20, top: 0, bottom: 20 }).show(ui, |ui| {
            ui.set_width(width - 40.0);
            ui.horizontal(|ui| ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), add));
        });
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
