#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod updater;

fn main() -> eframe::Result {
    updater::check_on_startup();

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("DUYRA")
            .with_inner_size([720.0, 460.0])
            .with_min_inner_size([560.0, 360.0]),
        ..Default::default()
    };

    eframe::run_ui_native("DUYRA", native_options, |ui, _frame| {
        app::show(ui);
    })
}
