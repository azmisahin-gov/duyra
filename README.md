# DUYRA

DUYRA, bilirkişinin ses materyalini güvenli ve yerel biçimde incelemesine yardımcı olmak üzere geliştirilen Rust masaüstü uygulamasıdır.

## Resmî indirme sayfası

**[DUYRA web sayfasını aç](https://azmisahin-gov.github.io/duyra/)**

**[En son Windows sürümünü indir](https://github.com/azmisahin-gov/duyra/releases/latest/download/duyra-windows-x86_64.exe)**

Uygulamayı bir kez indirin. Yeni sürüm yayımlandığında DUYRA açılışta GitHub Releases'i arka planda kontrol eder. Güncelleme paketi gömülü Ed25519 doğrulama anahtarıyla ve SHA-256 özetiyle doğrulanmadan yüklenmez. Doğrulama başarılı olursa uygulama kendini günceller ve yeni sürümle yeniden açılır.

## Windows uygulama konumu

Uygulama dosyasını kullanıcı hesabında yazılabilir bir konumda tutun:

`%LOCALAPPDATA%\Programs\DUYRA\duyra-windows-x86_64.exe`

DUYRA günlükleri ve uygulama verileri ayrı tutulur:

`%LOCALAPPDATA%\DUYRA\`

İlk klasörün önceden bulunması gerekmez; kullanıcı tarafından oluşturulabilir. Uygulamayı `C:\` kök dizinine veya yönetici izni gerektiren bir klasöre koymayın. Otomatik güncelleme, uygulamanın bulunduğu klasöre yazabilmelidir.

## İlk sürümün kapsamı

Ana ekran; Genel Bakış, İncelemeler, Delil Dosyaları, Ses İnceleme, Deşifre, Analiz, Raporlar ve Ayarlar için ortak gezinme düzenini sağlar. Henüz etkin olmayan bölümler açıkça belirtilir. Ses işleme ve delil iş akışları sırayla tamamlanacaktır.

## Teknoloji

- Rust
- egui / eframe
- İmzalı GitHub Releases güncelleyicisi
- Tauri, HTML, CSS ve JavaScript uygulama arayüzünde kullanılmaz.

## Geliştirme

Rust stable kurulu bir ortamda:

```powershell
cargo run
```

Kontroller:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Sürüm numarasının tek kaynağı `Cargo.toml` içindeki `package.version` alanıdır.

## Otomatik denetim ve yayın

Tek bir GitHub Actions iş akışı kod biçimini, Clippy denetimini ve testleri çalıştırır. Bu kontroller başarılı olduktan sonra `Cargo.toml` içindeki sürüm numarası henüz yayımlanmamışsa Windows x64 uygulamasını üretir, güncelleme paketini imzalar ve imzayı doğrular; ardından GitHub Releases'e yayımlar. Aynı sürüm zaten yayımlanmışsa yeni paket üretmez.

Yayın için depo ayarlarında `DUYRA_UPDATE_PRIVATE_KEY_BASE64` adlı Actions Secret tanımlı olmalıdır. Özel anahtar kaynak depoya veya yayın paketine eklenmez.

Web sayfası yalnızca `docs/` içeriği değiştiğinde yeniden yayımlanır. Sayfa, güncel sürüm numarasını ve sürüm notlarını GitHub Releases API'sinden canlı aldığı için her uygulama sürümünde ayrıca web sayfası dağıtımı gerekmez. Yeni bir özelliği web sayfasında tanıtmak için ilgili `docs/index.html` içeriğini güncellemek yeterlidir.

## Geliştirme ilkesi

[DUYRA geliştirme ilkeleri](GELISTIRME_ILKELERI.md): Tek hedef seç, tamamla, doğrula, sonra yayımla. Gerçek kullanıcı testi başarılı olmadan sonraki hedefe geçme.
