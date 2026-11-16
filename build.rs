fn main() {
    println!("cargo:rerun-if-changed=assets/duyra.ico");

    // Kaynak dosyası yalnızca Windows hedefinde EXE içine gömülür.
    // macOS ve Linux derlemeleri bu Windows aracını çalıştırmaya ihtiyaç duymaz.
    if std::env::var("CARGO_CFG_TARGET_OS").ok().as_deref() != Some("windows") {
        return;
    }

    let mut resource = winresource::WindowsResource::new();
    resource.set_language(0x041F); // Türkçe (Türkiye)
    resource
        .set_icon("assets/duyra.ico")
        .set("CompanyName", "DUYRA")
        .set("FileDescription", "DUYRA - Bilirkişi çalışma asistanı")
        .set("InternalName", "duyra")
        .set(
            "LegalCopyright",
            "Telif hakkı © 2026 DUYRA. Tüm hakları saklıdır.",
        )
        .set("OriginalFilename", "duyra.exe")
        .set("ProductName", "DUYRA");

    resource
        .compile()
        .expect("Windows uygulama simgesi ve dosya bilgileri EXE içine eklenemedi");
}
