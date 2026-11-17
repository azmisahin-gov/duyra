use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const UPDATE_VERIFYING_KEY: self_update::VerifyingKey = [
    0x5a, 0x60, 0xc2, 0xbd, 0x9e, 0x0d, 0x6b, 0xeb, 0x7b, 0xdf, 0x7e, 0x81, 0xa5, 0x38, 0x5c, 0x7b,
    0x13, 0xa6, 0xde, 0x31, 0x45, 0xec, 0x77, 0x54, 0x93, 0xab, 0x63, 0x91, 0x3f, 0x2f, 0x18, 0x73,
];

const LOG_FILE_NAME: &str = "duyra.log";
const MAX_LOG_BYTES: u64 = 512 * 1024;
const UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(6 * 60 * 60);
const INITIAL_RETRY_DELAY: Duration = Duration::from_secs(15 * 60);
const MAX_RETRY_DELAY: Duration = Duration::from_secs(4 * 60 * 60);

#[derive(Clone, Debug)]
pub struct UpdateSnapshot {
    pub current_version: String,
    pub stage: String,
    pub message: String,
    pub target_version: Option<String>,
    pub update_available: bool,
    pub error: Option<String>,
}

impl UpdateSnapshot {
    fn initial() -> Self {
        Self {
            current_version: env!("CARGO_PKG_VERSION").to_owned(),
            stage: "başlatılıyor".to_owned(),
            message: "Güncelleme denetimi hazırlanıyor.".to_owned(),
            target_version: None,
            update_available: false,
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
        let spawn_result = thread::Builder::new()
            .name("duyra-guncelleyici".to_owned())
            .spawn(move || update_check_loop(worker_status));

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
fn update_check_loop(status: SharedUpdateStatus) {
    let stamp_path = update_check_stamp_path();
    let guard = self_update::check_interval::UpdateCheckGuard::new(
        stamp_path.clone(),
        UPDATE_CHECK_INTERVAL,
    );
    let mut consecutive_failures = 0_u32;
    let mut retry_without_interval = false;

    loop {
        if !retry_without_interval {
            match guard.should_check() {
                Ok(true) => {}
                Ok(false) => {
                    let delay = remaining_check_delay(&stamp_path)
                        .unwrap_or(UPDATE_CHECK_INTERVAL);
                    if !delay.is_zero() {
                        if !preserve_current_update_notice(&status) {
                            set_status(
                                &status,
                                "kontrol_bekleniyor",
                                "Son kontrolün üzerinden altı saat geçince yeniden denetlenecek.",
                                None,
                            );
                        }
                        thread::sleep(delay + check_jitter());
                    }
                }
                Err(error) => {
                    log_event(
                        "WARN",
                        "kontrol_zamanlayıcısı",
                        &format!("Kontrol zamanı okunamadı; denetim sürdürülecek: {error}"),
                    );
                }
            }
        }
        retry_without_interval = false;

        if installation_in_progress(&status) {
            thread::sleep(Duration::from_secs(30));
            retry_without_interval = true;
            continue;
        }

        let stamp_saved = match guard.record_check() {
            Ok(()) => true,
            Err(error) => {
                log_event(
                    "WARN",
                    "kontrol_zamanlayıcısı",
                    &format!("Kontrol zamanı kaydedilemedi: {error}"),
                );
                false
            }
        };

        match check_for_update(&status) {
            Ok(()) => {
                consecutive_failures = 0;
                if !stamp_saved {
                    thread::sleep(UPDATE_CHECK_INTERVAL + check_jitter());
                }
            }
            Err(error) => {
                let delay = error
                    .rate_limit_delay()
                    .unwrap_or_else(|| retry_delay(consecutive_failures));
                consecutive_failures = consecutive_failures.saturating_add(1);
                log_event(
                    "WARN",
                    "kontrol_yeniden_deneme",
                    &format!(
                        "Güncelleme kontrolü başarısız. {} saniye sonra yeniden denenecek: {error}",
                        delay.as_secs()
                    ),
                );
                retry_without_interval = true;
                thread::sleep(delay);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn check_for_update(status: &SharedUpdateStatus) -> self_update::Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    log_event(
        "INFO",
        "sürüm_kontrolü",
        &format!("Mevcut sürüm: {current_version}"),
    );
    set_status(
        status,
        "kontrol_ediliyor",
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
            set_failure(status, "güncelleyici hazırlama", &error.to_string());
            return Err(error);
        }
    };

    match updater.is_update_available() {
        Ok(Some(release)) => {
            let target_version = release.version().to_owned();
            if let Ok(mut current) = status.lock() {
                current.stage = "güncelleme_bulundu".to_owned();
                current.message = format!(
                    "Yeni sürüm {target_version} bulundu. Kurmak için onayınızı bekliyor."
                );
                current.target_version = Some(target_version.clone());
                current.update_available = true;
                current.error = None;
            }
            log_event(
                "INFO",
                "sürüm_bulundu",
                &format!("Yeni sürüm bulundu; kullanıcı onayı bekleniyor: {target_version}"),
            );
            Ok(())
        }
        Ok(None) => {
            log_event("INFO", "sürüm_kontrolü", "Yeni sürüm bulunmadı.");
            set_status(
                status,
                "güncel",
                "DUYRA güncel. Yeni sürüm bulunamadı.",
                None,
            );
            Ok(())
        }
        Err(error) => {
            set_failure(status, "GitHub sürüm kontrolü", &error.to_string());
            Err(error)
        }
    }
}

pub fn install_approved_update(status: &SharedUpdateStatus) {
    #[cfg(target_os = "windows")]
    {
        let target_version = {
            let Ok(mut current) = status.lock() else {
                return;
            };
            if !current.update_available || current.stage != "güncelleme_bulundu" {
                return;
            }

            let target_version = current.target_version.clone();
            current.stage = "kurulum".to_owned();
            current.message =
                "Onaylanan güncelleme indiriliyor; imza ve bütünlük doğrulanacak.".to_owned();
            current.update_available = false;
            current.error = None;
            target_version
        };

        log_event(
            "INFO",
            "kullanıcı_onayı",
            &format!(
                "Kullanıcı güncellemeyi onayladı: {}",
                target_version.as_deref().unwrap_or("son sürüm")
            ),
        );

        let worker_status = Arc::clone(status);
        let spawn_result = thread::Builder::new()
            .name("duyra-guncelleme-kurulumu".to_owned())
            .spawn(move || apply_approved_update(worker_status));

        if let Err(error) = spawn_result {
            set_failure(status, "kurulum iş parçacığı", &error.to_string());
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        set_status(
            status,
            "desteklenmiyor",
            "Otomatik güncelleme bu işletim sistemi derlemesinde etkin değil.",
            None,
        );
    }
}

#[cfg(target_os = "windows")]
fn apply_approved_update(status: SharedUpdateStatus) {
    let current_version = env!("CARGO_PKG_VERSION");
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

    match updater.update() {
        Ok(result) if result.is_updated() => {
            let installed_version = result.version().to_owned();
            log_event(
                "INFO",
                "kurulum",
                &format!(
                    "Kullanıcı onaylı güncelleme uygulandı: {installed_version}. Uygulama yeniden başlatılıyor."
                ),
            );
            set_status(
                &status,
                "yeniden_başlatma",
                &format!("{installed_version} kuruldu. DUYRA yeniden başlatılıyor."),
                Some(installed_version),
            );

            if let Err(error) = self_update::restart::restart() {
                set_failure(&status, "yeniden başlatma", &error.to_string());
            }
        }
        Ok(result) => {
            let installed_version = result.version().to_owned();
            set_status(
                &status,
                "güncel",
                &format!(
                    "Güncelleme tamamlandı; kurulu sürüm değişmedi: {installed_version}."
                ),
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

pub fn defer_update(status: &SharedUpdateStatus) {
    let target_version = {
        let Ok(mut current) = status.lock() else {
            return;
        };
        if !current.update_available {
            return;
        }

        current.update_available = false;
        current.stage = "ertelendi".to_owned();
        current.message = format!(
            "{} güncellemesi ertelendi. Sonraki denetimde yeniden bildirilecek.",
            current.target_version.as_deref().unwrap_or("Yeni sürüm")
        );
        current.target_version.clone()
    };

    log_event(
        "INFO",
        "güncelleme_ertelendi",
        &format!(
            "Kullanıcı güncellemeyi erteledi: {}",
            target_version.as_deref().unwrap_or("sürüm belirtilmedi")
        ),
    );
}

fn installation_in_progress(status: &SharedUpdateStatus) -> bool {
    status
        .lock()
        .map(|current| current.stage == "kurulum")
        .unwrap_or(false)
}

fn preserve_current_update_notice(status: &SharedUpdateStatus) -> bool {
    status
        .lock()
        .map(|current| {
            current.update_available
                || current.stage == "ertelendi"
                || current.stage == "güncel"
                || current.error.is_some()
        })
        .unwrap_or(false)
}

fn update_check_stamp_path() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .or_else(|| std::env::var_os("APPDATA"))
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);

    base.join("DUYRA").join("update-check.stamp")
}

fn remaining_check_delay(stamp_path: &Path) -> Option<Duration> {
    let last_check = fs::read_to_string(stamp_path)
        .ok()?
        .trim()
        .parse::<u64>()
        .ok()?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();

    Some(remaining_check_delay_from_timestamps(last_check, now))
}

fn remaining_check_delay_from_timestamps(last_check: u64, now: u64) -> Duration {
    let elapsed = now.saturating_sub(last_check);
    UPDATE_CHECK_INTERVAL.saturating_sub(Duration::from_secs(elapsed))
}

fn check_jitter() -> Duration {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() % 901)
        .unwrap_or_default();
    Duration::from_secs(seconds)
}

fn retry_delay(failures: u32) -> Duration {
    INITIAL_RETRY_DELAY
        .saturating_mul(1_u32 << failures.min(4))
        .min(MAX_RETRY_DELAY)
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
        current.update_available = false;
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
        current.update_available = false;
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
        "DUYRA tanılama özeti\nSürüm: {}\nİşletim sistemi: {} {}\nAşama: {}\nHedef sürüm: {}\nGüncelleme var: {}\nHata: {}\n\nNot: Bu özet, kullanıcı tarafından isteğe bağlı paylaşım içindir. Günlük dosyası ve delil dosyaları bu metne eklenmez.",
        snapshot.current_version,
        std::env::consts::OS,
        std::env::consts::ARCH,
        snapshot.stage,
        snapshot.target_version.as_deref().unwrap_or("Belirlenmedi"),
        if snapshot.update_available { "Evet" } else { "Hayır" },
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
            update_available: true,
            error: Some("Bağlantı kurulamadı".to_owned()),
        };

        let summary = support_summary(&snapshot);
        assert!(summary.contains("0.1.4"));
        assert!(summary.contains("GitHub sürüm kontrolü"));
        assert!(summary.contains("v0.1.5"));
        assert!(summary.contains("Güncelleme var: Evet"));
        assert!(summary.contains("Bağlantı kurulamadı"));
        assert!(!summary.contains("duyra.log"));
    }

    #[test]
    fn update_interval_uses_the_remaining_time() {
        let five_hours_ago = 10_000;
        let now = five_hours_ago + 5 * 60 * 60;
        assert_eq!(
            remaining_check_delay_from_timestamps(five_hours_ago, now),
            Duration::from_secs(60 * 60)
        );
    }

    #[test]
    fn expired_update_interval_is_due_now() {
        let six_hours_ago = 10_000;
        let now = six_hours_ago + 6 * 60 * 60 + 10;
        assert_eq!(
            remaining_check_delay_from_timestamps(six_hours_ago, now),
            Duration::ZERO
        );
    }
}
