use serde::{Deserialize, Serialize}; // Mengimpor trait Deserialize dan Serialize dari crate serde
use std::collections::HashMap; // Mengimpor tipe data HashMap dari standar library

// Struktur UserData untuk menyimpan data pengguna
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserData {
    pub id: Option<String>, // Opsional ID pengguna
    pub user_code: Option<String>, // Opsional kode pengguna
    pub username: Option<String>, // Opsional nama pengguna
    pub password: Option<String>, // Opsional kata sandi pengguna
    pub email: Option<String>, // Opsional alamat email pengguna
}

// Struktur Response untuk menyimpan respons dari server
#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub status: String, // Status respons
    pub status_code: u16, // Kode status HTTP
    pub data: T, // Data respons
    pub amount: usize, // Jumlah data dalam respons
}

// Struktur ErrorResponse untuk menyimpan respons error dari server
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub status: String, // Status error
    pub status_code: u16, // Kode status HTTP
    pub error: ErrorDetails, // Detail error
    pub request_id: HashMap<String, String>, // ID permintaan yang menyebabkan error
}

// Struktur ErrorDetails untuk menyimpan detail dari error
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorDetails {
    pub code: String, // Kode error
    pub message: HashMap<String, HashMap<String, String>>, // Pesan error
    pub details: String, // Detail tambahan error
    pub path: String, // Jalur request yang menyebabkan error
    pub suggestion: String, // Saran untuk mengatasi error
}

// Implementasi untuk struktur ErrorResponse
impl ErrorResponse {
    // Fungsi pembuat baru untuk ErrorResponse
    pub(crate) fn new(status_code: u16, code: &str, message: HashMap<String, HashMap<String, String>>, details: &str, path: &str, suggestion: &str, request_id: HashMap<String, String>) -> Self {
        ErrorResponse {
            status: "error".to_string(), // Mengatur status error
            status_code, // Mengatur kode status HTTP
            error: ErrorDetails { // Mengatur detail error
                code: code.to_string(), // Mengatur kode error
                message, // Mengatur pesan error
                details: details.to_string(), // Mengatur detail tambahan error
                path: path.to_string(), // Mengatur jalur request yang menyebabkan error
                suggestion: suggestion.to_string(), // Mengatur saran untuk mengatasi error
            },
            request_id, // Mengatur ID permintaan yang menyebabkan error
        }
    }
}
