### Dokumentasi Penggunaan Endpoint POST `/add/users`

#### Deskripsi
Endpoint ini digunakan untuk menambahkan data pengguna ke dalam database.

#### Request
- **Metode HTTP:** POST
- **URL:** `http://localhost:8000/add/users`
- **Body:**
    - Pilih format `JSON`
    - Masukkan data pengguna dalam bentuk array seperti contoh berikut:

```json
[
    {
        "username": "add_data_1",
        "email": "add_data_1@example.com",
        "password": "add_data_1_password"
    },
    {
        "username": "add_data_2",
        "email": "add_data_2@example.com",
        "password": "add_data_2_password"
    }
]
```

#### Respon

Jika data pengguna berhasil ditambahkan, respon akan berupa pesan sukses dalam format JSON:
```json
{
    "status": "success",
    "status_code": 200,
    "data": {
        "data_index_1": {
            "id": "a2c234e8-8920-461c-8ad0-7db20a43d7e0",
            "user_code": "USR-6",
            "username": "add_data_1",
            "password": "add_data_1_password",
            "email": "add_data_1@example.com"
        },
        "data_index_2": {
            "id": "184464b3-d2c0-4f50-86b8-71d5f40dc318",
            "user_code": "USR-7",
            "username": "add_data_2",
            "password": "add_data_2_password",
            "email": "add_data_2@example.com"
        }
    },
    "amount": 2
}
```

Jika terdapat field yang tidak diisi, respon akan berupa pesan error dalam format JSON:
```json
{
    "status": "error",
    "status_code": 400,
    "error": {
        "code": "VALIDATION_ERROR",
        "message": {
            "data_index_1": {
                "username": "Username cannot be empty"
            },
            "data_index_2": {
                "email": "Email cannot be empty",
                "password": "Password cannot be empty"
            }
        },
        "details": "",
        "path": "/add/users",
        "suggestion": "Fill all required fields"
    },
    "request_id": {
        "data_index_1": "fc08aab9-499a-49c6-ba6e-de67f4f3c9b4",
        "data_index_2": "cd230cbc-3201-46d1-9867-eae44622e457"
    }
}
```

#### Contoh Penggunaan (Postman)
1. Buka Postman.
2. Pilih metode `POST`.
3. Masukkan URL `http://localhost:8000/add/users`.
4. Pilih tab `Body`.
5. Pilih format `JSON`.
6. Masukkan data pengguna dalam bentuk array seperti contoh di atas.
7. Klik tombol "Send".

#### Catatan
- Pastikan server Actix-web berjalan pada `http://localhost:8000`.
- Pastikan struktur data pengguna sesuai dengan contoh yang diberikan.