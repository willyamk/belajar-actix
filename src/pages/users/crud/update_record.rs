use std::collections::HashMap; // Mengimpor tipe data HashMap dari standar library
use actix_web::{put, post, web, HttpResponse}; // Mengimpor makro put, post, web, dan HttpResponse dari crate actix_web
use bcrypt::{hash, DEFAULT_COST}; // Mengimpor fungsi hash dan konstanta DEFAULT_COST dari crate bcrypt
use uuid::Uuid; // Mengimpor tipe data Uuid dari crate uuid

use crate::{db_connection, sql_operation}; // Mengimpor modul db_connection dan sql_operation dari crate lokal
use crate::pages::users::user_struct::{UserData, Response, ErrorResponse}; // Mengimpor struct UserData, Response, dan ErrorResponse dari modul user_struct di crate lokal

// Handler untuk endpoint POST "/edit/csv/users" untuk mengedit data pengguna dari file CSV
#[post("/edit/csv/users")]
pub async fn update_users_data_from_csv(csv_data: web::Bytes) -> HttpResponse {
    let csv_data = csv_data.to_vec(); // Mengkonversi Bytes menjadi vektor u8

    let mut rdr = csv::Reader::from_reader(csv_data.as_slice()); // Membuat reader CSV dari data CSV

    // Memastikan kolom user_code ada dalam header CSV
    match rdr.headers() {
        Ok(headers) => {
            if !headers.iter().any(|h| h == "user_code") {
                return HttpResponse::BadRequest().json(ErrorResponse::new( // Mengembalikan respons BadRequest dengan pesan error
                   400,
                   "MISSING_USER_CODE_COLUMN",
                   HashMap::new(),
                   "CSV file must contain 'user_code' column",
                   "/edit/csv/users",
                   "Add 'user_code' column in csv file",
                   generate_request_ids(&vec![]),
                ));
            }
        }
        Err(e) => {
            eprintln!("Error reading CSV headers: {:?}", e); // Cetak pesan error
            return HttpResponse::BadRequest().json(ErrorResponse::new( // Mengembalikan respons BadRequest dengan pesan error
               400,
               "CSV_HEADER_ERROR",
               HashMap::new(),
               "",
               "",
               "",
               HashMap::new(),
            ));
        }
    };

    let mut records: Vec<UserData> = Vec::new(); // Membuat vektor untuk menyimpan data pengguna dari CSV
    for result in rdr.deserialize() { // Iterasi melalui setiap baris CSV
        let record: Result<UserData, csv::Error> = result; // Mengubah hasil deserialisasi menjadi Result
        match record { // Penanganan error atau sukses
            Ok(record) => records.push(record), // Jika sukses, tambahkan data pengguna ke vektor
            Err(e) => { // Jika terjadi error
                eprintln!("Error parsing CSV record: {:?}", e); // Cetak pesan error
                return HttpResponse::BadRequest().json(ErrorResponse::new( // Mengembalikan respons BadRequest dengan pesan error
                   400,
                   "CSV_PARSE_ERROR",
                   HashMap::new(),
                   "",
                   "",
                   "",
                   HashMap::new(),
                ));
            }
        }
    }

    // Memproses pembaruan data pengguna
    let (error_response, success_message, amount) = process_update_users_data(records).await;
    if let Some(error_response) = error_response { // Jika terdapat respons error
        return HttpResponse::BadRequest().json(error_response); // Kembalikan respons BadRequest dengan respons error
    }

    // Jika terdapat pesan sukses, kembalikan respons OK dengan pesan sukses
    if !success_message.is_empty() {
        let success_response = Response {
            status: "success".to_string(),
            status_code: 200,
            data: success_message,
            amount,
        };
        return HttpResponse::Ok().json(success_response);
    }

    // Jika tidak ada data yang diperbarui, kembalikan respons OK dengan pesan
    HttpResponse::Ok().body("No data updated")
}

fn generate_request_ids(records: &Vec<UserData>) -> HashMap<String, String> {
    let mut request_ids = HashMap::new();
    for (index, _) in records.iter().enumerate() {
        request_ids.insert(format!("id_index_{}", index + 1), format!("USR-{}", index + 1));
    }
    request_ids
}

// Fungsi async untuk memproses pembaruan data pengguna dari CSV
async fn process_update_users_data(records: Vec<UserData>) -> (Option<ErrorResponse>, HashMap<String, String>, usize) {
    let client = match db_connection::DbConnection::create_default_connection().await { // Membuat koneksi ke database
        Ok(client) => client, // Jika sukses, mendapatkan koneksi
        Err(_) => return (Some(ErrorResponse::new( // Jika gagal, mengembalikan respons error
           500,
           "DB_CONNECTION_ERROR",
           HashMap::new(),
           "",
           "",
           "",
           HashMap::new(),
        )), HashMap::new(), 0),
    };
    let sql_operation = sql_operation::SqlOperations::new(client); // Membuat operasi SQL

    let mut success_message: HashMap<String, String> = HashMap::new(); // Inisialisasi HashMap untuk pesan sukses
    let mut request_ids: HashMap<String, String> = HashMap::new(); // Inisialisasi HashMap untuk ID request
    let mut updated_count = 0; // Inisialisasi jumlah data yang diperbarui

    for (index, user_data) in records.iter().enumerate() { // Iterasi melalui setiap data pengguna
        let username = user_data.username.as_ref().map_or_else(|| "".to_string(), |x| x.clone()); // Mendapatkan username
        let email = user_data.email.as_ref().map_or_else(|| "".to_string(), |x| x.clone()); // Mendapatkan email

        let hashed_password = match &user_data.password { // Menghasilkan hash password jika ada
            Some(password) => match hash(password, DEFAULT_COST) {
                Ok(hashed) => Some(hashed),
                Err(_) => {
                    eprintln!("Error hashing password"); // Cetak pesan error jika gagal menghasilkan hash
                    return (Some(ErrorResponse::new( // Kembalikan respons error jika gagal
                         500,
                         "HASHING_ERROR",
                         HashMap::new(),
                         "",
                         "",
                         "",
                         HashMap::new(),
                    )), success_message, updated_count);
                }
            },
            None => Some(String::new()), // Jika tidak ada password, gunakan string kosong
        };

        // Memperbarui data pengguna di database
        match sql_operation.update_record(
            "tb_users",
            &format!(
                "username = '{}', password = '{}', email = '{}'",
                username,
                hashed_password.unwrap_or_else(|| "".to_string()),
                email
            ),
            &format!(
                "user_code = '{}'",
                user_data.user_code.as_ref().map_or_else(|| "".to_string(), |x| x.clone())
            ),
        ).await {
            Ok(_) => { // Jika sukses
                let data_key = format!("data_index_{}", index + 1); // Mendapatkan kunci data
                success_message.insert( // Masukkan pesan sukses ke HashMap
                                        data_key.clone(),
                                        format!("{} data updated successfully", user_data.user_code.as_ref().map_or_else(|| "".to_string(), |x| x.clone())),
                );
                request_ids.insert(data_key, Uuid::new_v4().to_string()); // Masukkan ID request ke HashMap
                updated_count += 1; // Tambahkan jumlah data yang diperbarui
            }
            Err(_) => return (Some(ErrorResponse::new( // Jika gagal, kembalikan respons error
               500,
               "UPDATE_ERROR",
               HashMap::new(),
               "",
               "",
               "",
               HashMap::new(),
            )), success_message, updated_count),
        }
    }

    (None, success_message, updated_count) // Kembalikan tuple tanpa respons error
}

// Handler untuk endpoint PUT "/edit/user/{id}" untuk memperbarui data pengguna berdasarkan ID
#[put("/edit/user/{id}")]
async fn update_user_data(
    user_data: web::Json<UserData>, // Data pengguna yang diterima sebagai JSON
    path: web::Path<String>, // ID pengguna yang diterima sebagai bagian dari path
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner(); // Mendapatkan ID pengguna dari path
    let mut validation_errors = HashMap::new(); // Inisialisasi HashMap untuk error validasi

    // Memvalidasi data pengguna
    if user_data.username.as_ref().map_or(true, |v| v.trim().is_empty()) {
        let mut message_inner = HashMap::new();
        message_inner.insert("additional_detail".to_string(), "Username is required".to_string());
        validation_errors.insert("username".to_string(), message_inner.clone());
    }

    if user_data.password.as_ref().map_or(true, |v| v.trim().is_empty()) {
        let mut message_inner = HashMap::new();
        message_inner.insert("additional_detail".to_string(), "Password is required".to_string());
        validation_errors.insert("password".to_string(), message_inner.clone());
    }

    if user_data.email.as_ref().map_or(true, |v| v.trim().is_empty()) {
        let mut message_inner = HashMap::new();
        message_inner.insert("additional_detail".to_string(), "Email is required".to_string());
        validation_errors.insert("email".to_string(), message_inner.clone());
    }

    let message = HashMap::new(); // Inisialisasi HashMap untuk pesan

    let mut request_id = HashMap::new(); // Inisialisasi HashMap untuk ID request
    request_id.insert("id".to_string(), user_id.clone()); // Masukkan ID pengguna ke HashMap

    // Jika terdapat error validasi, kembalikan respons BadRequest dengan respons error
    if !validation_errors.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse::new(
            400,
            "INVALID_INPUT",
            validation_errors,
            "Validation error",
            &format!("/edit/user/{}", user_id),
            "Please provide all required fields",
            request_id,
        )));
    }

    let username = user_data.username.clone().unwrap(); // Mendapatkan username
    let hashed_password = match &user_data.password { // Menghasilkan hash password jika ada
        Some(password) => match hash(password, DEFAULT_COST) {
            Ok(hashed) => Some(hashed),
            Err(_) => {
                eprintln!("Error hashing password"); // Cetak pesan error jika gagal menghasilkan hash
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse::new( // Kembalikan respons error jika gagal
                   500,
                   "HASHING_ERROR",
                   message,
                   "Failed to hash password",
                   &format!("/edit/user/{}", user_id),
                   "Try again later",
                   request_id,
                )));
            }
        },
        None => Some(String::new()), // Jika tidak ada password, gunakan string kosong
    };
    let email = user_data.email.clone().unwrap(); // Mendapatkan email

    let client = match db_connection::DbConnection::create_default_connection().await { // Membuat koneksi ke database
        Ok(client) => client, // Jika sukses, mendapatkan koneksi
        Err(_) => { // Jika gagal, kembalikan respons error
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse::new(
                500,
                "DB_CONNECTION_ERROR",
                message,
                "Failed to connect to database",
                &format!("/edit/user/{}", user_id),
                "Try again later",
                request_id,
            )));
        }
    };

    let sql_operation = sql_operation::SqlOperations::new(client); // Membuat operasi SQL

    let user_exists = match sql_operation // Memeriksa apakah pengguna ada dalam database
        .get_record("tb_users", "*", &format!("WHERE id = '{}'", user_id))
        .await
    {
        Ok(records) => {
            if records.is_empty() { // Jika pengguna tidak ditemukan, kembalikan respons NotFound
                return Ok(HttpResponse::NotFound().json(ErrorResponse::new(
                    404,
                    "USER_NOT_FOUND",
                    message,
                    "User not found",
                    &format!("/edit/user/{}", user_id),
                    "User with the provided ID does not exist",
                    request_id,
                )));
            } else {
                let db_user_data = UserData { // Jika ditemukan, ambil data pengguna dari database
                    id: Some(user_id.clone()),
                    user_code: records[0].get("user_code"),
                    username: records[0].get("username"),
                    password: records[0].get("password"),
                    email: records[0].get("email"),
                };

                db_user_data // Kembalikan data pengguna dari database
            }
        }
        Err(_) => { // Jika terjadi error saat mengambil data pengguna dari database
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse::new( // Kembalikan respons error
               500,
               "DB_OPERATION_ERROR",
               message,
               "Failed to fetch user data",
               &format!("/edit/user/{}", user_id),
               "Try again later",
               request_id,
            )));
        }
    };

    if !user_exists.id.is_some() { // Jika pengguna tidak ditemukan, kembalikan respons NotFound
        let mut request_id = HashMap::new();
        request_id.insert("id".to_string(), user_id.clone());

        return Ok(HttpResponse::NotFound().json(ErrorResponse::new(
            404,
            "USER_NOT_FOUND",
            message,
            "User not found",
            &format!("/edit/user/{}", user_id),
            "User with the provided ID does not exist",
            request_id,
        )));
    }

    let update_result = sql_operation // Memperbarui data pengguna di database
        .update_record(
            "tb_users",
            &format!(
                "username = '{}', password = '{}', email = '{}'",
                username,
                hashed_password.as_ref().unwrap(),
                email
            ),
            &format!("id = '{}'", user_id),
        )
        .await;

    match update_result { // Menangani hasil pembaruan data pengguna
        Ok(_) => Ok(HttpResponse::Ok().json(Response { // Jika sukses, kembalikan respons OK dengan data pengguna yang diperbarui
            status: "success".to_string(),
            status_code: 200,
            data: UserData {
                id: Some(user_id),
                user_code: user_exists.user_code,
                username: Some(username),
                password: hashed_password.clone(),
                email: Some(email),
            },
            amount: 1,
        })),
        Err(e) => { // Jika terjadi error saat memperbarui data pengguna
            println!("Error: {:?}", e); // Cetak pesan error

            Ok(HttpResponse::InternalServerError().json(ErrorResponse::new( // Kembalikan respons error
                500,
                "UPDATE_ERROR",
                message,
                "Failed to update user data",
                &format!("/edit/user/{}", user_id),
                "Try again later",
                request_id,
            )))
        }
    }
}
