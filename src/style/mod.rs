pub use visuals::{dark as dark_visuals, light as light_visuals};

mod fonts;
mod text;
mod visuals;

pub fn load_fonts(ctx: &egui::Context) {
    // Load and set the fonts
    fonts::set(ctx);

    // Set the text styles
    ctx.style_mut(|style| {
        style.text_styles = text::styles();
    });
}

pub fn set_theme(ctx: &egui::Context, theme: eframe::Theme) {
    // Set the style
    ctx.set_visuals(match theme {
        eframe::Theme::Dark => dark_visuals().clone(),
        eframe::Theme::Light => light_visuals().clone(),
    });
}
