use egui::{FontFamily, FontId, TextStyle};

pub fn styles() -> std::collections::BTreeMap<TextStyle, FontId> {
    [
        (
            TextStyle::Small,
            FontId::new(12.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(20.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Heading,
            FontId::new(48.0, FontFamily::Name("InterDisplay".into())),
        ),
        (
            TextStyle::Monospace,
            FontId::new(16.0, FontFamily::Monospace),
        ),
        (
            TextStyle::Name("navigation_panel".into()),
            FontId::new(24.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Name("navigation_panel_display".into()),
            FontId::new(24.0, FontFamily::Name("InterDisplay".into())),
        ),
        (
            TextStyle::Name("social".into()),
            FontId::new(24.0, FontFamily::Name("FontAwesomeBrands".into())),
        ),
    ]
    .into()
}
