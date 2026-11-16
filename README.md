# DUYRA

DUYRA, bilirkişinin ses materyalini güvenli ve yerel biçimde incelemesine yardımcı olmak üzere geliştirilen Rust masaüstü uygulamasıdır.

## Resmî indirme sayfası

**[DUYRA web sayfasını aç](https://azmisahin-gov.github.io/duyra/)**

**[En son Windows sürümünü indir](https://github.com/azmisahin-gov/duyra/releases/latest/download/duyra-windows-x86_64.exe)**

Uygulamayı bir kez indirin. Yeni sürüm yayımlandığında DUYRA açılışta GitHub Releases'i arka planda kontrol eder. Güncelleme paketi gömülü Ed25519 doğrulama anahtarıyla ve SHA-256 özetiyle doğrulanmadan yüklenmez. Doğrulama başarılı olursa uygulama kendini günceller ve yeni sürümle yeniden açılır.

İlk sürüm Windows x64 içindir. Uygulama dosyasının bulunduğu klasörün kullanıcı tarafından yazılabilir olması gerekir. Güncelleme için internet bağlantısı gerekir; ağ veya yayın hatası uygulamanın mevcut sürümde açılmasını engellemez.

## İlk sürümün kapsamı

İlk ekran DUYRA logosunu, ürün adını ve sürüm numarasını gösterir. Ses işleme, deşifre, analiz, veri saklama ve rapor üretimi henüz bu sürümün kapsamında değildir.

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

## Sürüm yayımlama

GitHub Actions, sürüm numarası değiştiğinde Windows x64 uygulamasını derler, test eder, güncelleme ZIP'ini özel anahtarla imzalar ve GitHub Releases'e yükler. İmzalama anahtarı kaynak depoya konmaz.

Yayın için depo ayarlarında `DUYRA_UPDATE_PRIVATE_KEY_BASE64` adlı Actions Secret tanımlanmış olmalıdır. Secret bulunmazsa sürüm otomatik yayımlanmaz.

## Proje ilkesi

Bugün küçük bir ürün yapıyoruz. Yarın büyümesine izin verecek kadar düzenli, bugün geliştirmeyi zorlaştırmayacak kadar sade tutuyoruz.
