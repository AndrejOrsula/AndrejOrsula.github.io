#![allow(dead_code)]

const PAGE: crate::page::Page = crate::page::Page::Software;

#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct SoftwarePageConfig {}

impl Default for SoftwarePageConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[must_use = "You should call .update()"]
#[derive(Default)]
pub struct SoftwarePage {
    pub cfg: SoftwarePageConfig,
}

impl eframe::App for SoftwarePage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::utils::egui::ScrollableFramedCentralPanel::default().show(ctx, |ui| {
            crate::utils::egui::centered_strong_heading(ui, PAGE.title());
            // TODO: Add Software page
        });
    }
}

impl SoftwarePage {}
