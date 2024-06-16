use actix_web::{get, HttpResponse, web}; // Mengimpor makro get, HttpResponse, dan web dari crate actix_web
use serde::{Serialize, Deserialize}; // Mengimpor trait Serialize dan Deserialize dari crate serde

use crate::{db_connection, sql_operation}; // Mengimpor modul db_connection dan sql_operation dari crate lokal
use crate::pages::users::user_struct::UserData; // Mengimpor struct UserData dari modul users_struct di crate lokal

// Struktur PaginationParams untuk menyimpan parameter paginasi dari query
#[derive(Deserialize)]
struct PaginationParams {
    page: Option<u64>, // Nomor halaman (opsional)
}

// Struktur PaginatedUserData untuk menyimpan data pengguna yang dipaginasi
#[derive(Serialize)]
struct PaginatedUserData {
    current_page: u64, // Nomor halaman saat ini
    total_pages: u64, // Jumlah total halaman
    user_data: Vec<UserData>, // Data pengguna
}

// Struktur ErrorResponseGetUser untuk menyimpan respons error dari endpoint GET user
#[derive(Serialize, Deserialize)]
struct ErrorResponseGetUser {
    status: String, // Status error
    status_code: u16, // Kode status HTTP
    error: ErrorDetailGetUser, // Detail error
    request: String, // Jenis permintaan yang menyebabkan error
}

// Struktur ErrorDetailGetUser untuk menyimpan detail dari error
#[derive(Serialize, Deserialize)]
struct ErrorDetailGetUser {
    code: String, // Kode error
    message: String, // Pesan error
    path: String, // Jalur request yang menyebabkan error
}

// Fungsi async untuk menghasilkan respons error
async fn generate_error_response(error_code: &str, message: &str, path: &str, status_code: u16) -> HttpResponse {
    let error_response = ErrorResponseGetUser {
        status: "Error".to_string(),
        status_code,
        error: ErrorDetailGetUser {
            code: error_code.to_string(),
            message: message.to_string(),
            path: path.to_string(),
        },
        request: "GET".to_string(),
    };
    HttpResponse::build(http::StatusCode::from_u16(status_code).unwrap())
        .json(error_response)
}

// Fungsi async untuk mengambil data pengguna berdasarkan filter dan path
async fn fetch_user_data(filter: &str, path: &str) -> Result<HttpResponse, HttpResponse> {
    // Membuat koneksi dengan database
    let client = match db_connection::DbConnection::create_default_connection().await {
        Ok(client) => client,
        Err(_) => return Ok(generate_error_response(
            "DB_CONNECTION_ERROR",
            "Error connecting to database", path, 500).await),
    };
    let sql_operation = sql_operation::SqlOperations::new(client);

    // Mencari data pengguna berdasarkan filter
    let search_user_data = sql_operation.get_record(
        "tb_users",
        "*",
        &format!("{} ORDER BY CAST(SUBSTRING(user_code, 5) AS INTEGER) ASC", filter)
    ).await;

    match search_user_data {
        Ok(rows) => {
            if rows.is_empty() {
                return Err(generate_error_response(
                    "DATA_NOT_FOUND",
                    "Data Not Found",
                    path,
                    404
                ).await);
            }

            let data: Vec<UserData> = rows
                .iter()
                .map(|row| UserData {
                    id: row.get("id"),
                    user_code: row.get("user_code"),
                    username: row.get("username"),
                    password: row.get("password"),
                    email: row.get("email"),
                })
                .collect();

            let data_json = serde_json::json!({ "user_data": data });

            Ok(HttpResponse::Ok().content_type("application/json").body(data_json.to_string()))
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Ok(generate_error_response(
                "SEARCH_ERROR",
                "Failed to search data",
                path,
                500
            ).await)
        }
    }
}

// Handler untuk endpoint GET "/users" untuk mendapatkan semua pengguna
#[get("/users")]
pub async fn get_all_users() -> HttpResponse {
    fetch_user_data(
        "WHERE id = '' OR user_code ILIKE '%'", "/users"
    ).await.unwrap_or_else(|error_response| error_response)
}

// Handler untuk endpoint GET "/user/{id}" untuk mendapatkan pengguna berdasarkan ID
#[get("/user/{id}")]
pub async fn get_user_by_id(path: web::Path<String>) -> HttpResponse {
    let user_id = &path.into_inner();
    let path = format!("/user/{}", user_id);

    fetch_user_data(
        &format!("WHERE id = '{}' OR user_code ILIKE '%{}%'", user_id, user_id), &path)
        .await.unwrap_or_else(|error_response| error_response)
}

// Handler untuk endpoint GET "/paginate/users" untuk mendapatkan pengguna dengan paginasi
#[get("/paginate/users")]
pub async fn paginate_users(query_params: web::Query<PaginationParams>) -> HttpResponse {
    let page_size = 10;
    let current_page = query_params.page.unwrap_or(1);
    let offset = (current_page - 1) * page_size;

    let path = "/paginate/users";
    let client = match db_connection::DbConnection::create_default_connection().await {
        Ok(client) => client,
        Err(_) => return generate_error_response(
            "DB_CONNECTION_ERROR",
            "Error connecting to database", path, 500).await,
    };
    let sql_operation = sql_operation::SqlOperations::new(client);

    let search_user_data = sql_operation.get_record(
        "tb_users",
        "*",
        &format!("ORDER BY CAST(SUBSTRING(user_code, 5) AS INTEGER) ASC LIMIT {} OFFSET {}", page_size, offset),
    ).await;

    match search_user_data {
        Ok(rows) => {
            let total_count = sql_operation.count_record("tb_users", "").await.unwrap_or(0);
            let total_pages = (total_count as f64 / page_size as f64).ceil() as u64;

            if rows.is_empty() && current_page > 1 {
                let error_path = format!("{}?page={}", path, current_page);
                return generate_error_response(
                    "DATA_NOT_FOUND",
                    "Data Not Found",
                    &error_path,
                    404
                ).await;
            }

            let data: Vec<UserData> = rows
                .iter()
                .map(|row| UserData {
                    id: row.get("id"),
                    user_code: row.get("user_code"),
                    username: row.get("username"),
                    password: row.get("password"),
                    email: row.get("email"),
                })
                .collect();

            let response = PaginatedUserData {
                current_page,
                total_pages,
                user_data: data,
            };

            HttpResponse::Ok().content_type("application/json").json(response)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            generate_error_response(
                "SEARCH_ERROR",
                "Failed to search data",
                path,
                500
            ).await
        }
    }
}