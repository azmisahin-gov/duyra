use eframe::egui::{self, Color32, CornerRadius, RichText, Sense, Stroke, Vec2};

#[derive(Default)]
pub struct DuyraApp;

impl eframe::App for DuyraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Color32::from_rgb(15, 23, 31)))
            .show(ctx, |ui| {
                let available_height = ui.available_height();

                ui.vertical_centered(|ui| {
                    ui.add_space(((available_height - 190.0).max(0.0)) * 0.48);

                    draw_logo(ui);

                    ui.add_space(22.0);
                    ui.label(
                        RichText::new("DUYRA")
                            .size(34.0)
                            .strong()
                            .color(Color32::from_rgb(239, 246, 249)),
                    );

                    ui.add_space(9.0);
                    ui.label(
                        RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .size(14.0)
                            .color(Color32::from_rgb(157, 177, 189)),
                    );
                });
            });
    }
}

fn draw_logo(ui: &mut egui::Ui) {
    let (rect, _response) = ui.allocate_exact_size(Vec2::splat(88.0), Sense::hover());
    let painter = ui.painter();

    painter.rect_filled(
        rect,
        CornerRadius::same(22),
        Color32::from_rgb(37, 65, 81),
    );

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
