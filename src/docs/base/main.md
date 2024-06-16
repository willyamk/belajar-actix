### Dokumentasi Penggunaan Server

#### Deskripsi
Skrip ini berfungsi untuk mengkonfigurasi dan menginisialisasi server menggunakan Actix Web untuk menyediakan layanan web. Server ini diatur untuk mengizinkan CORS dari `http://localhost:3000` dan menyediakan layanan pengguna dengan operasi CRUD.

#### Langkah-langkah Penggunaan

1. **Persiapan**
   Pastikan Anda telah mengonfigurasi dan menyiapkan lingkungan pengembangan dengan dependensi yang diperlukan.

2. **Inisialisasi Logger**
   Logger lingkungan diinisialisasi menggunakan `env_logger::init()`. Ini akan mengaktifkan logging di lingkungan.

3. **Konfigurasi Server**
   Server diinisialisasi menggunakan `HttpServer::new()`. Di dalamnya, aplikasi dikonfigurasi menggunakan `App::new()` dengan CORS yang diizinkan dari `http://localhost:3000`.

4. **Konfigurasi Layanan Pengguna**
   Layanan pengguna diatur menggunakan fungsi `configure_services_users()` dari modul `user_main`. Ini menambahkan semua rute yang diperlukan untuk operasi CRUD pada data pengguna.

5. **Binding Server**
   Server kemudian diikat ke alamat `127.0.0.1:8000` menggunakan `server.bind()`. Jika berhasil, pesan informasi akan ditampilkan dan server akan dijalankan.

6. **Jalankan Server**
   Untuk menjalankan server, gunakan perintah `cargo watch -x run`.

#### Contoh Penggunaan
```rust
use actix_web::{App, HttpServer};
use actix_cors::Cors;
use std::io;
use log::{info, error};
use pages::users::main as user_main;

#[path = "./config/db_connection.rs"]
pub mod db_connection;

#[path = "./config/sql_operation.rs"]
pub mod sql_operation;

mod pages {
   pub mod users {
      pub mod main;
      pub mod user_struct;
      pub mod crud {
         pub mod insert_record;
         pub mod get_record;
         pub mod update_record;
         pub mod delete_record;
      }
   }
}

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
   env_logger::init();

   let server = HttpServer::new(|| {
      App::new()
              .wrap(
                 Cors::permissive()
                         .allowed_origin("http://localhost:3000")
              )
              .configure(user_main::configure_services_users)
   });

   match server.bind("127.0.0.1:8000") {
      Ok(server) => {
         info!("Server started at http://127.0.0.1:8000");
         server.run().await?;
      },
      Err(e) => {
         error!("Failed to bind address: {}", e);
         return Err(io::Error::new(io::ErrorKind::Other, e));
      }
   }

   Ok(())
}
```

#### Catatan
- Pastikan modul dan fungsi yang diperlukan telah diimpor dan diatur dengan benar sesuai dengan struktur proyek Anda.
- Sesuaikan pengaturan CORS dan alamat server sesuai dengan kebutuhan proyek Anda.
- Perhatikan bahwa fungsi `main()` ditandai dengan `#[actix_web::main]` untuk memungkinkan async runtime Actix Web berjalan.