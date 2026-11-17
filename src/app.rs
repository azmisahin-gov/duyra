use std::time::Duration;

use eframe::egui::{self, Align, Color32, CornerRadius, Layout, RichText, Sense, Stroke, Vec2};

use crate::{
    examinations::ExaminationStore,
    system::WriteProtectionPreflight,
    updater::{self, SharedUpdateStatus, UpdateSnapshot},
};

#[cfg(target_os = "windows")]
use crate::system::{
    PolicyAuthorityCheck, PolicyChangeAssessment, WritePolicyStatus, WriteProtectionSupport,
    inspect_windows_policy_authority, windows_write_protection_summary,
};

#[cfg(target_os = "windows")]
#[derive(Default)]
struct PolicyAuthorityUiState {
    result: Option<PolicyAuthorityCheck>,
    receiver: Option<std::sync::mpsc::Receiver<PolicyAuthorityCheck>>,
    preflight: Option<WriteProtectionPreflight>,
    report_status: Option<Result<(), &'static str>>,
}

#[cfg(target_os = "windows")]
impl PolicyAuthorityUiState {
    fn start(&mut self, preflight: WriteProtectionPreflight) {
        if self.receiver.is_some() {
            return;
        }

        self.preflight = Some(preflight);
        self.report_status = None;
        self.result = None;

        let (sender, receiver) = std::sync::mpsc::channel();
        self.receiver = Some(receiver);

        let spawn_result = std::thread::Builder::new()
            .name("duyra-politika-kaynak-denetimi".to_owned())
            .spawn(move || {
                let result = inspect_windows_policy_authority();
                let _ = sender.send(result);
            });

        if spawn_result.is_err() {
            self.receiver = None;
            self.result = Some(PolicyAuthorityCheck::unknown());
            self.save_report();
        }
    }

    fn save_report(&mut self) {
        if let (Some(preflight), Some(check)) = (&self.preflight, &self.result) {
            self.report_status =
                Some(crate::diagnostics::write_windows_report(preflight, check).map(|_| ()));
        }
    }

    fn finish(&mut self, check: PolicyAuthorityCheck) {
        self.result = Some(check);
        self.receiver = None;
        self.save_report();
    }

    fn poll(&mut self) {
        let result = self.receiver.as_ref().map(|receiver| receiver.try_recv());

        match result {
            Some(Ok(check)) => self.finish(check),
            Some(Err(std::sync::mpsc::TryRecvError::Disconnected)) => {
                self.finish(PolicyAuthorityCheck::unknown());
            }
            Some(Err(std::sync::mpsc::TryRecvError::Empty)) | None => {}
        }
    }
}

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

pub struct ExaminationUiState {
    examinations: ExaminationStore,
    selected_examination: Option<String>,
    examination_notice: Option<String>,
    #[cfg(target_os = "windows")]
    policy_authority: PolicyAuthorityUiState,
}

impl ExaminationUiState {
    pub fn load(write_protection: &WriteProtectionPreflight) -> Self {
        #[cfg(not(target_os = "windows"))]
        let _ = write_protection;

        let examinations = ExaminationStore::open_default();
        let examination_notice = examinations.notice();

        Self {
            examinations,
            selected_examination: None,
            examination_notice,
            #[cfg(target_os = "windows")]
            policy_authority: {
                let mut state = PolicyAuthorityUiState::default();
                state.start(write_protection.clone());
                state
            },
        }
    }
}

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
}

pub fn show(
    root_ui: &mut egui::Ui,
    graphics_info: Option<&(String, String)>,
    graphics_adapters: &[String],
    write_protection: &WriteProtectionPreflight,
    update_status: &SharedUpdateStatus,
    selected_view: &mut WorkspaceView,
    examination_ui: &mut ExaminationUiState,
) {
    let examinations = &mut examination_ui.examinations;
    let selected_examination = &mut examination_ui.selected_examination;
    let examination_notice = &mut examination_ui.examination_notice;

    #[cfg(target_os = "windows")]
    {
        examination_ui.policy_authority.poll();
        if examination_ui.policy_authority.receiver.is_some() {
            root_ui
                .ctx()
                .request_repaint_after(Duration::from_millis(100));
        }
    }

    root_ui.ctx().set_visuals(egui::Visuals::dark());
    root_ui.ctx().style_mut_of(egui::Theme::Dark, |style| {
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
                                                examinations,
                                                selected_examination,
                                                selected_view,
                                                examination_notice,
                                            ),
                                            WorkspaceView::Workspaces => draw_workspaces(
                                                content,
                                                examinations,
                                                selected_examination,
                                                selected_view,
                                                examination_notice,
                                            ),
                                            WorkspaceView::Settings => {
                                                #[cfg(target_os = "windows")]
                                                {
                                                    draw_settings(
                                                        content,
                                                        write_protection,
                                                        &mut examination_ui.policy_authority,
                                                    )
                                                }
                                                #[cfg(not(target_os = "windows"))]
                                                {
                                                    draw_settings(content, write_protection)
                                                }
                                            }
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

    let label = RichText::new(view.title()).size(13.0).color(foreground);

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
                        RichText::new("YEREL ÇALIŞMA")
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
    examinations: &mut ExaminationStore,
    selected_examination: &mut Option<String>,
    selected_view: &mut WorkspaceView,
    examination_notice: &mut Option<String>,
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
                        RichText::new("YENİ İNCELEME KAYDI OLUŞTURULABİLİR")
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
        draw_summary_card(
            &mut columns[0],
            "İncelemeler",
            &examinations.records().len().to_string(),
            "Yerel kayıt",
        );
        draw_summary_card(&mut columns[1], "Delil dosyaları", "—", "Henüz dosya yok");
        draw_summary_card(&mut columns[2], "Raporlar", "—", "Henüz rapor yok");
    });

    ui.add_space(16.0);
    let width = ui.available_width();
    if width >= 860.0 {
        ui.columns(2, |columns| {
            draw_recent_work_card(
                &mut columns[0],
                examinations,
                selected_examination,
                selected_view,
                examination_notice,
            );
            draw_system_card(&mut columns[1], graphics_info, graphics_adapters);
        });
    } else {
        draw_recent_work_card(
            ui,
            examinations,
            selected_examination,
            selected_view,
            examination_notice,
        );
        ui.add_space(12.0);
        draw_system_card(ui, graphics_info, graphics_adapters);
    }
}

fn draw_summary_card(ui: &mut egui::Ui, title: &str, value: &str, description: &str) {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(egui::Margin::same(16))
        .show(ui, |ui| {
            ui.label(RichText::new(title).size(12.0).strong().color(TEXT));
            ui.add_space(8.0);
            ui.label(RichText::new(value).size(24.0).strong().color(TEXT));
            ui.label(RichText::new(description).size(10.0).color(MUTED));
        });
}

fn draw_recent_work_card(
    ui: &mut egui::Ui,
    examinations: &mut ExaminationStore,
    selected_examination: &mut Option<String>,
    selected_view: &mut WorkspaceView,
    examination_notice: &mut Option<String>,
) {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(egui::Margin::same(18))
        .show(ui, |ui| {
            ui.label(
                RichText::new("SON İNCELEMELER")
                    .size(10.0)
                    .strong()
                    .color(MUTED),
            );
            ui.add_space(14.0);

            if examinations.records().is_empty() {
                ui.label(
                    RichText::new("Henüz bir inceleme başlatılmadı")
                        .size(13.0)
                        .strong()
                        .color(TEXT),
                );
                ui.label(
                    RichText::new("İlk inceleme kaydınızı oluşturarak başlayın.")
                        .size(10.0)
                        .color(MUTED),
                );
            } else {
                for record in examinations.records().iter().take(3) {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&record.number).strong().color(TEXT));
                        ui.label(RichText::new(&record.status).size(10.0).color(MUTED));
                        if ui.small_button("Aç").clicked() {
                            *selected_examination = Some(record.id.clone());
                            *selected_view = WorkspaceView::Workspaces;
                        }
                    });
                }
            }

            ui.add_space(10.0);
            if ui.button("Yeni İnceleme Başlat").clicked() {
                create_new_examination(
                    examinations,
                    selected_examination,
                    selected_view,
                    examination_notice,
                );
            }

            if let Some(notice) = examination_notice.as_ref() {
                ui.add_space(8.0);
                ui.label(RichText::new(notice).size(10.0).color(WARNING));
            }
        });
}

fn create_new_examination(
    examinations: &mut ExaminationStore,
    selected_examination: &mut Option<String>,
    selected_view: &mut WorkspaceView,
    examination_notice: &mut Option<String>,
) {
    match examinations.create_record() {
        Ok(record) => {
            *selected_examination = Some(record.id.clone());
            *selected_view = WorkspaceView::Workspaces;
            *examination_notice = Some(format!(
                "{} numaralı inceleme kaydı bu bilgisayara kaydedildi.",
                record.number
            ));
        }
        Err(error) => {
            *examination_notice = Some(format!("İnceleme kaydı oluşturulamadı: {error}"));
        }
    }
}

fn draw_workspaces(
    ui: &mut egui::Ui,
    examinations: &mut ExaminationStore,
    selected_examination: &mut Option<String>,
    selected_view: &mut WorkspaceView,
    examination_notice: &mut Option<String>,
) {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(14))
        .inner_margin(egui::Margin::same(22))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("İNCELEME KAYITLARI")
                            .size(10.0)
                            .strong()
                            .color(MUTED),
                    );
                    ui.label(
                        RichText::new("İncelemeleriniz bu bilgisayarda saklanır.")
                            .size(12.0)
                            .color(TEXT),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Yeni İnceleme").clicked() {
                        create_new_examination(
                            examinations,
                            selected_examination,
                            selected_view,
                            examination_notice,
                        );
                    }
                });
            });

            ui.add_space(14.0);

            if examinations.storage_error().is_some() {
                ui.label(
                    RichText::new("Yerel kayıt alanı kullanılamıyor. Kayıt oluşturma devre dışı.")
                        .size(11.0)
                        .color(WARNING),
                );
            }

            if examinations.records().is_empty() {
                ui.label(
                    RichText::new("Henüz kayıtlı inceleme yok.")
                        .size(13.0)
                        .strong()
                        .color(TEXT),
                );
                ui.label(
                    RichText::new("Yeni İnceleme düğmesiyle yerel bir inceleme kaydı oluşturun.")
                        .size(10.0)
                        .color(MUTED),
                );
            } else {
                for record in examinations.records() {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(&record.number)
                                    .size(12.0)
                                    .strong()
                                    .color(TEXT),
                            );
                            ui.label(
                                RichText::new(format!("Durum: {}", record.status))
                                    .size(10.0)
                                    .color(MUTED),
                            );
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.small_button("Aç").clicked() {
                                *selected_examination = Some(record.id.clone());
                            }
                        });
                    });
                    ui.separator();
                }
            }

            if let Some(notice) = examination_notice.as_ref() {
                ui.add_space(8.0);
                ui.label(RichText::new(notice).size(10.0).color(WARNING));
            }
        });

    if let Some(id) = selected_examination.as_deref()
        && let Some(record) = examinations.find(id)
    {
        ui.add_space(14.0);
        egui::Frame::new()
            .fill(SURFACE_RAISED)
            .stroke(Stroke::new(1.0, BORDER))
            .corner_radius(CornerRadius::same(12))
            .inner_margin(egui::Margin::same(18))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("AÇIK İNCELEME")
                        .size(10.0)
                        .strong()
                        .color(ACCENT),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new(&record.number)
                        .size(18.0)
                        .strong()
                        .color(TEXT),
                );
                ui.label(
                    RichText::new(format!("Durum: {}", record.status))
                        .size(11.0)
                        .color(MUTED),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new(
                        "Bu ilk aşamada yalnızca inceleme numarası ve oluşturulma bilgisi saklanır. Delil aygıtı bağlantısı henüz etkin değildir.",
                    )
                    .size(10.0)
                    .color(MUTED),
                );
            });
    }
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
            ui.label(RichText::new("SİSTEM BİLGİSİ").size(10.0).strong().color(MUTED));
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

fn draw_settings(
    ui: &mut egui::Ui,
    preflight: &WriteProtectionPreflight,
    #[cfg(target_os = "windows")] policy_authority: &mut PolicyAuthorityUiState,
) {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(14))
        .inner_margin(egui::Margin::same(22))
        .show(ui, |ui| {
            ui.label(
                RichText::new("WINDOWS YAZMA KORUMASI ÖN KONTROLÜ")
                    .size(10.0)
                    .strong()
                    .color(ACCENT),
            );
            ui.add_space(6.0);
            ui.label(
                RichText::new(
                    "Bu adım yalnızca bilgisayarın mevcut durumunu okur. Hiçbir sistem ayarını değiştirmez.",
                )
                .size(12.0)
                .color(TEXT),
            );
            ui.add_space(16.0);

            match preflight {
                #[cfg(target_os = "windows")]
                WriteProtectionPreflight::Windows(status) => {
                    draw_preflight_row(
                        ui,
                        "Windows",
                        status.product_name.as_deref().unwrap_or("Bilgi okunamadı"),
                    );
                    draw_preflight_row(
                        ui,
                        "Edisyon",
                        status.edition_id.as_deref().unwrap_or("Bilgi okunamadı"),
                    );
                    draw_preflight_row(
                        ui,
                        "Derleme",
                        &status
                            .build_number
                            .map(|build| build.to_string())
                            .unwrap_or_else(|| "Bilgi okunamadı".to_owned()),
                    );
                    draw_preflight_row(
                        ui,
                        "Uygulama yetkisi",
                        match status.process_is_elevated {
                            Some(true) => "Yönetici olarak yükseltilmiş",
                            Some(false) => "Yükseltilmiş değil",
                            None => "Kontrol edilemedi",
                        },
                    );

                    ui.add_space(12.0);
                    ui.label(
                        RichText::new("İŞLETİM SİSTEMİ DESTEK DURUMU")
                            .size(10.0)
                            .strong()
                            .color(MUTED),
                    );
                    let (support_text, support_color) = match &status.support {
                        WriteProtectionSupport::Supported => (
                            "Windows edisyonu ve derlemesi Microsoft destek listesiyle eşleşiyor.",
                            SUCCESS,
                        ),
                        WriteProtectionSupport::UnsupportedEdition(_edition) => (
                            "Bu Windows edisyonu destek listesinde doğrulanamadı.",
                            WARNING,
                        ),
                        WriteProtectionSupport::UnsupportedBuild(_build) => (
                            "Windows derlemesi gereken en düşük sürümün altında.",
                            WARNING,
                        ),
                        WriteProtectionSupport::Unknown => (
                            "Destek durumu için gerekli Windows bilgileri okunamadı.",
                            WARNING,
                        ),
                    };
                    ui.label(RichText::new(support_text).size(11.0).color(support_color));
                    match &status.support {
                        WriteProtectionSupport::UnsupportedEdition(edition) => {
                            ui.label(
                                RichText::new(format!("Okunan edisyon: {edition}"))
                                    .size(10.0)
                                    .color(MUTED),
                            );
                        }
                        WriteProtectionSupport::UnsupportedBuild(build) => {
                            ui.label(
                                RichText::new(format!("Okunan derleme: {build}"))
                                    .size(10.0)
                                    .color(MUTED),
                            );
                        }
                        _ => {}
                    }

                    ui.add_space(12.0);
                    ui.label(
                        RichText::new("BİLGİSAYAR GRUP İLKESİ KAYDI")
                            .size(10.0)
                            .strong()
                            .color(MUTED),
                    );
                    let (policy_text, policy_color) = match &status.computer_policy {
                        WritePolicyStatus::Enabled => (
                            "HKLM içindeki Deny_Write değeri etkin (1) görünüyor.",
                            ACCENT,
                        ),
                        WritePolicyStatus::Disabled => (
                            "HKLM içindeki Deny_Write değeri devre dışı (0).",
                            WARNING,
                        ),
                        WritePolicyStatus::NotConfigured => (
                            "İlgili HKLM Grup İlkesi değeri tanımlı değil.",
                            WARNING,
                        ),
                        WritePolicyStatus::UnexpectedValue(_value) => (
                            "İlgili HKLM Grup İlkesi değeri beklenmeyen bir sayı içeriyor.",
                            WARNING,
                        ),
                        WritePolicyStatus::Unknown(_) => (
                            "İlgili HKLM Grup İlkesi değeri okunamadı.",
                            WARNING,
                        ),
                    };
                    ui.label(RichText::new(policy_text).size(11.0).color(policy_color));
                    match &status.computer_policy {
                        WritePolicyStatus::UnexpectedValue(value) => {
                            ui.label(
                                RichText::new(format!("Okunan değer: {value}"))
                                    .size(10.0)
                                    .color(MUTED),
                            );
                        }
                        WritePolicyStatus::Unknown(error) => {
                            ui.label(RichText::new(error).size(10.0).color(MUTED));
                        }
                        _ => {}
                    }

                    if !status.read_warnings.is_empty() {
                        ui.add_space(10.0);
                        for warning in &status.read_warnings {
                            ui.label(RichText::new(warning).size(10.0).color(WARNING));
                        }
                    }

                    ui.add_space(14.0);
                    if ui.button("Ön kontrol sonucunu kopyala").clicked() {
                        ui.ctx()
                            .copy_text(windows_write_protection_summary(status));
                        ui.label(
                            RichText::new("Ön kontrol özeti panoya kopyalama için hazırlandı.")
                                .size(10.0)
                                .color(SUCCESS),
                        );
                    }
                    ui.label(
                        RichText::new(
                            "Özet yalnızca bu düğmeye bastığınızda panoya kopyalanır; dosyaya kaydedilmez veya gönderilmez.",
                        )
                        .size(10.0)
                        .color(MUTED),
                    );

                    ui.add_space(18.0);
                    ui.separator();
                    ui.label(
                        RichText::new("POLİTİKA YÖNETİM KAYNAĞI")
                            .size(10.0)
                            .strong()
                            .color(MUTED),
                    );
                    let checking = policy_authority.receiver.is_some();
                    let check_button_label = if checking {
                        "Yönetim kaynağı denetleniyor…"
                    } else if policy_authority.result.is_some() {
                        "Yönetim kaynağını yeniden denetle"
                    } else {
                        "Yönetim kaynağını denetle"
                    };

                    if ui
                        .add_enabled(!checking, egui::Button::new(check_button_label))
                        .clicked()
                    {
                        policy_authority.start(preflight.clone());
                    }

                    if checking {
                        ui.ctx()
                            .request_repaint_after(std::time::Duration::from_millis(100));
                        ui.label(
                            RichText::new("Windows ilke ve yönetim kayıtları salt okunur inceleniyor.")
                                .size(10.0)
                                .color(MUTED),
                        );
                    }

                    if let Some(check) = &policy_authority.result {
                        let group_policy_text = match check.group_policy_applied {
                            Some(true) => "Uygulanan Grup İlkesi bulundu",
                            Some(false) => "İlgili Grup İlkesi ayarı saptanmadı",
                            None => "Grup İlkesi kaynağı belirlenemedi",
                        };
                        let mdm_enrollment_text = match check.mdm_enrollment_detected {
                            Some(true) => {
                                "MDM kayıt işareti bulundu; eski/kalıntı kayıt olasılığı nedeniyle etkin yönetim doğrulanamadı"
                            }
                            Some(false) => "MDM kayıt işareti saptanmadı",
                            None => "MDM kayıt durumu belirlenemedi",
                        };
                        let mdm_policy_text = match check.mdm_policy_present {
                            Some(true) => "PolicyManager Storage ayarı bulundu",
                            Some(false) => "PolicyManager Storage ayarı saptanmadı",
                            None => "PolicyManager durumu belirlenemedi",
                        };
                        let domain_text = match check.domain_joined {
                            Some(true) => "Bilgisayar etki alanına katılmış",
                            Some(false) => "Etki alanı üyeliği saptanmadı",
                            None => "Etki alanı üyeliği belirlenemedi",
                        };
                        let existing_value_text = match check.current_policy_value_present {
                            Some(true) => "Mevcut Deny_Write değeri var; kaynağı ayrıca doğrulanmalı",
                            Some(false) => "Mevcut Deny_Write değeri yok",
                            None => "Mevcut Deny_Write değeri doğrulanamadı",
                        };

                        for line in [
                            group_policy_text,
                            mdm_enrollment_text,
                            mdm_policy_text,
                            domain_text,
                            existing_value_text,
                        ] {
                            ui.label(RichText::new(line).size(10.0).color(MUTED));
                        }

                        let (assessment_text, assessment_color) = match check.assessment() {
                            PolicyChangeAssessment::Managed => (
                                "Yönetilen ortam işareti bulundu. DUYRA bu politikayı değiştirmeyecek.",
                                WARNING,
                            ),
                            PolicyChangeAssessment::Reviewable => (
                                "Bilinen yönetim kaynağı saptanmadı. Bu yalnızca ön değerlendirmedir; henüz değişiklik izni değildir.",
                                SUCCESS,
                            ),
                            PolicyChangeAssessment::Unknown => (
                                "Kaynak kesinleştirilemedi. Güvenlik gereği politika değişikliği engellenmelidir.",
                                WARNING,
                            ),
                        };
                        ui.add_space(5.0);
                        ui.label(
                            RichText::new(assessment_text)
                                .size(11.0)
                                .strong()
                                .color(assessment_color),
                        );
                    } else if !checking {
                        ui.label(
                            RichText::new(
                                "Denetim çalıştırılana kadar politika değişikliği uygunluğu bilinmiyor.",
                            )
                            .size(10.0)
                            .color(MUTED),
                        );
                    }

                    ui.add_space(12.0);
                    ui.separator();
                    ui.label(
                        RichText::new("OTOMATİK TANILAMA RAPORU")
                            .size(10.0)
                            .strong()
                            .color(MUTED),
                    );
                    match &policy_authority.report_status {
                        Some(Ok(())) => {
                            ui.label(
                                RichText::new("Başlangıç denetimi tamamlandı; yerel tanılama raporu güncellendi.")
                                    .size(10.0)
                                    .color(SUCCESS),
                            );
                            if ui.button("Tanılama klasörünü aç").clicked()
                                && let Err(error) = crate::diagnostics::open_diagnostics_folder()
                            {
                                ui.label(RichText::new(error).size(10.0).color(WARNING));
                            }
                        }
                        Some(Err(error)) => {
                            ui.label(
                                RichText::new(format!("Tanılama raporu kaydedilemedi: {error}"))
                                    .size(10.0)
                                    .color(WARNING),
                            );
                        }
                        None if checking => {
                            ui.label(
                                RichText::new("Denetim arka planda sürüyor; tamamlandığında rapor otomatik kaydedilecek.")
                                    .size(10.0)
                                    .color(MUTED),
                            );
                        }
                        None => {
                            ui.label(
                                RichText::new("Tanılama raporu henüz oluşturulmadı.")
                                    .size(10.0)
                                    .color(MUTED),
                            );
                        }
                    }
                    ui.label(
                        RichText::new(crate::diagnostics::diagnostics_directory_hint())
                            .size(10.0)
                            .color(MUTED),
                    );
                    ui.label(
                        RichText::new(
                            "Kayıtlar yalnızca yerel diskte tutulur; hiçbir yere gönderilmez. Rapor kullanıcı/kurum adlarını, hesap bilgilerini, cihaz kimliklerini veya ham kayıt defteri/komut çıktılarını içermez.",
                        )
                        .size(10.0)
                        .color(MUTED),
                    );

                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(
                            "Bu denetim yalnızca okuma yapar. Grup İlkesi, kayıt defteri veya USB aygıtı değiştirilmez. Yönetim kaynağı belirlenemezse sonraki aç/kapat işlemi sunulmaz.",
                        )
                        .size(10.0)
                        .color(MUTED),
                    );
                }
                #[cfg(not(target_os = "windows"))]
                WriteProtectionPreflight::UnsupportedPlatform => {
                    ui.label(
                        RichText::new(
                            "Bu ilk ön kontrol Windows içindir. Diğer işletim sistemlerinin yazma koruması ayrı olarak değerlendirilecektir.",
                        )
                        .size(12.0)
                        .color(WARNING),
                    );
                }
            }
        });

    ui.add_space(12.0);
    egui::Frame::new()
        .fill(SURFACE_RAISED)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(egui::Margin::same(18))
        .show(ui, |ui| {
            ui.label(
                RichText::new("ÖNEMLİ GÜVENLİK SINIRI")
                    .size(10.0)
                    .strong()
                    .color(WARNING),
            );
            ui.add_space(6.0);
            ui.label(
                RichText::new(
                    "Bu ekran yalnızca Windows sistem bilgilerini ve HKLM'deki Grup İlkesi kayıt göstergesini okur. Ayar değiştirmez, yönetici yetkisi istemez ve USB aygıtını açmaz.",
                )
                .size(11.0)
                .color(TEXT),
            );
            ui.add_space(6.0);
            ui.label(
                RichText::new(
                    "İlke kaydının etkin görünmesi, belirli bir USB aygıtının fiilen salt okunur olduğunu tek başına kanıtlamaz. MDM/Intune yönetim işaretleri başlangıçta arka planda denetlenir; gerçek aygıt durumu ayrıca doğrulanmalıdır.",
                )
                .size(10.0)
                .color(MUTED),
            );
            ui.add_space(6.0);
            ui.label(
                RichText::new(
                    "Windows'un bu bilgisayar ilkesi tüm çıkarılabilir disk sınıfını etkiler; yalnızca DUYRA'nın seçtiği tek USB belleği değil.",
                )
                .size(10.0)
                .color(MUTED),
            );
        });
}

#[cfg(target_os = "windows")]
fn draw_preflight_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label(RichText::new(label).size(11.0).color(MUTED));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new(value).size(11.0).strong().color(TEXT));
        });
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

fn draw_status_indicator(ui: &mut egui::Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(8.0), Sense::hover());
    ui.painter().circle_filled(rect.center(), 3.5, color);
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
            update_available: false,
            error: Some("Güncelleme durumu kilitlenmiş.".to_owned()),
        });

    let is_busy = snapshot.error.is_none()
        && matches!(snapshot.stage.as_str(), "kontrol_ediliyor" | "kurulum");
    if is_busy {
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
                if is_busy {
                    ui.spinner();
                } else if snapshot.error.is_some() || snapshot.update_available {
                    draw_status_indicator(ui, WARNING);
                } else {
                    draw_status_indicator(ui, SUCCESS);
                }

                let status_text = if snapshot.error.is_some() {
                    "Güncelleme denetiminde sorun var".to_owned()
                } else {
                    match snapshot.stage.as_str() {
                        "desteklenmiyor" => {
                            "Otomatik güncelleme bu sistemde etkin değil".to_owned()
                        }
                        "kontrol_ediliyor" => "Güncellemeler denetleniyor".to_owned(),
                        "kontrol_bekleniyor" => {
                            "Sonraki güncelleme denetimi için süre bekleniyor".to_owned()
                        }
                        "güncelleme_bulundu" => format!(
                            "Yeni sürüm bulundu: {}",
                            snapshot.target_version.as_deref().unwrap_or("Yeni sürüm")
                        ),
                        "kurulum" => "Onaylandı: indiriliyor, doğrulanıyor ve kuruluyor".to_owned(),
                        "ertelendi" => format!(
                            "{} güncellemesi ertelendi",
                            snapshot.target_version.as_deref().unwrap_or("Yeni sürüm")
                        ),
                        "yeniden_başlatma" => {
                            "Güncelleme kuruldu; DUYRA yeniden başlatılıyor".to_owned()
                        }
                        _ => "DUYRA güncel".to_owned(),
                    }
                };

                let status_color = if snapshot.error.is_some() || snapshot.update_available {
                    WARNING
                } else {
                    MUTED
                };
                ui.label(RichText::new(status_text).size(10.0).color(status_color));

                if snapshot.update_available {
                    if ui.small_button("Şimdi güncelle").clicked() {
                        updater::install_approved_update(update_status);
                    }
                    if ui.small_button("Daha sonra").clicked() {
                        updater::defer_update(update_status);
                    }
                }

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
                    ui.add_space(8.0);
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
