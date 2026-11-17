# Windows çıkarılabilir disk yazma engeli — tek seferlik VM kabul testi

**Durum:** Prosedür hazır; gerçek aygıt testi henüz yapılmış sayılmaz. Bu belge bir donanımsal yazma engelleyici sertifikasyonu değildir.

## Amaç ve sınır

Bu kabul testi, Windows'un Removable Disks: Deny write access ilkesinin yalnızca ilgili çıkarılabilir disk sınıfında, boş ve ayrılmış bir test USB belleğine etkisini gözlemlemek içindir. DUYRA politika kaydını salt okunur gösterir; aygıt yazma denemesini kendisi yapmaz.

Microsoft, RemovableDiskDenyWriteAccess ayarının ilgili çıkarılabilir depolama sınıfına yazmayı engellediğini belirtir. Ayar cihaz kapsamlıdır; Windows 10 sürüm 1809 ve sonrası ile Pro, Enterprise, Education ve IoT Enterprise edisyonlarında listelenir. Ağ paylaşımları, host klasör eşlemeleri ve sanal disk dosyaları bu politika tarafından otomatik korunmuş sayılmaz.

Windows Grup İlkesi konumu:

**Computer Configuration > Administrative Templates > System > Removable Storage Access > Removable Disks: Deny write access**

USB SSD'nin Windows tarafından çıkarılabilir disk ya da sabit disk olarak sınıflandırılması önemlidir. USB kart okuyucu/microSD kapsamı da okuyucunun Windows'a hangi aygıt sınıfı olarak sunulduğuna bağlıdır. İlk test için standart bir USB flash bellek kullan; bu tek testten USB SSD veya microSD davranışı çıkarma.

Kaynak: [Microsoft Learn — Policy CSP: Storage](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-storage), RemovableDiskDenyWriteAccess.

## Test öncesi durdurma koşulları

Aşağıdakilerden biri varsa testi başlatma:

- Ayrılmış Windows 11 Pro konuk VM tamamen kapatılamıyor veya snapshot alınamıyorsa.
- Konuk VM'nin etki alanı/MDM/kuruluş yönetiminde olmadığı doğrulanamıyorsa.
- DUYRA yönetim kaynağı değerlendirmesi Unknown veya Managed gösteriyorsa ve neden açıklığa kavuşturulmadıysa.
- Grup İlkesi ayarı başlangıçta Not Configured değilse. Önceden Enabled/Disabled ayarı değiştirme veya üzerine yazma.
- Test USB'sinin boş ve atılabilir olduğu kesin değilse.
- Test USB'sinin konuk VM'ye bağlandığı ve doğru aygıt olduğu kesin olarak doğrulanamıyorsa.
- USB bellekte donanımsal yazma koruma anahtarı varsa veya başlangıç yazma testi başarısızsa.

Bir durdurma koşulunda Registry'yi elle değiştirme, farklı bir aygıta geçme veya gerçek delil medyasını bağlama.

## A. İzole ortamı hazırla

1. Windows 11 Pro VM'yi tamamen kapat.
2. VirtualBox'ta temiz başlangıç durumunun snapshot'ını al ve snapshot'ın geri yüklenebilir olduğunu doğrula.
3. Test boyunca host klasör eşlemesi veya yazılabilir paylaşılan disk kullanma. Host'taki delil klasörlerine ve kişisel dosyalara erişme.
4. Yalnızca boş ve test için ayrılmış, yeniden kullanılabilir/atılabilir bir USB bellek seç. İçinde korunması gereken veri olmadığını doğrula.
5. VM'yi başlat ve USB belleği yalnızca konuk VM'ye USB passthrough ile bağla.
6. Windows Gezgini'nde görünen sürücünün test USB'si olduğunu doğrula. Host/guest sahipliği belirsizse durdur.

## B. Başlangıç durumunu doğrula

1. DUYRA > Ayarlar bölümünde Windows desteğini, yönetim kaynağı değerlendirmesini ve mevcut politika kaydını salt okunur gözlemle.
2. Yerel Grup İlkesi Düzenleyicisi'ni aç ve yukarıdaki Removable Disks: Deny write access ayarını bul.
3. Ayarın Not Configured olduğunu doğrula. Farklı bir durum görürsen değiştirme; testi durdur ve temiz snapshot'a dön.
4. Test USB belleğine benzersiz ve zararsız bir başlangıç dosyası oluştur. Dosyanın oluştuğunu doğrula, ardından yalnızca bu test dosyasını sil.
5. Başlangıç yazma testi başarısızsa devam etme. Test belleği veya bağlantısı zaten yazmaya kapalı/sorunlu olabilir.

## C. Politikayı yalnızca konuk VM'de uygula

1. Yerel Grup İlkesi Düzenleyicisi'nde Removable Disks: Deny write access ayarını aç.
2. Enabled seçeneğini işaretle ve uygula.
3. Politikanın konuk işletim sistemine uygulanmasını sağlamak için VM'yi yeniden başlat.
4. Gerekirse test USB belleğini konuk VM'den çıkarıp yeniden bağla. Host üzerinde USB'ye dosya kopyalama.

Bu işlem yalnızca konuk VM'nin yerel Grup İlkesi'ni değiştirir. DUYRA bu ayarı değiştirmez; Registry'ye yazmaz ve UAC istemez.

## D. Gerçek yazma davranışını gözle

1. Test USB belleğinin konuk VM'de göründüğünü ve okunabildiğini kontrol et.
2. Windows Gezgini üzerinden test belleğinde ikinci, yeni ve zararsız bir dosya oluşturmayı dene.
3. Beklenen sonuç, yazma girişiminin Windows tarafından reddedilmesidir.
4. Yazma başarılı olursa test başarısızdır. Politika kaydının Enabled görünmesi bu aygıtta yazmanın engellendiğini kanıtlamaz.
5. DUYRA Ayarlar ekranında politika kaydını ayrıca gözlemle. Gerçek aygıt yazma doğrulamasının DUYRA tarafından otomatik olarak Verified yapılmasını bekleme; uygulama testi yürütmez veya sertifikalandırmaz.

## E. Ayarı geri al ve ortamı temizle

1. Konuk Windows içindeki Yerel Grup İlkesi Düzenleyicisi'nde aynı ayarı Not Configured konumuna getir. Bu geri alma işlemini DUYRA değil, Windows yönetim aracı yapar.
2. VM'yi yeniden başlat ve gerekirse USB belleği yeniden bağla.
3. Test USB belleğinde yeni, zararsız bir dosya oluşturmayı dene. Yazmanın tekrar mümkün olması, konuk VM'de ilkenin geri alındığını doğrular.
4. Yalnızca test sırasında oluşturduğun dosyaları sil ve USB belleği Windows üzerinden güvenli şekilde çıkar.
5. VM'yi kapat. Test sırasında beklenmeyen bir durum oluştuysa USB güvenli biçimde ayrıldıktan sonra temiz snapshot'a dön.
6. Snapshot yalnızca konuk VM'nin sanal sistem durumunu geri yükler; fiziksel test USB belleğindeki dosyaları geri almaz. USB'deki test dosyalarını ayrıca sil ve bu medyayı başka amaçla kullanmadan önce durumunu doğrula.
7. Bu kabul işlemi tamamlandıktan sonra VirtualBox testlerini sonlandır; grafik uyumluluğu araştırmasına geri dönme.

## Sonuç kaydı

Issue #13'e yalnızca aşağıdaki sınırlı sonucu yaz:

- Konuk VM başlangıç politikası: Not Configured / farklı — farklıysa test durdurulur.
- Başlangıç yazma denemesi: başarılı / başarısız.
- Politika etkin görünümü: evet / hayır / belirsiz.
- Politika sonrası yazma denemesi: engellendi / engellenmedi / test edilemedi.
- Not Configured durumuna dönüş sonrası yazma: başarılı / başarısız / test edilemedi.
- Snapshot geri yükleme: başarılı / başarısız.
- Genel sonuç: başarılı / başarısız / belirsiz.

USB seri numarası, sürücü etiketi, kullanıcı adı, domain/tenant bilgisi, test dosyası yolu veya delil içeriği kaydetme. Sonuç yalnızca kullanılan konuk VM ve test USB belleği için geçerlidir; diğer USB SSD, microSD okuyucu, host klasör eşlemesi veya sanal diskler için genellenemez.

## Başarı ölçütü

Test ancak şu koşulların tümü sağlanırsa başarılı sayılır: başlangıçta USB yazılabilir durumdaydı; politika etkinleştirildiğinde yeni yazma reddedildi; ayar Not Configured konumuna geri alındığında yazma yeniden mümkün oldu; test medyası güvenli biçimde çıkarıldı ve konuk VM başlangıç snapshot'ına döndürüldü.

Aygıt sınıfı belirsizse, politika sonrası yazma başarılıysa veya geri dönüş doğrulanamıyorsa sonuç Belirsiz/Başarısız kalır. DUYRA gerçek delil koruması doğrulandı iddiasında bulunmaz.

## Kaynaklar

- [Microsoft Storage Policy CSP — RemovableDiskDenyWriteAccess](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-storage)
- [Microsoft ADMX Removable Storage Policy CSP](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-admx-removablestorage)
