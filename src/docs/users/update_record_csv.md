### Dokumentasi Penggunaan Endpoint POST `/edit/csv/users`

#### Deskripsi
Endpoint ini digunakan untuk mengedit data pengguna dari file CSV. File CSV harus berisi kolom `user_code` untuk mengidentifikasi data yang akan diperbarui.

#### Request
- **Metode HTTP:** POST
- **URL:** `http://localhost:8000/edit/csv/users`
- **Body:**
    - Pilih format `binary`.
    - Unggah file CSV yang berisi data pengguna yang akan diperbarui. Pastikan file CSV memiliki format yang sesuai dengan contoh di bawah.

#### Contoh Data CSV
```
user_code,username,password,email
USR-1,edit_password_1@example.com,edit_password_1,edit_password_1
USR-2,edit_password_2@example.com,edit_password_2,edit_password_2
USR-3,edit_password_3@example.com,edit_password_3,edit_password_3
```

#### Respon

Jika pembaruan data pengguna berhasil, respon akan berupa pesan sukses dalam format JSON:
```json
{
    "status": "success",
    "status_code": 400,
    "data": {
        "data_index_1": "USR-1 data updated successfully",
        "data_index_2": "USR-2 data updated successfully",
        "data_index_3": "USR-3 data updated successfully"
    },
    "amount": 3
}
```

Jika kolom `user_code` tidak ada dalam file CSV, respon akan berupa pesan error:
```json
{
    "status": "error",
    "status_code": 400,
    "error": {
        "code": "MISSING_USER_CODE_COLUMN",
        "message": {},
        "details": "CSV file must contain 'user_code' column",
        "path": "/edit/csv/users",
        "suggestion": "Add 'user_code' column in csv file"
    },
    "request_id": {}
}
```

#### Contoh Penggunaan (Postman)
1. Buka Postman.
2. Pilih metode `POST`.
3. Masukkan URL `http://localhost:8000/edit/csv/users`.
4. Pilih tab `Body`.
5. Pilih format `binary`.
6. Unggah file CSV yang berisi data pengguna yang akan diperbarui.
7. Klik tombol "Send".

#### Catatan
- Pastikan server Actix-web berjalan pada `http://localhost:8000`.
- Pastikan file CSV sesuai dengan format yang diberikan.
- Pastikan file CSV mengandung kolom `user_code` untuk mengidentifikasi data yang akan diperbarui.