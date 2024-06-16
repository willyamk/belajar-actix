### Dokumentasi Penggunaan Endpoint GET `/user/{id}`

#### Deskripsi
Endpoint ini digunakan untuk mendapatkan informasi pengguna berdasarkan ID atau user_code.

#### Request
- **Metode HTTP:** GET
- **URL:** `http://localhost:8000/user/{id}`
    - `{id}`: ID atau user_code yang ingin ditemukan.

#### Respon

Jika data pengguna ditemukan, respon akan berupa informasi pengguna dalam format JSON:
```json
{
    "user_data": [
        {
            "id": "3e4b7a56-1536-4232-8709-6c8f27095c89",
            "user_code": "USR-2",
            "username": "data_2",
            "password": "$2b$12$7Zv//xurXaa7WHzjLXrfLOe4c/FzTci53puu6YBOQhhRrkgfGROLi",
            "email": "test_2@gmail.com"
        }
    ]
}
```

Jika data pengguna tidak ditemukan, respon akan berupa pesan error dalam format JSON:
```json
{
    "status": "Error",
    "status_code": 404,
    "error": {
        "code": "DATA_NOT_FOUND",
        "message": "Data Not Found",
        "path": "/user/c9cfb351-06f1-4244-a6c0-4a509569b671"
    },
    "request": "GET"
}
```

#### Contoh Penggunaan (Postman)
1. Buka Postman.
2. Pilih metode `GET`.
3. Masukkan URL `http://localhost:8000/user/{id}`.
4. Ganti `{id}` dengan id atau user_code yang ingin dicari.
5. Klik tombol "Send".

#### Catatan
- Pastikan server Actix-web berjalan pada `http://localhost:8000`.
- Pastikan database terhubung dengan benar untuk mengakses data pengguna.
- Pesan error akan dikirim dengan kode status HTTP 404 jika data pengguna tidak ditemukan, dan dengan kode status HTTP 500 jika terjadi kesalahan dalam menghubungkan ke database atau melakukan pencarian data.