## user_struct.rs

### UserData
Struktur `UserData` digunakan untuk menyimpan data pengguna. Struktur ini memiliki beberapa bidang yang bersifat opsional, termasuk ID pengguna, kode pengguna, nama pengguna, kata sandi pengguna, dan alamat email pengguna.

### Response<T>
Struktur `Response` digunakan untuk menyimpan respons dari server. Struktur ini memiliki empat bidang utama:
- `status`: Menyimpan status respons.
- `status_code`: Menyimpan kode status HTTP.
- `data`: Menyimpan data respons.
- `amount`: Menyimpan jumlah data dalam respons.

### ErrorResponse
Struktur `ErrorResponse` digunakan untuk menyimpan respons error dari server. Struktur ini memiliki beberapa bidang yang meliputi:
- `status`: Menyimpan status error.
- `status_code`: Menyimpan kode status HTTP.
- `error`: Menyimpan detail dari error.
- `request_id`: Menyimpan ID permintaan yang menyebabkan error.

### ErrorDetails
Struktur `ErrorDetails` digunakan untuk menyimpan detail dari error. Struktur ini memiliki beberapa bidang yang meliputi:
- `code`: Menyimpan kode error.
- `message`: Menyimpan pesan error.
- `details`: Menyimpan detail tambahan error.
- `path`: Menyimpan jalur request yang menyebabkan error.
- `suggestion`: Menyimpan saran untuk mengatasi error.

## Implementasi Fungsi

### ErrorResponse::new
Fungsi `new` pada struktur `ErrorResponse` digunakan untuk membuat instance baru dari `ErrorResponse`. Fungsi ini membutuhkan beberapa parameter, seperti kode status HTTP, kode error, pesan error, detail tambahan error, jalur request yang menyebabkan error, saran untuk mengatasi error, dan ID permintaan yang menyebabkan error.

## Penggunaan
Script ini dapat digunakan dalam aplikasi Rust untuk mengelola respons dari server dan menangani error dengan lebih efektif.

