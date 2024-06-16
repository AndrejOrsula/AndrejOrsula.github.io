#[allow(unused)]
const PAGE: crate::page::Page = crate::page::Page::About;

#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct AboutPageConfig {
    /// Maximum width of the content
    #[builder(default = 768.0)]
    pub max_content_width: f32,
    /// Size of the profile picture
    #[builder(default = 208.0)]
    pub profile_picture_size: f32,
    /// Font size for the author's name
    #[builder(default = 56.0)]
    pub author_name_font_size: f32,
    /// Font size for the author's title
    #[builder(default = 24.0)]
    pub author_title_font_size: f32,
    /// Minimum space from the bottom of the page when centering vertically
    #[builder(default = 16.0)]
    pub centering_min_bottom: f32,
    /// Initial estimate for the height of the content when centering vertically
    #[builder(default = 424.0)]
    pub centering_initial_content_height_estimate: f32,
    /// Height of the navigation buttons
    #[cfg(target_arch = "wasm32")]
    #[builder(default = 80.0)]
    pub navigation_button_height: f32,
    /// Maximum width of the navigation buttons
    /// Lower values will result in more columns
    #[cfg(target_arch = "wasm32")]
    #[builder(default = 256.0)]
    pub max_button_width: f32,
    /// Spacing between the navigation buttons
    #[cfg(target_arch = "wasm32")]
    #[builder(default = 3.0)]
    pub button_spacing: f32,
}

impl Default for AboutPageConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[must_use = "You should call .update()"]
pub struct AboutPage {
    pub cfg: AboutPageConfig,
    space_to_center_vertically: f32,
    is_first_render: bool,
    commonmark_cache: egui_commonmark::CommonMarkCache,
}

impl Default for AboutPage {
    fn default() -> Self {
        Self {
            cfg: Default::default(),
            space_to_center_vertically: 0.0,
            is_first_render: true,
            commonmark_cache: Default::default(),
        }
    }
}

impl eframe::App for AboutPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::utils::egui::ScrollableFramedCentralPanel::builder()
            .max_content_width(self.cfg.max_content_width)
            .build()
            .show(ctx, |ui| {
                self.maybe_init_centering_space(ui);
                let used_page_rect = self.show(ui);
                self.update_centering_space(ui, used_page_rect.height());
            });
    }
}

impl AboutPage {
    fn show(&mut self, ui: &mut egui::Ui) -> egui::Rect {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(self.space_to_center_vertically);

            crate::utils::egui::strong_heading_sized(
                ui,
                crate::AUTHOR_NAME_FULL,
                self.cfg.author_name_font_size,
            );
            ui.label(
                egui::RichText::new(crate::AUTHOR_TITLE)
                    .weak()
                    .size(self.cfg.author_title_font_size),
            );

            ui.add_space(2.0 * ui.spacing().item_spacing.y);

            if ui.ctx().screen_rect().width() >= self.cfg.max_content_width {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                        ui.add_space(2.0 * ui.spacing().item_spacing.y);
                        self.show_profile_picture(ui, 0.2);
                    });

                    self.show_bio(ui);
                });
            } else {
                ui.add_space(6.0 * ui.spacing().item_spacing.y);
                self.show_profile_picture(ui, 0.5);
                self.show_bio(ui);
            }

            // Extra navigation buttons for the web version
            #[cfg(target_arch = "wasm32")]
            self.show_extra_navigation_buttons(ui);

            // TODO: Add News timeline
        })
        .response
        .rect
    }

    fn show_profile_picture(&mut self, ui: &mut egui::Ui, rounding_factor: f32) -> egui::Response {
        ui.add(
            egui::Image::new(crate::macros::include_content_image!("images/profile.png"))
                .fit_to_exact_size(egui::vec2(
                    self.cfg.profile_picture_size,
                    self.cfg.profile_picture_size,
                ))
                .rounding(rounding_factor * self.cfg.profile_picture_size),
        )
    }

    fn show_bio(&mut self, ui: &mut egui::Ui) -> egui::InnerResponse<()> {
        egui_commonmark::commonmark_str!("bio", ui, &mut self.commonmark_cache, "content/bio.md")
    }

    #[cfg(target_arch = "wasm32")]
    fn show_extra_navigation_buttons(&mut self, ui: &mut egui::Ui) {
        if crate::ENABLED_PAGES.len() > 1 {
            ui.add_space(4.0 * ui.spacing().item_spacing.y);
            egui::Grid::new("extra_nav_buttons")
                .spacing(egui::vec2(self.cfg.button_spacing, self.cfg.button_spacing))
                .show(ui, |ui| {
                    let screen_width = ui
                        .ctx()
                        .screen_rect()
                        .width()
                        .min(self.cfg.max_content_width);
                    let mut n_columns = ((screen_width / self.cfg.max_button_width).floor()
                        as usize)
                        .max(1)
                        .min(crate::ENABLED_PAGES.len() - 1);
                    loop {
                        if (crate::ENABLED_PAGES.len() - 1) % n_columns == 0 {
                            break;
                        }
                        n_columns -= 1;
                    }
                    let button_width = (screen_width
                        - self.cfg.button_spacing * ui.spacing().button_padding.x)
                        / n_columns as f32
                        - ui.spacing().button_padding.x;

                    ui.style_mut().override_text_style =
                        Some(egui::TextStyle::Name("navigation_panel_display".into()));
                    crate::ENABLED_PAGES
                        .into_iter()
                        .filter(|&page| page != PAGE)
                        .enumerate()
                        .for_each(|(i, page)| {
                            let button = ui
                                .add(
                                    egui::Button::new(page.title())
                                        .frame(true)
                                        .rounding(self.cfg.button_spacing)
                                        .min_size(egui::Vec2::new(
                                            button_width,
                                            self.cfg.navigation_button_height,
                                        )),
                                )
                                .on_hover_text(page.description());
                            if button.clicked() {
                                crate::utils::egui::open_url_on_page(ui.ctx(), page, true);
                            } else if button.middle_clicked() {
                                crate::utils::egui::open_url_on_page(ui.ctx(), page, false);
                            }

                            if (i + 1) % n_columns == 0 {
                                ui.end_row();
                            }
                        });
                });
        }
    }

    fn update_centering_space(&mut self, ui: &mut egui::Ui, used_page_height: f32) {
        let content_height = used_page_height - self.space_to_center_vertically;
        let empty_height = ui.ctx().available_rect().height() - content_height;
        self.space_to_center_vertically =
            (0.5 * empty_height - self.cfg.centering_min_bottom - ui.spacing().item_spacing.y)
                .floor()
                .max(0.0);
    }

    fn maybe_init_centering_space(&mut self, ui: &mut egui::Ui) {
        if self.is_first_render {
            self.is_first_render = false;
            if ui.ctx().screen_rect().width() >= self.cfg.max_content_width {
                self.update_centering_space(ui, self.cfg.centering_initial_content_height_estimate);
            }
        }
    }
}
