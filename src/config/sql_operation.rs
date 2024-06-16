use tokio_postgres::{Client}; // Mengimpor Client dari crate tokio_postgres
use std::io; // Mengimpor modul io dari standar library

// Mendefinisikan struct SqlOperations untuk operasi SQL menggunakan client PostgreSQL
pub struct SqlOperations {
    client: Client, // Client PostgreSQL
}

#[allow(dead_code)] // Menyematkan atribut untuk mengabaikan kode yang tidak digunakan
impl SqlOperations {
    // Fungsi untuk membuat instance baru dari SqlOperations
    pub fn new(client: Client) -> Self {
        SqlOperations { client }
    }

    // Fungsi untuk mengeksekusi query SQL dan mengembalikan hasilnya
    async fn execute_query(&self, query: &str) -> Result<Vec<tokio_postgres::Row>, io::Error> {
        // Mempersiapkan statement SQL
        let statement = self.client.prepare(query).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Mengeksekusi query dan mendapatkan hasil dalam bentuk baris
        let rows = self.client.query(&statement, &[]).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(rows)
    }

    // Fungsi untuk mengeksekusi update SQL
    async fn execute_update(&self, query: &str) -> Result<(), io::Error> {
        // Mengeksekusi update query
        self.client.execute(query, &[]).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(())
    }

    // Fungsi publik untuk mendapatkan record dari tabel
    pub async fn get_record(
        &self,
        table_name: &str,
        fields: &str,
        where_clause: &str
    ) -> Result<Vec<tokio_postgres::Row>, io::Error> {
        // Membuat query SQL untuk mendapatkan record
        let query = format!("SELECT {} FROM {} {}", fields, table_name, where_clause);
        self.execute_query(&query).await
    }

    // Fungsi publik untuk memasukkan record ke dalam tabel
    pub async fn insert_record(
        &self,
        table_name: &str,
        fields: &str,
        values: &str
    ) -> Result<(), io::Error> {
        // Membuat query SQL untuk memasukkan record
        let query = format!("INSERT INTO {} ({}) VALUES ({})", table_name, fields, values);
        self.execute_update(&query).await
    }

    // Fungsi publik untuk memperbarui record di dalam tabel
    pub async fn update_record(
        &self,
        table_name: &str,
        values: &str,
        where_clause: &str
    ) -> Result<(), io::Error> {
        // Membuat query SQL untuk memperbarui record
        let query = format!("UPDATE {} SET {} WHERE {}", table_name, values, where_clause);
        self.execute_update(&query).await
    }

    // Fungsi publik untuk menghapus record dari tabel
    pub async fn delete_record(
        &self,
        table_name: &str,
        where_clause: &str
    ) -> Result<(), io::Error> {
        // Membuat query SQL untuk menghapus record
        let query = format!("DELETE FROM {} WHERE {}", table_name, where_clause);
        self.execute_update(&query).await
    }

    // Fungsi publik untuk mengosongkan tabel
    pub async fn truncate_table(
        &self,
        table_name: &str
    ) -> Result<(), io::Error> {
        // Membuat query SQL untuk mengosongkan tabel
        let query = format!("TRUNCATE TABLE {}", table_name);
        self.execute_update(&query).await
    }

    // Fungsi publik untuk menghitung jumlah record di dalam tabel
    pub async fn count_record(
        &self,
        table_name: &str,
        where_clause: &str
    ) -> Result<i64, io::Error> {
        // Membuat query SQL untuk menghitung jumlah record
        let query = format!("SELECT COUNT(*) FROM {} {}", table_name, where_clause);

        let rows = self.execute_query(&query).await?;

        // Mendapatkan hasil perhitungan dari baris pertama
        let count: i64 = rows[0].get(0);
        Ok(count)
    }

    // Fungsi publik untuk menghasilkan nomor otomatis berdasarkan prefix
    pub async fn auto_number(
        &self,
        table_name: &str,
        fields: &str,
        prefix: &str
    ) -> Result<String, io::Error> {
        // Membuat query SQL untuk mendapatkan nomor terakhir
        let query = format!(
            "SELECT MAX(SUBSTRING({}, POSITION('-' IN {}) + 1)::INTEGER) FROM {}",
            fields, fields, table_name
        );

        let rows = self.execute_query(&query).await?;

        // Mendapatkan nomor terakhir dari baris pertama
        let last_number: Option<i32> = rows[0].get(0);

        // Menghasilkan nomor berikutnya
        let next_number = last_number.unwrap_or(0) + 1;

        // Menggabungkan prefix dengan nomor berikutnya
        Ok(format!("{}{}", prefix, next_number))
    }
}
