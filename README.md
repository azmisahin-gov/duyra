# DUYRA

**DUYRA**, bilirkişinin ses materyalini güvenli ve yerel biçimde incelemesine yardımcı olmak üzere geliştirilen masaüstü uygulamasıdır.

## İndir ve çalıştır

**[DUYRA'nın en son Windows sürümünü indir](https://github.com/azmisahin-gov/duyra/releases/latest)**

GitHub Releases sayfasında:

- `duyra-windows-x86_64.exe`: İndirip doğrudan çalıştırılabilir uygulama.
- `duyra-windows-x86_64.zip`: Uygulamanın ZIP paketi.
- `SHA256SUMS.txt`: İndirilen EXE dosyasının SHA-256 özeti.

İlk sürüm Windows x64 içindir. Kurulum sihirbazı gerektirmez. İndirilen EXE dosyasını açın. Windows, uygulama kod imzası henüz bulunmadığı için SmartScreen uyarısı gösterebilir.

## İlk sürüm

İlk sürüm yalnızca DUYRA logosunu, ürün adını ve sürüm numarasını gösterir. Ses işleme, deşifre, analiz, veri saklama ve rapor üretimi henüz bu sürümün kapsamında değildir.

## Teknoloji

- Rust
- egui / eframe
- Yerel masaüstü arayüzü
- Tauri, HTML, CSS ve JavaScript kullanılmaz.

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

## Yayın süreci

`main` dalına yeni bir sürüm numarasıyla gönderim yapıldığında GitHub Actions uygulamayı Windows x64 için derler, test eder ve aynı sürüm daha önce yayımlanmamışsa GitHub Releases sayfasında otomatik yayımlar. Sürüm numarası değişmediyse yeni yayın oluşturulmaz.

Otomatik uygulama güncellemesi henüz etkin değildir. İmzalı güncelleme mekanizması ve istemci tarafı imza doğrulaması tamamlanmadan güncelleme otomatik kurulmaz.

## Proje ilkesi

Bugün küçük bir ürün yapıyoruz. Yarın büyümesine izin verecek kadar düzenli, bugün geliştirmeyi zorlaştırmayacak kadar sade tutuyoruz.
