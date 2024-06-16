### Dokumentasi Penggunaan Endpoint DELETE `/clear/users`

#### Deskripsi
Endpoint ini digunakan untuk menghapus semua data pengguna dari database.

#### Request
- **Metode HTTP:** DELETE
- **URL:** `http://localhost:8000/clear/users`

#### Respon

Jika penghapusan semua data pengguna berhasil, respon akan berupa pesan sukses dalam format JSON yang mencantumkan data pengguna yang dihapus:
```json
{
    "status": "success",
    "status_code": 200,
    "data": {
        "data_index_1": {
            "id": "45c7ebb2-5a54-4a71-a7d7-3b90f629741b",
            "user_code": "USR-4",
            "username": "data_4",
            "password": "$2b$12$Q75vN24TteC3YYGvgdsbpuO3KVi53w1mNaKmCSoPUVU9ZZoAyiXz.",
            "email": "test_4@gmail.com"
        },
        "data_index_2": {
            "id": "b121d115-c092-4b03-839b-6bd23a244e8b",
            "user_code": "USR-5",
            "username": "data_5",
            "password": "$2b$12$3bHV6xFxw9ouo7jxD705KeCMcuCwlfZ2XdoYOzmcnuBykb1jmcB1S",
            "email": "test_5@gmail.com"
        },
        "data_index_3": {
            "id": "ffb05c6b-705f-439d-b8a5-90f3b1ab0fe9",
            "user_code": "USR-2",
            "username": "edit_password_2@example.com",
            "password": "$2b$12$aLMgYZ6.V7tfwewP78DyKeMaLVX8PK7Y4PDM2fO3usliaGeYnTkfm",
            "email": "edit_password_2"
        },
        "data_index_4": {
            "id": "870f72ed-2539-4b68-aeba-83d22915fb24",
            "user_code": "USR-3",
            "username": "edit_password_3@example.com",
            "password": "$2b$12$UI/a5.dFIRbq6tWGkKxIee9K55DFoSB/R0pkvLR7AaPyByVyvh2ce",
            "email": "edit_password_3"
        }
    },
    "amount": 4
}
```

#### Contoh Penggunaan (Postman)
1. Buka Postman.
2. Pilih metode `DELETE`.
3. Masukkan URL `http://localhost:8000/clear/users`.
4. Klik tombol "Send".

#### Catatan
- Pastikan server Actix-web berjalan pada `http://localhost:8000`.
- Semua data pengguna akan dihapus dari database.