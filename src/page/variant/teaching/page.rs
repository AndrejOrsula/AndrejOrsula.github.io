const PAGE: crate::page::Page = crate::page::Page::Teaching;

#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct TeachingPageConfig {
    /// Maximum width of the content
    #[builder(default = 1024.0)]
    pub max_content_width: f32,
    /// Font size for the name of the course
    #[builder(default = 32.0)]
    pub course_heading_font_size: f32,
}

impl Default for TeachingPageConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[must_use = "You should call .update()"]
#[derive(Default)]
pub struct TeachingPage {
    pub cfg: TeachingPageConfig,
    commonmark_cache: egui_commonmark::CommonMarkCache,
}

impl eframe::App for TeachingPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::utils::egui::ScrollableFramedCentralPanel::builder()
            .max_content_width(self.cfg.max_content_width)
            .build()
            .show(ctx, |ui| {
                crate::utils::egui::centered_strong_heading(ui, PAGE.title());
                ui.add_space(6.0 * ui.spacing().item_spacing.y);
                self.show_lu(ui);
            });
    }
}

impl TeachingPage {
    fn show_lu(&mut self, ui: &mut egui::Ui) {
        crate::utils::egui::heading_sized(
            ui,
            "2022 – Present",
            0.8 * self.cfg.course_heading_font_size,
        );
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            crate::utils::egui::strong_heading_sized(
                ui,
                "Robotic Manipulation in Space",
                self.cfg.course_heading_font_size,
            );

            // Separator before the copy button
            ui.add(
                egui::Label::new(
                    egui::RichText::new("|")
                        .size(self.cfg.course_heading_font_size)
                        .weak(),
                )
                .selectable(false),
            );

            ui.style_mut().override_text_style = Some(egui::TextStyle::Name("social".into()));
            let button = ui
                .add(egui::Button::new(
                    egui::RichText::new("\u{f09b}").size(1.2 * self.cfg.course_heading_font_size),
                ))
                .on_hover_text("Repository");
            crate::utils::egui::clickable_url(
                button,
                "https://github.com/snt-spacer/phantomx_pincher",
            );
        });

        ui.add_space(4.0 * ui.spacing().item_spacing.y);

        let image_size = self
            .cfg
            .max_content_width
            .min(ui.ctx().available_rect().width())
            - 2.0 * ui.spacing().item_spacing.x;
        ui.add(
            egui::Image::new(crate::macros::include_content_image!(
                "teaching/images/rmins.png"
            ))
            .fit_to_exact_size(egui::vec2(image_size, image_size)),
        );

        egui_commonmark::commonmark_str!(
            ui,
            &mut self.commonmark_cache,
            "content/teaching/rmins.md"
        );
    }
}
