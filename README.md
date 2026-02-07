# 🎬 MediaForge - Gelişmiş Medya Dönüştürücü

Modern, glassmorphic tasarımlı, FFmpeg destekli profesyonel video ve ses dönüştürücü.

## ✨ Özellikler

### 🎯 Temel Özellikler
- **Tüm Format Desteği**: MP4, AVI, MOV, MKV, WebM, FLV, WMV, MP3, WAV, FLAC, AAC, OGG ve daha fazlası
- **Drag & Drop**: Dosyaları sürükle bırak ile kolay yükleme
- **Hazır Profiller**: 6 farklı kullanım senaryosu için optimize edilmiş preset'ler
- **Gerçek Zamanlı İlerleme**: Dönüştürme sırasında anlık durum takibi

### 🛠️ Gelişmiş Ayarlar
- **Video Codec Seçimi**: H.264, H.265, VP9 ve daha fazlası
- **Audio Codec Seçimi**: AAC, MP3, Opus, FLAC
- **Özelleştirilebilir Bitrate**: Video ve ses için ayrı ayrı
- **Çözünürlük Kontrolü**: 4K'dan SD'ye kadar tüm seçenekler
- **FPS Ayarı**: 24fps ile 120fps arası
- **Kalite Kontrolü**: CRF değeri ile hassas ayar
- **Hız/Kalite Dengesi**: Ultra hızlı'dan en iyi kalite'ye 9 farklı preset

### 🎨 Öne Çıkan Tasarım
- **Glassmorphic UI**: Modern, şeffaf ve katmanlı tasarım
- **Retro-Futuristic**: Gelecekçi ama sıcak hissiyat
- **Animasyonlu Arayüz**: Smooth geçişler ve mikro-animasyonlar
- **Aurora Arkaplan**: Canlı, animasyonlu gradient efektler
- **Özel Fontlar**: Orbitron ve Space Mono ile profesyonel görünüm

## 🚀 Kurulum

### Ön Gereksinimler

1. **Node.js** (v18 veya üzeri)
   ```bash
   # Windows (Chocolatey ile)
   choco install nodejs

   # macOS (Homebrew ile)
   brew install node

   # Linux (Ubuntu/Debian)
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt-get install -y nodejs
   ```

2. **Rust** (v1.70 veya üzeri)
   ```bash
   # Tüm platformlar
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **FFmpeg** (En önemli gereksinim!)
   ```bash
   # Windows (Chocolatey ile)
   choco install ffmpeg

   # macOS (Homebrew ile)
   brew install ffmpeg

   # Linux (Ubuntu/Debian)
   sudo apt update
   sudo apt install ffmpeg
   ```

   FFmpeg'in doğru kurulduğunu test edin:
   ```bash
   ffmpeg -version
   ```

### Projeyi Çalıştırma

1. **Bağımlılıkları Yükle**
   ```bash
   cd mediaforge
   npm install
   ```

2. **Development Modunda Çalıştır**
   ```bash
   npm run tauri dev
   ```

3. **Production Build**
   ```bash
   npm run tauri build
   ```

## 📖 Kullanım

### Hızlı Dönüştürme

1. Ana ekranda dosya sürükle-bırak alanına medya dosyanızı ekleyin
2. "Hızlı Dönüştür" sekmesinde bir preset seçin:
   - **Yüksek Kalite Video**: En iyi görüntü kalitesi için
   - **Web Optimize**: Web için küçük dosya boyutu
   - **Mobil Uyumlu**: Mobil cihazlar için optimize
   - **Sadece Ses Çıkar**: Videodan MP3 oluştur
   - **Kayıpsız Audio**: FLAC formatında mükemmel ses
   - **4K Video**: Ultra HD dönüşüm

3. Hedef formatı seçin (Video veya Audio kategorisinden)
4. "Dönüştür" butonuna tıklayın
5. Kayıt konumunu seçin ve işlem başlasın!

### Gelişmiş Ayarlar

"Gelişmiş Ayarlar" sekmesinde tüm parametreleri manuel olarak kontrol edebilirsiniz:

**Video Ayarları:**
- Codec (H.264, H.265, VP9)
- Bitrate (örn: 5000k)
- Çözünürlük (4K, 2K, Full HD, HD, SD)
- FPS (24, 30, 60, 120)
- Kalite (CRF: 0-51, düşük = daha iyi)

**Audio Ayarları:**
- Codec (AAC, MP3, Opus, FLAC)
- Bitrate (örn: 320k)

**Performans:**
- Hız/Kalite Dengesi (ultrafast → veryslow)

## 🎨 Arayüz Detayları

### Renkler
- **Primary**: Cyan (#00f5ff) - Ana vurgu rengi
- **Secondary**: Magenta (#ff00ff) - İkincil vurgular
- **Accent**: Yellow (#ffed4e) - Önemli elementler
- **Dark**: Deep Space (#0a0e27) - Arkaplan

### Animasyonlar
- **Aurora Arkaplan**: 20 saniye döngüsü ile canlı gradient efekt
- **Hover Efektleri**: Tüm butonlarda smooth scale ve glow
- **Geçiş Animasyonları**: Fade, slide ve scale efektleri
- **İlerleme Çubuğu**: Shimmer efekti ile canlı görünüm

### Fontlar
- **Orbitron**: Başlıklar ve UI elementleri için
- **Space Mono**: İçerik ve form elementleri için

## 🔧 Teknik Detaylar

### Teknoloji Stack
- **Frontend**: React 18 + TypeScript
- **Backend**: Rust + Tauri
- **Build Tool**: Vite
- **Media Processing**: FFmpeg

### Desteklenen Formatlar

**Video:**
- MP4 (MPEG-4)
- AVI (Audio Video Interleave)
- MKV (Matroska)
- MOV (QuickTime)
- WebM (Web Media)
- FLV (Flash Video)
- WMV (Windows Media)
- M4V (iTunes Video)

**Audio:**
- MP3 (MPEG Audio Layer 3)
- WAV (Waveform Audio - Lossless)
- FLAC (Free Lossless Audio Codec)
- AAC (Advanced Audio Coding)
- OGG (Ogg Vorbis)
- WMA (Windows Media Audio)
- M4A (MPEG-4 Audio)
- OPUS (Modern Codec)

### FFmpeg Parametreleri

Uygulama şu FFmpeg parametrelerini kullanır:
- `-c:v`: Video codec
- `-c:a`: Audio codec
- `-b:v`: Video bitrate
- `-b:a`: Audio bitrate
- `-s`: Çözünürlük
- `-r`: FPS
- `-crf`: Kalite (Constant Rate Factor)
- `-preset`: Hız/kalite dengesi

## 🎯 Kullanım Senaryoları

### 1. YouTube İçin Optimize
- Preset: "Web Optimize"
- Format: MP4
- 1280x720, 30fps
- Video: 1500k, Audio: 128k

### 2. Sosyal Medya
- Preset: "Mobil Uyumlu"
- Format: MP4
- 854x480, 24fps
- Küçük dosya boyutu

### 3. Arşivleme
- Preset: "Yüksek Kalite Video"
- Format: MKV
- Orijinal çözünürlük
- CRF: 18 (çok yüksek kalite)

### 4. Podcast/Müzik
- Preset: "Kayıpsız Audio"
- Format: FLAC
- Tam kalite, sıkıştırmasız

## 🐛 Sorun Giderme

### FFmpeg Bulunamıyor
```bash
# PATH'e eklendiğinden emin olun
which ffmpeg  # macOS/Linux
where ffmpeg  # Windows
```

### Rust Derlenmiyor
```bash
# Rust'ı güncelleyin
rustup update

# Hedef platformu ekleyin
rustup target add x86_64-pc-windows-msvc  # Windows
```

### Node Modülleri Hatası
```bash
# node_modules'ü temizle ve yeniden yükle
rm -rf node_modules package-lock.json
npm install
```

## 📝 Lisans

Bu proje MIT lisansı altında lisanslanmıştır.

## 🙌 Katkıda Bulunma

Pull request'ler memnuniyetle karşılanır! Büyük değişiklikler için önce bir issue açarak neyi değiştirmek istediğinizi tartışın.

## 📞 İletişim

Sorularınız veya önerileriniz için issue açabilirsiniz.

---

**Not**: Bu uygulama FFmpeg'in gücünü modern bir arayüzle birleştirir. FFmpeg açık kaynaklı bir projedir ve [FFmpeg lisansına](https://ffmpeg.org/legal.html) tabidir.

🚀 **MediaForge ile medya dönüştürme artık çok daha kolay ve şık!**
