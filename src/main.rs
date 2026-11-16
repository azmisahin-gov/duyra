#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("DUYRA")
            .with_inner_size([720.0, 460.0])
            .with_min_inner_size([560.0, 360.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DUYRA",
        native_options,
        Box::new(|_creation_context| Ok(Box::new(app::DuyraApp))),
    )
}
