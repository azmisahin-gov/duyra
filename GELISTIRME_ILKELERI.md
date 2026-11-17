# DUYRA geliştirme ilkeleri ve ürün kararları

> Bugün küçük bir ürün yapıyoruz. Yarın büyümesine izin verecek kadar düzenli, bugün geliştirmeyi zorlaştırmayacak kadar sade tutuyoruz.

Bu belge, DUYRA'nın çalışma biçimi ve ürün kararları için ortak başvuru noktasıdır. Geliştirici, bilirkişi, yeni ekip üyesi veya başka bir yapay zekâ yardımcısı projeye buradan devam edebilmelidir. Sohbette ortaya çıkan fikirler, burada uygun sınıfa alınmadıkça uygulanacak özellik veya kesin karar sayılmaz.

## 1. Çalışma kuralımız

**Tek hedef seç. Tamamla. Doğrula. Sonra yayımla. Doğrulamadan sonraki hedefe geçme.**

1. Her çalışmada yalnızca bir kullanıcı ihtiyacını ele alırız. Kapsamı ve başarı ölçütünü işe başlamadan belirleriz.
2. İhtiyaç dışı altyapı, bağımlılık, ayar ve belge eklemeyiz. Bugünkü işi çözmeyen karmaşıklığı sonraya bırakırız.
3. Kod değişikliğini önce hızlı denetimlerden geçiririz: biçim, Clippy, test ve Windows derlemesi.
4. Denetimler geçince özellik tamamlanmış sayılmaz. Gerçek kullanıcı akışını da kabul ölçütlerine göre deneriz.
5. Hata görülürse yeni özelliğe geçmeyiz. Önce hatayı görünür kılar, tekrar üretir, düzeltir ve aynı testi yeniden yaparız.
6. Bir Release'in yayımlanması, özelliğin kullanıcıda çalıştığının kanıtı değildir. Kullanıcı doğrulaması tamamlanmadan işi kapatmayız.

## 2. Yeni fikirleri ve kararları yönetme

DUYRA geliştirilirken yeni fikirler çıkması beklenir. Fikirlerin unutulmaması gerekir; ancak her fikir mevcut hedefi değiştirmez.

Her yeni konu aşağıdaki sınıflardan biriyle kaydedilir:

- **Temel ilke:** Ürünün ve çalışma yönteminin değişmemesi gereken yönü.
- **Alınmış ürün kararı:** Üzerinde anlaşılan, ileride uygulaması yapılacak tasarım tercihi.
- **Değerlendirilecek konu:** Henüz teknik, hukuki veya kullanıcı deneyimi yönünden araştırılması gereken fikir.
- **Gelecek işi:** Sırası geldiğinde ayrı hedef olarak ele alınacak iş.
- **Mevcut hedef:** Şu anda tamamlanmakta olan tek iş.

Yeni fikirler mevcut hedefin kapsamına sessizce eklenmez. Önce bu belgeye kısa ve doğru bir not düşülür. Mevcut hedef tamamlanıp doğrulandıktan sonra önceliklendirme yapılır. Bir fikri kaydetmek, onu hemen geliştirme taahhüdü değildir.

Bu belge, kararların ve ertelenen konuların tek kaynağıdır. Commit geçmişi değişiklikleri ve tarihçeyi, sürüm notları yayımlanan sürümün gerçek içeriğini, web sayfası ise kullanıcıya dönük indirme ve tanıtım bilgisini taşır. Aynı ürün kararlarını Release notlarına veya web sayfasına kopyalayarak çoğaltmayız.

## 3. Ürün yaklaşımı

DUYRA, bilirkişinin ses materyalini güvenli ve yerel biçimde incelemesine, yaptığı işlemleri takip etmesine ve rapor hazırlamasına yardımcı olan masaüstü aracıdır.

- Uygulama, bilirkişinin mesleki değerlendirmesinin yerine geçmez; makine çıktısını kesin bulgu gibi sunmaz.
- Arayüzde çalışmayan işlevler varmış gibi gösterilmez. Hazır olmayan bölümler açıkça belirtilir.
- Teknik kayıtlar, yapılan işlemi ve kullanılan yöntemi açıklamaya yardımcı olur. Uygulamanın tek başına hukuki uygunluk veya delil kabulü garantisi verdiği iddia edilmez.
- Öncelik yeni özellik sayısı değil; doğru, anlaşılır, güvenilir ve doğrulanabilir iş akışıdır.

## 4. Delil bütünlüğü ve izlenebilirlik

**Temel ilke: DUYRA, orijinal delil dosyasını değiştirmemelidir.** Delil kaynağı ile delilden üretilen çalışma çıktıları birbirinden ayrı ele alınır.

İleride uygulanacak inceleme akışı şu gereksinimleri karşılamalıdır:

- Kaynak dosyanın tanımı, konumu, boyutu ve kriptografik özeti (hash) kaydedilebilmelidir.
- Kaynak salt okunur incelenebilmeli veya bilirkişinin seçtiği yöntemle ayrı bir çalışma kopyası oluşturulabilmelidir. Çalışma kopyası kullanılırsa kaynak ve kopya hash değerleri karşılaştırılarak doğrulanmalıdır.
- Salt okunur dosya açmak tek başına bütün fiziksel ortamın yazmaya karşı korunduğu anlamına gelmez. Gerekli olduğunda uygun yazma engelleme ve ortam koruma tedbirleri ayrıca değerlendirilmelidir.
- Hangi kaynağın, hangi yöntemle, ne zaman ve hangi uygulama/model sürümüyle işlendiği kaydedilmelidir.
- Dinleme işaretleri, zaman damgaları, deşifre taslağı, bilirkişi düzeltmeleri, notlar ve rapor sürümleri izlenebilir olmalıdır.
- Delil içeriği veya tanılama verisi kullanıcı açıkça seçmeden ağa gönderilmemelidir.

Bu maddeler teknik tasarım hedefidir; uygulanmadan önce ilgili mesleki standartlar ve yürürlükteki mevzuat ayrıca incelenmelidir.

## 5. İnceleme kayıtlarının kalıcılığı

Her şeyi yalnızca bellekte (RAM) tutmak temel yaklaşım olmayacaktır. RAM, ses çözümleme ve geçici işlem tamponları için kullanılabilir; ancak kullanıcının inceleme emeği kalıcı olarak saklanabilmelidir.

- İnceleme kaydı, kaynak delil dosyasından ayrı tutulmalıdır.
- Uygulama kapatılıp açıldığında kullanıcı önceki incelemelerine ulaşabilmeli ve kaydedilmiş çalışmasına devam edebilmelidir.
- Kaynak çıkarılmış, taşınmış veya değişmişse uygulama bunu sessizce geçiştirmemeli; yeniden doğrulama istemelidir.
- Notlar, düzeltmeler ve rapor taslakları kontrollü biçimde kaydedilmeli; değişiklik geçmişi gerektiğinde görülebilmelidir.
- Delil dosyasını uygulama veri klasörüne gelişigüzel kopyalamak varsayılan davranış olmamalıdır. Saklama, kopyalama ve yedekleme yöntemi açıkça tasarlanmalıdır.

Veri tabanı, şifreleme, yedekleme ve saklama politikalarının ayrıntıları ayrı tasarım hedefleri olarak ele alınacaktır; bu belge bunların hemen uygulanmasını istemez.

## 6. Güncelleme güvenliği ve kullanıcı kontrolü

Mevcut sürümün güncelleyicisi açılışta kontrol yapar ve yeni sürüm bulunduğunda otomatik kurulum/yeniden başlatma davranışına sahiptir. Bu davranış, delil inceleme iş akışları eklenmeden önce güvenli hâle getirilmelidir.

**Kararlaştırılan hedef davranış:**

- Güncelleme kontrolü varsayılan olarak 6 saatte bir yapılır.
- Yeni sürüm bulunduğunda kullanıcı bilgilendirilir.
- Paket arka planda indirilebilir; ancak imza ve bütünlük doğrulaması tamamlanmadan kurulamaz.
- Kurulum kullanıcı onayıyla veya kullanıcının açıkça etkinleştirdiği güvenli/boşta kalma politikasına göre yapılabilir.
- Açık inceleme, kaydedilmemiş değişiklik veya devam eden işlem varken uygulama kendiliğinden kapanıp yeniden başlamaz.
- Kullanıcı güncellemeyi erteleyebilir.

Bu bölüm hedeflenen davranışı tarif eder; kodda uygulanıp test edilmeden mevcut özellik gibi tanıtılmaz.

## 7. İmzalama anahtarı ile lisansın ayrılması

Güncelleme paketlerini doğrulamak için kullanılan imzalama anahtarı, kullanım lisansı veya kullanıcı yetkilendirmesi değildir.

- Güncelleme imzalama private key yalnızca yayın altyapısında gizli tutulur; uygulamaya veya kullanıcıya verilmez. Uygulamadaki public key yayın paketinin doğrulanması içindir.
- İleride davet, lisans veya kurumsal yetkilendirme gerekirse ayrı bir lisans imzalama/doğrulama düzeni tasarlanır.
- Lisans anahtarları, güncelleme anahtarlarıyla aynı amaçta veya aynı yerde kullanılmaz.
- Ücretsiz, davetli ve ücretli dağıtım seçenekleri henüz karara bağlanmamıştır. Bu nedenle bugünkü geliştirmeye lisans sunucusu veya hesap sistemi eklenmez.

## 8. Tanılama ve gizlilik

- Güncelleme hataları Windows kullanıcı uygulama verisi altındaki DUYRA/logs/duyra.log dosyasına yazılır; dosya boyutu sınırlandırılır.
- Günlükler ve tanılama bilgileri hiçbir zaman kendiliğinden ağa veya GitHub'a gönderilmez.
- Kullanıcı isterse uygulama içinden günlük klasörünü açabilir veya kişisel klasör yolları ayıklanmış tanılama özetini kopyalayabilir.
- Tam günlük dosyası paylaşılmadan önce kullanıcı tarafından incelenmelidir. Delil dosyaları, kayıt içerikleri ve hassas dosya yolları destek kaydına eklenmemelidir.

- Windows yazma koruması ve yönetim kaynağı ön kontrolleri uygulama açılışında otomatik, arka planda ve salt okunur çalışır. Sonuçlar delil inceleme kayıtlarına değil, ayrı tanılama alanına yazılır.
- Windows tanılama raporu `%LOCALAPPDATA%\\DUYRA\\diagnostics\\latest-report.json` konumunda tutulur; sabit kodlu olay kayıtları `events.jsonl` dosyasına eklenir. Olay dosyası 256 KiB sınırında döndürülür.
- Tanılama klasörü Ayarlar ekranından açılabilir. Kullanıcıdan PowerShell, registry çıktısı veya ekran görüntüsü ile elle bilgi taşıması beklenmez.
- Rapor yalnızca uygulama sürümü, Windows edisyon/derleme bilgisi ve kontrol durumlarını içerir. Hesap/kullanıcı adı, cihaz/tenant kimliği, domain adı, özel dosya yolu, ham komut/registry çıktısı, delil içeriği veya USB seri/aygıt kimliği kaydedilmez.
- Tanılama kayıtları yerel diskte kalır ve kendiliğinden ağa, GitHub'a veya geliştiriciye gönderilmez. Paylaşım gerektiğinde kullanıcı raporu kendisi inceler ve açıkça paylaşır.
- Bu altyapı uygulama geneline yeniden kullanılabilir sabit bileşen/kontrol kodu/durum kayıtları sağlar. Serbest metin veya ham sistem çıktısı tanılama API'sine verilmez.
- Güncelleme hataları için `DUYRA/logs/duyra.log` ayrı kalır; sistem tanılama raporunun kapsamı değildir.
## 9. Bitti sayma ve yayın ölçütü

Bir iş ancak aşağıdakiler sağlandığında tamamdır:

- Tanımlanan işi gerçekten yapıyor.
- Hata ve sınır durumları anlaşılır şekilde ele alınıyor.
- Otomatik kontroller geçiyor.
- Gerçek kullanım testi başarılı; sonucu sürüm ve gözlenen davranışla kaydedilmiş.
- Yalnızca gerekli kod, test ve kısa sürüm notu depoya eklenmiş.

Otomatik güncelleme için kabul testi:

1. Bilgisayarda önceki kararlı sürüm kurulu/açılabilir durumda tutulur.
2. Yeni sürüm GitHub Release'e yayımlanır ve imzalı paket doğrulanır.
3. Eski sürüm açılır; kontrolün ve güncelleme sonucunun kullanıcıya görünür olduğu doğrulanır.
4. Paket indirilir, imza ve bütünlük doğrulanır, uygulama değiştirilir ve yeniden başlatılır.
5. Yeniden açılan uygulamanın sürüm numarası beklenen yeni sürüm olmalıdır.
6. Başarısızlıkta neden kaydedilir; eski uygulama kullanılabilir kalır. Başarı kanıtlanmadan yeni özelliğe geçilmez.

**Yayın kontrol listesi:** kod denetimi → Windows derlemesi → imzalı paketi doğrulama → Release → önceki sürümden gerçek güncelleme → yeni sürüm numarasını doğrulama → kullanıcı kabulü.

## 10. Son tamamlanan hedef ve sıradaki karar

**Tamamlanan hedef: CI üzerinden DUYRA ana ekranının otomatik görüntüsünü almak ve görüntüyü incelemek.**

- GitHub Actions üzerinde `.github/workflows/gorsel-denetim.yml` ile ayrı bir görsel denetim workflow'u kuruldu.
- Windows hosted runner iki denemede de uygulamayı derledi; ancak sanal grafik aygıtı `Microsoft Hyper-V Video` olarak göründü ve `egui_glow` için gerekli OpenGL 2.0+ bağlamı oluşturulamadı. İkinci denemede stderr açıkça `OpenGL(PainterError("egui_glow requires opengl 2.0+."))` hatasını verdi. Windows ekran yakalama yolu bu nedenle zorlanmadı (Actions runs 43987188203 ve 43987622095).
- İlk Linux denemesi `winit` platform özelliği etkin olmadığı için derlemede başarısız oldu (Actions run 43988044823). Üretim Cargo ayarını değiştirmeden yalnızca görsel deneme komutuna `eframe/x11` özelliği eklendi.
- Son Linux denemesinde DUYRA, Xvfb sanal ekranında Mesa `llvmpipe` yazılım OpenGL çizicisiyle başarıyla derlendi ve açıldı; pencere bulundu ve PNG üretildi (Actions run 43988327972).
- PNG, 1180×780 boyutundadır. Artifact kimliği `14286377159`, artifact adı `duyra-gorsel-denetim-linux-5b9e25578e3bce364781cda9de73cd1d56f524df`; GitHub Actions üzerinde 7 gün saklanır. Görüntü Release paketine veya web sitesine eklenmez.
- Artifact indirildi ve gerçekten açılarak incelendi. Genel Bakış yerleşimi, gezinme, kartlar, metinler ve durum çubuğu görünür; görüntüde gerçek delil veya kullanıcı verisi yoktur.
- Aynı commit'in normal Windows CI denetimi de başarılıdır (Actions run 43988327942).
- Linux görseli, ortak egui arayüzü için otomatik erken regresyon kontrolüdür; Windows başlık çubuğu, Windows DPI/driver davranışı veya Windows'a özgü sistem bilgilerini kanıtlamaz. Windows sürümünün gerçek kullanıcı kabulü gerektiğinde kullanıcı ekran görüntüsüyle ayrıca doğrulanır.

**Güncelleme kabul testi beklemede:** v0.1.9'un "Şimdi güncelle / Daha sonra" bildirimi, sonraki doğal Release yayımlandığında gerçek kullanıcı bilgisayarında sınanacaktır. Sırf test amacıyla boş Release çıkarılmaz.

**Sıradaki ürün hedefi henüz başlatılmadı.** İlk açılış pencere konumunun ortalanması/son konumun hatırlanması gözlemi 12.4'te kayıtlıdır; sonraki hedef seçilirken dikkate alınacaktır.

## 11. Kapsamı küçük tutma

- Yeni özellik başına ayrı hedef, kabul ölçütü ve gerekli olduğunda ayrı Release.
- Teknik belge yalnızca karar, kullanım veya bakım için gerekiyorsa eklenir.
- Geliştirme ilkeleri ve ürün kararları bu dosyada tutulur; README buraya bağlantı verir.
- Çalışmayan veya sınanmamış bir davranış hiçbir yerde tamamlanmış gibi anlatılmaz.

## 12. Sonraki hedefler ve ekran doğrulama yöntemi

Bu bölüm sıradaki işleri görünür kılar; işler mevcut hedef tamamlanmadan başlatılmaz.

### 12.1. Otomatik güncelleyiciyi güvenli hâle getirme

**Kod hedefi v0.1.9'da uygulandı ve CI'dan geçti.** Gerçek kullanıcıda v0.1.8 → v0.1.9 ilk geçişi, v0.1.8'in eski açılışta otomatik güncelleme davranışıyla yapıldı. Yeni bildirim akışı henüz daha yeni bir Release olmadığı için gerçek kullanıcı ekranında sınanmadı.

Uygulanan davranış:

- İlk açılışta son kontrolün üzerinden altı saat geçmişse sürüm denetimi yapılır; kontrol zamanı kullanıcı uygulama verisinde saklanır.
- DUYRA açık kaldığında altı saatlik kontrol döngüsü sürer.
- Zamanlamaya küçük gecikme eklenir; geçici hatalarda artan bekleme ve hız sınırı yanıtlarında sunucunun yeniden deneme bilgisi kullanılır.
- Yeni sürüm bulunduğunda alt durum çubuğunda sürüm bilgisi ile "Şimdi güncelle" ve "Daha sonra" seçenekleri gösterilir.
- "Şimdi güncelle" açık kullanıcı onayıdır. İndirme, SHA-256 bütünlük ve Ed25519 imza doğrulaması, kurulum ve yeniden başlatma bu onaydan sonra gerçekleşir.
- "Daha sonra" kurulum yapmaz; yeni denetim zamanında yeniden bildirim verilebilir.
- Kullanıcı onayı olmadan uygulama kendiliğinden kapanmaz, kurulum yapmaz veya yeniden başlatılmaz.
- Hatalarda mevcut sürüm çalışmaya devam eder ve hata yerel günlüğe yazılır.

Bilinçli sınırlar:

- İndirme, kullanıcı onayından önce arka planda başlamaz.
- "İşimi bitirince otomatik kur" henüz yoktur. İnceleme, kaydedilmemiş değişiklik ve çalışan görevleri izleyen gerçek bir yaşam döngüsü modeli kurulmadan otomatik boşta-kalma kurulumu yapılmaz.
- Ayarlar ekranına bu aşamada ek tercih koyulmaz; kontrol aralığı altı saat olarak sabittir.

**Bekleyen gerçek kullanıcı kabul testi:** v0.1.9'dan sonraki ilk doğal Release yayımlandığında, DUYRA açıkken altı saatlik kontrol veya sonraki açılışta yeni sürüm bildiriminin görünmesi; "Daha sonra"nın kurulum yapmaması; "Şimdi güncelle"nin imzalı paketi kurup uygulamayı yeniden başlatması doğrulanacaktır. Sırf bu testi oluşturmak için boş sürüm çıkarılmaz.

### 12.2. Ekran görüntüsüyle gerçek kullanıcı doğrulaması

- Asistan, kullanıcının Windows masaüstünü bu sohbet içinde sürekli ve kendiliğinden canlı izleyemez.
- Görsel kontrol için kullanıcı her ilgili sürüm/arayüz değişikliğinden sonra uygulamanın ekran görüntüsünü sohbete yükler.
- Tercih edilen görüntü, DUYRA penceresinin tamamını; pencere boyutu, metinler, gezinme ve durum çubuğu görünecek şekilde içermelidir. Yerleşim sorunu şüphesinde tam ekran görüntüsü de eklenebilir.
- Görüntü, kod/CI kontrolünün yerine geçmez; otomatik denetimden sonra gerçek kullanıcı arayüzü kabul testinin kanıtıdır.
- Delil içeriği, kişisel bilgiler veya gizli dosya yolları içeren görüntüler paylaşılmamalıdır.
- Ekran görüntüleri varsayılan olarak depoya eklenmez. Yalnızca otomatik görsel regresyon testi için açıkça gerekli ve temizlenmiş bir örnek olduğunda değerlendirilir.
- Görsel incelemede tespit edilen her sorun önce mevcut hedefin parçası mı diye değerlendirilir; yeni fikir olması tek başına kapsamı genişletmez.


### 12.3. CI üzerinden otomatik ekran görüntüsü alma

**Teknik deneme tamamlandı ve doğrulandı.**

- Workflow: `.github/workflows/gorsel-denetim.yml`
- Tetiklenme: İlgili kaynak dosyalar değiştiğinde hem `main` dalına gönderimde hem de `main` hedefli PR'da çalışır. Böylece görsel ön kontrol birleştirmeden önce de yapılır.
- Yakalama betiği: `scripts/ci-ekran-goruntusu.sh`
- Çalışma ortamı: `ubuntu-latest`, Xvfb ve Mesa `llvmpipe` yazılım OpenGL çizicisi.
- Derleme komutu, yalnızca Linux CI için `cargo build --release --features eframe/x11` şeklindedir. Üretim `Cargo.toml` ayarlarına veya Windows uygulama derlemesine dokunulmaz.
- Görünür DUYRA penceresi `xdotool` ile bulunur; ImageMagick `import` komutuyla pencere PNG'si alınır.
- Artifact, Actions çalışma kaydına yüklenir ve 7 gün saklanır. Release paketlerine veya web sitesine eklenmez.
- Son başarılı örnek Actions run 43988327972, artifact 14286377159'dur. PNG 1180×780 boyutunda indirildi ve görsel olarak incelendi.
- Windows hosted runner'da ekran yakalama denenmiş, fakat Microsoft Hyper-V Video sanal bağdaştırıcısı OpenGL 2.0+ sunmadığından pencere başlatılamamıştır. Bu nedenle otomatik görsel ön kontrolü Linux'ta yapılır.
- Linux görüntüsü ana arayüzün erken regresyon kontrolüdür. Windows'a özgü pencere süslemesi, DPI, GPU ve yerel sistem metinleri için kullanıcı kabul testi yerine geçmez.
- Test penceresinde gerçek delil veya kullanıcı verisi bulunmaz.

### 12.4. İlk açılış pencere konumu

1920×1080 masaüstünden gönderilen v0.1.8 görüntüsünde DUYRA penceresi ilk açılışta ekranın sol üst tarafına yakın konumlanmıştır; kullanıcı pencereyi elle taşımamıştır.

Bu bir gözlem olarak kaydedilmiştir. Henüz kesin karar değildir. İleride küçük bir arayüz hedefinde şu seçenekler değerlendirilecektir:

- İlk açılışta pencereyi kullanılabilir ekran alanının ortasında açmak.
- Sonraki açılışlarda kullanıcının son pencere konumunu ve boyutunu hatırlamak.
- Kaydedilen konum artık bağlı ekranlarda mevcut değilse pencereyi görünür ve erişilebilir bir konuma geri almak.

Uygulama varsayılan olarak tam ekran başlatılmak zorunda değildir. Bu davranış değiştirilmeden önce ilk açılış ve sonraki açılış senaryoları ayrı kabul ölçütleriyle kararlaştırılmalıdır.



## 13. İlk inceleme akışı ve delil koruması

Bu bölüm, bilirkişinin ilk kez bir inceleme başlatmasından delil kaynağının okunmasına kadar izlenecek sırayı belirler. Amaç, ilk kullanılabilir işlevi geciktirmeden sunarken koruma doğrulanmadan delil üzerinde işlem yapılmasını önlemektir.

### 13.1. Kararlaştırılan kullanıcı akışı

1. Bilirkişi DUYRA'yı açar ve **Yeni İnceleme** kaydı oluşturur. Bu kayıt, delil aygıtı bağlanmadan önce oluşturulur.
2. DUYRA kendi benzersiz **İnceleme No** değerini üretir. Bu numara DUYRA içindeki çalışma kaydını tanımlar; mahkeme, soruşturma, esas veya dosya numarası değildir.
3. Resmî dosya/soruşturma/esas numarası, ileride ayrı ve isteğe bağlı bir alanda kullanıcının gireceği dış referanstır. DUYRA bu numarayı kendisi üretmez.
4. Delil aygıtı bağlanmadan önce koruma hazırlığı yapılır. İşletim sistemi, sürüm, yetki ve koruma yöntemi kontrol edilir.
5. Koruma yöntemi etkin ve doğrulanmış değilse DUYRA delil aygıtını bağlamaya veya içeriğini incelemeye hazır olduğunu iddia etmez.
6. Koruma doğrulandıktan sonra bilirkişi aygıtı bağlar. DUYRA doğru aygıtı model/seri/kapasite gibi bilgilerle tanımlar ve bilirkişiye kaynağın doğru aygıt olduğunu onaylatır.
7. Kaynak dosyalar listelenir ve hash değerleri alınır. Ses incelemesi, delil kaynağını değiştirmeyen açıkça tanımlanmış yöntemle başlar.
8. Deşifre, not, işaret ve rapor gibi çalışma çıktıları kaynak delilden ayrı saklanır.

### 13.2. İnceleme numarası ve kayıt

- DUYRA'nın ürettiği numara yalnızca uygulama içi incelemeyi tanımlar; resmî dosya numarası yerine geçmez.
- İlk kayıt aşamasında yalnızca UUID tabanlı benzersiz kimlik, görüntülenecek İnceleme No, oluşturulma zamanı, kayıt şeması sürümü ve basit durum saklanır.
- İlk aşamada delil dosyası, dosya adı, dosya içeriği, mahkeme/soruşturma numarası veya kişisel bilgi kaydedilmez.
- Windows kayıt alanı: `%LOCALAPPDATA%\\DUYRA\\incelemeler`.
- Linux kayıt alanı: `$XDG_DATA_HOME/duyra/incelemeler`; değişken yoksa `~/.local/share/duyra/incelemeler`.
- Kayıtlar sürümlenmiş ayrı JSON dosyaları olarak tutulur. Her kayıt ayrı dosyada olduğundan bir kaydın bozulması diğer kayıt dosyalarının üzerine yazılmasına yol açmamalıdır.
- Bu ilk aşamadaki metadata dosyaları şifrelenmiş değildir. Bu nedenle hassas dosya referansı, kimlik bilgisi veya delil içeriği saklanmaz. Şifreleme ve yedekleme politikası, hassas kullanıcı verisi kaydedilmeden önce ayrı hedef olarak tasarlanacaktır.
- Kayıt klasörü oluşturulamıyor veya yazılamıyorsa hata kullanıcıya gösterilir; geçici klasöre ya da uygulama dizinine sessizce kayıt yapılmaz.
- Uygulama kapatılıp açıldığında mevcut kayıtlar tekrar listelenebilmelidir.

### 13.3. Delil aygıtı yazma koruması

**Ürün yolu, doğrulanabilir yazılımsal korumayı esas alır.** Donanımsal yazma engelleyici zorunlu gereksinim veya ilk kurulum şartı değildir; kurumda mevcutsa ek katman olarak kullanılabilir. Bununla birlikte DUYRA, Windows bilgisayar ilkesini kendi başına açıp kapatmaz ve gerçek aygıt yazma engelini yalnızca politika kaydından çıkarsamaz.

Yazılımsal koruma da tek başına “delilin değiştirilemezliği” veya adli imaj alma garantisi değildir. DUYRA yalnızca işletim sistemi ve ilgili aygıt için gerçekten gözlemlenebilen durumu, kapsamını ve doğrulama sınırını raporlayacaktır.

Windows için araştırılan yazılımsal seçenek:

- Microsoft'un **Removable Disks: Deny write access** cihaz ilkesi, Windows 10 sürüm 1809 ve sonrası için Pro, Enterprise, Education ve IoT Enterprise sürümlerinde belgelenmiştir. Kaynak: [Microsoft Storage Policy CSP](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-storage).
- İlke çıkarılabilir disk sınıfına yazmayı engeller; yalnızca DUYRA'nın seçtiği tek USB belleğe uygulanmış gibi sunulamaz. Diğer çıkarılabilir disklerin yazma davranışını da etkileyebileceği kullanıcıya önceden açıklanmalıdır.
- Belgelenen destek listesinde Windows Home yer almaz. Home sürümünde bu ilkenin var olduğu veya aynı garantiyi sağladığı varsayılmayacaktır.
- DUYRA Windows sistem ilkesini değiştirmez; Registry yazımı, UAC yükseltmesi veya otomatik geri alma sunmaz. Politika gerekiyorsa yetkili Windows yönetim aracıyla sistem yöneticisi tarafından ayrıca yönetilir.
- DUYRA, mevcut politika kaydını ve bilinen yönetim sinyallerini salt okunur raporlar. Politika kaydı ile ilgili aygıtın gerçek salt okunur davranışı ayrı değerlendirilir; yalnızca HKLM değerinden aygıt koruması sonucu çıkarılmaz.

Linux için:

- `blockdev --setro` aygıt algılandıktan sonra blok aygıtı salt okunur işaretleyebilir. Man sayfası, önceden yazılabilir bağlanmış dosya sisteminin bu değişiklikten etkilenmeyebileceğini ve yeniden bağlama gerektiğini açıkça belirtir. Kaynak: [Linux blockdev(8)](https://man7.org/linux/man-pages/man8/blockdev.8.html).
- Sadece dosya sistemini `ro` bağlamak da her dosya sistemi ve durumda yeterli olmayabilir; örneğin günlük tekrar oynatma yazmaya neden olabilir. Kaynak: [Linux mount(8)](https://www.man7.org/linux/man-pages/man8/mount.8.html).
- Bu nedenle Linux'ta aygıt keşfi, otomatik bağlama sırası ve blok aygıtını salt okunur yapma adımları ayrıca tasarlanıp gerçek Linux sisteminde test edilmelidir.
- Linux CI derlemesi ve ekran görüntüsü, Linux delil korumasının veya üretim desteğinin kanıtı değildir.

**Koruma doğrulama kuralı:** Koruma yöntemi, işletim sistemi desteği ve aygıtın salt okunur durumu kanıtlanamıyorsa DUYRA durumu “Koruma doğrulandı” yapmaz ve delil okuma aşamasına geçmez. Testler önce gerçek dava delili olmayan, ayrılmış deneme USB aygıtıyla yapılır.

### 13.4. Sıralı geliştirme hedefleri

1. **Tamamlandı — v0.1.10:** Yeni İnceleme kaydı oluşturma, yerel kalıcı saklama, uygulama yeniden açıldığında listeleme ve kaydı tekrar açma kodlandı; PR #1 ana dala birleştirildi ve Release yayımlandı. Gerçek Windows kullanıcı kabul testi de başarılıdır. Bu aşamada delil aygıtına erişilmez.
2. **Tamamlandı — v0.1.11–v0.1.15:** Windows sürüm/edisyon/yetki, bilinen yönetim sinyalleri ve ilgili politika kaydı salt okunur denetlenir. Tanılama yerel ve sınırlı tutulur. Politika kaydı ile gerçek aygıt yazma davranışı arayüzde ayrı gösterilir. DUYRA Registry/Grup İlkesi değiştirmez, MDM'ye müdahale etmez ve USB aygıtını açmaz.
3. **Bekleyen tek kabul kapısı — Issue #13:** Ayrılmış Windows 11 Pro konuk VM ve boş test USB belleğiyle ilgili çıkarılabilir disk ilkesinin bir defalık davranış testi. Test gerçek delil medyası veya kişisel host üzerinde yapılmaz. Bu test, yalnızca denenen VM/USB kombinasyonunu doğrular; donanımsal yazma engelleyici veya tüm depolama yolları için genel garanti değildir. Test tamamlandıktan sonra VirtualBox kullanımı kapanır.
4. **Sıradaki ana ürün hedefi:** Koruma sınırları doğrulanmış bir kaynak aygıtı salt okunur tanımlama ve durumunu değerlendirme. Aygıt sınıfı, doğru kaynak olduğuna ilişkin kullanıcı onayı ve işletim sisteminin gerçekten sunduğu salt okunur durumu belirlenmeden dosya içeriği açılmaz. Belirsiz veya desteklenmeyen durumda akış durur.
5. **Sonraki hedef:** Kaynak dosya envanteri ve SHA-256 hash kaydı; kaynak ile çalışma çıktılarının ayrılması. Orijinal delil dosyasına yazma yapılmaz.
6. **Sonraki hedef:** Ses dosyasını kaynağı değiştirmeden dinleme ve zaman damgalı inceleme işaretleri.
7. **Sonraki hedef:** Hassas not, düzeltme ve rapor taslakları için şifreleme, yedekleme ve kurtarma politikasını belirleyip uygulama.
8. **Sonraki hedefler:** Deşifre, analiz ve bilirkişi raporu iş akışları.

Her hedef ayrı kabul ölçütleriyle test edilir. Delil kaynağının koruma durumu doğrulanmadan gerçek delil dosyası açılmaz.


### 13.5. Kaynak aygıtını tanımlama ve salt okunur durumu — tasarım sınırları

Bu, Issue #13'teki tek seferlik izole VM kabul testi tamamlandıktan sonra başlayacak ana ürün hedefidir. Bu bölüm tasarım sınırlarını kaydeder; gerçek aygıt keşfi, disk/volume açma veya veri okuma kodu eklemez.

**İlk aşama — yalnızca mevcut aygıt adaylarını listeleme**

- Windows'ta mevcut PnP aygıt arayüzleri, Microsoft'un Configuration Manager veya SetupAPI arayüzleriyle listelenmelidir. Adaylar için yalnızca sistemde şu anda mevcut olan aygıtlar alınır: `CM_Get_Device_Interface_List` ile `CM_GET_DEVICE_INTERFACE_LIST_PRESENT` veya `SetupDiGetClassDevs` ile `DIGCF_PRESENT | DIGCF_DEVICEINTERFACE`, ardından `SetupDiEnumDeviceInterfaces`.
- Aygıt özellikleri gerekiyorsa `CM_Get_Device_Interface_Property`, `SetupDiGetDeviceInterfaceProperty` veya `SetupDiGetDeviceProperty` gibi belgelenmiş PnP API'leri kullanılmalıdır. Aygıt keşfi için Registry anahtarları doğrudan taranmaz.
- Bu ilk aşamada fiziksel disk veya volume için `CreateFile` çağrısı yapılmaz; `DeviceIoControl` çalıştırılmaz; bölüm/volume bağlama, ayırma veya otomatik bağlama ayarı değiştirilmez; sektör, dosya sistemi veya dosya içeriği okunmaz/yazılmaz.
- Sistem tarafından döndürülen her aygıt otomatik seçilmez. Birden fazla benzer aygıt varsa uygulama seçim yapmaz; bilirkişi adayın doğru kaynak olduğunu ayrıca onaylamadan ilerlenmez.
- Seri numarası, PnP instance ID, volume kimliği, sürücü harfi ve tam aygıt yolu tanılama kayıtlarına yazılmaz. Gerekli geçici tanımlayıcıların arayüzde gösterilip gösterilmeyeceği ayrıca kararlaştırılır.
- Windows'un aygıtı Removable/F​​ixed veya başka sınıfta sunması belirsizse sınıf tahmin edilmez; sonuç Bilinmiyor kalır.

**İkinci aşama — yazılabilirlik ve bağlama durumunu ayrı gözlemleme**

- Windows'ta `IOCTL_DISK_IS_WRITABLE`, Microsoft belgelerinde disk yazılabilirliğini belirleyen bir `DeviceIoControl` isteği olarak tanımlanır. Bu sorgu bir aygıt tanıtıcısı gerektirdiğinden ilk PnP listeleme aşamasına eklenmez; gerçek aygıta erişim sınırı olarak ayrı değerlendirilir.
- Sorgunun hata vermesi otomatik olarak salt okunur anlamına gelmez. Yalnızca açıkça yazma korumasını bildiren sonuç salt okunur gözlemine eşlenebilir; erişim reddi, desteklenmeyen istek, aygıt hatası ve diğer tüm hatalar Bilinmiyor/Doğrulanamadı kalır.
- Linux'ta `blockdev --getro` / `BLKROGET` çekirdeğin blok aygıtı salt okunur bayrağını bildirir. Bu, donanımsal yazma engelleyici veya bütün yazma yolları için garanti değildir. Blok aygıtı bayrağı, dosya sisteminin mevcut bağlama seçeneklerinden ayrı raporlanmalıdır.
- İlk uygulama hedefinde `blockdev --setro`, `blockdev --setrw`, `BLKROSET`, mount/remount/unmount veya otomatik bağlama ayarı değişikliği yapılmaz.
- Önceden yazılabilir bağlanmış dosya sistemi, blok aygıtı bayrağından farklı bir durum gösterebilir. Bağlama durumu güvenilir ve salt okunur biçimde belirlenemiyorsa dosya içeriği açılmaz.
- Windows ve Linux için gözlenen durumlar tek bir “Koruma doğrulandı” sonucuna indirgenmez. Aygıt sınıfı, blok katmanı bayrağı, dosya sistemi bağlama durumu ve varsa politika sonucu ayrı alanlardır.

**Durum modelinin asgari ilkeleri**

- Aygıt keşfi: Desteklenmiyor / Aday yok / Adaylar bulundu / Bilinmiyor.
- Salt okunur gözlem: Salt okunur bildirildi / Yazılabilir bildirildi / Desteklenmiyor / Bilinmiyor.
- Bir sorgunun desteklenmemesi veya hata vermesi hiçbir zaman salt okunur başarı sayılmaz.
- Aygıtın kimliği veya koruma durumu belirsizse akış kapalı kalır; kullanıcıdan farklı aygıtları denemesi istenmez.
- Bir sorgunun “salt okunur” demesi, medyanın donanımsal korumalı, adli imaj alınmış veya değiştirilemez olduğunu tek başına kanıtlamaz.

**Test ve kabul kapısı**

1. Issue #13'ün kullanıcı kabul testi tamamlanmadan gerçek kaynak aygıtına yönelik yeni uygulama kodu başlamaz.
2. İlk kod hedefi platform API'sinden ayrılmış sahte aygıt sağlayıcısı ve durum eşleme testleridir.
3. CI yalnızca sahte aygıtlarla çalışır; gerçek disk, USB, Registry veya mount işlemi yapmaz.
4. Windows ve Linux sağlayıcıları ayrı, sınırlı arayüzlerle geliştirilir. Herhangi bir aygıt tanıtıcısı açılması veya DeviceIoControl/BLKROGET sorgusu ayrı teknik inceleme ve izole VM kabul testi gerektirir.
5. Kullanıcı kabulü yalnızca snapshot alınmış izole test ortamı ve boş/atılabilir test medyasıyla yapılır. Gerçek delil medyası hiçbir zaman test aygıtı değildir.
6. Kaynak aygıt kimliği, sınıfı ve koruma durumu doğrulanmadan kaynak dosyası listelenmez veya açılmaz.

Teknik kaynaklar:

- Microsoft — [Enumerating installed device interfaces](https://learn.microsoft.com/en-us/windows-hardware/drivers/install/enumerating-installed-device-interface-classes)
- Microsoft — [Accessing device properties](https://learn.microsoft.com/en-us/windows-hardware/drivers/install/accessing-device-properties)
- Microsoft — [IOCTL_DISK_IS_WRITABLE](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-ioctl_disk_is_writable)
- Linux man-pages — [blockdev(8)](https://man7.org/linux/man-pages/man8/blockdev.8.html)
- Linux kernel — [BLKROGET / BLKROSET ioctl definitions](https://github.com/torvalds/linux/blob/master/tools/include/uapi/linux/fs.h)

## 14. İlk inceleme kaydı — v0.1.10

**Kod hedefi ve gerçek kullanıcı kabul testi tamamlandı.**

- PR #1: `İlk inceleme kaydını yerel olarak oluştur ve sakla`; squash merge commit: `43f541ddb308f192eb386493bc9271f9d5903b3e`.
- Release: [v0.1.10](https://github.com/azmisahin-gov/duyra/releases/tag/v0.1.10). Windows ilk indirme `.exe`, imzalı otomatik güncelleme ZIP'i ve `SHA256SUMS.txt` yayımlandı.
- Ana dal Windows CI run: `43994622910`. Kod biçimi, Clippy, testler, sürüm/anahtar kontrolü, Windows x64 derlemesi, dosya bilgileri, imzalı ZIP hazırlama ve Release yayımlama adımları başarılıdır.
- Ana dal görsel CI run: `43994622845`; artifact `14287474999`. 1180×780 PNG indirildi ve açılarak incelendi. Ana ekranda inceleme sayacı ve **Yeni İnceleme Başlat** düğmesi yerleşimi uygundur.
- Web sayfası dağıtımı: `43994622892`, başarılı.
- Görsel CI artık PR'larda da çalışır. PR görsel run `43994337788`, artifact `14289665936`; PR Windows CI run `43994337587`, başarılıdır.

### v0.1.10 kapsamı ve sınırı

- Bilirkişi ana ekrandan **Yeni İnceleme Başlat** eylemiyle bir kayıt açabilir. DUYRA UUID tabanlı benzersiz uygulama içi İnceleme No üretir.
- Bu numara resmî mahkeme, soruşturma, esas veya dosya numarası değildir. Resmî referans ileride ayrı ve isteğe bağlı alan olacaktır.
- Kayıtlar Windows'ta `%LOCALAPPDATA%\\DUYRA\\incelemeler`, Linux'ta kullanıcı veri klasörü altında tutulur. Her inceleme ayrı JSON dosyasıdır; yeniden açılışta listelenir.
- Bu aşamada kayıt yalnızca uygulama içi kimlik, numara, oluşturulma zamanı, şema sürümü ve başlangıç durumunu tutar. Delil dosyası, resmî dosya numarası veya hassas not saklanmaz.
- JSON metadata dosyaları şifrelenmiş değildir. Hassas veri, dosya yolu veya dava bilgisi eklenmeden önce şifreleme, yedekleme ve kurtarma hedefi tamamlanmalıdır.
- USB/delil aygıtı algılama, yazma koruması, kopyalama, hash alma, ses oynatma ve deşifre bu sürümde yoktur. Bu nedenle bu sürüm tek başına gerçek delil medyası incelemesine hazır kabul edilmez.

### Kullanıcı kabul testi — 17 Kasım 2026

Kullanıcı, Windows üzerinde v0.1.9 → v0.1.10 otomatik güncellemesini ve aşağıdaki akışı kendi bilgisayarında gerçek kullanıcı testiyle tamamladı:

- Güncelleme bildirimi onaylandı; v0.1.10 kuruldu ve uygulama sorunsuz yeniden başladı.
- **Yeni İnceleme Başlat** iki kez kullanıldı; birbirinden farklı iki uygulama içi İnceleme No üretildi.
- Her iki kayıt İncelemeler listesinde **Hazırlanıyor** durumuyla göründü.
- Kayıtlardan biri **Aç** ile seçildiğinde aşağıdaki **Açık İnceleme** alanında seçili numara gösterildi; diğer kayıt seçildiğinde alan seçime göre değişti.
- DUYRA kapatılıp yeniden açıldı; Genel Bakış sayacı 2 yerel kayıt gösterdi, Son İncelemeler listesinde iki kayıt kaldı ve İncelemeler sayfasında her iki kayıt tekrar açılabildi.
- Gerçek delil, dava numarası veya hassas kişisel veri kullanılmadı.

**Kabul sonucu: başarılı.** Bu hedef artık kullanıcı kabulü açısından kapalıdır. Benzer oluşturma/yeniden açma akışı bundan sonra sahte verili CI etkileşim testiyle de otomatikleştirilecek; gerçek Windows sürüm, izin ve kullanıcı ortamı kontrolleri gerektiğinde kullanıcı kabulü ayrıca korunacaktır.


## 15. Otomatik inceleme akışı testi ve kullanıcı kabulü

**Sahte verili CI smoke testi tamamlandı ve ana dala birleştirildi.**

- PR #2: **İnceleme oluşturma ve yeniden açma akışını CI'da otomatik doğrula**; squash merge commit: `89e45c9bc07d71b433af8b1e1188ab8b2920ad8b`.
- Workflow: `.github/workflows/gorsel-denetim.yml`; test betiği: `scripts/ci-inceleme-akisi.sh`.
- Windows PR CI run `43998902297` ve ana dal push CI run `43999207706` başarılı: biçim, Clippy, testler geçti; PR'da Windows x64 release derlemesi de başarılıdır. Ana dalda sürüm zaten yayımlandığı için yeni derleme/yayın adımları atlandı.
- Linux PR görsel/etkileşim CI run `43998902293`, artifact `14292020043`; ana dal push görsel/etkileşim CI run `43999207671` başarılıdır. PR artifact'ı indirildi ve ekran görüntüleri açılarak incelendi.
- PR #1 ve #2 birleştirildi; açık PR kalmadı. Uzakta `feature/ilk-inceleme-kaydi` ve `test/ci-inceleme-akisi` dalları duruyor. Kullanılabilir GitHub bağlantısı bu oturumda dal silme işlemi sunmadığı için bu iki eski dal henüz temizlenemedi; yeni işler `main` üzerinden ayrı dallarda başlatılmalıdır.
- Smoke test geçici `XDG_DATA_HOME` kullanır; kullanıcı bilgisayarını, gerçek delili veya mevcut kullanıcı kayıtlarını kullanmaz.
- Test Genel Bakış'tan ilk kaydı oluşturur, DUYRA'yı kapatıp tekrar açar, kaydın geri geldiğini doğrular, ikinci kaydı oluşturur, JSON içeriklerinde iki farklı UUID/İnceleme No ve beklenen durumu kontrol eder.
- Her iki **Aç** düğmesi tıklanır. Açık İnceleme alanındaki içerik için alınan iki PNG karşılaştırılır; son başarılı çalışmada farklılık `862` piksel olarak ölçüldü. Her adımın ekran görüntüsü, stdout/stderr, kayıt JSON'ları ve durum özeti 7 günlük CI artifact'ında saklanır.
- İlk iki koşuda yalnızca test betiğinin koordinat ve ImageMagick çıktı ayrıştırma hataları düzeltildi. Ürün kodunda bir hata bulunmadı; son koşu başarıyla tamamlandı.
- Linux Xvfb/llvmpipe testi ortak egui kullanıcı akışının otomatik regresyon kontrolüdür; Windows işletim sistemi politikası, Windows izinleri, gerçek USB davranışı veya Windows yerel grafik sürücüsü için kanıt sayılmaz.
- Windows'taki gerçek kullanıcı testi zaten v0.1.10 kabulinde yapıldı. Bundan sonra yalnızca işletim sistemi/aygıt davranışı veya Windows'a özgü kullanıcı deneyimi gerektiren kontrollerde kullanıcı kabulü isteyeceğiz. Kullanıcıdan gerçek delil veya hassas ekran görüntüsü istenmeyecek.
- Test başarısız olursa log, JSON kayıtları ve ekran görüntüsü CI artifact'ında saklanır; hata kaynağı uygulama mı test ortamı mı ayrıştırılır.

### Yazılım öncelikli koruma kararı

- Ürün yazılımsal yazma koruması üzerine ilerleyecek; donanımsal yazma engelleyici zorunlu ürün gereksinimi değildir.
- Donanım engelleyici mevcut olduğunda ek bir kontrol olarak kullanılabilir, ancak yazılım akışının çalışması için şart koşulmaz.
- Yazılımsal koruma etkinleştirilirken bilgisayara veya bilirkişinin mevcut iş akışına zarar verebilecek sessiz/geniş kapsamlı sistem değişikliği yapılmaz. Özellikle Windows'un çıkarılabilir disk ilkesi sistem kapsamını etkileyebildiği için kullanıcı onayı ve etki açıklaması gerekir.
- Yazılım ile kanıtlanabilir koruma sağlanamıyorsa DUYRA başarısız/uyumsuz durumu açıkça bildirir; koruma doğrulanmış gibi göstermeden delil kaynağına erişim açılmaz.

### Sıradaki ürün hedefi

Windows üzerinde yazılımsal delil yazma-koruma ön kontrolünü tasarlamak: desteklenen Windows sürümü/edisyonu, yönetici yetkisi, mevcut removable-storage ilkesi ve etkinin kapsamını yalnızca okumak; hiçbir sistem ilkesini kullanıcı onayı olmadan değiştirmemek. İlk küçük hedef sadece **durum tespiti ve kullanıcıya açıklama** olacak; USB aygıtını açma/kopyalama bu hedefe eklenmeyecek.

## 16. Windows yazma koruması ön kontrolü — v0.1.11

**Mevcut hedef:** Windows işletim sistemi ve bilgisayar düzeyindeki çıkarılabilir disk yazma engelleme ilkesini yalnızca okumak; henüz USB aygıtına erişmemek.

### Kapsam

- Windows ürün adı, `EditionID` ve `CurrentBuildNumber` değerlerini kayıt defterinden salt okunur olarak oku.
- DUYRA işleminin mevcut erişim belirtecinde yükseltilmiş yönetici yetkisi olup olmadığını göster.
- Yalnızca şu bilgisayar düzeyi Grup İlkesi kaydını oku: `HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\RemovableStorageDevices\\{53f5630d-b6bf-11d0-94f2-00a0c91efb8b}\\Deny_Write`.
- Değer `1` ise kayıt etkin, `0` ise devre dışı, yoksa tanımlı değil, başka bir değer ise beklenmeyen olarak gösterilir. Okuma hatası bilinmiyor durumudur.
- Edisyon ve derleme bilgilerini Microsoft'un `RemovableDiskDenyWriteAccess` destek kapsamıyla karşılaştır: Windows 10 sürüm 1809 / derleme 17763 ve sonrası; Pro, Enterprise, Education ve IoT Enterprise edisyon aileleri. Kaynak: [Microsoft Storage Policy CSP](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-storage).
- Kontrol Ayarlar ekranında açıklanır; Linux'ta bu Windows kontrolünün uygulanmadığı belirtilir.

### Bilinçli olarak yapılmayanlar

- Kayıt defterine yazma, Grup İlkesi değiştirme, politika etkinleştirme veya devre dışı bırakma yoktur.
- Yönetici izni isteme veya UAC yükseltmesi yoktur.
- USB aygıtı keşfi, bağlama, dosya listeleme, kopyalama veya hash alma yoktur.
- MDM/Intune, kullanıcı kapsamlı ilkeler, diğer sınıf ilkeleri ve gerçek aygıt salt-okunur durumu bu hedefte kapsanmaz. Arayüz, okunan HKLM değeri dışında kapsamlı bir etkin politika değerlendirmesi yaptığını iddia etmez.
- İlke etkin görünse bile “Delil koruma altında” durumu gösterilmez. İlke tüm çıkarılabilir disk sınıfını etkileyebilir; yalnızca DUYRA'nın seçtiği tek USB'yi değil.

### Kabul ölçütleri

1. Windows CI derlemesi ve testleri geçer.
2. Salt okunur sistem bilgileri ve politika kaydı Settings ekranında görüntülenir.
3. Linux görsel CI'sında Ayarlar ekranı açılır ve Windows'a özgü bu kontrolün mevcut olmadığı doğru biçimde gösterilir.
4. Smoke test ve birim testleri hiçbir registry/policy değişikliği yapmaz.
5. UI, “ilke kaydı” ile “aygıt salt okunur doğrulaması” arasındaki farkı açıkça anlatır.

Bir sonraki ayrı hedef, kullanıcı onaylı ve etkisi açıklanmış bir Windows politika etkinleştirme yardımını değerlendirmek olabilir. Bu hedef tamamlanmadan politika değiştirme veya USB'ye erişim eklenmez.

### Kullanıcı kabul testi — 17 Kasım 2026

**v0.1.11 ön kontrol ekranı Windows üzerinde kullanıcı tarafından doğrulandı.**

- Uygulama normal açıldığında yönetici yetkisi olmadığı doğru gösterildi.
- Uygulama “Yönetici olarak çalıştır” ile açıldığında yükseltilmiş yetki doğru gösterildi.
- Her iki açılışta Windows sürüm/edisyon/derleme bilgisi ve destek kapsamı gösterildi.
- Her iki açılışta ilgili HKLM Deny_Write kaydı “tanımlı değil” olarak görüntülendi.
- Kullanıcı ekran görüntülerini sağladı. Görüntüler özel bilgisayar ayrıntısı içerebildiğinden depoya veya Release'e eklenmedi.
- Kabul sonucu: **başarılı**. Bu test, yönetici yetkisinin doğru tespit edildiğini doğrular; yazma korumasının etkin olduğunu veya USB'nin salt okunur olduğunu kanıtlamaz.
- Ön kontrol bilgileri bu sürümde Ayarlar'a veya günlük dosyasına kaydedilmez. Ayarlar ekranında ön kontrol özetini kopyalama düğmesi de henüz yoktur. Kullanıcının metni seçerek kopyalayamaması bu sürümün mevcut sınırıdır; kabul edilen ön kontrol işlevini başarısız kılmaz.

### v0.1.12 tamamlanan kullanılabilirlik hedefi

- Ayarlar ekranına **“Ön kontrol sonucunu kopyala”** eylemi eklendi; PR #5 tek commit ile birleştirildi ve v0.1.12 yayımlandı.
- Özet yalnızca kullanıcı düğmeye bastığında panoya aktarılmak üzere hazırlanır. Otomatik kayıt, günlük yazımı ve ağ gönderimi eklenmedi.
- Kullanıcı gerçek Windows ortamında panoya kopyalamanın başarısız olduğunu bildirdi. Kullanıcı, bu özetin mevcut yazma koruması hedefi için zorunlu olmadığını belirtti. Bu nedenle pano davranışı ayrı bir kullanılabilirlik konusu olarak kaldı; USB yazma koruması geliştirmesini bloke etmez.
- Politika etkinleştirme/kapatma ve USB aygıtına erişim bu küçük hedefin kapsamına alınmadı; ayrı karar ve kabul ölçütü gerektirir.

## 17. Depo, dal, PR ve commit yönetimi

**Dil tutarlılığı kararı — 17 Kasım 2026:** DUYRA'nın ürün belgeleri ve geliştirici iletişimi Türkçedir. Git geçmişinde görülen İngilizce conventional-commit önekleri ve İngilizce alt commit gövdeleri bu kuralla uyumlu değildir. Bundan sonraki commit ve PR metinleri Türkçe yazılır; geçmiş commitler yeniden yazılarak değiştirilmez.

**Amaç:** Git geçmişini, CI sonuçlarını ve açık işleri okunabilir tutmak; kullanıcının neyin tamamlandığını tek bakışta anlayabilmesini sağlamak.

1. Her yeni hedef doğrudan güncel main dalından başlatılır. Eski veya birleştirilmiş bir özellik dalından yeni iş türetilmez.
2. Aynı anda tek ürün hedefi ve mümkün olduğunca tek aktif çalışma dalı/PR tutulur. Bağımsız işler paralel dallara bölünmez; sıraya alınır.
3. Bir hedef için tek, kapsamı açık PR açılır. İlişkili kod, test ve zorunlu belge değişiklikleri aynı PR'da gruplanır. Aynı hedef için düzeltme PR'ı veya ikinci bir PR açılmaz.
4. Commit, her küçük deneme veya CI hatası için değil, anlamlı ve gözden geçirilebilir bir değişiklik birimi oluştuğunda atılır. Biçim/Clippy/test hataları aynı çalışma dalında düzeltilir. Normal durumda bir hedefin PR'ı az sayıda, anlamlı commit içermelidir; commit sayısı kendi başına ilerleme ölçütü değildir.
5. Depodaki insan tarafından yazılan Git geçmişi Türkçe tutulur: commit başlığı ve gövdesi, PR başlığı/açıklaması, squash birleştirme mesajı ve commit içi açıklamalar Türkçe olmalıdır. İngilizce `feat:`, `fix:`, `docs:`, `chore:` gibi önekler kullanılmaz. Kod tanımlayıcıları, API/standart adları, kütüphane terimleri ve üçüncü taraf araçların zorunlu İngilizce metinleri özgün biçiminde kalabilir. Her mesaj kısa ve yapılan gerçek değişikliği anlatır.
6. CI başarısızsa önce hangi adımın ve neden başarısız olduğu bulunur. Ürün hatası ile CI ortamı sorunu ayrıştırılır; düzeltme aynı PR'da yapılır ve son commit'in ilgili kontrolleri yeniden çalıştırılır. Başarısız bir koşu gizlenmez veya başarı gibi sunulmaz.
7. PR yalnızca son değişiklikler incelendikten ve gerekli kontroller geçtikten sonra birleştirilir. Tercih edilen yöntem squash merge'dür; her hedef ana dala tek, anlaşılır birleştirme commit'i olarak girmelidir.
8. Birleştirmeden sonra main başının beklenen commit olduğu, ana dal CI işlerinin başarılı olduğu, PR'ın kapandığı ve gereksiz dal kalmadığı doğrulanır. Birleştirilmiş çalışma dalı silinir. Depoda normal durumda yalnızca main ve o anda üzerinde çalışılan tek dal bulunur.
9. Dal silme işlemi kullanılan GitHub bağlantısında desteklenmiyorsa bu durum hemen ve açıkça bildirilir. Silinmiş gibi rapor verilmez; kullanıcıdan GitHub arayüzünden silmesi istenir ve ardından doğrudan dal listesi yeniden kontrol edilir.
10. Release yalnızca yayımlanabilir ürün çıktısı ve gerekli kontroller hazır olduğunda çıkarılır. Her commit veya dokümantasyon değişikliği yeni Release gerektirmez.
11. İş kapanırken tek durum özeti verilir: hedef, PR/merge commit'i, son CI sonucu, varsa Release, kullanıcı kabul durumu, kalan açık işler ve dal temizliği. Tamamlanmamış iş tamamlandı diye sunulmaz.

**Çalışma hatırlatıcısı:** Önce planla, kapsamı sabitle, tek yerde uygula, aynı PR içinde düzelt, son hâlini doğrula, birleştir, dalı temizle ve sonucu kaydet. Daha çok commit/PR üretmek değil, doğrulanmış ürünü sade bir geçmişle teslim etmek başarıdır.


## 18. Windows çıkarılabilir disk yazma koruması — kapsam kararı ve güvenli tasarım

**Kullanıcı kapsamı anlama kararı — 17 Kasım 2026**

Kullanıcı, Microsoft'un `Removable Disks: Deny write access` bilgisayar düzeyi ilkesinin yalnızca seçilen tek bir USB belleği değil, ilgili çıkarılabilir disk sınıfındaki uygun aygıtları etkilediğini kabul etti.

Bu kayıt yalnızca politikanın sistem kapsamını anladığını gösterir. DUYRA'ya Registry/Grup İlkesi yazma, aygıt açma, yönetici yükseltmesi veya politika değiştirme yetkisi vermez. Sonraki teknik incelemede güvenli koşullu Registry yazma/geri alma yolu doğrulanamadığı için DUYRA'nın otomatik aç/kapat işlevi kapsam dışı bırakılmıştır.

### 18.1. Kapsam ve kullanıcıya açıklanacak etki

- Microsoft belgesine göre `RemovableDiskDenyWriteAccess`, cihaz kapsamlıdır; Windows 10 sürüm 1809 ve sonrası ile Pro, Enterprise, Education ve IoT Enterprise edisyon ailelerinde desteklenir.
- Etkinleştirildiğinde yazma erişimi ilgili çıkarılabilir depolama sınıfında engellenir. Bu, tek bir DUYRA incelemesine veya yalnızca o anda seçilmiş USB'ye özel değildir.
- USB bellekler ve USB kart okuyucu üzerinden bağlanan microSD kartlar için gerçek sınıflandırma/etki ayrıca sınanmalıdır. Kart okuyucunun Windows'a hangi aygıt sınıfı olarak sunulduğu doğrulanmadan bütün microSD aygıtlarının kapsandığı varsayılmaz.
- Harici bir yönetim aracıyla politika uygulanacaksa bilgisayardaki diğer uygun çıkarılabilir disklerin de etkileneceği önceden açıklanır. Kullanıcının kapsamı anlamış olması DUYRA'ya politika yazma yetkisi vermez.
- İlke, Windows'un tüm dosya yazma yollarını engelleyen genel bir koruma değildir. Ağ paylaşımları, host klasör eşlemeleri, sanal disk dosyaları ve başka aygıt sınıfları bu ilkeye bakılarak koruma altında sayılmaz.
- Windows Home veya destek kapsamı dışındaki Windows sürümleri için bu ilkenin çalıştığı varsayılmaz.

### 18.2. Windows Sandbox ve VirtualBox kullanımı

Kullanıcı Windows Sandbox ve VirtualBox gibi sanallaştırma araçlarını kullanabilir. Bu ürünlerin farklı depolama yolları, çıkarılabilir disk ilkesiyle karıştırılmamalıdır.

- **Windows Sandbox:** Host klasörleri Sandbox içine eşlenebilir ve her eşlemenin `ReadOnly` davranışı ayrı belirlenir. Yazılabilir eşlenmiş klasörde yapılan değişiklikler Sandbox kapatıldıktan sonra host üzerinde kalabilir. DUYRA'nın çıkarılabilir disk ilkesi, bu klasör eşlemelerini otomatik olarak salt okunur yapmaz. Gerekli klasör koruması Sandbox yapılandırmasında ayrıca `ReadOnly=true` ile sağlanmalıdır.
- **VirtualBox:** Fiziksel USB aygıtı konuk sisteme USB passthrough ile verilebilir. Konuk sistem aygıtı kullanmaya başladığında aygıt host üzerinde kullanılamaz hâle gelebilir. Host Windows ilkesinin, konuk işletim sistemindeki yazma erişimini de engellediği varsayılmaz; konuk işletim sisteminin kendi koruması ayrıca değerlendirilir.
- **Sanal sabit diskler:** VDI/VHD gibi sanal disk dosyaları ile host'a bağlı fiziksel çıkarılabilir USB aynı şey değildir. Sanal disk dosyasının normal host dosya sistemi üzerindeki yazma davranışının bu politika tarafından engellendiği iddia edilmez.
- Bu nedenle test matrisi en az şu senaryoları ayrı tutar: host'a bağlı USB bellek, USB kart okuyucu içindeki microSD, Sandbox host-klasör eşlemesi (salt okunur/yazılabilir) ve VirtualBox fiziksel USB passthrough. Her senaryo yalnızca kendi doğruladığı kapsam için raporlanır.

### 18.3. Değişiklik kaynağını koruma ve geri alma şartları

HKLM `SOFTWARE\\Policies` altında `Deny_Write` değerinin bulunması tek başına bu değeri kimin yönettiğini kanıtlamaz. Yerel Grup İlkesi, etki alanı Grup İlkesi veya MDM/Intune gibi yönetim mekanizmaları aynı politika alanını yönetebilir.

**Mimari karar:** DUYRA bu politikayı açıp kapatmaz, Registry'ye yazmaz ve otomatik geri alma yapmaz. Windows Registry için dış yöneticiler karşısında güvenli koşullu güncelleme/geri alma garantisi doğrulanamadığından bu işlem ürün akışına eklenmeyecektir.

1. DUYRA Windows edisyonu, mevcut politika değeri ve bilinen yönetim sinyallerini salt okunur inceler.
2. Yönetim kaynağı belirsizse veya kurumsal yönetim işareti varsa sonuç fail-closed kalır; DUYRA politika değiştirmez ve yönetici yetkisi istemez.
3. Yetkili bir Windows yönetim aracıyla uygulanmış politika DUYRA tarafından yalnızca raporlanır. Uygulama kaynağının kesin sahipliği yalnızca bir Registry değerinden çıkarılmaz.
4. Kullanıcıya politika kaydının görünmesi ile gerçek aygıt yazma davranışının doğrulanması ayrı gösterilir.
5. DUYRA, gerçek aygıt testi yapmadıkça veya harici test kaydını doğrulayamadıkça “delil koruması doğrulandı” sonucunu üretmez.
6. Geri alma, bu tek seferlik kabul testi için DUYRA tarafından değil, test öncesi alınan VM snapshot'ının geri yüklenmesiyle yapılır. Bu işlem konuk VM durumunu geri alır; fiziksel test medyasındaki dosyaları geri almaz.

### 18.4. Test ve yayın koşulları

- GitHub Actions gerçek runner üzerindeki Registry/Grup İlkesi ayarlarını değiştirmez. İşlem protokolü testleri yalnızca sahte sağlayıcıyla yürütülür.
- Windows CI biçim, Clippy, birim testleri ve x64 derlemeyi doğrular. Linux görsel CI yalnızca ortak arayüzü doğrular; Windows ilkesinin veya gerçek USB davranışının kanıtı sayılmaz.
- Tek seferlik gerçek aygıt kabulü yalnızca snapshot alınmış, ayrılmış Windows 11 VM'sinde ve boş/atılabilir test USB belleğinde yapılır. USB-SSD veya microSD, Windows'un ilgili çıkarılabilir disk sınıfına girdiği ayrıca doğrulanmadan başarılı test örneği sayılmaz.
- Testte yerel/kurumsal yönetim kaynağı belirsizse işlem durdurulur. Politika yalnızca Windows'un yetkili yönetim aracıyla konuk VM içinde uygulanır; DUYRA bu işlemi yapmaz.
- Test sonrasında konuk VM, önceden alınmış snapshot'a döndürülür. Snapshot fiziksel USB medyasının içeriğini geri almaz.
- Gerçek delil aygıtına hiçbir koşulda yazma denemesi yapılmaz. Test sonucu yalnızca test edilen konuk VM ve test medyası kombinasyonu için raporlanır.

### 18.5. Aşamalı uygulama sırası

1. **Salt okunur ön kontrol (tamamlandı):** RSoP/Grup İlkesi, MDM kayıt işaretleri, hedef Storage CSP kaydı, bilgisayarın etki alanı üyeliği ve mevcut politika değeri değerlendirilir. Eksik veya belirsiz sinyal “Bilinmiyor” olarak kalır.
2. **Otomatik politika değiştirme (kapsam dışı):** Gerçek Windows Registry API'leriyle güvenli koşullu yazma/geri alma yolu belgelenemediği için DUYRA'da bu işlem sunulmayacaktır. PR #15'teki işlem modeli yalnızca sahte sağlayıcı testidir.
3. **Harici politika gözlemi (mevcut):** Yetkili Windows yönetim aracıyla uygulanmış politika DUYRA tarafından salt okunur gösterilir. Ön koşulların uygun görünmesi DUYRA'ya yazma yetkisi vermez.
4. **Tek seferlik gerçek aygıt kabulü (manuel, bekliyor):** Ayrılmış VM snapshot'ı ve boş/atılabilir test USB belleği kullanılır. Politika yalnızca konuk Windows'un desteklenen yönetim aracıyla uygulanır; test sonucu ve snapshot geri dönüşü ayrı kaydedilir. DUYRA, Registry'ye yazmaz, yönetim aracı başlatmaz ve geri alma yapmaz.

RSoP sorgusu `root\RSOP\Computer` içindeki `RSOP_RegistryPolicySetting` sınıfını kullanır; yalnızca tam ve başarılı bir sonuç “uygulandı/yok” olarak kabul edilir. Sorgu erişilemez veya zaman aşımına uğrarsa “Bilinmiyor” döner. MDM kayıt anahtarının varlığı yalnızca yönetim işaretidir; tek başına etkin Intune politikasının kanıtı değildir. Bu nedenle bu denetim bir kaynak ön değerlendirmesidir; yönetim kaynağının kesin olarak çözüldüğünü veya değişiklik izni verildiğini iddia etmez.

Kaynaklar:
- Microsoft MDM enrollment troubleshooting (eski enrollment registry girdileri kalabilir; Admin event ID 75/76): https://learn.microsoft.com/en-us/windows/client-management/mdm-diagnose-enrollment
- Microsoft Storage Policy CSP: https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-storage
- Windows Sandbox yapılandırması: https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-configure-using-wsb-file
- Oracle VirtualBox USB desteği: https://download.virtualbox.org/virtualbox/7.0.0_BETA3/UserManual.pdf


### 18.6. Windows Registry koşullu yazma sınırı — 17 Kasım 2026

İşlem/geri alma tasarımı sırasında Windows kayıt defteri API'lerinin dışarıdan gelen eşzamanlı değişiklikleri ezmeden koşullu güncelleme yapmaya elverişli olup olmadığı ayrıca incelendi.

Microsoft'un *Writing and Deleting Registry Data* belgesi, bir Registry anahtarının yazma işlemi sırasında eşzamanlamayı sağlamak amacıyla kilitlenemeyeceğini açıkça belirtir. Aynı belge, birden fazla Registry işleminin KTM/TxR işlemi içinde yürütülebileceğini söyler; ancak bu, başka yönetim kaynağından gelen değişiklik karşısında belirli bir değeri atomik olarak “beklenen sürüm hâlâ aynıysa yaz/sil” biçiminde güncelleyen bir compare-and-set sözleşmesi olarak belgelenmemiştir.

Ek gözlemler:

- `RegQueryInfoKey` anahtarın ve o anahtara bağlı değer girdilerinin en son değiştirilme zamanını verir. Bu zaman damgası hangi değerin, hangi kaynak tarafından değiştirildiğini gösteren bir sahiplik belirteci değildir.
- `RegNotifyChangeKeyValue` anahtar değişikliği için bildirim sağlar. Bildirim, kontrol ile sonraki yazma/silme arasındaki yarış penceresini atomik olarak kapatmaz.
- `RegQueryMultipleValues` aynı anahtardaki birden fazla değeri tutarlı/atomik biçimde okuyabilir. Microsoft'un okuma belgesi, başka uygulamaların bir değeri okuma ile kullanma arasında değiştirebileceği uyarısını ayrıca yapar; atomik çoklu okuma koşullu yazma anlamına gelmez.
- DUYRA örnekleri arasında kullanılan bir mutex, Grup İlkesi, MDM/Intune, PowerShell veya başka bir yönetim sürecinin aynı Registry alanına yazmasını engellemez.
- TxR/KTM'nin çoklu işlemleri bir transaction içinde gruplaması, tek başına haricî yazarlar karşısında gerekli compare-and-set / compare-and-remove garantisinin mevcut olduğunu kanıtlamaz. Bu garanti resmi belgelerle ve desteklenen Windows sürümlerinde yarış testleriyle gösterilmeden TxR çözüm olarak kabul edilmeyecektir.

**Mimari karar:**

1. `read → timestamp/revision kontrolü → RegSetValueEx/RegDeleteValue → read-back` akışı güvenli koşullu yazma veya geri alma olarak sunulmayacaktır. Kontrol ile yazma arasındaki zaman aralığında dış kaynak değişiklik yapabilir.
2. Registry anahtarının ACL/security descriptor değerlerini kilit amacıyla değiştirmek bu işin kapsamı dışındadır; mevcut yönetim güvenlik sınırlarını değiştiren ayrı ve daha yüksek riskli bir sistem müdahalesi olur.
3. PR #15 ile eklenen `src/policy_transaction.rs`, yalnızca sahte sağlayıcıda atomik koşullu yazma/silme sözleşmesinin beklenen davranışını test eder. `#[cfg(test)]` altındadır; gerçek Windows Registry sağlayıcısı değildir ve CI sonucu Windows kayıt defteri atomikliğini kanıtlamaz.
4. Bu nedenle DUYRA'ya HKLM Registry/Grup İlkesi yazımı veya otomatik geri alma eklenmeyecektir. Salt okunur yönetim kaynağı ön kontrolü ve yetkili yönetim aracıyla uygulanmış politikanın salt okunur gözlemi mevcut ürün davranışıdır.
5. Otomatik değişiklik ancak yönetim kaynağı ve sahipliği güvenilir biçimde belirlenebildiğinde, dış kaynak değişikliklerine karşı platformun desteklediği gerçek koşullu işlem yolu kanıtlandığında ve geri alma/çökme kurtarma senaryoları izole VM'de doğrulandığında yeniden ele alınabilir. Bu şartlar sağlanmıyorsa davranış fail-closed kalır.

Kaynaklar (Microsoft Learn, 17 Kasım 2026 tarihinde incelendi):

- Writing and Deleting Registry Data: https://learn.microsoft.com/en-us/windows/win32/sysinfo/writing-and-deleting-registry-data
- Retrieving Data from the Registry: https://learn.microsoft.com/en-us/windows/win32/sysinfo/retrieving-data-from-the-registry
- RegQueryInfoKey: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryinfokeyw
- RegNotifyChangeKeyValue: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regnotifychangekeyvalue
- RegOpenKeyTransacted: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeytransacteda
- Kernel Transaction Manager: https://learn.microsoft.com/en-us/windows/win32/ktm/kernel-transaction-manager-portal

### 18.7. Tek seferlik VM kabul prosedürü

Gerçek aygıt doğrulaması yalnızca [Windows VM kabul prosedüründeki](docs/windows-write-protection-vm-acceptance.md) güvenlik kapıları ve geri dönüş adımlarıyla yürütülür. Test medyası fiziksel olduğundan VM snapshot'ı medyanın içeriğini geri almaz. DUYRA'nın politika yazma veya geri alma akışı yoktur.

## 19. Uygulama genelinde otomatik tanılama altyapısı

**Ürün ilkesi:** Kullanıcı, DUYRA'nın güvenli biçimde kendi başına yapabileceği bir tanılamayı elle komut çalıştırarak veya ham çıktıyı sohbete taşıyarak yapmak zorunda bırakılmamalıdır.

### Çalışma biçimi

- Salt okunur ve düşük etkili başlangıç kontrolleri uygulama açılışında arka planda çalışır; arayüz bloke edilmez.
- Her kontrol bir bileşen adı, sabit kontrol kimliği, durum ve zaman damgası üretir. Hata durumları da rapora "bilinmiyor/okunamadı" şeklinde eklenir; ham komut çıktısı saklanmaz.
- Kullanıcıya ait gerçek veri, delil dosyası, USB içeriği veya ağ bağlantısı gerektiren kontroller otomatik başlangıç denetimine dahil edilmez.
- Daha yüksek riskli testler (yönetici yükseltmesi, policy/registry yazma, sürücü/aygıt açma, yazma/geri alma) otomatik çalıştırılmaz; yalnızca izole VM, snapshot ve ayrılmış test aygıtıyla yürütülür.
- Denetim tamamlandığında güncel rapor ve sınırlı olay geçmişi yerel tanılama klasörüne yazılır. Rapor aynı dosya üzerinde güncellenir; olay kaydı boyut sınırında döndürülür.

### Yerel inceleme deposu başlangıç denetimi — 17 Kasım 2026

Uygulama başlatılırken inceleme kayıt deposunun kullanılabilirliği sabit kodlu bir tanılama olayı olarak kaydedilir:

- ok / examination_store_ready: Depo açıldı ve kayıt dosyaları okunabildi.
- warning / examination_store_load_warnings: Depodaki bazı kayıtlar okunamadı. Sorunlu dosyalar silinmez veya değiştirilmez.
- unknown / examination_store_unavailable: Depo klasörü oluşturulamadı, izinleri korunamadı veya depo açılamadı.

Olay yalnızca local_storage, examination_store, durum ve sabit kod alanlarını içerir. Kayıt klasörü yolu, inceleme numarası/UUID'si, dosya adı, dosya içeriği veya işletim sistemi hata metni tanılama günlüğüne eklenmez. İnceleme kaydı deposu ile tanılama deposu ayrı kalır. Bu kontrol mevcut inceleme deposunu açma/okuma davranışının sonucunu raporlar; delil dosyalarını taramaz, açmaz veya değiştirmez.

Kabul testleri; kullanılabilir boş depo, okunamayan fakat korunan kayıt dosyası ve kullanılamayan depo yolu senaryolarını kapsar.

### Gizlilik ve bütünlük

- Tanılama API'si serbest metin değil, sabit bileşen/kontrol/durum kodları alır. Böylece kullanıcı adı, UPN, tenant/device GUID, domain adı, dosya yolu, seri numarası ve ham çıktıların yanlışlıkla kaydedilmesi önlenir.
- İlk sürüm için rapor JSON, olay günlüğü JSONL biçimindedir. Şema sürümlenir; alanların kapsamı açıkça belgelenir.
- Dosyalar kullanıcı uygulama verisi altında tutulur; ağ veya telemetri gönderimi yoktur. Paylaşım her zaman kullanıcı eylemidir.
- Tanılama dosyaları inceleme/delil geçmişinden ayrı kalır. Uygulama olayları ile delil işlemleri aynı günlükte birleştirilmez.
- Yazma hatası, eksik izin veya başarısız kontrol saklanamazsa kullanıcıya açıkça bildirilir; başarılı kayıt yapılmış gibi gösterilmez.

### Aşamalı kapsam

1. **İlk adım:** Windows başlangıç ön kontrolü ve politika yönetim kaynağı bulgularını otomatik raporla.
2. **Sonraki modüller:** Güncelleme doğrulaması, uygulama bağımlılıkları, ses işleme motoru ve rapor oluşturma gibi modüller aynı sabit olay API'sine katılır.
3. **İleri aşama:** Kullanıcı onayıyla tanılama paketini inceleme ve seçmeli dışa aktarma; otomatik upload hiçbir zaman varsayılan olmaz.
4. Her yeni denetim için risk sınıfı, saklanan alanlar, rapor şeması ve kabul testi belgelenmeden otomatik başlangıç kontrolüne eklenmez.
