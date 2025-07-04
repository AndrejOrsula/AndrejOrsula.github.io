use crate::page::Page;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    theme: egui::Theme,
    #[serde(skip)]
    current_page: Page,
    #[serde(skip)]
    pages: rustc_hash::FxHashMap<Page, Box<dyn eframe::App>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            theme: egui::Theme::Dark,
            current_page: Page::default(),
            pages: crate::ENABLED_PAGES
                .into_iter()
                .map(|page| (page, page.default_app()))
                .collect(),
        }
    }
}

impl App {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Enable image loading
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Load the fonts
        crate::style::load_fonts(&cc.egui_ctx);

        // // Enable screen web reader support
        // cc.egui_ctx.options_mut(|o| o.screen_reader = true);

        // Construct the app state
        let app = if let Some(storage) = cc.storage {
            // Try to restore previous state
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            // Otherwise, use default state
            Self::default()
        };

        // Set the theme
        crate::style::set_theme(&cc.egui_ctx, app.theme);

        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Synchronize the page URL and content if the URL contains a hash
        #[cfg(target_arch = "wasm32")]
        if let Some(page) = frame.info().web_info.location.hash.strip_prefix('#') {
            if let Some(page) = crate::ENABLED_PAGES
                .into_iter()
                .find(|x| x.to_string().eq_ignore_ascii_case(page))
            {
                // If a known page was requested, update the current page
                self.current_page = page;

                // If the page is a redirect, redirect to it
                if self.current_page.redirect_page().is_some() {
                    crate::utils::egui::open_url_on_page(ctx, self.current_page, true);
                }
            } else {
                // If an unknown page was requested, update the URL to open the default page
                crate::utils::egui::open_url_on_page(ctx, Page::default(), true);
            }
        } else {
            // Otherwise, update the URL to match the current page
            if self.current_page != Page::default() {
                crate::utils::egui::open_url_on_page(ctx, self.current_page, true);
            }
        }

        // Navigation panel that allows switching between page
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let screen_width = ctx.screen_rect().width();

                // Navigation
                {
                    if screen_width >= self.pages.len() as f32 * 175.0 {
                        // Horizontal menu
                        ui.style_mut().override_text_style =
                            Some(egui::TextStyle::Name("navigation_panel_display".into()));
                        ui.spacing_mut().item_spacing.x = 16.0;
                        self.navigation_buttons(ui);
                    } else {
                        // Vertical menu
                        ui.style_mut().override_text_style =
                            Some(egui::TextStyle::Name("navigation_panel".into()));
                        ui.menu_button("\u{e5d2}", |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                ui.style_mut().override_text_style =
                                    Some(egui::TextStyle::Name("navigation_panel_display".into()));
                                ui.spacing_mut().item_spacing.y = 4.0;
                                self.navigation_buttons(ui);
                            });
                        });
                    }
                }

                // Socials
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if screen_width >= crate::ENABLED_SOCIALS.len() as f32 * 48.0 {
                        // Buttons
                        ui.style_mut().override_text_style =
                            Some(egui::TextStyle::Name("social".into()));
                        ui.spacing_mut().item_spacing.x = 8.0;
                        for social in crate::ENABLED_SOCIALS {
                            social.show(ui);
                        }
                    } else {
                        // Menu
                        ui.style_mut().override_text_style =
                            Some(egui::TextStyle::Name("navigation_panel".into()));
                        ui.menu_button("\u{e80d}", |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                                ui.style_mut().override_text_style =
                                    Some(egui::TextStyle::Name("social".into()));
                                ui.spacing_mut().item_spacing.y = 8.0;
                                for social in crate::ENABLED_SOCIALS.into_iter().rev() {
                                    social.show(ui);
                                }
                            });
                        });
                    }
                });
            });
        });

        // Bottom panel
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                self.dark_mode_toggle_button(ui);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.source_code_button(ui);
                    self.warn_if_debug_build(ui);
                });
            });
        });

        // Update the current page
        for page in crate::ENABLED_PAGES {
            if self.current_page == page || ctx.memory(egui::Memory::everything_is_visible) {
                self.pages
                    .get_mut(&page)
                    .unwrap()
                    .as_mut()
                    .update(ctx, frame);
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(&mut *self)
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl App {
    fn navigation_buttons(&mut self, ui: &mut egui::Ui) {
        for page in crate::ENABLED_PAGES {
            // Add a separator if requested
            if crate::SEPARATE_MENU_AT.contains(&page) {
                ui.separator();
            }

            if self.current_page == page {
                ui.add(egui::Button::new(page.title()))
                    .highlight()
                    .on_hover_text(format!("{} (current page)", page.description()));
            } else {
                let button = ui
                    .add(egui::Button::new(page.title()))
                    .on_hover_text(page.description());
                // If the button is clicked, change the current page
                if button.clicked() {
                    // Change URL to the new page in the same tab
                    crate::utils::egui::open_url_on_page(ui.ctx(), page, true);

                    // Close menu if it is open
                    ui.close_menu();
                } else {
                    // Open URL in a new page if the middle mouse button is clicked
                    if button.middle_clicked() {
                        crate::utils::egui::open_url_on_page(ui.ctx(), page, false);
                    }
                }
            }
        }
    }

    pub fn dark_mode_toggle_button(&mut self, ui: &mut egui::Ui) {
        let (icon, tooltip, target_visuals) = match self.theme {
            egui::Theme::Dark => (
                "\u{e51c}",
                "Switch to light mode",
                crate::style::light_visuals(),
            ),
            egui::Theme::Light => (
                "\u{e518}",
                "Switch to dark mode",
                crate::style::dark_visuals(),
            ),
        };

        if ui
            .add(egui::Button::new(icon))
            .on_hover_text(tooltip)
            .clicked()
        {
            ui.ctx().set_visuals(target_visuals.to_owned());
            self.theme = match self.theme {
                egui::Theme::Dark => egui::Theme::Light,
                egui::Theme::Light => egui::Theme::Dark,
            };
        }
    }

    pub fn source_code_button(&mut self, ui: &mut egui::Ui) {
        let button = ui
            .add(egui::Button::new(
                egui::RichText::new("\u{f323} source \u{e6a1}").font(egui::FontId::monospace(
                    ui.style().text_styles[&egui::TextStyle::Button].size,
                )),
            ))
            .on_hover_text("View the source code on GitHub");
        crate::utils::egui::clickable_url(button, env!("CARGO_PKG_REPOSITORY"));
    }

    pub fn warn_if_debug_build(&mut self, ui: &mut egui::Ui) {
        if cfg!(debug_assertions) {
            let screen_width = ui.ctx().screen_rect().width();
            if screen_width > 360.0 {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add(egui::Button::new("⚠ Debug build ⚠"))
                        .on_hover_ui(|ui| {
                            ui.label(
                                egui::RichText::new(format!(
                                    "Current page: {:?}\n\
                                     Screen size:  {:?}\n\
                                    ",
                                    self.current_page,
                                    ui.ctx().screen_rect().size(),
                                ))
                                .font(egui::FontId::monospace(
                                    ui.style().text_styles[&egui::TextStyle::Button].size,
                                )),
                            );
                        })
                });
            }
        }
    }
}
