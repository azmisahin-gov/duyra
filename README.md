# DUYRA

DUYRA, Rust ile geliştirilen yerel bir masaüstü uygulamasıdır.

## İlk sürümün kapsamı

Şimdilik uygulama yalnızca ürün kimliğini gösterir:

- DUYRA logosu
- Uygulama adı
- Cargo.toml içinden alınan sürüm numarası

Ses işleme, deşifre, analiz, veri saklama ve rapor üretimi bu başlangıç sürümünün kapsamında değildir.

## Teknoloji

- Rust
- egui / eframe
- Yerel masaüstü arayüzü; Tauri, HTML, CSS veya JavaScript kullanılmaz.

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

İlk yerel derleme `Cargo.lock` dosyasını oluşturur. Uygulama deposunda bu dosya sürüm kontrolüne eklenmelidir.

## Sürümleme ve yayın

Sürüm numarası tek kaynak olarak `Cargo.toml` içindeki `package.version` alanından okunur.

`v*` etiketi gönderildiğinde GitHub Actions Windows x64 derlemesini oluşturup ZIP paketini GitHub Release'e ekler.

> Not: Bu ilk iskelette otomatik güncelleme kurulumu henüz etkin değildir. Otomatik güncelleme, imzalama anahtarı ve istemci tarafı imza doğrulaması birlikte yapılandırılıp test edildikten sonra açılacaktır. İmzasız paketler otomatik güncelleme yoluyla kurulmayacaktır.

## Proje ilkesi

Bugün küçük bir ürün yapıyoruz. Yarın büyümesine izin verecek kadar düzenli, bugün geliştirmeyi zorlaştırmayacak kadar sade tutuyoruz.
