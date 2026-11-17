#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod diagnostics;
mod examinations;
mod icon;
mod system;
mod updater;

use eframe::glow::HasContext;

fn main() -> eframe::Result {
    let update_status = updater::check_on_startup();
    let graphics_adapters = system::video_adapters();
    let write_protection = system::write_protection_preflight();
    diagnostics::record_application_startup();

    let mut examination_ui = app::ExaminationUiState::load(&write_protection);

    // VirtualBox 3D acceleration can expose a DX12/Vulkan adapter whose
    // presentation path renders a black window. Test the WGPU GLES backend
    // in isolation before considering any renderer fallback or release change.
    let mut wgpu_options = eframe::WgpuConfiguration::default();
    if let eframe::egui_wgpu::WgpuSetup::CreateNew(setup) = &mut wgpu_options.wgpu_setup {
        setup.instance_descriptor.backends = eframe::wgpu::Backends::GL;
    }

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("DUYRA")
            .with_icon(icon::window_icon())
            .with_inner_size([1180.0, 780.0])
            .with_min_inner_size([920.0, 640.0]),
        renderer: eframe::Renderer::Wgpu,
        wgpu_options,
        ..Default::default()
    };

    let mut graphics_checked = false;
    let mut graphics_info: Option<(String, String)> = None;
    let mut selected_view = app::WorkspaceView::Overview;

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
                } else if let Some(wgpu) = frame.wgpu_render_state() {
                    let adapter = wgpu.adapter.get_info();
                    graphics_info = Some((adapter.name, adapter.vendor.to_string()));
                }
            }
        }

        app::show(
            ui,
            graphics_info.as_ref(),
            &graphics_adapters,
            &write_protection,
            &update_status,
            &mut selected_view,
            &mut examination_ui,
        );
    })
}
