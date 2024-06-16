### Dokumentasi Penggunaan Endpoint PUT `/edit/user/{id}`

#### Deskripsi
Endpoint ini digunakan untuk memperbarui data pengguna berdasarkan ID.

#### Request
- **Metode HTTP:** PUT
- **URL:** `http://localhost:8000/edit/user/{id}`
    - `{id}`: ID unik pengguna yang akan diperbarui.
- **Body:**
    - Pilih format `row`.
    - Masukkan data pengguna yang akan diperbarui dalam format JSON, seperti contoh berikut:

```json
{
    "username": "updates_username_data_1",
    "email": "updates_email_data_1@example.com",
    "password": "123_Updated_pa$sw@rd_456"
}
```

#### Respon

Jika pembaruan data pengguna berhasil, respon akan berupa pesan sukses dalam format JSON:
```json
{
    "status": "success",
    "status_code": 200,
    "data": {
        "id": "38504291-4362-4fd8-9a86-4b655c837a5d",
        "user_code": "USR-1",
        "username": "updates_username_data_1",
        "password": "$2b$12$Gpp1S/2K6r/.kteb3mx6cesRQv8kae3GhUsap9UV7y4RSICZEzVy.",
        "email": "updates_email_data_1@example.com"
    },
    "amount": 1
}
```

Jika ada data yang tidak lengkap, respon akan berupa pesan error:
```json
{
    "status": "error",
    "status_code": 400,
    "error": {
        "code": "INVALID_INPUT",
        "message": {
            "password": {
                "additional_detail": "Password is required"
            },
            "email": {
                "additional_detail": "Email is required"
            },
            "username": {
                "additional_detail": "Username is required"
            }
        },
        "details": "Validation error",
        "path": "/edit/user/38504291-4362-4fd8-9a86-4b655c837a5d",
        "suggestion": "Please provide all required fields"
    },
    "request_id": {
        "id": "38504291-4362-4fd8-9a86-4b655c837a5d"
    }
}
```

Jika ID pengguna tidak ditemukan, respon akan berupa pesan error:
```json
{
    "status": "error",
    "status_code": 404,
    "error": {
        "code": "USER_NOT_FOUND",
        "message": {},
        "details": "User not found",
        "path": "/edit/user/2a2c0ad4-f63d-4812-9e3b-23ce7e776d1d",
        "suggestion": "User with the provided ID does not exist"
    },
    "request_id": {
        "id": "2a2c0ad4-f63d-4812-9e3b-23ce7e776d1d"
    }
}
```

#### Contoh Penggunaan (Postman)
1. Buka Postman.
2. Pilih metode `PUT`.
3. Masukkan URL `http://localhost:8000/edit/user/{id}`.
4. Ganti `{id}` dengan ID pengguna yang ingin diperbarui.
5. Pilih tab `Body`.
6. Pilih format `row`.
7. Masukkan data pengguna yang ingin diperbarui dalam format JSON.
8. Klik tombol "Send".

#### Catatan
- Pastikan server Actix-web berjalan pada `http://localhost:8000`.
- Pastikan struktur data JSON sesuai dengan contoh yang diberikan.
- Pastikan untuk memberikan ID pengguna yang valid saat melakukan pembaruan.