use eframe::egui::IconData;

const ICON_SIZE: usize = 128;
const BACKGROUND: [u8; 3] = [37, 65, 81];
const FOREGROUND: [u8; 3] = [235, 245, 248];

/// DUYRA işaretini pencere başlığı, görev çubuğu ve uygulama değiştirici için üretir.
/// Harici görüntü kütüphanesi veya çalışma zamanı dosyası gerektirmez.
pub fn window_icon() -> IconData {
    let mut rgba = vec![0; ICON_SIZE * ICON_SIZE * 4];
    let bars = [
        (36.0_f32, 24.0_f32),
        (50.0, 42.0),
        (64.0, 64.0),
        (78.0, 48.0),
        (92.0, 28.0),
    ];

    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let index = (y * ICON_SIZE + x) * 4;
            let pixel = &mut rgba[index..index + 4];

            let background_distance =
                rounded_rectangle_distance(px, py, 64.0, 64.0, 57.0, 57.0, 25.0);
            blend(pixel, BACKGROUND, coverage(background_distance));

            for (center_x, height) in bars.iter().copied() {
                let top = 64.0 - height / 2.0;
                let bottom = 64.0 + height / 2.0;
                let distance = capsule_distance(px, py, center_x, top, bottom, 3.2);
                blend(pixel, FOREGROUND, coverage(distance));
            }
        }
    }

    IconData {
        rgba,
        width: ICON_SIZE as u32,
        height: ICON_SIZE as u32,
    }
}

fn rounded_rectangle_distance(
    x: f32,
    y: f32,
    center_x: f32,
    center_y: f32,
    half_width: f32,
    half_height: f32,
    radius: f32,
) -> f32 {
    let qx = (x - center_x).abs() - (half_width - radius);
    let qy = (y - center_y).abs() - (half_height - radius);
    qx.max(0.0).hypot(qy.max(0.0)) + qx.max(qy).min(0.0) - radius
}

fn capsule_distance(x: f32, y: f32, center_x: f32, start_y: f32, end_y: f32, radius: f32) -> f32 {
    let nearest_y = y.clamp(start_y, end_y);
    (x - center_x).hypot(y - nearest_y) - radius
}

fn coverage(distance: f32) -> f32 {
    (0.5 - distance).clamp(0.0, 1.0)
}

fn blend(pixel: &mut [u8], color: [u8; 3], alpha: f32) {
    if alpha <= 0.0 {
        return;
    }

    let destination_alpha = f32::from(pixel[3]) / 255.0;
    let output_alpha = alpha + destination_alpha * (1.0 - alpha);

    for (destination_channel, source_channel) in pixel[..3].iter_mut().zip(color) {
        let destination = f32::from(*destination_channel);
        let source = f32::from(source_channel);
        let output =
            (source * alpha + destination * destination_alpha * (1.0 - alpha)) / output_alpha;
        *destination_channel = output.round() as u8;
    }

    pixel[3] = (output_alpha * 255.0).round() as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_icon_has_expected_rgba_dimensions() {
        let icon = window_icon();

        assert_eq!(icon.width, 128);
        assert_eq!(icon.height, 128);
        assert_eq!(icon.rgba.len(), 128 * 128 * 4);
        assert!(icon.rgba.iter().any(|value| *value > 0));
    }
}
