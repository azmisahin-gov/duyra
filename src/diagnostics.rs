//! DUYRA için yerel, sınırlı ve kimlik bilgisi içermeyen tanılama kayıtları.
//!
//! Tanılama kaydı yalnızca uygulamanın sabit kodlu kontrol kimliklerini ve sonuç
//! durumlarını saklar. Kullanıcı/cihaz/kurum adları, hesaplar, GUID'ler, seri
//! numaraları, ham registry değerleri veya komut çıktıları bu modüle aktarılmaz.

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;

const EVENT_LOG_NAME: &str = "events.jsonl";
const PREVIOUS_EVENT_LOG_NAME: &str = "events.jsonl.1";
const REPORT_FILE_NAME: &str = "latest-report.json";
const MAX_EVENT_LOG_BYTES: u64 = 256 * 1024;
const DIAGNOSTIC_SCHEMA_VERSION: u32 = 1;

static DIAGNOSTIC_LOCK: Mutex<()> = Mutex::new(());

#[derive(Serialize)]
struct DiagnosticEvent {
    schema_version: u32,
    timestamp_unix: u64,
    application_version: &'static str,
    component: &'static str,
    check: &'static str,
    status: &'static str,
    code: &'static str,
}

fn timestamp_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn diagnostics_directory() -> Result<PathBuf, &'static str> {
    #[cfg(target_os = "windows")]
    {
        let base = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .ok_or("Windows kullanıcı veri klasörü bulunamadı.")?;
        Ok(base.join("DUYRA").join("diagnostics"))
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(state_home) = std::env::var_os("XDG_STATE_HOME") {
            let path = PathBuf::from(state_home);
            if path.is_absolute() {
                return Ok(path.join("duyra").join("diagnostics"));
            }
        }

        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or("Kullanıcı veri klasörü bulunamadı.")?;
        Ok(home
            .join(".local")
            .join("state")
            .join("duyra")
            .join("diagnostics"))
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or("Kullanıcı veri klasörü bulunamadı.")?;
        Ok(home
            .join("Library")
            .join("Logs")
            .join("DUYRA")
            .join("diagnostics"))
    }
}

fn prepare_directory() -> Result<PathBuf, &'static str> {
    let directory = diagnostics_directory()?;
    fs::create_dir_all(&directory).map_err(|_| "Tanılama klasörü oluşturulamadı.")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))
            .map_err(|_| "Tanılama klasörü erişim izinleri korunamadı.")?;
    }

    Ok(directory)
}

fn append_event_at(
    path: &std::path::Path,
    component: &'static str,
    check: &'static str,
    status: &'static str,
    code: &'static str,
) -> Result<(), &'static str> {
    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory).map_err(|_| "Tanılama klasörü oluşturulamadı.")?;
    }

    if fs::metadata(path)
        .map(|metadata| metadata.len() > MAX_EVENT_LOG_BYTES)
        .unwrap_or(false)
    {
        let previous = path.with_file_name(PREVIOUS_EVENT_LOG_NAME);
        let _ = fs::remove_file(&previous);
        fs::rename(path, previous).map_err(|_| "Önceki tanılama kaydı döndürülemedi.")?;
    }

    let event = DiagnosticEvent {
        schema_version: DIAGNOSTIC_SCHEMA_VERSION,
        timestamp_unix: timestamp_unix(),
        application_version: env!("CARGO_PKG_VERSION"),
        component,
        check,
        status,
        code,
    };
    let serialized =
        serde_json::to_vec(&event).map_err(|_| "Tanılama kaydı serileştirilemedi.")?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|_| "Tanılama kaydı açılamadı.")?;
    file.write_all(&serialized)
        .and_then(|()| file.write_all(b"\n"))
        .map_err(|_| "Tanılama kaydı yazılamadı.")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|_| "Tanılama kaydı erişim izinleri korunamadı.")?;
    }

    Ok(())
}

/// Uygulama genelindeki modüllerin kullanabileceği, yalnızca sabit kodlu alanları
/// kabul eden olay kaydı. Serbest metin veya ham sistem çıktısı kabul etmez.
pub fn record_event(
    component: &'static str,
    check: &'static str,
    status: &'static str,
    code: &'static str,
) {
    let Ok(_guard) = DIAGNOSTIC_LOCK.lock() else {
        return;
    };
    let Ok(directory) = prepare_directory() else {
        return;
    };
    let _ = append_event_at(&directory.join(EVENT_LOG_NAME), component, check, status, code);
}

pub fn record_application_startup() {
    record_event("application", "startup", "ok", "application_started");
}

#[cfg(target_os = "windows")]
fn tri_state(value: Option<bool>, positive: &'static str, negative: &'static str) -> &'static str {
    match value {
        Some(true) => positive,
        Some(false) => negative,
        None => "unknown",
    }
}

#[cfg(target_os = "windows")]
fn write_report_atomically(path: &std::path::Path, bytes: &[u8]) -> Result<(), &'static str> {
    let parent = path.parent().ok_or("Tanılama rapor yolu geçersiz.")?;
    let temporary = parent.join(".latest-report.tmp");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temporary)
        .map_err(|_| "Tanılama raporu oluşturulamadı.")?;

    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|_| "Tanılama raporu diske yazılamadı.")?;
    drop(file);

    fs::rename(&temporary, path).map_err(|_| {
        let _ = fs::remove_file(&temporary);
        "Tanılama raporu tamamlanamadı."
    })?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|_| "Tanılama raporu erişim izinleri korunamadı.")?;
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn write_windows_report(
    preflight: &crate::system::WriteProtectionPreflight,
    authority: &crate::system::PolicyAuthorityCheck,
) -> Result<PathBuf, &'static str> {
    use crate::system::{
        PolicyChangeAssessment, WritePolicyStatus, WriteProtectionPreflight,
        WriteProtectionSupport,
    };

    let _guard = DIAGNOSTIC_LOCK
        .lock()
        .map_err(|_| "Tanılama kaydı kullanılamıyor.")?;
    let directory = prepare_directory()?;

    let (windows_name, edition, build, elevated, support, computer_policy) = match preflight {
        WriteProtectionPreflight::Windows(status) => {
            let policy = match &status.computer_policy {
                WritePolicyStatus::Enabled => "enabled",
                WritePolicyStatus::Disabled => "disabled",
                WritePolicyStatus::NotConfigured => "not_configured",
                WritePolicyStatus::UnexpectedValue(_) => "unexpected_value",
                WritePolicyStatus::Unknown(_) => "unknown",
            };
            let support = match &status.support {
                WriteProtectionSupport::Supported => "supported",
                WriteProtectionSupport::UnsupportedEdition(_) => "unsupported_edition",
                WriteProtectionSupport::UnsupportedBuild(_) => "unsupported_build",
                WriteProtectionSupport::Unknown => "unknown",
            };

            (
                status.product_name.as_deref().unwrap_or("unknown"),
                status.edition_id.as_deref().unwrap_or("unknown"),
                status.build_number,
                status.process_is_elevated,
                support,
                policy,
            )
        }
    };

    let assessment = match authority.assessment() {
        PolicyChangeAssessment::Managed => "managed_signal_detected",
        PolicyChangeAssessment::Reviewable => "no_known_management_signal",
        PolicyChangeAssessment::Unknown => "unknown_or_incomplete",
    };

    let report = serde_json::json!({
        "schema_version": DIAGNOSTIC_SCHEMA_VERSION,
        "generated_at_unix": timestamp_unix(),
        "application": {
            "name": "DUYRA",
            "version": env!("CARGO_PKG_VERSION"),
            "platform": std::env::consts::OS,
            "architecture": std::env::consts::ARCH
        },
        "system": {
            "windows_product": windows_name,
            "windows_edition": edition,
            "windows_build": build,
            "application_is_elevated": elevated
        },
        "checks": {
            "windows_write_protection_support": support,
            "computer_deny_write_policy": computer_policy,
            "group_policy_applied": tri_state(
                authority.group_policy_applied,
                "applied",
                "not_detected"
            ),
            "mdm_enrollment_registry_marker": tri_state(
                authority.mdm_enrollment_detected,
                "present_ambiguous_historical_marker",
                "not_detected"
            ),
            "mdm_storage_policy": tri_state(
                authority.mdm_policy_present,
                "present",
                "not_detected"
            ),
            "domain_join": tri_state(
                authority.domain_joined,
                "joined",
                "not_joined"
            ),
            "current_deny_write_value": tri_state(
                authority.current_policy_value_present,
                "present",
                "not_present"
            ),
            "policy_authority_assessment": assessment
        },
        "privacy": {
            "user_or_account_names_included": false,
            "device_or_tenant_identifiers_included": false,
            "domain_name_included": false,
            "registry_values_or_raw_command_output_included": false,
            "file_paths_or_usb_identifiers_included": false,
            "network_data_or_files_uploaded": false
        },
        "interpretation": {
            "mdm_enrollment_marker_alone_proves_active_management": false,
            "unknown_assessment_allows_policy_changes": false,
            "policy_was_modified_by_this_check": false,
            "usb_devices_were_accessed": false
        }
    });

    let serialized = serde_json::to_vec_pretty(&report)
        .map_err(|_| "Tanılama raporu hazırlanamadı.")?;
    let path = directory.join(REPORT_FILE_NAME);
    write_report_atomically(&path, &serialized)?;

    let _ = append_event_at(
        &directory.join(EVENT_LOG_NAME),
        "windows",
        "policy_authority",
        assessment,
        "read_only_check_completed",
    );
    let _ = append_event_at(
        &directory.join(EVENT_LOG_NAME),
        "diagnostics",
        "report",
        "created",
        "windows_system_report_written",
    );

    Ok(path)
}

pub fn diagnostics_directory_hint() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "%LOCALAPPDATA%\\DUYRA\\diagnostics"
    }
    #[cfg(target_os = "linux")]
    {
        "$XDG_STATE_HOME/duyra/diagnostics (veya ~/.local/state/duyra/diagnostics)"
    }
    #[cfg(target_os = "macos")]
    {
        "~/Library/Logs/DUYRA/diagnostics"
    }
}

pub fn open_diagnostics_folder() -> Result<(), &'static str> {
    let directory = prepare_directory()?;

    #[cfg(target_os = "windows")]
    let result = Command::new("explorer.exe").arg(&directory).spawn();

    #[cfg(target_os = "linux")]
    let result = Command::new("xdg-open").arg(&directory).spawn();

    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(&directory).spawn();

    result
        .map(|_| ())
        .map_err(|_| "Tanılama klasörü açılamadı.")
}

#[cfg(test)]
mod tests {
    use super::{append_event_at, diagnostics_directory_hint};
    use std::{fs, path::PathBuf};

    fn temporary_path() -> PathBuf {
        std::env::temp_dir().join(format!(
            "duyra-diagnostics-test-{}-{}.jsonl",
            std::process::id(),
            super::timestamp_unix()
        ))
    }

    #[test]
    fn event_log_contains_only_fixed_diagnostic_fields() {
        let path = temporary_path();
        append_event_at(
            &path,
            "application",
            "startup",
            "ok",
            "application_started",
        )
        .expect("event should be written");

        let line = fs::read_to_string(&path).expect("event should be readable");
        let event: serde_json::Value =
            serde_json::from_str(line.trim()).expect("event should be valid JSON");

        assert_eq!(event["component"], "application");
        assert_eq!(event["check"], "startup");
        assert_eq!(event["status"], "ok");
        assert_eq!(event["code"], "application_started");
        assert!(event.get("username").is_none());
        assert!(event.get("device_id").is_none());
        assert!(event.get("tenant_id").is_none());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn diagnostic_directory_hint_is_not_a_user_specific_path() {
        let hint = diagnostics_directory_hint();
        assert!(!hint.contains("Users\\"));
        assert!(!hint.contains("/home/"));
    }
}
