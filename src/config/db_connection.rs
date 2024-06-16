use tokio_postgres::{ Client, NoTls }; // Mengimpor Client dan NoTls dari crate tokio_postgres
use std::{fmt, io}; // Mengimpor modul fmt dan io dari standar library

// Mendefinisikan struct DbConnection untuk menyimpan konfigurasi koneksi database
#[derive(Debug)]
pub struct DbConnection {
    pub username: String, // Username untuk koneksi database
    pub password: String, // Password untuk koneksi database
    pub host: String, // Host untuk koneksi database
    pub port: u16, // Port untuk koneksi database
    pub database: String, // Nama database
}

// Implementasi metode untuk struct DbConnection
impl DbConnection {
    // Metode untuk membuat koneksi database
    pub async fn create_connection(&self) -> Result<Client, io::Error> {
        // Mengubah konfigurasi koneksi menjadi URL koneksi database
        let database_url = self.to_string();

        // Mencoba untuk membuat koneksi ke database
        let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // Menjalankan koneksi secara asynchronous
        tokio::spawn(connection);

        // Mengembalikan objek client jika berhasil
        Ok(client)
    }

    // Metode untuk membuat koneksi database dengan konfigurasi default
    pub async fn create_default_connection() -> Result<Client, io::Error> {
        // Mendefinisikan konfigurasi default untuk koneksi database
        let default_config = DbConnection {
            username: "postgres".to_string(),
            password: "24062002".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database: "db_belajar_actix".to_string(),
        };

        // Membuat koneksi dengan menggunakan konfigurasi default
        default_config.create_connection().await
    }
}

// Implementasi trait fmt::Display untuk struct DbConnection
impl fmt::Display for DbConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Menghasilkan string URL koneksi database berdasarkan konfigurasi
        write!(
            f,
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}
