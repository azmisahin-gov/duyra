# Windows çıkarılabilir disk yazma engeli — tek seferlik VM kabul testi

**Durum:** Prosedür hazır; gerçek aygıt testi henüz yapılmış sayılmaz. Bu belge bir donanımsal yazma engelleyici sertifikasyonu değildir.

## Amaç ve sınır

Bu kabul testi, Windows'un **Removable Disks: Deny write access** ilkesinin yalnızca ilgili çıkarılabilir disk sınıfında, boş ve ayrılmış bir test USB belleğine etkisini doğrular.

Microsoft, `RemovableDiskDenyWriteAccess` ayarının ilgili çıkarılabilir depolama sınıfına yazmayı engellediğini belirtir. İlke tek başına bütün USB bağlantılarını, sabit disk olarak sınıflandırılan USB-SSD'leri, ağ paylaşımlarını, VirtualBox sanal disk dosyalarını veya host klasör eşlemelerini korumaz. Test edilen aygıtın Windows tarafından hangi sınıfta sunulduğu kabul kaydına yazılmalıdır.

Windows Grup İlkesi konumu:

`Computer Configuration > Administrative Templates > System > Removable Storage Access > Removable Disks: Deny write access`

Microsoft'un Storage Policy CSP belgesi bu ayarı cihaz kapsamlı ve Windows 10 sürüm 1809 ve sonrası ile Pro, Enterprise, Education ve IoT Enterprise edisyonlarında desteklenen bir ilke olarak tanımlar. Kurum/MDM yönetimi görülen veya yönetim kaynağı belirsiz konuk sistemde bu testi yapma.

## Güvenlik kapısı

Aşağıdakilerin tümü sağlanmadan teste başlama:

- Ayrılmış Windows 11 konuk VM kapalıyken snapshot alındı ve snapshot'ın geri yüklenebilir olduğu biliniyor.
- VM'de paylaşılan host klasörleri, yazılabilir paylaşılan diskler ve gereksiz USB yönlendirmeleri kapalı.
- Yalnızca boş, yeniden kullanılabilir olmaması sorun yaratmayacak, test için ayrılmış bir USB bellek kullanılacak. Gerçek delil USB/SSD/microSD'si kesinlikle kullanılmayacak.
- Konuk Windows etki alanına/MDM'ye kayıtlı değil ve politika yönetim kaynağı belirsiz değil. Bilinmiyor veya yönetiliyor sonucu çıkarsa dur.
- Host üzerindeki Registry veya Grup İlkesi değiştirilmez. USB passthrough sırasında host bu test medyasına erişemeyebilir; bu yalnızca ayrılmış test medyası için kabul edilir.

**USB-SSD veya microSD ile test**, Windows'un aygıtı ilgili çıkarılabilir disk sınıfında sunduğu doğrulanmadıkça bu ilke için kabul testi sayılmaz. Sınıf net değilse sonuç “belirsiz” olarak kaydedilir; cihazın davranışından koruma genellemesi yapılmaz.

## Uygulama

1. Snapshot alındıktan sonra yalnızca test USB belleğini konuk VM'ye USB passthrough ile bağla. Host'ta aygıtın aynı anda yazılabilir bağlı kalmadığını doğrula.
2. Konuk Windows'ta aygıtın beklenen çıkarılabilir disk olarak tanındığını ve doğru test medyası olduğunu doğrula. Aygıt sınıfı belirsizse devam etme.
3. İlke henüz etkin değilken test medyasına zararsız bir işaret dosyası yazıp okunabildiğini doğrula. Bu adım yalnızca ayrılmış test medyasında yapılır.
4. Konuk Windows'un Yerel Grup İlkesi Düzenleyicisi üzerinden yukarıdaki **Removable Disks: Deny write access** ayarını **Enabled** yap. DUYRA bu ayarı değiştirmez; bu işlem yalnızca izole konuk VM'de yetkili Windows yönetim aracıyla yapılır.
5. Konuk Windows politikasının uygulanmasını bekle; gerekirse konuk sistemi yeniden başlat. Ayarın etkin olduğunu Windows yönetim aracı/ilke görünümünden doğrula.
6. Test USB belleğinde ikinci, yeni bir işaret dosyası oluşturmayı dene. Beklenen sonuç yazma erişiminin Windows tarafından reddedilmesidir. Mevcut test dosyasını yalnızca okuyarak kontrol et; biçimlendirme, bölümleme veya silme denemesi yapma.
7. DUYRA açıksa Ayarlar ekranında yalnızca politika kaydının durumunu gözlemle. DUYRA'nın “gerçek aygıt yazma davranışı” durumu testin sonucu ne olursa olsun kendiliğinden değişmez; uygulama bu testi gerçekleştirmez veya sertifikalandırmaz.

## Geri dönüş ve kayıt

1. Konuk Windows içinde test USB belleğini güvenle kaldır ve VM'yi normal biçimde kapat.
2. Test öncesi alınmış snapshot'ı geri yükle. Bu yalnızca konuk VM'nin durumunu ve içindeki ilke ayarlarını geri döndürür; fiziksel test USB belleğinde test sırasında oluşturulmuş dosyaları silmez.
3. Test USB belleğini delil medyası olarak yeniden kullanma. Üzerindeki test işaret dosyaları kalabilir; medya yalnızca test amacıyla ayrılmış olarak tutulmalıdır.
4. Sonucu aşağıdaki şablonla yerel geliştirme/ kabul kaydına yaz. Cihazın seri numarasını, delil içeriğini veya kişisel bilgileri rapora ekleme.

## Kabul sonucu

**Başarılı** yalnızca şu koşulların tümü sağlanırsa kaydedilebilir: test medyasının çıkarılabilir disk sınıfında olduğu doğrulandı; politika öncesi yazma başarılıydı; yönetim aracı ilkesi etkinleştirdi; ilke sonrası yeni dosya oluşturma reddedildi; konuk VM snapshot'a geri döndürüldü.

Aşağıdaki durumlardan herhangi birinde sonuç **Belirsiz/Başarısız** olur: aygıt sabit disk olarak sunuluyor veya sınıfı belirlenemiyor; yönetim kaynağı belirsiz/yönetiliyor; test yazma öncesi başarısız; ilke sonrası yazma hâlâ başarılı; VM snapshot geri yüklemesi doğrulanamıyor.

### Kayıt şablonu

- Test tarihi:
- Konuk Windows sürümü/edisyonu:
- VM snapshot adı:
- Test medyası sınıfı (çıkarılabilir disk / sabit disk / belirsiz):
- İlke öncesi yazma: başarılı / başarısız
- İlke etkin görünümü: evet / hayır / belirsiz
- İlke sonrası yeni dosya yazma: engellendi / başarılı / belirsiz
- Snapshot geri yükleme: başarılı / başarısız
- Sonuç: başarılı / başarısız / belirsiz
- Not: yalnızca bu konuk VM ve test medyası için geçerlidir.

## Kaynaklar

- [Microsoft Storage Policy CSP — RemovableDiskDenyWriteAccess](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-storage)
- [Microsoft ADMX Removable Storage Policy CSP](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-admx-removablestorage)
