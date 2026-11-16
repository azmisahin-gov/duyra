use std::time::Duration;

use eframe::egui::{self, Align, Color32, CornerRadius, Layout, RichText, Sense, Stroke, Vec2};

use crate::updater::{self, SharedUpdateStatus, UpdateSnapshot};

const BACKGROUND: Color32 = Color32::from_rgb(15, 23, 31);
const SIDEBAR: Color32 = Color32::from_rgb(18, 30, 40);
const SURFACE: Color32 = Color32::from_rgb(26, 43, 55);
const SURFACE_RAISED: Color32 = Color32::from_rgb(31, 52, 66);
const BORDER: Color32 = Color32::from_rgb(48, 68, 80);
const TEXT: Color32 = Color32::from_rgb(235, 243, 247);
const MUTED: Color32 = Color32::from_rgb(153, 174, 186);
const ACCENT: Color32 = Color32::from_rgb(131, 198, 216);
const SUCCESS: Color32 = Color32::from_rgb(112, 194, 151);
const WARNING: Color32 = Color32::from_rgb(235, 166, 96);

const SIDEBAR_WIDTH: f32 = 224.0;
const FOOTER_HEIGHT: f32 = 48.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WorkspaceView {
    #[default]
    Overview,
    Workspaces,
    Evidence,
    Audio,
    Transcription,
    Analysis,
    Reports,
    Settings,
}

impl WorkspaceView {
    const ALL: [Self; 7] = [
        Self::Overview,
        Self::Workspaces,
        Self::Evidence,
        Self::Audio,
        Self::Transcription,
        Self::Analysis,
        Self::Reports,
    ];

    fn title(self) -> &'static str {
        match self {
            Self::Overview => "Genel Bakış",
            Self::Workspaces => "İncelemeler",
            Self::Evidence => "Delil Dosyaları",
            Self::Audio => "Ses İnceleme",
            Self::Transcription => "Deşifre",
            Self::Analysis => "Analiz",
            Self::Reports => "Raporlar",
            Self::Settings => "Ayarlar",
        }
    }

    fn subtitle(self) -> &'static str {
        match self {
            Self::Overview => "İnceleme çalışmalarınızın genel durumu.",
            Self::Workspaces => "İncelemelerinizi ve çalışma dosyalarınızı yönetin.",
            Self::Evidence => "Kaynakları ve çalışma kopyalarını görüntüleyin.",
            Self::Audio => "Ses kayıtlarını dinleyin ve inceleyin.",
            Self::Transcription => "Deşifre taslaklarını inceleyin ve düzeltin.",
            Self::Analysis => "Ses inceleme ve ölçüm çalışmalarını yönetin.",
            Self::Reports => "İnceleme bulgularını raporlayın.",
            Self::Settings => "DUYRA uygulama tercihleri ve sistem bilgileri.",
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            Self::Overview => "⌂",
            Self::Workspaces => "▣",
            Self::Evidence => "▤",
            Self::Audio => "♫",
            Self::Transcription => "≋",
            Self::Analysis => "◉",
            Self::Reports => "▧",
            Self::Settings => "⚙",
        }
    }
}

pub fn show(
    root_ui: &mut egui::Ui,
    graphics_info: Option<&(String, String)>,
    graphics_adapters: &[String],
    update_status: &SharedUpdateStatus,
    selected_view: &mut WorkspaceView,
) {
    root_ui.ctx().set_visuals(egui::Visuals::dark());
    root_ui.ctx().style_mut(|style| {
        style.visuals.panel_fill = BACKGROUND;
        style.visuals.window_fill = SURFACE;
        style.visuals.widgets.noninteractive.bg_fill = SURFACE;
        style.visuals.widgets.noninteractive.fg_stroke.color = TEXT;
        style.visuals.widgets.inactive.bg_fill = SURFACE_RAISED;
        style.visuals.widgets.inactive.fg_stroke.color = TEXT;
        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(43, 70, 86);
        style.visuals.widgets.hovered.fg_stroke.color = TEXT;
        style.visuals.widgets.active.bg_fill = Color32::from_rgb(49, 83, 100);
        style.visuals.widgets.active.fg_stroke.color = TEXT;
        style.spacing.item_spacing = Vec2::new(10.0, 8.0);
        style.spacing.button_padding = Vec2::new(12.0, 9.0);
    });

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(BACKGROUND))
        .show(root_ui, |root| {
            let available = root.available_size();

            root.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                ui.allocate_ui_with_layout(
                    Vec2::new(SIDEBAR_WIDTH, available.y),
                    Layout::top_down(Align::Min),
                    |sidebar| draw_sidebar(sidebar, selected_view),
                );

                let main_width = (available.x - SIDEBAR_WIDTH - 1.0).max(320.0);
                ui.allocate_ui_with_layout(
                    Vec2::new(main_width, available.y),
                    Layout::top_down(Align::Min),
                    |main| {
                        draw_header(main, *selected_view);

                        main.separator();
                        let body_height = (main.available_height() - FOOTER_HEIGHT - 1.0).max(0.0);
                        main.allocate_ui_with_layout(
                            Vec2::new(main.available_width(), body_height),
                            Layout::top_down(Align::Min),
                            |body| {
                                egui::ScrollArea::vertical()
                                    .auto_shrink([false, false])
                                    .show(body, |content| {
                                        content.add_space(18.0);
                                        match selected_view {
                                            WorkspaceView::Overview => draw_overview(
                                                content,
                                                graphics_info,
                                                graphics_adapters,
                                            ),
                                            view => draw_placeholder(content, *view),
                                        }
                                        content.add_space(20.0);
                                    });
                            },
                        );
                        main.separator();
                        draw_status_bar(main, update_status);
                    },
                );
            });
        });
}

fn draw_sidebar(ui: &mut egui::Ui, selected_view: &mut WorkspaceView) {
    egui::Frame::new()
        .fill(SIDEBAR)
        .inner_margin(egui::Margin::same(16))
        .show(ui, |ui| {
            ui.set_min_height(ui.available_height());

            ui.horizontal(|ui| {
                draw_brand_mark(ui, 38.0);
                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.label(RichText::new("DUYRA").size(19.0).strong().color(TEXT));
                    ui.label(
                        RichText::new("BİLİRKİŞİ ÇALIŞMA ALANI")
                            .size(8.5)
                            .color(MUTED),
                    );
                });
            });

            ui.add_space(28.0);
            ui.label(
                RichText::new("ÇALIŞMA ALANI")
                    .size(10.0)
                    .strong()
                    .color(MUTED),
            );
            ui.add_space(8.0);

            for view in WorkspaceView::ALL {
                if navigation_item(ui, view, *selected_view) {
                    *selected_view = view;
                }
            }

            ui.add_space(18.0);
            ui.separator();
            ui.add_space(10.0);

            if navigation_item(ui, WorkspaceView::Settings, *selected_view) {
                *selected_view = WorkspaceView::Settings;
            }

            ui.add_space(12.0);
            ui.label(
                RichText::new(format!("Sürüm {}", env!("CARGO_PKG_VERSION")))
                    .size(10.0)
                    .color(MUTED),
            );
            ui.label(
                RichText::new("Yerel masaüstü uygulaması")
                    .size(9.0)
                    .color(MUTED),
            );
        });
}

fn navigation_item(ui: &mut egui::Ui, view: WorkspaceView, selected: WorkspaceView) -> bool {
    let is_selected = view == selected;
    let foreground = if is_selected { TEXT } else { MUTED };
    let background = if is_selected {
        Color32::from_rgb(38, 66, 81)
    } else {
        Color32::TRANSPARENT
    };

    let label = RichText::new(format!("{}   {}", view.symbol(), view.title()))
        .size(13.0)
        .color(foreground);

    ui.add_sized(
        [SIDEBAR_WIDTH - 32.0, 40.0],
        egui::Button::new(label)
            .fill(background)
            .stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(9)),
    )
    .clicked()
}

fn draw_header(ui: &mut egui::Ui, view: WorkspaceView) {
    egui::Frame::new()
        .fill(BACKGROUND)
        .inner_margin(egui::Margin {
            left: 24,
            right: 24,
            top: 15,
            bottom: 12,
        })
        .show(ui, |ui| {
            ui.set_min_height(60.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("DUYRA  /  ÇALIŞMA ALANI")
                            .size(9.0)
                            .strong()
                            .color(ACCENT),
                    );
                    ui.label(RichText::new(view.title()).size(21.0).strong().color(TEXT));
                    ui.label(RichText::new(view.subtitle()).size(11.0).color(MUTED));
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .size(11.0)
                            .color(MUTED),
                    );
                    ui.add_space(12.0);
                    ui.label(
                        RichText::new("●  YEREL ÇALIŞMA")
                            .size(10.0)
                            .strong()
                            .color(SUCCESS),
                    );
                });
            });
        });
}

fn draw_overview(
    ui: &mut egui::Ui,
    graphics_info: Option<&(String, String)>,
    graphics_adapters: &[String],
) {
    ui.add_space(2.0);
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(14))
        .inner_margin(egui::Margin::same(22))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                draw_brand_mark(ui, 62.0);
                ui.add_space(14.0);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("DUYRA çalışma alanına hoş geldiniz")
                            .size(20.0)
                            .strong()
                            .color(TEXT),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(
                            "Adli ses inceleme sürecinizi tek bir yerden, düzenli ve izlenebilir biçimde yürütün.",
                        )
                        .size(12.0)
                        .color(MUTED),
                    );
                    ui.add_space(9.0);
                    ui.label(
                        RichText::new("İLK İNCELEME AKIŞI HAZIRLANIYOR")
                            .size(9.0)
                            .strong()
                            .color(ACCENT),
                    );
                });
            });
        });

    ui.add_space(16.0);
    ui.label(
        RichText::new("ÇALIŞMA ÖZETİ")
            .size(10.0)
            .strong()
            .color(MUTED),
    );
    ui.add_space(8.0);
    ui.columns(3, |columns| {
        draw_summary_card(&mut columns[0], "İncelemeler", "Henüz çalışma yok", "▣");
        draw_summary_card(&mut columns[1], "Delil dosyaları", "Henüz dosya yok", "▤");
        draw_summary_card(&mut columns[2], "Raporlar", "Henüz rapor yok", "▧");
    });

    ui.add_space(16.0);
    let width = ui.available_width();
    if width >= 860.0 {
        ui.columns(2, |columns| {
            draw_recent_work_card(&mut columns[0]);
            draw_system_card(&mut columns[1], graphics_info, graphics_adapters);
        });
    } else {
        draw_recent_work_card(ui);
        ui.add_space(12.0);
        draw_system_card(ui, graphics_info, graphics_adapters);
    }
}

fn draw_summary_card(ui: &mut egui::Ui, title: &str, description: &str, symbol: &str) {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(egui::Margin::same(16))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(symbol).size(19.0).color(ACCENT));
                ui.label(RichText::new(title).size(12.0).strong().color(TEXT));
            });
            ui.add_space(8.0);
            ui.label(RichText::new("—").size(24.0).strong().color(TEXT));
            ui.label(RichText::new(description).size(10.0).color(MUTED));
        });
}

fn draw_recent_work_card(ui: &mut egui::Ui) {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(egui::Margin::same(18))
        .show(ui, |ui| {
            ui.label(
                RichText::new("SON ÇALIŞMALAR")
                    .size(10.0)
                    .strong()
                    .color(MUTED),
            );
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("▣").size(24.0).color(ACCENT));
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Henüz bir inceleme başlatılmadı")
                            .size(13.0)
                            .strong()
                            .color(TEXT),
                    );
                    ui.label(
                        RichText::new(
                            "İnceleme oluşturma ve kayıt akışı sonraki geliştirme hedefidir.",
                        )
                        .size(10.0)
                        .color(MUTED),
                    );
                });
            });
        });
}

fn draw_system_card(
    ui: &mut egui::Ui,
    graphics_info: Option<&(String, String)>,
    graphics_adapters: &[String],
) {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(egui::Margin::same(18))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("▦").size(18.0).color(ACCENT));
                ui.label(RichText::new("SİSTEM BİLGİSİ").size(10.0).strong().color(MUTED));
            });
            ui.add_space(10.0);

            if graphics_adapters.is_empty() {
                ui.label(
                    RichText::new("Grafik bağdaştırıcısı bilgisi alınamadı.")
                        .size(11.0)
                        .color(MUTED),
                );
            } else {
                for adapter in graphics_adapters {
                    ui.label(RichText::new(adapter).size(11.0).color(TEXT));
                }
            }

            if let Some((renderer, _vendor)) = graphics_info {
                ui.add_space(5.0);
                ui.label(
                    RichText::new(format!("Ekranı çizen: {renderer}"))
                        .size(10.0)
                        .color(MUTED),
                );
            }

            ui.add_space(8.0);
            ui.label(
                RichText::new("Bu bilgiler grafik aygıtlarını gösterir; ses işlemlerinde GPU hızlandırmasını garanti etmez.")
                    .size(9.0)
                    .color(MUTED),
            );
        });
}

fn draw_placeholder(ui: &mut egui::Ui, view: WorkspaceView) {
    ui.add_space(2.0);
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(14))
        .inner_margin(egui::Margin::same(24))
        .show(ui, |ui| {
            ui.set_min_height(220.0);
            ui.vertical_centered(|ui| {
                ui.add_space(25.0);
                ui.label(RichText::new(view.symbol()).size(34.0).color(ACCENT));
                ui.add_space(10.0);
                ui.label(
                    RichText::new(format!("{} bölümü henüz hazır değil", view.title()))
                        .size(18.0)
                        .strong()
                        .color(TEXT),
                );
                ui.add_space(6.0);
                ui.label(
                    RichText::new("Bu bölüm, geliştirme sırası geldiğinde tamamlanıp doğrulanarak kullanıma açılacak.")
                        .size(11.0)
                        .color(MUTED),
                );
                ui.add_space(12.0);
                ui.label(
                    RichText::new("Şimdilik burada işlem yapılmaz.")
                        .size(10.0)
                        .color(WARNING),
                );
            });
        });
}

fn draw_status_bar(ui: &mut egui::Ui, update_status: &SharedUpdateStatus) {
    let snapshot = update_status
        .lock()
        .map(|status| status.clone())
        .unwrap_or_else(|_| UpdateSnapshot {
            current_version: env!("CARGO_PKG_VERSION").to_owned(),
            stage: "durum okunamadı".to_owned(),
            message: "Güncelleme durumu okunamadı.".to_owned(),
            target_version: None,
            error: Some("Güncelleme durumu kilitlenmiş.".to_owned()),
        });

    let is_checking = snapshot.error.is_none()
        && !matches!(
            snapshot.stage.as_str(),
            "güncel" | "yeniden_başlatma" | "desteklenmiyor"
        );
    if is_checking {
        ui.ctx().request_repaint_after(Duration::from_millis(250));
    }

    egui::Frame::new()
        .fill(SIDEBAR)
        .inner_margin(egui::Margin {
            left: 18,
            right: 18,
            top: 9,
            bottom: 9,
        })
        .show(ui, |ui| {
            ui.set_min_height(FOOTER_HEIGHT - 1.0);
            ui.horizontal(|ui| {
                if is_checking {
                    ui.spinner();
                } else if snapshot.error.is_some() {
                    ui.label(RichText::new("!").strong().color(WARNING));
                } else {
                    ui.label(RichText::new("●").color(SUCCESS));
                }

                let status_text = if snapshot.error.is_some() {
                    "Güncelleme denetiminde sorun var"
                } else if snapshot.stage == "desteklenmiyor" {
                    "Otomatik güncelleme bu sistemde etkin değil"
                } else if is_checking {
                    "Güncellemeler denetleniyor"
                } else {
                    "DUYRA güncel"
                };

                ui.label(RichText::new(status_text).size(10.0).color(
                    if snapshot.error.is_some() {
                        WARNING
                    } else {
                        MUTED
                    },
                ));

                if snapshot.error.is_some() {
                    ui.add_space(8.0);
                    if ui.small_button("Günlük klasörü").clicked()
                        && let Err(error) = updater::open_log_folder()
                    {
                        ui.ctx()
                            .copy_text(format!("Günlük klasörü açılamadı: {error}"));
                    }
                    if ui.small_button("Tanı özeti").clicked() {
                        ui.ctx().copy_text(updater::support_summary(&snapshot));
                    }
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .size(10.0)
                            .color(MUTED),
                    );
                    ui.label(RichText::new("Yerel çalışma").size(10.0).color(MUTED));
                });
            });
        });
}

fn draw_brand_mark(ui: &mut egui::Ui, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(
        rect,
        CornerRadius::same((size * 0.25).round() as u8),
        Color32::from_rgb(37, 65, 81),
    );

    let center = rect.center();
    let heights = [0.19_f32, 0.33, 0.50, 0.38, 0.22];
    let foreground = Color32::from_rgb(235, 245, 248);
    let stroke_width = (size * 0.052).max(1.5);
    let dot_radius = (size * 0.026).max(1.0);
    let gap = size * 0.14;

    for (index, relative_height) in heights.iter().enumerate() {
        let x = center.x + (index as f32 - 2.0) * gap;
        let half_height = size * relative_height / 2.0;
        let top = egui::pos2(x, center.y - half_height);
        let bottom = egui::pos2(x, center.y + half_height);

        painter.line_segment([top, bottom], Stroke::new(stroke_width, foreground));
        painter.circle_filled(top, dot_radius, foreground);
        painter.circle_filled(bottom, dot_radius, foreground);
    }
}
