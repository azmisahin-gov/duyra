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

## 10. Şu anki hedef

**Mevcut hedef: Windows CI üzerinde DUYRA ana ekranının otomatik görüntüsünü alıp inceleyebilmek.**

- v0.1.7 ana çalışma alanı yayımlandı; kullanıcı ekranında görülen simge ve alt durum çubuğu kusurları v0.1.8 ile düzeltildi ve 1920×1080 gerçek ekranda doğrulandı.
- v0.1.8 → v0.1.9 güncellemesi gerçek kullanıcı bilgisayarında, uygulama kapatılıp yeniden açıldığında tamamlandı.
- v0.1.9 güncelleyici değişikliği CI kontrollerinden geçti ve Release yayımlandı. Altı saatlik kontrol, kullanıcı onayıyla kurulum ve erteleme kodda bulunuyor.
- v0.1.9'in yeni "Şimdi güncelle / Daha sonra" bildirimini kullanıcı ekranında doğrulamak için daha yeni bir gerçek sürüm gerekir. Sırf test için boş Release yayımlanmaz; bu kabul testi sonraki doğal sürümde yapılacaktır.
- İlk Windows CI görsel denemesi Actions run 43987188203'te uygulamayı derledi; ancak uygulama pencere açılmadan `1` çıkış koduyla sonlandı. Oturum numarası `2` idi; tanı artifact'i 14285593012 yüklendi, fakat PNG üretilemedi.
- İkinci deneme için yakalama betiğine standart çıktı/hata ve grafik bağdaştırıcısı tanılaması eklendi. Yeni deneme Actions run 43987622095 üzerinden izlenmektedir.
- Görsel yakalama çalıştığı doğrulanana kadar bu otomasyon tamamlanmış sayılmaz.
- Görüntü alınabilse bile bu, kullanıcının kendi bilgisayarındaki son görsel kabul testinin yerine geçmez. Kullanıcı bilgisayarı ile CI ortamının çözünürlüğü ve grafik sürücüsü farklı olabilir.
- CI ekran yakalama yöntemi güvenilir değilse zorlanmaz; kullanıcıdan ekran görüntüsü alma yöntemi devam eder. Gerekirse etkileşimli Windows runner seçeneği ayrıca değerlendirilir.
- Bu hedef doğrulanmadan yeni delil/deşifre özelliklerine geçilmez.

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

**Aktif teknik deneme:** `.github/workflows/gorsel-denetim.yml` ve `scripts/ci-ekran-goruntusu.ps1` eklendi. İlk çalışma uygulama pencere açmadan çıktığı için başarısız oldu; ikinci çalışmada stderr ve grafik aygıtı da kaydediliyor (Actions run 43987622095).

Hedeflenen yaklaşım:

- Windows uygulaması derlendikten sonra CI ortamında DUYRA başlatılır.
- Uygulama penceresi açıldıktan ve arayüz kararlı hâle geldikten sonra Genel Bakış ekranının görüntüsü alınır.
- Görüntü, GitHub Actions çalışma kaydına bağlı bir artifact olarak saklanır; Release paketine ve web sayfasına eklenmez.
- İlk aşamada yalnızca uygulamanın açılması ve ana ekranın temel yerleşimi kontrol edilir. Sonraki ekranlar geliştikçe kapsama alınır.
- Ekran görüntüsünün CI ortamından güvenilir biçimde alınabildiği, indirilebildiği ve mühendis/inceleme sürecinde gerçekten görüntülenebildiği küçük bir teknik denemeyle doğrulanır.
- CI ortamı grafik uygulamayı güvenilir biçimde başlatamıyor veya görüntüyü inceleme sürecine sunamıyorsa bu yöntem zorlanmaz; manuel ekran görüntüsü gönderme yöntemi devam eder.
- Görüntüde kullanıcı verisi, gerçek delil veya kişisel dosya yolu bulunmamalıdır. Test verisi sabit ve yapay olmalıdır.

Bu otomasyon, gerçek kullanıcının bilgisayarındaki görünümün birebir garantisi değildir. Sadece tekrarlanabilir bir erken görsel regresyon kontrolüdür; kullanıcı kabul testinin yerine geçmez.

### 12.4. İlk açılış pencere konumu

1920×1080 masaüstünden gönderilen v0.1.8 görüntüsünde DUYRA penceresi ilk açılışta ekranın sol üst tarafına yakın konumlanmıştır; kullanıcı pencereyi elle taşımamıştır.

Bu bir gözlem olarak kaydedilmiştir. Henüz kesin karar değildir. İleride küçük bir arayüz hedefinde şu seçenekler değerlendirilecektir:

- İlk açılışta pencereyi kullanılabilir ekran alanının ortasında açmak.
- Sonraki açılışlarda kullanıcının son pencere konumunu ve boyutunu hatırlamak.
- Kaydedilen konum artık bağlı ekranlarda mevcut değilse pencereyi görünür ve erişilebilir bir konuma geri almak.

Uygulama varsayılan olarak tam ekran başlatılmak zorunda değildir. Bu davranış değiştirilmeden önce ilk açılış ve sonraki açılış senaryoları ayrı kabul ölçütleriyle kararlaştırılmalıdır.

