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

**Mevcut hedef: v0.1.8 ana ekran görsel düzeltmelerini tamamlamak ve gerçek kullanıcı ekranında doğrulamak.**

- v0.1.7 ana çalışma alanı kullanıcı bilgisayarında açıldı ve ekran görüntüsüyle incelendi.
- Kullanıcı ekranında bazı Unicode simgelerin yazı tipi nedeniyle boş/kutu karakter olarak göründüğü ve alt durum çubuğundaki metinlerin birbirine yaklaştığı görüldü.
- Bu hedefte simgeler yazı tipine bağlı olmayan yerel çizimlerle gösterilir ve alt durum çubuğu aralıkları düzeltilir.
- Windows CI kontrolleri, testler ve derleme başarılı olmalı; v0.1.8 Release yayımlanmalıdır.
- v0.1.7'den v0.1.8'e gerçek güncelleme denenmeli ve yeni sürüm açıldıktan sonra ekran görüntüsü yeniden incelenmelidir.
- Bu hedef doğrulanmadan otomatik güncelleyici iyileştirmesine veya yeni delil/deşifre özelliklerine geçilmez.

## 11. Kapsamı küçük tutma

- Yeni özellik başına ayrı hedef, kabul ölçütü ve gerekli olduğunda ayrı Release.
- Teknik belge yalnızca karar, kullanım veya bakım için gerekiyorsa eklenir.
- Geliştirme ilkeleri ve ürün kararları bu dosyada tutulur; README buraya bağlantı verir.
- Çalışmayan veya sınanmamış bir davranış hiçbir yerde tamamlanmış gibi anlatılmaz.

## 12. Sonraki hedefler ve ekran doğrulama yöntemi

Bu bölüm sıradaki işleri görünür kılar; işler mevcut hedef tamamlanmadan başlatılmaz.

### 12.1. Otomatik güncelleyiciyi güvenli hâle getirme

Mevcut kodun gerçek davranışı:

- Güncelleme kontrolü yalnızca uygulama başlangıcında bir kez yapılır.
- Altı saatlik periyodik kontrol yoktur.
- Yeni sürüm bulunursa mevcut uygulama paketi indirir, imza doğrulaması yapar, kurar ve uygulamayı otomatik yeniden başlatır.
- Kullanıcıya önce karar vereceği bir güncelleme bildirimi ve erteleme akışı henüz yoktur.
- Açık inceleme/işlem durumunu takip ederek yeniden başlatmayı güvenli ana erteleme mekanizması henüz yoktur.

Kullanıcının istediği sonraki hedef:

- Varsayılan kontrol aralığı 6 saat olmalıdır; açılış kontrolü ve uygulama açık kaldığı sürece periyodik kontrol birlikte ele alınmalıdır.
- Kontrol zamanları eşzamanlı istek yükü oluşturmamak için rastgele küçük gecikmeyle dağıtılmalı; ağ sınırı ve geçici hatalarda artan bekleme uygulanmalıdır.
- Yeni sürüm bulunduğunda kullanıcı açıkça bilgilendirilmelidir.
- Kullanıcı isterse "Şimdi güncelle" diyebilmelidir.
- Diğer durumda indirme ve imza/bütünlük doğrulaması arka planda yapılabilir; kurulum ve yeniden başlatma kullanıcının çalışmasını kesmemelidir.
- Güvenli boşta kalma algısı tek başına yeterli sayılmamalı; kullanıcının onayı/tercihi, açık inceleme, kaydedilmemiş değişiklik ve süren işler birlikte dikkate alınmalıdır.
- Güncelleme başarısız olursa mevcut sürüm çalışmaya devam etmeli ve hata yerel günlüğe yazılmalıdır.
- Güncelleme imzalama anahtarı ile olası lisans anahtarları birbirinden ayrı kalmalıdır.

Bu iş, v0.1.8 ana ekran düzeltmesi doğrulandıktan sonra; delil işleme özellikleri eklenmeden önce ayrı bir hedef olarak ele alınacaktır. Uygulanıp gerçek kullanıcı tarafından test edilmeden mevcut davranış gibi anlatılmayacaktır.

### 12.2. Ekran görüntüsüyle gerçek kullanıcı doğrulaması

- Asistan, kullanıcının Windows masaüstünü bu sohbet içinde sürekli ve kendiliğinden canlı izleyemez.
- Görsel kontrol için kullanıcı her ilgili sürüm/arayüz değişikliğinden sonra uygulamanın ekran görüntüsünü sohbete yükler.
- Tercih edilen görüntü, DUYRA penceresinin tamamını; pencere boyutu, metinler, gezinme ve durum çubuğu görünecek şekilde içermelidir. Yerleşim sorunu şüphesinde tam ekran görüntüsü de eklenebilir.
- Görüntü, kod/CI kontrolünün yerine geçmez; otomatik denetimden sonra gerçek kullanıcı arayüzü kabul testinin kanıtıdır.
- Delil içeriği, kişisel bilgiler veya gizli dosya yolları içeren görüntüler paylaşılmamalıdır.
- Ekran görüntüleri varsayılan olarak depoya eklenmez. Yalnızca otomatik görsel regresyon testi için açıkça gerekli ve temizlenmiş bir örnek olduğunda değerlendirilir.
- Görsel incelemede tespit edilen her sorun önce mevcut hedefin parçası mı diye değerlendirilir; yeni fikir olması tek başına kapsamı genişletmez.

