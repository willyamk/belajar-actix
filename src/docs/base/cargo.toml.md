### Dokumentasi Penggunaan Proyek "belajar-actix"

#### Deskripsi
Proyek "belajar-actix" adalah proyek Rust yang menggunakan kerangka kerja Actix Web untuk membuat layanan web. Proyek ini memiliki dependensi yang diperlukan untuk berbagai fungsi seperti manipulasi CSV, koneksi database, enkripsi password, dan lainnya.

#### Konfigurasi Proyek

```toml
[package]
name = "belajar-actix"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-cors = "0.7.0"
actix-web = "4.7.0"
html-escape = "0.2.13"
serde = { version = "1.0.203", features = ["derive"] }
serde_json = "1.0.117"
uuid = { version = "1.8.0", features = [ "v4", "fast-rng", "macro-diagnostics"] }
tokio = { version = "1.38.0", features = ["full"] }
tokio-postgres = { version = "0.7.10", features = ["default"] }
csv = "1.3.0"
actix-multipart = "0.6.2"
log = "0.4.21"
env_logger = "0.11.3"
bcrypt = "0.15.1"
http = "0.2.12"
```

#### Dependencies

- **actix-cors**: Untuk mengizinkan CORS dalam aplikasi Actix Web.
- **actix-web**: Kerangka kerja web yang tangguh dan efisien untuk Rust.
- **html-escape**: Untuk menghindari injeksi HTML.
- **serde**: Untuk serialisasi dan deserialisasi data.
- **serde_json**: Untuk bekerja dengan data JSON.
- **uuid**: Untuk menghasilkan UUID.
- **tokio**: Runtime async untuk Rust.
- **tokio-postgres**: Koneksi database PostgreSQL untuk tokio.
- **csv**: Untuk membaca dan menulis data CSV.
- **actix-multipart**: Untuk menangani data multipart pada Actix Web.
- **log**: Untuk logging dalam aplikasi.
- **env_logger**: Logger berbasis env untuk Rust.
- **bcrypt**: Untuk mengenkripsi dan memverifikasi password dengan bcrypt.
- **http**: Tipe data HTTP untuk manipulasi dan pembuatan HTTP.

#### Catatan Penggunaan
- Pastikan Anda telah menginstal Rust dan Cargo di sistem Anda sebelum menggunakan proyek ini.
- Pastikan untuk menambahkan dependensi dan konfigurasi lain yang dibutuhkan sesuai dengan kebutuhan proyek Anda.
- Proyek ini menggunakan Actix Web versi 4.7.0, pastikan Anda memahami perbedaan versi jika Anda mengubah versi Actix Web yang digunakan.
- Anda dapat menambahkan dependency baru ke proyek menggunakan perintah cargo add.