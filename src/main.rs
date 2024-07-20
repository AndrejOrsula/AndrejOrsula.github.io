#[cfg(target_arch = "wasm32")]
fn main() {
    #[cfg(debug_assertions)]
    eframe::WebLogger::init(log::LevelFilter::max()).ok();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "egui_canvas",
                eframe::WebOptions {
                    follow_system_theme: false,
                    ..Default::default()
                },
                Box::new(|cc| Ok(Box::new(portfolio_andrejorsula::App::new(cc)))),
            )
            .await
            .unwrap();
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    #[cfg(debug_assertions)]
    env_logger::init();

    let icon = image::load_from_memory_with_format(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/icons/favicon.ico"
        )),
        image::ImageFormat::Ico,
    )
    .unwrap()
    .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(egui::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        ..Default::default()
    };

    eframe::run_native(
        portfolio_andrejorsula::AUTHOR_NAME_FULL,
        native_options,
        Box::new(|cc| Ok(Box::new(portfolio_andrejorsula::App::new(cc)))),
    )
}
