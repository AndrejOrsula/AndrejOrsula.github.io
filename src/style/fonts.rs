use egui::{Context, FontTweak};

pub fn set(ctx: &Context) {
    let mut font_definitions = crate::macros::generate_font_definitions! {
        // Monospace
        "MonaspaceNeon" as Monospace [
            "MonaspaceNeon-Medium.otf",
        ],

        // Proportional
        "Inter" as Proportional [
            "Inter-Regular.otf",
        ],
        "InterDisplay" [
            "InterDisplay-Bold.otf",
        ],

        // Proportional extras
        "MaterialIcons" as Proportional [
            "MaterialIconsRound-Regular.otf",
        ],
        "FontAwesome" as Proportional [
            "FontAwesome-Regular.otf",
        ],
        "FontAwesomeBrands" as Proportional [
            "FontAwesomeBrands-Regular.otf",
        ],
    };

    // Tweak some fonts to improve their alignment
    {
        let font = font_definitions
            .font_data
            .get_mut("MonaspaceNeon-Medium.otf")
            .unwrap();
        *font = font.clone().tweak(FontTweak {
            y_offset_factor: 0.24,
            ..Default::default()
        });
    }
    {
        let font = font_definitions
            .font_data
            .get_mut("MaterialIconsRound-Regular.otf")
            .unwrap();
        *font = font.clone().tweak(FontTweak {
            y_offset_factor: 0.16,
            ..Default::default()
        });
    }

    ctx.set_fonts(font_definitions);
}
