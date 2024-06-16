use std::collections::HashMap; // Mengimpor HashMap dari standar library
use actix_web::{delete, HttpResponse, web}; // Mengimpor delete, HttpResponse, dan web dari crate actix_web

use crate::{db_connection, sql_operation}; // Mengimpor modul db_connection dan sql_operation dari modul induk
use crate::pages::users::user_struct::{ErrorResponse, Response, UserData}; // Mengimpor struct ErrorResponse, Response, dan UserData dari modul user_struct

// Fungsi handler untuk menghapus data pengguna berdasarkan ID
#[delete("/delete/user/{id}")]
async fn delete_user_data(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner(); // Mendapatkan ID pengguna dari path

    // Membuat koneksi database default
    let client = match db_connection::DbConnection::create_default_connection().await {
        Ok(client) => client,
        Err(_) => return generate_db_connection_error_response(&user_id).await,
    };

    let sql_operation = sql_operation::SqlOperations::new(client); // Membuat instance SqlOperations

    // Mengambil data pengguna dari database berdasarkan ID
    let user_record = match sql_operation.get_record(
        "tb_users",
        "*",
        &format!("WHERE id = '{}'", user_id)
    ).await {
        Ok(records) => {
            if let Some(row) = records.first() {
                // Jika data ditemukan, membentuk struct UserData
                let id = row.get("id");
                let user_code = row.get("user_code");
                let username = row.get("username");
                let password = row.get("password");
                let email = row.get("email");

                Some(UserData {
                    id,
                    user_code,
                    username,
                    password,
                    email,
                })
            } else {
                // Jika data tidak ditemukan, menghasilkan respons error
                return generate_error_response(
                    "DB_OPERATION_ERROR",
                    "Failed to retrieve user data",
                    &user_id,
                    500,
                ).await;
            }
        },
        Err(_) => return generate_db_operation_error_response(
            &user_id,
            "Failed to retrieve user data"
        ).await,
    };

    // Jika data pengguna ditemukan, menghapus data dari database
    if let Some(user_data) = user_record {
        let delete_result = sql_operation.delete_record(
            "tb_users",
            &format!("id = '{}'", user_id)
        ).await;

        // Jika penghapusan berhasil, menghasilkan respons sukses
        if delete_result.is_ok() {
            return HttpResponse::Ok().json(Response {
                status: "success".to_string(),
                status_code: 200,
                data: user_data,
                amount: 1,
            });
        }
    }

    // Jika terjadi kesalahan dalam penghapusan data, menghasilkan respons error server
    HttpResponse::InternalServerError().body("Error deleting user")
}

// Fungsi handler untuk menghapus semua data pengguna
#[delete("/clear/users")]
async fn clear_users_data() -> HttpResponse {
    // Membuat koneksi database default
    let client = match db_connection::DbConnection::create_default_connection().await {
        Ok(client) => client,
        Err(_) => return generate_db_connection_error_response("").await,
    };

    let sql_operation = sql_operation::SqlOperations::new(client); // Membuat instance SqlOperations

    // Mengambil semua data pengguna dari database
    match sql_operation.get_record("tb_users", "*", "").await {
        Ok(data) => {
            let amount = data.len(); // Mendapatkan jumlah data pengguna

            // Menghapus semua data pengguna dari database
            match sql_operation.truncate_table("tb_users").await {
                Ok(_) => {
                    // Jika penghapusan berhasil, membentuk respons sukses dengan data pengguna yang dihapus
                    let response_data: HashMap<String, UserData> = data.into_iter()
                        .enumerate()
                        .map(|(i, row)| {
                            let user_data = UserData {
                                id: row.get("id"),
                                user_code: row.get("user_code"),
                                username: row.get("username"),
                                password: row.get("password"),
                                email: row.get("email"),
                            };
                            (format!("data_index_{}", i + 1), user_data)
                        })
                        .collect();

                    let response = Response {
                        status: "success".to_string(),
                        status_code: 200,
                        data: response_data,
                        amount,
                    };

                    HttpResponse::Ok().json(response)
                },
                Err(_) => generate_db_operation_error_response(
                    "USR-*",
                    "Failed to clear user data"
                ).await,
            }
        },
        Err(_) => generate_db_operation_error_response(
            "USR-*",
            "Failed to retrieve user data"
        ).await,
    }
}

// Fungsi untuk menghasilkan respons error koneksi database
async fn generate_db_connection_error_response(user_id: &str) -> HttpResponse {
    generate_error_response(
        "DB_CONNECTION_ERROR",
        "Error connecting to database",
        user_id,
        500,
    ).await
}

// Fungsi untuk menghasilkan respons error operasi database
async fn generate_db_operation_error_response(user_id: &str, error_message: &str) -> HttpResponse {
    generate_error_response(
        "DB_OPERATION_ERROR",
        error_message,
        user_id,
        500,
    ).await
}

// Fungsi untuk menghasilkan respons error dengan format yang ditentukan
async fn generate_error_response(error_code: &str, error_message: &str, user_id: &str, status_code: u16) -> HttpResponse {
    let mut error_data = HashMap::new();
    error_data.insert("user_code".to_string(), "User code not found".to_string());
    error_data.insert("username".to_string(), "Username not found".to_string());
    error_data.insert("password".to_string(), "Password not found".to_string());
    error_data.insert("email".to_string(), "Email not found".to_string());

    let mut message = HashMap::new();
    message.insert("errors".to_string(), error_message.to_string());

    let mut message_container = HashMap::new();
    message_container.insert("data".to_string(), error_data);

    let mut request_id = HashMap::new();
    request_id.insert("id".to_string(), user_id.to_string());

    HttpResponse::build(http::StatusCode::from_u16(status_code).unwrap())
        .json(ErrorResponse::new(
            status_code,
            error_code,
            message_container,
            "Error",
            "Please try again later",
            &format!("/delete/user/{}", user_id),
            request_id,
        ))
}