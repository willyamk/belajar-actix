### Dokumentasi Penggunaan Endpoint DELETE `/delete/user/{id}`

#### Deskripsi
Endpoint ini digunakan untuk menghapus data pengguna berdasarkan ID.

#### Request
- **Metode HTTP:** DELETE
- **URL:** `http://localhost:8000/delete/user/{id}`
    - Ganti `{id}` dengan ID pengguna yang ingin dihapus.

#### Respon

Jika penghapusan data pengguna berhasil, respon akan berupa pesan sukses dalam format JSON:
```json
{
    "status": "success",
    "status_code": 200,
    "data": {
        "id": "38504291-4362-4fd8-9a86-4b655c837a5d",
        "user_code": "USR-1",
        "username": "edit_password_1@example.com",
        "password": "$2b$12$DrJZZHyYIDU/TNWpYQ4VROpxq4/epuT.J6u41oUU3Vpf2qZxvt3NW",
        "email": "edit_password_1"
    },
    "amount": 1
}
```

Jika ID pengguna tidak valid atau terjadi kesalahan dalam operasi database, respon akan berupa pesan error:
```json
{
    "status": "error",
    "status_code": 500,
    "error": {
        "code": "DB_OPERATION_ERROR",
        "message": {
            "data": {
                "password": "Password not found",
                "user_code": "User code not found",
                "username": "Username not found",
                "email": "Email not found"
            }
        },
        "details": "Error",
        "path": "Please try again later",
        "suggestion": "/delete/user/38504291-4362-4fd8-9a86-4b655c837a5d"
    },
    "request_id": {
        "id": "38504291-4362-4fd8-9a86-4b655c837a5d"
    }
}
```

#### Contoh Penggunaan (Postman)
1. Buka Postman.
2. Pilih metode `DELETE`.
3. Masukkan URL `http://localhost:8000/delete/user/{id}`.
4. Ganti `{id}` dengan ID pengguna yang ingin dihapus.
5. Klik tombol "Send".

#### Catatan
- Pastikan server Actix-web berjalan pada `http://localhost:8000`.
- Pastikan ID pengguna yang dihapus valid.
- Pastikan data pengguna yang dihapus memiliki ID yang sesuai dengan URL.