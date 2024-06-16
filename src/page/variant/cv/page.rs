#![allow(dead_code)]

const PAGE: crate::page::Page = crate::page::Page::Cv;

#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct CvPageConfig {}

impl Default for CvPageConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[must_use = "You should call .update()"]
#[derive(Default)]
pub struct CvPage {
    pub cfg: CvPageConfig,
}

impl eframe::App for CvPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::utils::egui::ScrollableFramedCentralPanel::default().show(ctx, |ui| {
            crate::utils::egui::centered_strong_heading(ui, PAGE.title());
            // TODO: Add CV page
        });
    }
}

impl CvPage {}
