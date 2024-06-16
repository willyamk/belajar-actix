### Dokumentasi Penggunaan Endpoint GET `/paginate/users`

#### Deskripsi
Endpoint ini digunakan untuk mendapatkan daftar pengguna dengan paginasi.

#### Request
- **Metode HTTP:** GET
- **URL:** `http://localhost:8000/paginate/users`
- **Query Parameters:**
    - `page`: Nomor halaman yang ingin ditampilkan.

#### Respon

Jika data pengguna ditemukan pada halaman yang diminta, respon akan berupa daftar pengguna yang dipaginasi dalam format JSON:
```json
{
    "current_page": 1,
    "total_pages": 1,
    "user_data": [
        {
            "id": "fd13de18-d6bc-4ab0-a226-4b9819511b17",
            "user_code": "USR-1",
            "username": "data_1",
            "password": "$2b$12$n31egTGt80uh1Z6TMPSYouVg5bScEOx7bCeqTNA62P4M5arHc122u",
            "email": "test_1@gmail.com"
        },
        {
            "id": "fe3ca230-a678-48ac-879b-f74d04fa7a4a",
            "user_code": "USR-2",
            "username": "data_2",
            "password": "$2b$12$XDyylKYWKGDs7dn/XwrXM.nkYG322lT3ugl4kCOywcsNrwRtbwPZm",
            "email": "test_2@gmail.com"
        },
        {
            "id": "e64adbd3-2c83-4a1c-838f-9cf1ad1257b0",
            "user_code": "USR-3",
            "username": "data_3",
            "password": "$2b$12$TqaxavlEgZ06JYPcG1hiV.Ua4N2UH8LWjFhxxSqkq/fwqzL8LRPou",
            "email": "test_3@gmail.com"
        },
        {
            "id": "3e0fe548-af90-4b43-8c1e-22501588ddd5",
            "user_code": "USR-4",
            "username": "data_4",
            "password": "$2b$12$3JIrYA/iMLpiK0rnfGUXGOMuTlDhbsIX1ubmPZOHGCwv/ZJb0SMKy",
            "email": "test_4@gmail.com"
        },
        {
            "id": "e9274a98-7053-4c46-845d-92526799a189",
            "user_code": "USR-5",
            "username": "data_5",
            "password": "$2b$12$.1AaQXLthl64knEHfw5KxeLi8wyZAWO7qnFHstBTsWd1dx30cGsTC",
            "email": "test_5@gmail.com"
        }
    ]
}
```

Jika halaman yang diminta tidak memiliki data pengguna, respon akan berupa pesan error dalam format JSON:
```json
{
    "status": "Error",
    "status_code": 404,
    "error": {
        "code": "DATA_NOT_FOUND",
        "message": "Data Not Found",
        "path": "/paginate/users?page=2"
    },
    "request": "GET"
}
```

#### Contoh Penggunaan (Postman)
1. Buka Postman.
2. Pilih metode `GET`.
3. Masukkan URL `http://localhost:8000/paginate/users`.
4. Pilih tab `Params`.
5. Tambahkan parameter dengan key `page` dan value berupa nomor halaman yang ingin ditampilkan.
6. Klik tombol "Send".

#### Catatan
- Pastikan server Actix-web berjalan pada `http://localhost:8000`.
- Pastikan database terhubung dengan benar untuk mengakses data pengguna.
- Jumlah item per halaman dapat disesuaikan di dalam kode dengan mengubah nilai `page_size`.