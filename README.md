# DUYRA

DUYRA, bilirkişinin ses materyalini güvenli ve yerel biçimde incelemesine yardımcı olmak üzere geliştirilen Rust masaüstü uygulamasıdır.

## Resmî indirme sayfası

**[DUYRA web sayfasını aç](https://azmisahin-gov.github.io/duyra/)**

**[En son Windows sürümünü indir](https://github.com/azmisahin-gov/duyra/releases/latest/download/duyra-windows-x86_64.exe)**

Uygulamayı bir kez indirin. DUYRA açılışta ve uygulama açık kaldığı sürece altı saatlik aralıklarla GitHub Releases üzerinde yeni sürüm arar. Kontrol aralığı kullanıcı bilgisayarında kaydedilir; uygulamanın kısa aralıklarla yeniden açılması gereksiz API isteği oluşturmaz. Yeni sürüm bulunduğunda alt durum çubuğunda bildirim gösterilir. **Kurulum yalnızca kullanıcı "Şimdi güncelle" düğmesine bastığında başlar.** İmzalı paket indirilir; Ed25519 imzası ve SHA-256 bütünlüğü doğrulanır; ardından uygulama kurulur ve yeniden başlatılır. "Daha sonra" seçeneği güncellemeyi bir sonraki denetime kadar erteler. DUYRA kendi kendine, kullanıcının onayı olmadan uygulama kapanışı veya yeniden başlatma yapmaz.

## Windows uygulama konumu

Uygulama dosyasını kullanıcı hesabında yazılabilir bir konumda tutun:

`%LOCALAPPDATA%\Programs\DUYRA\duyra-windows-x86_64.exe`

DUYRA günlükleri ve uygulama verileri ayrı tutulur:

`%LOCALAPPDATA%\DUYRA\`

İlk klasörün önceden bulunması gerekmez; kullanıcı tarafından oluşturulabilir. Uygulamayı `C:\` kök dizinine veya yönetici izni gerektiren bir klasöre koymayın. Otomatik güncelleme, uygulamanın bulunduğu klasöre yazabilmelidir.

## Otomatik sistem tanılaması

Windows'ta DUYRA açıldığında sistem ön kontrolü ve politika yönetim kaynağı denetimi arka planda otomatik çalışır. Sonuç tamamlandığında yerel bir JSON raporu oluşturulur:

- Rapor: `%LOCALAPPDATA%\DUYRA\diagnostics\latest-report.json`
- Olay kaydı: `%LOCALAPPDATA%\DUYRA\diagnostics\events.jsonl` (256 KiB sınırında döndürülür; önceki dosya `events.jsonl.1` olarak tutulur)

Ayarlar ekranındaki **Tanılama klasörünü aç** düğmesi bu klasörü açar. Kullanıcıdan PowerShell komutu, kayıt defteri çıktısı veya ekran görüntüsü toplaması beklenmez. Denetimler salt okunurdur; kayıt defteri/Grup İlkesi değiştirilmez, MDM bağlantısı kesilmez ve USB aygıtı açılmaz.

Tanılama kayıtları yerel diskte kalır; ağa, GitHub'a veya DUYRA geliştiricilerine gönderilmez. Rapor yalnızca uygulama sürümü, Windows edisyon/derleme bilgisi ve kontrol sonuçlarını içerir. Kullanıcı/hesap adı, cihaz/tenant kimliği, etki alanı adı, ham komut/registry çıktısı, dosya yolu veya USB kimliği kaydedilmez. Bu rapor sistem tanılamasıdır; inceleme/delil içeriğini kapsamaz.

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
