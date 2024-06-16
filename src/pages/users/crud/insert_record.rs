use std::collections::HashMap; // Mengimpor tipe data HashMap dari standar library
use actix_web::{post, web, HttpResponse}; // Mengimpor makro post, web, dan HttpResponse dari crate actix_web
use bcrypt::{hash, DEFAULT_COST}; // Mengimpor fungsi hash dan konstanta DEFAULT_COST dari crate bcrypt
use csv::ReaderBuilder; // Mengimpor ReaderBuilder dari crate csv
use uuid::Uuid; // Mengimpor Uuid dari crate uuid
use html_escape::encode_text; // Mengimpor fungsi encode_text dari crate html_escape

use crate::{db_connection, sql_operation}; // Mengimpor modul db_connection dan sql_operation dari crate lokal
use crate::pages::users::user_struct::{UserData, Response, ErrorResponse}; // Mengimpor struct UserData, Response, dan ErrorResponse dari modul users_struct di crate lokal

// Handler untuk endpoint POST "/add/users" untuk menambahkan data pengguna
#[post("/add/users")]
pub async fn add_users_data(users_data: web::Json<Vec<UserData>>) -> HttpResponse {
    let (error_response, success_responses, amount) = process_users_data(users_data.into_inner()).await;
    if let Some(error_response) = error_response {
        return HttpResponse::BadRequest().json(error_response);
    }
    HttpResponse::Ok().json(Response {
        status: "success".to_string(),
        status_code: 200,
        data: success_responses,
        amount,
    })
}

// Handler untuk endpoint POST "/add/csv/users" untuk menambahkan data pengguna dari file CSV
#[post("/add/csv/users")]
pub async fn add_csv_users_data(csv_data: web::Bytes) -> HttpResponse {
    let csv_data = csv_data.to_vec();
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_slice());
    let mut records: Vec<UserData> = Vec::new();

    for result in reader.deserialize() {
        match result {
            Ok(record) => {
                let user_data: UserData = record;
                records.push(user_data);
            },
            Err(e) => {
                eprintln!("Error parsing CSV record: {:?}", e);
                return HttpResponse::BadRequest().body("Error parsing CSV data");
            }
        }
    }

    let (error_response, success_responses, amount) = process_users_data(records).await;
    if let Some(error_response) = error_response {
        return HttpResponse::BadRequest().json(error_response);
    }
    HttpResponse::Ok().json(Response {
        status: "success".to_string(),
        status_code: 200,
        data: success_responses,
        amount,
    })
}

// Fungsi async untuk memproses data pengguna
async fn process_users_data(users_data: Vec<UserData>) -> (Option<ErrorResponse>, HashMap<String, UserData>, usize) {
    let mut error_messages: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut request_ids: HashMap<String, String> = HashMap::new();
    let mut has_error = false;
    let mut success_responses: HashMap<String, UserData> = HashMap::new();
    let mut amount = 0;

    for (index, user_data) in users_data.iter().enumerate() {
        let mut field_errors: HashMap<String, String> = HashMap::new();
        if user_data.username.is_none() || user_data.username.as_ref().unwrap().is_empty() {
            field_errors.insert("username".to_string(), "Username cannot be empty".to_string());
            has_error = true;
        }
        if user_data.password.is_none() || user_data.password.as_ref().unwrap().is_empty() {
            field_errors.insert("password".to_string(), "Password cannot be empty".to_string());
            has_error = true;
        }
        if user_data.email.is_none() || user_data.email.as_ref().unwrap().is_empty() {
            field_errors.insert("email".to_string(), "Email cannot be empty".to_string());
            has_error = true;
        }
        if !field_errors.is_empty() {
            let data_key = format!("data_index_{}", index + 1);
            error_messages.insert(encode_text(&data_key).to_string(), field_errors); // Melindungi dari serangan XSS
            request_ids.insert(encode_text(&data_key).to_string(), Uuid::new_v4().to_string()); // Melindungi dari serangan XSS
        }
    }

    if has_error {
        let error_response = ErrorResponse::new(
            400,
            &encode_text("VALIDATION_ERROR").to_string(), // Melindungi dari serangan XSS
            error_messages,
            "",
            "/add/users",
            "Fill all required fields",
            request_ids,
        );
        return (Some(error_response), success_responses, amount);
    }

    let client = match db_connection::DbConnection::create_default_connection().await {
        Ok(client) => client,
        Err(_) => return (Some(ErrorResponse::new(
            500,
            "DB_CONNECTION_ERROR",
            HashMap::new(),
            "",
            "",
            "",
            HashMap::new())
        ), success_responses, amount),
    };
    let sql_operation = sql_operation::SqlOperations::new(client);

    for (index, user_data) in users_data.iter().enumerate() {
        let id = Uuid::new_v4().to_string();

        let hashed_password = match &user_data.password {
            Some(password) => match hash(password, DEFAULT_COST) {
                Ok(hashed) => Some(hashed),
                Err(_) => return (Some(ErrorResponse::new(500, "HASHING_ERROR", HashMap::new(), "", "", "", HashMap::new())), success_responses, amount),
            },
            None => None,
        };

        let kode = match sql_operation.auto_number("tb_users", "user_code", "USR-").await {
            Ok(auto_number) => auto_number,
            Err(_) => return (Some(ErrorResponse::new(500, "AUTO_NUMBER_ERROR", HashMap::new(), "", "", "", HashMap::new())), success_responses, amount),
        };

        match sql_operation.insert_record(
            "tb_users",
            "id, user_code, username, password, email",
            &format!(
                "'{}', '{}', '{}', '{}', '{}'",
                id, kode, user_data.username.as_ref().unwrap_or(&"".to_string()), hashed_password.as_ref().unwrap_or(&"".to_string()), user_data.email.as_ref().unwrap_or(&"".to_string())
            ),
        ).await {
            Ok(_) => {
                amount += 1;
                let data_key = format!("data_index_{}", index + 1);
                success_responses.insert(encode_text(&data_key).to_string(), UserData { // Melindungi dari serangan XSS
                    id: Some(id.clone()),
                    user_code: Some(kode),
                    username: user_data.username.clone(),
                    password: user_data.password.clone(),
                    email: user_data.email.clone(),
                });
            },
            Err(_) => return (Some(ErrorResponse::new(500, "INSERT_ERROR", HashMap::new(), "", "", "", HashMap::new())), success_responses, amount),
        }
    }

    (None, success_responses, amount)
}