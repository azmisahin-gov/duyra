use eframe::egui::{self, Color32, CornerRadius, RichText, Sense, Stroke, Vec2};

pub fn show(
    root_ui: &mut egui::Ui,
    graphics_info: Option<&(String, String)>,
    graphics_adapters: &[String],
) {
    root_ui.ctx().set_visuals(egui::Visuals::dark());

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(Color32::from_rgb(15, 23, 31)))
        .show(root_ui, |ui| {
            let available_height = ui.available_height();

            ui.vertical_centered(|ui| {
                ui.add_space(((available_height - 280.0).max(0.0)) * 0.40);

                draw_logo(ui);

                ui.add_space(18.0);
                ui.label(
                    RichText::new("DUYRA")
                        .size(34.0)
                        .strong()
                        .color(Color32::from_rgb(239, 246, 249)),
                );

                ui.add_space(8.0);
                ui.label(
                    RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                        .size(14.0)
                        .color(Color32::from_rgb(157, 177, 189)),
                );

                ui.add_space(24.0);
                draw_graphics_indicator(ui, graphics_info, graphics_adapters);
            });
        });
}

fn draw_logo(ui: &mut egui::Ui) {
    let (rect, _response) = ui.allocate_exact_size(Vec2::splat(88.0), Sense::hover());
    let painter = ui.painter();

    painter.rect_filled(rect, CornerRadius::same(22), Color32::from_rgb(37, 65, 81));

    let center = rect.center();
    let heights = [16.0, 28.0, 43.0, 32.0, 19.0];
    let foreground = Color32::from_rgb(235, 245, 248);

    for (index, height) in heights.iter().enumerate() {
        let x = center.x + (index as f32 - 2.0) * 12.0;
        let top = egui::pos2(x, center.y - height / 2.0);
        let bottom = egui::pos2(x, center.y + height / 2.0);

        painter.line_segment([top, bottom], Stroke::new(4.5, foreground));
        painter.circle_filled(top, 2.25, foreground);
        painter.circle_filled(bottom, 2.25, foreground);
    }
}

fn draw_graphics_indicator(
    ui: &mut egui::Ui,
    graphics_info: Option<&(String, String)>,
    graphics_adapters: &[String],
) {
    let adapter_count = graphics_adapters.len().max(1);
    let line_count = 1 + adapter_count + usize::from(graphics_info.is_some());
    let block_height = 28.0 + line_count as f32 * 16.0;
    let block_width = ui.available_width();

    ui.allocate_ui_with_layout(
        Vec2::new(block_width, block_height),
        egui::Layout::left_to_right(egui::Align::Center)
            .with_main_align(egui::Align::Center),
        |ui| {
            let (icon_rect, _response) =
                ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
            let painter = ui.painter();
            let icon_bg = Color32::from_rgb(28, 48, 60);
            let icon_fg = Color32::from_rgb(131, 198, 216);

            painter.rect_filled(icon_rect, CornerRadius::same(10), icon_bg);

            let chip = egui::Rect::from_center_size(icon_rect.center(), Vec2::splat(14.0));
            painter.rect_filled(chip, CornerRadius::same(3), icon_fg);

            for offset in [-7.0, 0.0, 7.0] {
                painter.line_segment(
                    [
                        egui::pos2(chip.left() - 4.0, chip.center().y + offset),
                        egui::pos2(chip.left(), chip.center().y + offset),
                    ],
                    Stroke::new(1.5, icon_fg),
                );
                painter.line_segment(
                    [
                        egui::pos2(chip.right(), chip.center().y + offset),
                        egui::pos2(chip.right() + 4.0, chip.center().y + offset),
                    ],
                    Stroke::new(1.5, icon_fg),
                );
                painter.line_segment(
                    [
                        egui::pos2(chip.center().x + offset, chip.top() - 4.0),
                        egui::pos2(chip.center().x + offset, chip.top()),
                    ],
                    Stroke::new(1.5, icon_fg),
                );
                painter.line_segment(
                    [
                        egui::pos2(chip.center().x + offset, chip.bottom()),
                        egui::pos2(chip.center().x + offset, chip.bottom() + 4.0),
                    ],
                    Stroke::new(1.5, icon_fg),
                );
            }

            ui.add_space(10.0);
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("GRAFİK BİRİMLERİ")
                        .size(10.0)
                        .strong()
                        .color(Color32::from_rgb(131, 198, 216)),
                );

                if graphics_adapters.is_empty() {
                    ui.label(
                        RichText::new("Grafik bağdaştırıcısı listelenemedi")
                            .size(13.0)
                            .color(Color32::from_rgb(157, 177, 189)),
                    );
                } else {
                    for adapter in graphics_adapters {
                        ui.label(
                            RichText::new(adapter)
                                .size(13.0)
                                .color(Color32::from_rgb(223, 235, 240)),
                        );
                    }
                }

                if let Some((renderer, _vendor)) = graphics_info {
                    ui.label(
                        RichText::new(format!("Ekranı çizen: {renderer}"))
                            .size(11.0)
                            .color(Color32::from_rgb(157, 177, 189)),
                    );
                }
            });
        },
    )
    .response
    .on_hover_text(
        "Grafik birimleri Windows aygıt listesinden okunur.          'Ekranı çizen' satırı DUYRA'nın mevcut grafik bağlamını gösterir.          Bu liste ses çözümleme için GPU hızlandırma desteğini kanıtlamaz.",
    );
}
