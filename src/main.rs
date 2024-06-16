use actix_web::{App, HttpServer}; // Mengimpor komponen App dan HttpServer dari crate actix_web
use actix_cors::Cors; // Mengimpor komponen Cors dari crate actix_cors
use std::io; // Mengimpor modul io dari standar library
use log::{info, error}; // Mengimpor makro info dan error dari crate log
use pages::users::main as user_main; // Mengimpor modul main dari pages::users dan mengaliasinya sebagai user_main

// Mengimpor modul db_connection dari file "./config/db_connection.rs"
#[path = "./config/db_connection.rs"]
pub mod db_connection;

// Mengimpor modul sql_operation dari file "./config/sql_operation.rs"
#[path = "./config/sql_operation.rs"]
pub mod sql_operation;

// Mendefinisikan modul pages yang memiliki submodul users
mod pages {
    // Mendefinisikan modul users
    pub mod users {
        // Mendefinisikan submodul main
        pub mod main;
        // Mendefinisikan submodul user_struct
        pub mod user_struct;

        // Mendefinisikan submodul crud yang memiliki beberapa submodul untuk operasi CRUD
        pub mod crud {
            // Submodul untuk operasi insert_record
            pub mod insert_record;
            // Submodul untuk operasi get_record
            pub mod get_record;
            // Submodul untuk operasi update_record
            pub mod update_record;
            // Submodul untuk operasi delete_record
            pub mod delete_record;
        }
    }
}

// Fungsi utama yang akan dijalankan ketika program dieksekusi
#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    // Inisialisasi logger lingkungan
    env_logger::init();

    // Membuat instance HttpServer baru
    let server = HttpServer::new(|| {
        // Mengonfigurasi aplikasi dengan CORS dan layanan pengguna
        App::new()
            .wrap(
                // Mengonfigurasi CORS dengan pengaturan yang permisif
                Cors::permissive()
                    .allowed_origin("http://localhost:3000") // Mengizinkan asal "http://localhost:3000"
            )
            .configure(user_main::configure_services_users) // Mengonfigurasi layanan pengguna
    });

    // Mencoba untuk mengikat server ke alamat "127.0.0.1:8000"
    match server.bind("127.0.0.1:8000") {
        Ok(server) => {
            // Jika berhasil, menampilkan pesan informasi dan menjalankan server
            info!("Server started at http://127.0.0.1:8000");
            server.run().await?; // Menjalankan server secara asynchronous
        },
        Err(e) => {
            // Jika gagal, menampilkan pesan error dan mengembalikan error
            error!("Failed to bind address: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    }

    Ok(()) // Mengembalikan nilai Ok jika tidak ada error
}
