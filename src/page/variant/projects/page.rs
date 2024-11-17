#![allow(dead_code)]

const PAGE: crate::page::Page = crate::page::Page::Projects;

#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct ProjectsPageConfig {}

impl Default for ProjectsPageConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[must_use = "You should call .update()"]
#[derive(Default)]
pub struct ProjectsPage {
    pub cfg: ProjectsPageConfig,
}

impl eframe::App for ProjectsPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::utils::egui::ScrollableFramedCentralPanel::default().show(ctx, |ui| {
            crate::utils::egui::centered_strong_heading(ui, PAGE.title());
            // TODO: Add Software page
        });
    }
}

impl ProjectsPage {}
