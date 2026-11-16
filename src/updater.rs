use std::time::Duration;

// Bu anahtar yalnızca DUYRA sürüm paketlerinin imzasını doğrulamak için kullanılır.
// Özel imzalama anahtarı uygulamaya veya kaynak depoya hiçbir zaman eklenmez.
const UPDATE_VERIFYING_KEY: self_update::VerifyingKey = [
    0x5a, 0x60, 0xc2, 0xbd, 0x9e, 0x0d, 0x6b, 0xeb,
    0x7b, 0xdf, 0x7e, 0x81, 0xa5, 0x38, 0x5c, 0x7b,
    0x13, 0xa6, 0xde, 0x31, 0x45, 0xec, 0x77, 0x54,
    0x93, 0xab, 0x63, 0x91, 0x3f, 0x2f, 0x18, 0x73,
];

/// Güncelleme denetimini arka planda başlatır.
/// Ağ veya yayın hatası uygulamanın açılmasını engellemez.
pub fn check_on_startup() {
    #[cfg(target_os = "windows")]
    {
        let _ = std::thread::Builder::new()
            .name("duyra-guncelleyici".to_owned())
            .spawn(check_and_apply);
    }
}

#[cfg(target_os = "windows")]
fn check_and_apply() {
    let mut config = self_update::backends::github::Update::configure();

    config
        .repo_owner("azmisahin-gov")
        .repo_name("duyra")
        .bin_name("duyra")
        .target("x86_64-pc-windows-msvc")
        .asset_identifier("windows-update")
        .bin_path_in_archive("duyra.exe")
        .current_version(env!("CARGO_PKG_VERSION"))
        .verifying_keys(vec![UPDATE_VERIFYING_KEY])
        .check_install_path_writable(true)
        .timeout(Duration::from_secs(30))
        .retries(2)
        .no_confirm(true)
        .show_output(false)
        .show_download_progress(false);

    let Ok(updater) = config.build() else {
        return;
    };

    let Ok(status) = updater.update() else {
        // Çevrimdışı kullanım, GitHub sınırları veya yazma izni sorunları
        // mevcut sürümün çalışmasını engellemez.
        return;
    };

    if status.is_updated() {
        // Windows'ta yeni dosya diske alındıktan sonra yeni süreç başlatılır.
        let _ = self_update::restart::restart();
    }
}
