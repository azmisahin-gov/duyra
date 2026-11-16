#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod system;
mod updater;

use eframe::glow::HasContext;

fn main() -> eframe::Result {
    let update_status = updater::check_on_startup();
    let graphics_adapters = system::video_adapters();

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("DUYRA")
            .with_inner_size([720.0, 500.0])
            .with_min_inner_size([560.0, 420.0]),
        ..Default::default()
    };

    let mut graphics_checked = false;
    let mut graphics_info: Option<(String, String)> = None;

    eframe::run_ui_native("DUYRA", native_options, move |ui, frame| {
        if !graphics_checked {
            graphics_checked = true;
            if let Some(gl) = frame.gl() {
                let info = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                    (
                        gl.get_parameter_string(0x1F01),
                        gl.get_parameter_string(0x1F00),
                    )
                }));

                if let Ok((renderer, vendor)) = info
                    && !renderer.trim().is_empty()
                {
                    graphics_info = Some((renderer, vendor));
                }
            }
        }

        app::show(
            ui,
            graphics_info.as_ref(),
            &graphics_adapters,
            &update_status,
        );
    })
}
