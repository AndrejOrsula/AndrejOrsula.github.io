#![allow(dead_code)]

const PAGE: crate::page::Page = crate::page::Page::Blog;

#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct BlogPageConfig {}

impl Default for BlogPageConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[must_use = "You should call .update()"]
#[derive(Default)]
pub struct BlogPage {
    pub cfg: BlogPageConfig,
}

impl eframe::App for BlogPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::utils::egui::ScrollableFramedCentralPanel::default().show(ctx, |ui| {
            crate::utils::egui::centered_strong_heading(ui, PAGE.title());
            // TODO: Add Blog page
        });
    }
}

impl BlogPage {}
