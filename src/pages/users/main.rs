use actix_web::{web}; // Mengimpor modul web dari crate actix_web

use crate::pages::users::crud::*; // Mengimpor semua fungsi CRUD dari modul crud di dalam modul users pada modul pages

// Fungsi untuk mengonfigurasi layanan pengguna dengan menambahkan rute-rute yang diperlukan
pub fn configure_services_users(cfg: &mut web::ServiceConfig) {
    // Menambahkan rute untuk menambahkan data pengguna
    cfg.service(insert_record::add_users_data);
    // Menambahkan rute untuk menambahkan data pengguna dari file CSV
    cfg.service(insert_record::add_csv_users_data);
    // Menambahkan rute untuk mendapatkan semua data pengguna
    cfg.service(get_record::get_all_users);
    // Menambahkan rute untuk mendapatkan data pengguna berdasarkan ID
    cfg.service(get_record::get_user_by_id);
    // Menambahkan rute untuk mem-paginate data pengguna
    cfg.service(get_record::paginate_users);
    // Menambahkan rute untuk memperbarui data pengguna dari file CSV
    cfg.service(update_record::update_users_data_from_csv);
    // Menambahkan rute untuk memperbarui data pengguna
    cfg.service(update_record::update_user_data);
    // Menambahkan rute untuk menghapus data pengguna berdasarkan ID
    cfg.service(delete_record::delete_user_data);
    // Menambahkan rute untuk menghapus semua data pengguna
    cfg.service(delete_record::clear_users_data);
}