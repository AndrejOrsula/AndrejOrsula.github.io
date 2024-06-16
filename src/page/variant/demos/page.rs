#![allow(dead_code)]

const PAGE: crate::page::Page = crate::page::Page::Demos;

#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct DemosPageConfig {}

impl Default for DemosPageConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[must_use = "You should call .update()"]
#[derive(Default)]
pub struct DemosPage {
    pub cfg: DemosPageConfig,
}

impl eframe::App for DemosPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::utils::egui::ScrollableFramedCentralPanel::default().show(ctx, |ui| {
            crate::utils::egui::centered_strong_heading(ui, PAGE.title());
            // TODO: Add Demos page
        });
    }
}

impl DemosPage {}
