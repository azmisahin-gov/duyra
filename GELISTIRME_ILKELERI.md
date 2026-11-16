# DUYRA geliştirme ilkeleri

> Bugün küçük bir ürün yapıyoruz. Yarın büyümesine izin verecek kadar düzenli, bugün geliştirmeyi zorlaştırmayacak kadar sade tutuyoruz.

## Çalışma kuralımız

**Tek hedef seç. Tamamla. Doğrula. Sonra yayımla. Doğrulamadan sonraki hedefe geçme.**

1. Her çalışmada yalnızca bir kullanıcı ihtiyacını ele alırız. Kapsamı ve başarı ölçütünü işe başlamadan belirleriz.
2. İhtiyaç dışı altyapı, bağımlılık, ayar ve belge eklemeyiz. Bugünkü işi çözmeyen karmaşıklığı sonraya bırakırız.
3. Kod değişikliğini önce hızlı denetimlerden geçiririz: biçim, Clippy, test ve Windows derlemesi.
4. Denetimler geçince özellik tamamlanmış sayılmaz. Gerçek kullanıcı akışını da kabul ölçütlerine göre deneriz.
5. Hata görülürse yeni özelliğe geçmeyiz. Önce hatayı görünür kılar, tekrar üretir, düzeltir ve aynı testi yeniden yaparız.
6. Bir Release'in yayımlanması, özelliğin kullanıcıda çalıştığının kanıtı değildir. Kullanıcı doğrulaması tamamlanmadan işi kapatmayız.

## Bitti sayma ölçütü

Bir iş ancak aşağıdakiler sağlandığında tamamdır:

- Tanımlanan işi gerçekten yapıyor.
- Hata ve sınır durumları anlaşılır şekilde ele alınıyor.
- Otomatik kontroller geçiyor.
- Gerçek kullanım testi başarılı; sonucu sürüm ve gözlenen davranışla kaydedilmiş.
- Yalnızca gerekli kod, test ve kısa sürüm notu depoya eklenmiş.

## Güncelleme için özel kabul testi

Otomatik güncelleme ayrı bir altyapı işi olarak değil, kullanıcı deneyiminin parçası olarak test edilir:

1. Bilgisayarda önceki kararlı sürüm kurulu/açılabilir durumda tutulur.
2. Yeni sürüm GitHub Release'e yayımlanır ve imzalı paket doğrulanır.
3. Eski sürüm açılır; denetimin başladığı, yeni sürümün bulunduğu ve sonucun ne olduğu kullanıcıya görünür olur.
4. Paket indirilir, imza ve bütünlük doğrulanır, uygulama değiştirilir ve yeniden başlatılır.
5. Yeniden açılan uygulamanın sürüm numarası beklenen yeni sürüm olmalıdır.
6. Başarısızlıkta neden kaydedilir; eski uygulama kullanılabilir kalır. Başarı kanıtlanmadan yeni özellik çalışmasına geçilmez.

**Yayın kontrol listesi:** kod denetimi → Windows derlemesi → imzalı paketi doğrulama → Release → önceki sürümden gerçek güncelleme → yeni sürüm numarasını doğrulama.

## Kapsamı küçük tutma

- Yeni özellik başına ayrı bir Release ve kısa sürüm notu.
- Teknik belge yalnızca karar, kullanım veya bakım için gerekiyorsa eklenir.
- Geliştirme ilkeleri bu dosyada tutulur; README yalnızca buraya bağlantı verir.
- Çalışmayan veya sınanmamış bir davranış hiçbir yerde tamamlanmış gibi anlatılmaz.
