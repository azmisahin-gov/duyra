use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

const RECORD_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ExaminationRecord {
    pub schema_version: u32,
    pub id: String,
    pub number: String,
    pub created_at_unix: u64,
    pub status: String,
}

pub struct ExaminationStore {
    directory: Option<PathBuf>,
    records: Vec<ExaminationRecord>,
    storage_error: Option<String>,
    load_warnings: Vec<String>,
}

impl ExaminationStore {
    pub fn open_default() -> Self {
        match default_storage_directory() {
            Ok(directory) => Self::open_at(directory),
            Err(error) => Self {
                directory: None,
                records: Vec::new(),
                storage_error: Some(error),
                load_warnings: Vec::new(),
            },
        }
    }

    fn open_at(directory: PathBuf) -> Self {
        let mut store = Self {
            directory: Some(directory.clone()),
            records: Vec::new(),
            storage_error: None,
            load_warnings: Vec::new(),
        };

        if let Err(error) = fs::create_dir_all(&directory) {
            store.storage_error = Some(format!("İnceleme kayıt klasörü oluşturulamadı: {error}"));
            return store;
        }

        if let Err(error) = restrict_directory_permissions(&directory) {
            store.storage_error = Some(format!("İnceleme kayıt klasörü korunamadı: {error}"));
            return store;
        }

        if let Err(error) = store.load_records() {
            store.storage_error = Some(format!("İnceleme kayıtları okunamadı: {error}"));
        }

        store
    }

    pub fn records(&self) -> &[ExaminationRecord] {
        &self.records
    }

    pub fn storage_error(&self) -> Option<&str> {
        self.storage_error.as_deref()
    }

    /// Returns a fixed, privacy-safe startup state for local diagnostics.
    /// No path, record identifier, or underlying filesystem error is exposed.
    pub fn diagnostic_state(&self) -> (&'static str, &'static str) {
        if self.storage_error.is_some() {
            ("unknown", "examination_store_unavailable")
        } else if !self.load_warnings.is_empty() {
            ("warning", "examination_store_load_warnings")
        } else {
            ("ok", "examination_store_ready")
        }
    }

    pub fn notice(&self) -> Option<String> {
        if let Some(error) = &self.storage_error {
            return Some(format!(
                "İnceleme kayıt alanı kullanılamıyor. Yeni kayıt oluşturulamaz. {error}"
            ));
        }

        if self.load_warnings.is_empty() {
            return None;
        }

        Some(format!(
            "{} inceleme kaydı okunamadı. Sorunlu dosyalar silinmedi; kayıt klasöründe korundu.",
            self.load_warnings.len()
        ))
    }

    pub fn find(&self, id: &str) -> Option<&ExaminationRecord> {
        self.records.iter().find(|record| record.id == id)
    }

    pub fn create_record(&mut self) -> Result<ExaminationRecord, String> {
        if let Some(error) = &self.storage_error {
            return Err(format!("İnceleme kayıt alanı hazır değil: {error}"));
        }

        let directory = self
            .directory
            .as_ref()
            .ok_or_else(|| "İnceleme kayıt klasörü belirlenemedi.".to_owned())?;

        let id = Uuid::new_v4().to_string();
        let compact_id = id.replace('-', "").to_ascii_uppercase();
        let number = format!("DYR-{}", &compact_id[..12]);
        let created_at_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("Sistem saati okunamadı: {error}"))?
            .as_secs();

        let record = ExaminationRecord {
            schema_version: RECORD_SCHEMA_VERSION,
            id: id.clone(),
            number,
            created_at_unix,
            status: "Hazırlanıyor".to_owned(),
        };

        let serialized = serde_json::to_vec_pretty(&record)
            .map_err(|error| format!("İnceleme kaydı hazırlanamadı: {error}"))?;
        let temporary_path = directory.join(format!(".{id}.tmp"));
        let record_path = directory.join(format!("{id}.json"));

        let write_result = (|| -> Result<(), String> {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary_path)
                .map_err(|error| format!("Geçici kayıt dosyası oluşturulamadı: {error}"))?;

            file.write_all(&serialized)
                .map_err(|error| format!("İnceleme kaydı yazılamadı: {error}"))?;
            file.sync_all()
                .map_err(|error| format!("İnceleme kaydı diske aktarılamadı: {error}"))?;
            drop(file);

            fs::rename(&temporary_path, &record_path)
                .map_err(|error| format!("İnceleme kaydı tamamlanamadı: {error}"))?;

            Ok(())
        })();

        if let Err(error) = write_result {
            let _ = fs::remove_file(&temporary_path);
            return Err(error);
        }

        self.records.push(record.clone());
        self.sort_records();
        Ok(record)
    }

    fn load_records(&mut self) -> Result<(), String> {
        let directory = self
            .directory
            .as_ref()
            .ok_or_else(|| "İnceleme kayıt klasörü belirlenemedi.".to_owned())?;
        let entries = fs::read_dir(directory)
            .map_err(|error| format!("Kayıt klasörü listelenemedi: {error}"))?;

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    self.load_warnings.push(error.to_string());
                    continue;
                }
            };

            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }

            let record_bytes = match fs::read(&path) {
                Ok(bytes) => bytes,
                Err(error) => {
                    self.load_warnings.push(error.to_string());
                    continue;
                }
            };

            let record: ExaminationRecord = match serde_json::from_slice(&record_bytes) {
                Ok(record) => record,
                Err(error) => {
                    self.load_warnings.push(error.to_string());
                    continue;
                }
            };

            if record.schema_version != RECORD_SCHEMA_VERSION
                || Uuid::parse_str(&record.id).is_err()
            {
                self.load_warnings.push(
                    path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                );
                continue;
            }

            self.records.push(record);
        }

        self.sort_records();
        Ok(())
    }

    fn sort_records(&mut self) {
        self.records
            .sort_by_key(|record| std::cmp::Reverse(record.created_at_unix));
    }
}

#[cfg(target_os = "windows")]
fn default_storage_directory() -> Result<PathBuf, String> {
    let local_app_data = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok_or_else(|| "Windows kullanıcı uygulama verisi klasörü bulunamadı.".to_owned())?;

    Ok(local_app_data.join("DUYRA").join("incelemeler"))
}

#[cfg(target_os = "linux")]
fn default_storage_directory() -> Result<PathBuf, String> {
    if let Some(xdg_data_home) = std::env::var_os("XDG_DATA_HOME") {
        let path = PathBuf::from(xdg_data_home);
        if path.is_absolute() {
            return Ok(path.join("duyra").join("incelemeler"));
        }
    }

    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| "Kullanıcı veri klasörü bulunamadı.".to_owned())?;

    Ok(home
        .join(".local")
        .join("share")
        .join("duyra")
        .join("incelemeler"))
}

#[cfg(target_os = "macos")]
fn default_storage_directory() -> Result<PathBuf, String> {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| "Kullanıcı veri klasörü bulunamadı.".to_owned())?;

    Ok(home
        .join("Library")
        .join("Application Support")
        .join("DUYRA")
        .join("incelemeler"))
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn default_storage_directory() -> Result<PathBuf, String> {
    Err("Bu işletim sistemi için inceleme kayıt konumu tanımlı değil.".to_owned())
}

fn restrict_directory_permissions(directory: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(directory, fs::Permissions::from_mode(0o700))
            .map_err(|error| error.to_string())?;
    }

    #[cfg(not(unix))]
    let _ = directory;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_state_is_ok_for_an_empty_usable_store() {
        let directory = std::env::temp_dir().join(format!("duyra-test-{}", Uuid::new_v4()));
        let store = ExaminationStore::open_at(directory.clone());

        assert_eq!(store.diagnostic_state(), ("ok", "examination_store_ready"));

        fs::remove_dir_all(directory).expect("Test kayıt klasörü temizlenebilmeli.");
    }

    #[test]
    fn diagnostic_state_warns_without_deleting_unreadable_records() {
        let directory = std::env::temp_dir().join(format!("duyra-test-{}", Uuid::new_v4()));
        fs::create_dir_all(&directory).expect("Test klasörü oluşturulabilmeli.");
        let invalid_record = directory.join("unreadable.json");
        fs::write(&invalid_record, b"{ invalid json").expect("Test kaydı yazılabilmeli.");

        let store = ExaminationStore::open_at(directory.clone());

        assert_eq!(
            store.diagnostic_state(),
            ("warning", "examination_store_load_warnings")
        );
        assert!(invalid_record.exists(), "Sorunlu kayıt korunmalı.");

        fs::remove_dir_all(directory).expect("Test kayıt klasörü temizlenebilmeli.");
    }

    #[test]
    fn diagnostic_state_is_unknown_when_storage_path_is_unavailable() {
        let file_path = std::env::temp_dir().join(format!("duyra-test-{}", Uuid::new_v4()));
        fs::write(&file_path, b"not a directory").expect("Engelleyici dosya oluşturulabilmeli.");

        let store = ExaminationStore::open_at(file_path.clone());

        assert_eq!(
            store.diagnostic_state(),
            ("unknown", "examination_store_unavailable")
        );

        fs::remove_file(file_path).expect("Engelleyici test dosyası silinebilmeli.");
    }

    #[test]
    fn examination_record_is_available_after_reopening_store() {
        let directory = std::env::temp_dir().join(format!("duyra-test-{}", Uuid::new_v4()));
        let mut store = ExaminationStore::open_at(directory.clone());

        assert!(store.storage_error().is_none());

        let created = store
            .create_record()
            .expect("İnceleme kaydı oluşturulabilmeli.");

        let reopened = ExaminationStore::open_at(directory.clone());

        assert!(reopened.storage_error().is_none());
        assert_eq!(reopened.records().len(), 1);
        assert_eq!(reopened.records()[0], created);
        assert_eq!(
            reopened.find(&created.id).map(|record| &record.number),
            Some(&created.number)
        );

        fs::remove_dir_all(directory).expect("Test kayıt klasörü temizlenebilmeli.");
    }

    #[test]
    fn each_examination_receives_a_distinct_number() {
        let directory = std::env::temp_dir().join(format!("duyra-test-{}", Uuid::new_v4()));
        let mut store = ExaminationStore::open_at(directory.clone());

        let first = store.create_record().expect("İlk kayıt oluşturulmalı.");
        let second = store.create_record().expect("İkinci kayıt oluşturulmalı.");

        assert_ne!(first.id, second.id);
        assert_ne!(first.number, second.number);
        assert!(first.number.starts_with("DYR-"));

        fs::remove_dir_all(directory).expect("Test kayıt klasörü temizlenebilmeli.");
    }
}
