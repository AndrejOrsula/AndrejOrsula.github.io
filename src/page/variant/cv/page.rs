#[must_use = "You should call .update()"]
#[derive(Default)]
pub struct CvPage;

impl eframe::App for CvPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::utils::egui::ScrollableFramedCentralPanel::default().show(ctx, |_ui| {});
    }
}
