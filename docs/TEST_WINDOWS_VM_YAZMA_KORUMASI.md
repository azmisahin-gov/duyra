# Windows VM yazma koruması — tek seferlik kabul testi

**Durum:** Kullanıcı tarafından yürütülmesi gereken, henüz yapılmamış test prosedürü. Bu belge testin başarılı olduğunu iddia etmez.

## Amaç ve sınırlar

Bu prosedür, Windows'un **Removable Disks: Deny write access** bilgisayar ilkesinin boş ve ayrılmış bir USB test belleğinde gerçekten yazmayı engelleyip engellemediğini, DUYRA'nın politika kaydını nasıl raporladığından ayrı olarak gözlemlemek içindir.

- Test yalnızca snapshot alınmış, ayrılmış Windows 11 Pro konuk VM'sinde yapılır.
- Kişisel Windows host üzerindeki Registry/Grup İlkesi değiştirilmez.
- Gerçek delil USB/SSD/microSD aygıtı hiçbir aşamada bağlanmaz.
- Sadece boş, yeniden kullanılabilir/atılabilir bir test USB belleği kullanılır.
- DUYRA Registry/Grup İlkesi değiştirmez, yönetici yükseltmesi istemez, MDM ayarlarını değiştirmez ve USB aygıtını kendisi açmaz.
- VM host klasör eşlemeleri, paylaşılan klasörler ve sanal disk dosyaları bu testin kapsamı dışındadır.

## Politika kapsamı

Microsoft dokümantasyonunda bu ayar cihaz kapsamlıdır; Windows 10 sürüm 1809 ve sonrası ile Pro, Enterprise, Education ve IoT Enterprise edisyonları için listelenir. Grup İlkesi yolu:

**Computer Configuration > Administrative Templates > System > Removable Storage Access > Removable Disks: Deny write access**

Etkinleştirildiğinde yazma erişimi ilgili çıkarılabilir disk sınıfında reddedilir. Bu, tek bir DUYRA incelemesi veya yalnızca seçilen USB için özel bir ayar değildir. USB SSD'nin Windows tarafından çıkarılabilir disk olarak mı yoksa sabit disk olarak mı sınıflandırıldığı ayrıca önemlidir; ilk doğrulama için standart bir USB flash bellek kullanılması tercih edilir. USB kart okuyucu/microSD kapsamı bu tek testten çıkarılamaz.

Kaynak: [Microsoft Learn — Policy CSP: Storage](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-storage), “RemovableDiskDenyWriteAccess”.

## Test öncesi durdurma koşulları

Aşağıdakilerden biri varsa testi başlatma:

- VM snapshot'ı alınamıyorsa.
- VM'nin kuruluş/etki alanı/MDM tarafından yönetilmediği doğrulanamıyorsa.
- DUYRA yönetim kaynağı değerlendirmesi Unknown veya Managed gösteriyorsa ve bunun nedeni açıklığa kavuşturulmadıysa.
- Test USB'sinin boş ve atılabilir olduğu kesin değilse.
- Test USB'sinin host yerine konuk VM'ye bağlandığı kesin olarak doğrulanamıyorsa.
- Grup İlkesi ayarı başlangıçta zaten Enabled veya Disabled ise. Önceki durumun ne olduğu belgelenmeden değiştirme; bu prosedür yalnızca başlangıçta Not Configured olan temiz test VM'si için tasarlanmıştır.
- USB bellekte donanımsal yazma koruma anahtarı varsa veya temel yazma testi başarısızsa.

Bir durdurma koşulunda Registry'yi elle değiştirme, farklı bir aygıtı deneme veya gerçek delil medyasına geçme.

## A. İzole ortamı hazırla

1. Windows 11 Pro VM'yi tamamen kapat.
2. VirtualBox'ta mevcut temiz duruma ait snapshot al ve snapshot'ın göründüğünü doğrula.
3. Test sırasında host klasör eşlemesi kullanma. Host'taki delil klasörlerine veya kişisel dosyalara erişme.
4. Boş ve atılabilir USB belleği seç. İçinde korunması gereken veri olmadığını fiziksel olarak doğrula.
5. VM'yi başlat. USB belleği yalnızca konuk VM'ye USB passthrough ile bağla.
6. Windows Gezgini'nde görünen sürücünün test USB'si olduğunu doğrula. Aygıtın host/guest sahipliği belirsizse durdur.

## B. Başlangıç durumunu doğrula

1. DUYRA > Ayarlar bölümünde Windows desteği ve yazma politikası durumunu oku. DUYRA'nın bu ekrandaki sonucu yalnızca salt okunur ön değerlendirmedir.
2. Yerel Grup İlkesi Düzenleyicisi'ni aç.
3. Yukarıdaki Computer Configuration yolunda **Removable Disks: Deny write access** ayarını bul.
4. Ayarın **Not Configured** olduğunu doğrula. Farklı bir durum görürsen değiştirme; testi durdur ve snapshot'a dön.
5. USB belleğin normal yazılabilir olduğunu doğrulamak için test belleğine benzersiz bir başlangıç dosyası oluştur, dosyanın gerçekten oluştuğunu doğrula ve ardından dosyayı sil.
6. Başlangıç yazma testi başarısızsa politika testine geçme. Test belleği/bağlantısı zaten yazmaya kapalı veya sorunlu olabilir.

## C. Politikayı yalnızca konuk VM'de uygula

1. Yerel Grup İlkesi Düzenleyicisi'nde **Removable Disks: Deny write access** ayarını aç.
2. **Enabled** seçeneğini işaretle ve uygula.
3. Ayarın konuk işletim sistemine uygulanmasını sağlamak için VM'yi yeniden başlat.
4. Gerekirse USB belleği konuk VM'den çıkarıp yeniden bağla. Host üzerinde sürücüye dosya kopyalama.

Bu işlem sadece konuk VM'nin yerel Grup İlkesi'ni değiştirir. DUYRA bu işlemi yapmaz.

## D. Gerçek yazma davranışını gözle

1. USB belleğin konuk VM'de göründüğünü ve okunabildiğini kontrol et.
2. Windows Gezgini üzerinden test belleğinde yeni, zararsız bir deneme dosyası oluşturmayı dene.
3. Beklenen sonuç, yazma girişiminin Windows tarafından reddedilmesidir.
4. Yazma başarılı olursa test başarısızdır: politika kaydının Enabled görünmesine rağmen bu test aygıtında yazma engellenmemiştir. Sonucu başarılı olarak işaretleme.
5. DUYRA Ayarlar ekranına dön. Politika kaydı Enabled görünse bile gerçek aygıt yazma doğrulamasını DUYRA'nın otomatik olarak Verified yapmasını bekleme; test sonucu bu prosedürde elle kaydedilir.

## E. Ayarı geri al ve ortamı temizle

1. Yerel Grup İlkesi Düzenleyicisi'nde aynı ayarı **Not Configured** konumuna getir.
2. VM'yi yeniden başlat ve gerekirse USB belleği yeniden bağla.
3. Test USB'sinde yeni, zararsız bir dosya oluşturmayı dene; bu kez yazmanın tekrar mümkün olduğunu doğrula.
4. Oluşturduğun test dosyalarını sil ve USB belleği Windows üzerinden güvenli şekilde çıkar.
5. VM'yi kapat. Test sırasında beklenmeyen bir durum oluştuysa temiz snapshot'a dön.
6. VirtualBox testlerini bu kabul işlemi tamamlandıktan sonra sonlandır; grafik uyumluluğu araştırmasına geri dönme.

## Sonuç kaydı

Issue #13'e yalnızca şu sınırlı sonucu yaz:

- VM politikası başlangıç durumu: Not Configured / farklı — farklıysa test durdurulur.
- Başlangıç yazma denemesi: başarılı / başarısız.
- Politika sonrası yazma denemesi: engellendi / engellenmedi / test edilemedi.
- Geri alma sonrası yazma denemesi: başarılı / başarısız / test edilemedi.
- Test tamamlandı mı: evet / hayır.

USB seri numarası, sürücü etiketi, kullanıcı adı, domain/tenant bilgisi, test dosyası yolu veya delil içeriği kaydetme. Sonuç yalnızca kullanılan konuk VM ve test USB belleği için geçerlidir; diğer USB SSD, microSD okuyucu, host klasör eşlemesi veya sanal diskler için genellenemez.

## Başarı ölçütü

Test ancak başlangıç yazması başarılı, politika etkin durumdayken yazma engellenmiş ve politika Not Configured durumuna geri alındıktan sonra yazma yeniden başarılı olmuşsa bu tek cihaz/VM senaryosu için başarılı sayılır. Diğer tüm sonuçlar belirsiz veya başarısız olarak kalır; DUYRA gerçek delil koruması doğrulandı iddiasında bulunmaz.
