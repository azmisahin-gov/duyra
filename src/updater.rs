use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use std::time::Duration;

const UPDATE_VERIFYING_KEY: self_update::VerifyingKey = [
    0x5a, 0x60, 0xc2, 0xbd, 0x9e, 0x0d, 0x6b, 0xeb, 0x7b, 0xdf, 0x7e, 0x81, 0xa5, 0x38, 0x5c, 0x7b,
    0x13, 0xa6, 0xde, 0x31, 0x45, 0xec, 0x77, 0x54, 0x93, 0xab, 0x63, 0x91, 0x3f, 0x2f, 0x18, 0x73,
];

const LOG_FILE_NAME: &str = "duyra.log";
const MAX_LOG_BYTES: u64 = 512 * 1024;

#[derive(Clone, Debug)]
pub struct UpdateSnapshot {
    pub current_version: String,
    pub stage: String,
    pub message: String,
    pub target_version: Option<String>,
    pub error: Option<String>,
}

impl UpdateSnapshot {
    fn initial() -> Self {
        Self {
            current_version: env!("CARGO_PKG_VERSION").to_owned(),
            stage: "başlatılıyor".to_owned(),
            message: "Güncelleme denetimi başlatılıyor.".to_owned(),
            target_version: None,
            error: None,
        }
    }
}

pub type SharedUpdateStatus = Arc<Mutex<UpdateSnapshot>>;

pub fn check_on_startup() -> SharedUpdateStatus {
    let status = Arc::new(Mutex::new(UpdateSnapshot::initial()));

    #[cfg(target_os = "windows")]
    {
        let worker_status = Arc::clone(&status);
        let spawn_result = std::thread::Builder::new()
            .name("duyra-guncelleyici".to_owned())
            .spawn(move || check_and_apply(worker_status));

        if let Err(error) = spawn_result {
            set_failure(
                &status,
                "iş parçacığı başlatma",
                &format!("Güncelleme denetimi başlatılamadı: {error}"),
            );
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        set_status(
            &status,
            "desteklenmiyor",
            "Otomatik güncelleme bu işletim sistemi derlemesinde etkin değil.",
            None,
        );
    }

    status
}

#[cfg(target_os = "windows")]
fn check_and_apply(status: SharedUpdateStatus) {
    let current_version = env!("CARGO_PKG_VERSION");
    log_event(
        "INFO",
        "başlangıç",
        &format!("Mevcut sürüm: {current_version}"),
    );
    set_status(
        &status,
        "github_kontrolü",
        "GitHub Releases üzerinde yeni sürüm aranıyor.",
        None,
    );

    let mut config = self_update::backends::github::Update::configure();
    config
        .repo_owner("azmisahin-gov")
        .repo_name("duyra")
        .bin_name("duyra")
        .target("x86_64-pc-windows-msvc")
        .asset_identifier("windows-update")
        .bin_path_in_archive("duyra.exe")
        .current_version(current_version)
        .verifying_keys(vec![UPDATE_VERIFYING_KEY])
        .check_install_path_writable(true)
        .timeout(Duration::from_secs(30))
        .retries(2)
        .no_confirm(true)
        .show_output(false)
        .show_download_progress(false);

    let updater = match config.build() {
        Ok(updater) => updater,
        Err(error) => {
            set_failure(&status, "güncelleyici hazırlama", &error.to_string());
            return;
        }
    };

    let release = match updater.is_update_available() {
        Ok(Some(release)) => release,
        Ok(None) => {
            log_event("INFO", "sürüm_kontrolü", "Yeni sürüm bulunmadı.");
            set_status(
                &status,
                "güncel",
                "DUYRA güncel. Yeni sürüm bulunamadı.",
                None,
            );
            return;
        }
        Err(error) => {
            set_failure(&status, "GitHub sürüm kontrolü", &error.to_string());
            return;
        }
    };

    let target_version = release.version().to_owned();
    log_event(
        "INFO",
        "sürüm_bulundu",
        &format!("Yeni sürüm bulundu: {target_version}"),
    );
    set_status(
        &status,
        "paket_indirme_ve_doğrulama",
        &format!("{target_version} bulundu. Paket indiriliyor; bütünlük ve imza doğrulanacak."),
        Some(target_version.clone()),
    );

    match updater.update() {
        Ok(result) if result.is_updated() => {
            let installed_version = result.version().to_owned();
            log_event(
                "INFO",
                "kurulum",
                &format!(
                    "Güncelleme uygulandı: {installed_version}. Uygulama yeniden başlatılıyor."
                ),
            );
            set_status(
                &status,
                "yeniden_başlatma",
                &format!("{installed_version} kuruldu. DUYRA yeniden başlatılıyor."),
                Some(installed_version),
            );

            let Err(error) = self_update::restart::restart();
            set_failure(&status, "yeniden başlatma", &error.to_string());
        }
        Ok(result) => {
            let installed_version = result.version().to_owned();
            log_event(
                "INFO",
                "kurulum",
                &format!("Güncelleme sırasında sürüm değişmedi: {installed_version}"),
            );
            set_status(
                &status,
                "güncel",
                "Güncelleme denetimi tamamlandı. Uygulama sürümü değişmedi.",
                None,
            );
        }
        Err(error) => {
            set_failure(
                &status,
                "indirme, imza doğrulama veya kurulum",
                &error.to_string(),
            );
        }
    }
}

fn set_status(
    shared: &SharedUpdateStatus,
    stage: &str,
    message: &str,
    target_version: Option<String>,
) {
    if let Ok(mut current) = shared.lock() {
        current.stage = stage.to_owned();
        current.message = message.to_owned();
        current.target_version = target_version;
        current.error = None;
    }
    log_event("INFO", stage, message);
}

fn set_failure(shared: &SharedUpdateStatus, stage: &str, error: &str) {
    let message =
        format!("Güncelleme tamamlanamadı ({stage}). DUYRA mevcut sürümle çalışmaya devam ediyor.");
    if let Ok(mut current) = shared.lock() {
        current.stage = stage.to_owned();
        current.message = message;
        current.error = Some(error.to_owned());
    }
    log_event("ERROR", stage, error);
}

fn log_directory() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .or_else(|| std::env::var_os("APPDATA"))
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);

    base.join("DUYRA").join("logs")
}

fn log_event(level: &str, stage: &str, detail: &str) {
    let directory = log_directory();
    if fs::create_dir_all(&directory).is_err() {
        return;
    }

    let path = directory.join(LOG_FILE_NAME);
    if fs::metadata(&path)
        .map(|metadata| metadata.len() > MAX_LOG_BYTES)
        .unwrap_or(false)
    {
        let previous = directory.join("duyra.log.1");
        let _ = fs::remove_file(&previous);
        let _ = fs::rename(&path, previous);
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{timestamp} [{level}] [{stage}] {detail}");
    }
}

pub fn open_log_folder() -> Result<(), String> {
    let directory = log_directory();
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    Command::new("explorer")
        .arg(&directory)
        .spawn()
        .map(|_| ())
        .map_err(|error| error.to_string())
}

pub fn support_summary(snapshot: &UpdateSnapshot) -> String {
    let mut error = snapshot.error.clone().unwrap_or_else(|| "Yok".to_owned());

    for (variable, replacement) in [
        ("USERPROFILE", "%USERPROFILE%"),
        ("LOCALAPPDATA", "%LOCALAPPDATA%"),
        ("APPDATA", "%APPDATA%"),
    ] {
        if let Some(value) = std::env::var_os(variable) {
            let value = value.to_string_lossy();
            if !value.is_empty() {
                error = error.replace(value.as_ref(), replacement);
            }
        }
    }

    format!(
        "DUYRA tanılama özeti\nSürüm: {}\nİşletim sistemi: {} {}\nAşama: {}\nHedef sürüm: {}\nHata: {}\n\nNot: Bu özet, kullanıcı tarafından isteğe bağlı paylaşım içindir. Günlük dosyası ve delil dosyaları bu metne eklenmez.",
        snapshot.current_version,
        std::env::consts::OS,
        std::env::consts::ARCH,
        snapshot.stage,
        snapshot.target_version.as_deref().unwrap_or("Belirlenmedi"),
        error,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_summary_contains_version_and_stage() {
        let snapshot = UpdateSnapshot {
            current_version: "0.1.4".to_owned(),
            stage: "GitHub sürüm kontrolü".to_owned(),
            message: "Test".to_owned(),
            target_version: Some("v0.1.5".to_owned()),
            error: Some("Bağlantı kurulamadı".to_owned()),
        };

        let summary = support_summary(&snapshot);
        assert!(summary.contains("0.1.4"));
        assert!(summary.contains("GitHub sürüm kontrolü"));
        assert!(summary.contains("v0.1.5"));
        assert!(summary.contains("Bağlantı kurulamadı"));
        assert!(!summary.contains("duyra.log"));
    }
}
