lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Durum"),
        ("Your Desktop", "Sizin Masaüstünüz"),
        ("desk_tip", "Masaüstünüze bu ID ve şifre ile erişilebilir"),
        ("Password", "Şifre"),
        ("Ready", "Hazır"),
        ("Established", "Bağlantı sağlandı"),
        ("connecting_status", "Bağlanılıyor "),
        ("Enable Service", "Servisi aktif et"),
        ("Start Service", "Servisi başlat"),
        ("Service is running", "Servis çalışıyor"),
        ("Service is not running", "Servis çalışmıyor"),
        ("not_ready_status", "Hazır değil. Bağlantınızı kontrol edin"),
        ("Control Remote Desktop", "Bağlanılacak Uzak Bağlantı ID"),
        ("Transfer File", "Dosya transferi"),
        ("Connect", "Bağlan"),
        ("Recent Sessions", "Son Bağlanılanlar"),
        ("Address Book", "Adres Defteri"),
        ("Confirmation", "Onayla"),
        ("TCP Tunneling", "TCP Tünelleri"),
        ("Remove", "Kaldır"),
        ("Refresh random password", "Yeni rastgele şifre oluştur"),
        ("Set your own password", "Kendi şifreni oluştur"),
        ("Enable Keyboard/Mouse", "Klavye ve Fareye izin ver"),
        ("Enable Clipboard", "Kopyalanan geçici veriye izin ver"),
        ("Enable File Transfer", "Dosya Transferine izin ver"),
        ("Enable TCP Tunneling", "TCP Tüneline izin ver"),
        ("IP Whitelisting", "İzinli IP listesi"),
        ("ID/Relay Server", "ID/Relay Sunucusu"),
        ("Stop service", "Servisi Durdur"),
        ("Change ID", "ID Değiştir"),
        ("Website", "Website"),
        ("About", "Hakkında"),
        ("Mute", "Sustur"),
        ("Audio Input", "Ses Girişi"),
        ("Enhancements", ""),
        ("Hardware Codec", ""),
        ("Adaptive Bitrate", ""),
        ("ID Server", "ID Sunucu"),
        ("Relay Server", "Relay Sunucu"),
        ("API Server", "API Sunucu"),
        ("invalid_http", "http:// veya https:// ile başlamalıdır"),
        ("Invalid IP", "Geçersiz IP adresi"),
        ("id_change_tip", "Yalnızca a-z, A-Z, 0-9 ve _ (alt çizgi) karakterlerini kullanabilirsiniz. İlk karakter a-z veya A-Z olmalıdır. Uzunluk 6 ile 16 karakter arasında olmalıdır."),
        ("Invalid format", "Hatalı Format"),
        ("server_not_support", "Henüz sunucu tarafından desteklenmiyor"),
        ("Not available", "Erişilebilir değil"),
        ("Too frequent", "Çok sık"),
        ("Cancel", "İptal"),
        ("Skip", "Geç"),
        ("Close", "Kapat"),
        ("Retry", "Tekrar Dene"),
        ("OK", "Tamam"),
        ("Password Required", "Şifre Gerekli"),
        ("Please enter your password", "Lütfen şifrenizi giriniz"),
        ("Remember password", "Şifreyi hatırla"),
        ("Wrong Password", "Hatalı şifre"),
        ("Do you want to enter again?", "Tekrar giriş yapmak ister misiniz?"),
        ("Connection Error", "Bağlantı Hatası"),
        ("Error", "Hata"),
        ("Reset by the peer", "Eş tarafında sıfırla"),
        ("Connecting...", "Bağlanılıyor..."),
        ("Connection in progress. Please wait.", "Bağlantı sağlanıyor. Lütfen bekleyiniz."),
        ("Please try 1 minute later", "Lütfen 1 dakika sonra tekrar deneyiniz"),
        ("Login Error", "Giriş Hatalı"),
        ("Successful", "Başarılı"),
        ("Connected, waiting for image...", "Bağlandı. Görüntü bekleniyor..."),
        ("Name", "Ad"),
        ("Type", "Tip"),
        ("Modified", "Değiştirildi"),
        ("Size", "Boyut"),
        ("Show Hidden Files", "Gizli Dosyaları Göster"),
        ("Receive", "Al"),
        ("Send", "Gönder"),
        ("Refresh File", "Dosyayı yenile"),
        ("Local", "Yerel"),
        ("Remote", "Uzak"),
        ("Remote Computer", "Uzak Bilgisayar"),
        ("Local Computer", "Yerel Bilgisayar"),
        ("Confirm Delete", "Silmeyi Onayla"),
        ("Delete", "Sil"),
        ("Properties", "Özellikler"),
        ("Multi Select", "Çoklu Seçim"),
        ("Empty Directory", "Boş Klasör"),
        ("Not an empty directory", "Klasör boş değil"),
        ("Are you sure you want to delete this file?", "Bu dosyayı silmek istediğinize emin misiniz?"),
        ("Are you sure you want to delete this empty directory?", "Bu boş klasörü silmek istediğinize emin misiniz?"),
        ("Are you sure you want to delete the file of this directory?", "Bu klasördeki dosyayı silmek istediğinize emin misiniz?"),
        ("Do this for all conflicts", "Bunu tüm çakışmalar için yap"),
        ("This is irreversible!", "Bu işlem geri döndürülemez!"),
        ("Deleting", "Siliniyor"),
        ("files", "dosyalar"),
        ("Waiting", "Bekleniyor"),
        ("Finished", "Tamamlandı"),
        ("Speed", "Hız"),
        ("Custom Image Quality", "Özel Görüntü Kalitesi"),
        ("Privacy mode", "Gizlilik modu"),
        ("Block user input", "Kullanıcı girişini engelle"),
        ("Unblock user input", "Kullanı girişine izin ver"),
        ("Adjust Window", "Pencereyi Ayarla"),
        ("Original", "Orjinal"),
        ("Shrink", "Küçült"),
        ("Stretch", "Uzat"),
        ("Good image quality", "İyi görüntü kalitesi"),
        ("Balanced", "Dengelenmiş"),
        ("Optimize reaction time", "Tepki süresini optimize et"),
        ("Custom", "Özel"),
        ("Show remote cursor", "Uzaktaki fare imlecini göster"),
        ("Show quality monitor", ""),
        ("Disable clipboard", "Hafızadaki kopyalanmışları engelle"),
        ("Lock after session end", "Bağlantıdan sonra kilitle"),
        ("Insert", "Ekle"),
        ("Insert Lock", "Kilit Ekle"),
        ("Refresh", "Yenile"),
        ("ID does not exist", "ID bulunamadı"),
        ("Failed to connect to rendezvous server", "ID oluşturma sunucusuna bağlanılamadı"),
        ("Please try later", "Dağa sonra tekrar deneyiniz"),
        ("Remote desktop is offline", "Uzak masaüstü kapalı"),
        ("Key mismatch", "Anahtar uyumlu değil"),
        ("Timeout", "Zaman aşımı"),
        ("Failed to connect to relay server", "Relay sunucusuna bağlanılamadı"),
        ("Failed to connect via rendezvous server", "ID oluşturma sunucusuna bağlanılamadı"),
        ("Failed to connect via relay server", "Relay oluşturma sunucusuna bağlanılamadı"),
        ("Failed to make direct connection to remote desktop", "Uzak masaüstüne doğrudan bağlantı kurulamadı"),
        ("Set Password", "Şifre ayarla"),
        ("OS Password", "İşletim Sistemi Şifresi"),
        ("install_tip", "Kullanıcı Hesabı Denetimi nedeniyle, RustDesk bir uzak masaüstü olarak düzgün çalışmayabilir. Bu sorunu önlemek için, RustDesk'i sistem seviyesinde kurmak için aşağıdaki butona tıklayın."),
        ("Click to upgrade", "Yükseltmek için tıklayınız"),
        ("Click to download", "İndirmek için tıklayınız"),
        ("Click to update", "Güncellemek için tıklayınız"),
        ("Configure", "Ayarla"),
        ("config_acc", "Masaüstünüzü dışarıdan kontrol etmek için RustDesk'e \"Erişilebilirlik\""),
        ("config_screen", "Masaüstünüzü dışarıdan kontrol etmek için RustDesk'e \"Ekran Kaydı\" iznini vermeniz gerekir."),
        ("Installing ...", "Yükleniyor ..."),
        ("Install", "Yükle"),
        ("Installation", "Kurulum"),
        ("Installation Path", "Kurulacak olan konum"),
        ("Create start menu shortcuts", "Başlangıca kısayol oluştur"),
        ("Create desktop icon", "Masaüstüne kısayol oluştur"),
        ("agreement_tip", "Kurulumu başlatarak, lisans sözleşmesinin şartlarını kabul etmiş olursunuz."),
        ("Accept and Install", "Kabul Et ve Yükle"),
        ("End-user license agreement", "Son kullanıcı lisans anlaşması"),
        ("Generating ...", "Oluşturuluyor..."),
        ("Your installation is lower version.", "Kurulumunuz alt sürümdür."),
        ("not_close_tcp_tip", "Tüneli kullanırken bu pencereyi kapatmayın"),
        ("Listening ...", "Dinleniyor..."),
        ("Remote Host", "Uzak Sunucu"),
        ("Remote Port", "Uzak Port"),
        ("Action", "Eylem"),
        ("Add", "Ekle"),
        ("Local Port", "Yerel Port"),
        ("setup_server_tip", "Daha hızlı bağlantı için kendi sunucunuzu kurun"),
        ("Too short, at least 6 characters.", "Çok kısa en az 6 karakter gerekli."),
        ("The confirmation is not identical.", "Doğrulama yapılamadı."),
        ("Permissions", "İzinler"),
        ("Accept", "Kabul Et"),
        ("Dismiss", "Reddet"),
        ("Disconnect", "Bağlanıyı kes"),
        ("Allow using keyboard and mouse", "Klavye ve fare kullanımına izin ver"),
        ("Allow using clipboard", "Pano kullanımına izin ver"),
        ("Allow hearing sound", "Sesi duymaya izin ver"),
        ("Allow file copy and paste", "Dosya kopyalamaya ve yapıştırmaya izin ver"),
        ("Connected", "Bağlandı"),
        ("Direct and encrypted connection", "Doğrudan ve şifreli bağlantı"),
        ("Relayed and encrypted connection", "Aktarmalı ve şifreli bağlantı"),
        ("Direct and unencrypted connection", "Doğrudan ve şifrelenmemiş bağlantı"),
        ("Relayed and unencrypted connection", "Aktarmalı ve şifrelenmemiş bağlantı"),
        ("Enter Remote ID", "Uzak ID'yi Girin"),
        ("Enter your password", "Şifrenizi girin"),
        ("Logging in...", "Giriş yapılıyor..."),
        ("Enable RDP session sharing", "RDP oturum paylaşımını etkinleştir"),
        ("Auto Login", "Otomatik giriş"),
        ("Enable Direct IP Access", "Doğrudan IP Erişimini Etkinleştir"),
        ("Rename", "Yeniden adlandır"),
        ("Space", "Boşluk"),
        ("Create Desktop Shortcut", "Masaüstü kısayolu oluşturun"),
        ("Change Path", "Yolu değiştir"),
        ("Create Folder", "Klasör oluşturun"),
        ("Please enter the folder name", "Lütfen klasör adını girin"),
        ("Fix it", "Düzenle"),
        ("Warning", "Uyarı"),
        ("Login screen using Wayland is not supported", "Wayland kullanan giriş ekranı desteklenmiyor"),
        ("Reboot required", "Yeniden başlatma gerekli"),
        ("Unsupported display server ", "Desteklenmeyen görüntü sunucusu"),
        ("x11 expected", "x11 bekleniyor"),
        ("Port", "Port"),
        ("Settings", "Ayarlar"),
        ("Username", "Kullanıcı Adı"),
        ("Invalid port", "Geçersiz port"),
        ("Closed manually by the peer", "Eş tarafından manuel olarak kapatıldı"),
        ("Enable remote configuration modification", "Uzaktan yapılandırma değişikliğini etkinleştir"),
        ("Run without install", "Yüklemeden çalıştır"),
        ("Always connected via relay", "Her zaman röle ile bağlı"),
        ("Always connect via relay", "Always connect via relay"),
        ("whitelist_tip", "Bu masaüstüne yalnızca yetkili IP adresleri bağlanabilir"),
        ("Login", "Giriş yap"),
        ("Logout", "Çıkış yap"),
        ("Tags", "Etiketler"),
        ("Search ID", "ID Arama"),
        ("Current Wayland display server is not supported", "Mevcut Wayland görüntüleme sunucusu desteklenmiyor"),
        ("whitelist_sep", "Virgül, noktalı virgül, boşluk veya yeni satır ile ayrılmış"),
        ("Add ID", "ID Ekle"),
        ("Add Tag", "Etiket Ekle"),
        ("Unselect all tags", "Tüm etiketlerin seçimini kaldır"),
        ("Network error", "Bağlantı hatası"),
        ("Username missed", "Kullanıcı adı boş"),
        ("Password missed", "Şifre boş"),
        ("Wrong credentials", "Yanlış kimlik bilgileri"),
        ("Edit Tag", "Etiketi düzenle"),
        ("Unremember Password", "Şifreyi Unut"),
        ("Favorites", "Favoriler"),
        ("Add to Favorites", "Favorilere ekle"),
        ("Remove from Favorites", "Favorilerden çıkar"),
        ("Empty", "Boş"),
        ("Invalid folder name", "Geçersiz klasör adı"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Ana bilgisayar adı"),
        ("Discovered", "Keşfedilenler"),
        ("install_daemon_tip", "Başlangıçta başlamak için sistem hizmetini yüklemeniz gerekir."),
        ("Remote ID", "Uzak ID"),
        ("Paste", "Yapıştır"),
        ("Paste here?", "Buraya yapıştır?"),
        ("Are you sure to close the connection?", "Bağlantıyı kapatmak istediğinize emin misiniz?"),
        ("Download new version", "Yeni sürümü indir"),
        ("Touch mode", "Dokunmatik mod"),
        ("Mouse mode", "Fare modu"),
        ("One-Finger Tap", "Tek Parmakla Dokunma"),
        ("Left Mouse", "Sol Fare"),
        ("One-Long Tap", "Tek-Uzun Dokunma"),
        ("Two-Finger Tap", "İki-Parmak Dokunma"),
        ("Right Mouse", "Sağ Fare"),
        ("One-Finger Move", "Tek Parmakla Hareket"),
        ("Double Tap & Move", "Çift Dokun ve Taşı"),
        ("Mouse Drag", "Fare Sürükleme"),
        ("Three-Finger vertically", "Dikey olarak üç parmak"),
        ("Mouse Wheel", "Fare Tekerliği"),
        ("Two-Finger Move", "İki Parmakla Hareket"),
        ("Canvas Move", "Tuval Hareketi"),
        ("Pinch to Zoom", "İki parmakla yakınlaştır"),
        ("Canvas Zoom", "Tuval Yakınlaştırma"),
        ("Reset canvas", "Tuvali sıfırla"),
        ("No permission of file transfer", "Dosya aktarımı izni yok"),
        ("Note", "Not"),
        ("Connection", "Bağlantı"),
        ("Share Screen", "Ekranı Paylaş"),
        ("CLOSE", "KAPAT"),
        ("OPEN", "AÇ"),
        ("Chat", "Mesajlaş"),
        ("Total", "Toplam"),
        ("items", "öğeler"),
        ("Selected", "Seçildi"),
        ("Screen Capture", "Ekran görüntüsü"),
        ("Input Control", "Giriş Kontrolü"),
        ("Audio Capture", "Ses Yakalama"),
        ("File Connection", "Dosya Bağlantısı"),
        ("Screen Connection", "Ekran Bağlantısı"),
        ("Do you accept?", "Kabul ediyor musun?"),
        ("Open System Setting", "Sistem Ayarını Aç"),
        ("How to get Android input permission?", "Android giriş izni nasıl alınır?"),
        ("android_input_permission_tip1", "Uzak bir cihazın Android cihazınızı fare veya dokunma yoluyla kontrol edebilmesi için, RustDesk'in \"Erişilebilirlik\" özelliğini kullanmasına izin vermelisiniz."),
        ("android_input_permission_tip2", "Sonraki sistem ayarları sayfasına gidin, [Yüklü Hizmetler]'i bulun ve erişin, [RustDesk Girişi] hizmetini etkinleştirin."),
        ("android_new_connection_tip", "Yeni bir kontrol talebi alındı, cihazınızı kontrol etmesine izin verilsin mi."),
        ("android_service_will_start_tip", "Ekran Yakalamanın etkinleştirilmesi, hizmeti otomatik olarak başlatacak ve diğer cihazların bu cihazdan bağlantı talep etmesine izin verecektir."),
        ("android_stop_service_tip", "Hizmetin kapatılması, kurulan tüm bağlantıları otomatik olarak kapatacaktır."),
        ("android_version_audio_tip", "Mevcut Android sürümü ses yakalamayı desteklemiyor, lütfen Android 10 veya sonraki bir sürüme yükseltin."),
        ("android_start_service_tip", "Ekran paylaşım hizmetini başlatmak için [Hizmeti Başlat] veya AÇ [Ekran Yakalama] iznine dokunun."),
        ("Account", "Hesap"),
        ("Overwrite", "üzerine yaz"),
        ("This file exists, skip or overwrite this file?", "Bu dosya var, bu dosya atlansın veya üzerine yazılsın mı?"),
        ("Quit", "Çıkış"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Yardım"),
        ("Failed", "Arızalı"),
        ("Succeeded", "başarılı"),
        ("Someone turns on privacy mode, exit", "Birisi gizlilik modunu açarsa, çık"),
        ("Unsupported", "desteklenmiyor"),
        ("Peer denied", "eş reddedildi"),
        ("Please install plugins", "Lütfen eklentileri yükleyin"),
        ("Peer exit", "eş çıkışı"),
        ("Failed to turn off", "kapatılamadı"),
        ("Turned off", "Kapatıldı"),
        ("In privacy mode", "Gizlilik modunda"),
        ("Out privacy mode", "Gizlilik modu dışında"),
        ("Language", "Dil"),
        ("Keep RustDesk background service", ""),
        ("Ignore Battery Optimizations", ""),
        ("android_open_battery_optimizations_tip", ""),
        ("Connection not allowed", "bağlantıya izin verilmedi"),
        ("Use temporary password", "Geçici şifre kullan"),
        ("Use permanent password", "Kalıcı şifre kullan"),
        ("Use both passwords", "İki şifreyide kullan"),
        ("Set permanent password", "Kalıcı şifre oluştur"),
        ("Set temporary password length", ""),
        ("Enable Remote Restart", "Uzaktan yeniden başlatmayı aktif et"),
        ("Allow remote restart", "Uzaktan yeniden başlatmaya izin ver"),
        ("Restart Remote Device", "Uzaktaki cihazı yeniden başlat"),
        ("Are you sure you want to restart", "Yeniden başlatmak istediğinize emin misin?"),
        ("Restarting Remote Device", "Uzaktan yeniden başlatılıyor"),
        ("remote_restarting_tip", ""),
    ].iter().cloned().collect();
}
