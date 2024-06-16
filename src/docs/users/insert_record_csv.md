### Dokumentasi Penggunaan Endpoint POST `/add/csv/users`

#### Deskripsi
Endpoint ini digunakan untuk menambahkan data pengguna dari file CSV ke dalam database.

#### Request
- **Metode HTTP:** POST
- **URL:** `http://localhost:8000/add/csv/users`
- **Body:**
    - Pilih format `binary`.
    - Unggah file CSV dengan struktur data pengguna sebagai berikut:

```
username,email,password
data_1,test_1@gmail.com,123456
data_2,test_2@gmail.com,123456
data_3,test_3@gmail.com,123456
data_4,test_4@gmail.com,123456
data_5,test_5@gmail.com,123456
```

#### Respon

Jika data pengguna berhasil ditambahkan, respon akan berupa pesan sukses dalam format JSON:
```json
{
    "status": "success",
    "status_code": 200,
    "data": {
        "data_index_1": {
            "id": "38504291-4362-4fd8-9a86-4b655c837a5d",
            "user_code": "USR-1",
            "username": "data_1",
            "password": "123456",
            "email": "test_1@gmail.com"
        },
        "data_index_2": {
            "id": "ffb05c6b-705f-439d-b8a5-90f3b1ab0fe9",
            "user_code": "USR-2",
            "username": "data_2",
            "password": "123456",
            "email": "test_2@gmail.com"
        },
        "data_index_3": {
            "id": "870f72ed-2539-4b68-aeba-83d22915fb24",
            "user_code": "USR-3",
            "username": "data_3",
            "password": "123456",
            "email": "test_3@gmail.com"
        },
        "data_index_4": {
            "id": "45c7ebb2-5a54-4a71-a7d7-3b90f629741b",
            "user_code": "USR-4",
            "username": "data_4",
            "password": "123456",
            "email": "test_4@gmail.com"
        },
        "data_index_5": {
            "id": "b121d115-c092-4b03-839b-6bd23a244e8b",
            "user_code": "USR-5",
            "username": "data_5",
            "password": "123456",
            "email": "test_5@gmail.com"
        }
    },
    "amount": 5
}
```

Jika terdapat kesalahan dalam parsing data CSV, respon akan berupa pesan error:
```
Error parsing CSV data
```

#### Contoh Penggunaan (Postman)
1. Buka Postman.
2. Pilih metode `POST`.
3. Masukkan URL `http://localhost:8000/add/csv/users`.
4. Pilih tab `Body`.
5. Pilih format `binary`.
6. Unggah file CSV dengan struktur data pengguna seperti contoh di atas.
7. Klik tombol "Send".

#### Catatan
- Pastikan server Actix-web berjalan pada `http://localhost:8000`.
- Pastikan struktur data CSV sesuai dengan contoh yang diberikan.