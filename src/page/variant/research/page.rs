use include_dir::{include_dir, Dir};

use super::{Bibliography, BibliographyConfig};

const PAGE: crate::page::Page = crate::page::Page::Research;

#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct ResearchPageConfig {
    /// Font size for the publication heading
    #[builder(default = 36.0)]
    pub publication_font_size: f32,
    /// Configuration for the bibliography
    #[builder(default)]
    pub bibliography_cfg: BibliographyConfig,
}

impl Default for ResearchPageConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[must_use = "You should call .update()"]
pub struct ResearchPage {
    pub cfg: ResearchPageConfig,
    bibliography: Bibliography,
    commonmark_cache: egui_commonmark::CommonMarkCache,
}

impl Default for ResearchPage {
    fn default() -> Self {
        static BIB: &str = crate::macros::include_content_str!("publications/bibliography.bib");
        static THUMBNAILS: Dir =
            include_dir!("$CARGO_MANIFEST_DIR/content/publications/thumbnails");
        Self {
            cfg: Default::default(),
            bibliography: Bibliography::parse(BIB, Some(&THUMBNAILS)),
            commonmark_cache: Default::default(),
        }
    }
}

impl eframe::App for ResearchPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::utils::egui::ScrollableFramedCentralPanel::default().show(ctx, |ui| {
            crate::utils::egui::centered_strong_heading(ui, PAGE.title());
            egui_commonmark::commonmark_str!(
                "research",
                ui,
                &mut self.commonmark_cache,
                "content/research.md"
            );

            self.show_publications(ui);
        });
    }
}

impl ResearchPage {
    fn show_publications(&self, ui: &mut egui::Ui) {
        crate::utils::egui::centered_strong_heading_sized(
            ui,
            "Publications",
            self.cfg.publication_font_size,
        );
        self.bibliography.show(ui);
    }
}
