### Dokumentasi Penggunaan Endpoint GET `/users`

#### Deskripsi
Endpoint ini digunakan untuk mendapatkan daftar semua pengguna yang tersimpan dalam database.

#### Request
- **Metode HTTP:** GET
- **URL:** `http://localhost:8000/users`

#### Respon

Jika data pengguna ditemukan, respon akan berupa daftar pengguna dalam format JSON:
```json
{
    "user_data": [
        {
            "id": "ae060c38-2429-41f1-bc40-3d0722eb0636",
            "user_code": "USR-1",
            "username": "data_1",
            "password": "$2b$12$C7WuGLzYj3yL1NOzSIabrOyGbIOMCs3dBZrcAdq9x4h3Jk448mhey",
            "email": "test_1@gmail.com"
        },
        {
            "id": "c9cfb351-06f1-4244-a6c0-4a509569b671",
            "user_code": "USR-2",
            "username": "data_2",
            "password": "$2b$12$qVBRczI8dGa93rB.PfLkQOGM.r2.umuUbosfZxjPGXd0du7E2Ensy",
            "email": "test_2@gmail.com"
        },
        {
            "id": "795d8d4c-5d81-406d-9c9d-bce516aff17a",
            "user_code": "USR-3",
            "username": "data_3",
            "password": "$2b$12$fN/JiwYMsvdkHOI2ZlQcZOixGcPs3gzoI6pfXRTtc/s4z8lQ0ZFiC",
            "email": "test_3@gmail.com"
        },
        {
            "id": "64578447-cc13-4705-bbc7-54d776bc9133",
            "user_code": "USR-4",
            "username": "data_4",
            "password": "$2b$12$hKBY4MrBtgIscKlk7gTK/OfgtIVlHip54lkheIfYH64lB3OWBXQli",
            "email": "test_4@gmail.com"
        },
        {
            "id": "60893dea-a33a-428a-93c3-a5abd2eb787d",
            "user_code": "USR-5",
            "username": "data_5",
            "password": "$2b$12$ZOpiFruxZSMPKzIx10VTK.Mngb7Ik/pxzW3Qb/N08U3RTmJ8jW3N2",
            "email": "test_5@gmail.com"
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
        "path": "/users"
    },
    "request": "GET"
}
```

#### Contoh Penggunaan (Postman)
1. Buka Postman.
2. Pilih metode `GET`.
3. Masukkan URL `http://localhost:8000/users`.
4. Klik tombol "Send".

#### Catatan
- Pastikan server Actix-web berjalan pada `http://localhost:8000`.
- Pastikan database terhubung dengan benar untuk mengakses data pengguna.
- Pesan error akan dikirim dengan kode status HTTP 404 jika data pengguna tidak ditemukan, dan dengan kode status HTTP 500 jika terjadi kesalahan dalam menghubungkan ke database atau melakukan pencarian data.