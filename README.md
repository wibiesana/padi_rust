<div align="center">

# 🌾 Padi Rust Api Framework (Template)

**The Industrial-Grade, Zero-Bloat Rust Engine for the Modern Web**

[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GitHub](https://img.shields.io/badge/GitHub-wibiesana%2Fpadi__rust-blue?style=flat-square&logo=github)](https://github.com/wibiesana/padi_rust)

</div>

Padi Rust Api Framework is a lightweight, zero-bloat code-generation framework built on top of Axum, Tokio, and SQLx.

---

## 🚀 Quick Start

### 1. Setup Environment
Salin file `.env.example` ke `.env`:
```bash
cp .env.example .env
```

### 2. Jalankan Migrasi Database
```bash
padi migrate
# atau: cargo run -- migrate
```

### 3. Jalankan Server API
```bash
padi serve
# atau: cargo run
```
Server akan berjalan di `http://127.0.0.1:8080`.

---

## 🛠️ CLI Console Commands

Gunakan `padi` (Windows: `padi.bat`, Linux/macOS: `./padi`, atau `cargo run -- <command>`):

```bash
# Tampilkan bantuan
padi

# Generate CRUD lengkap untuk 1 tabel (Model, Resource, Controller, Routes)
padi g products

# Generate CRUD untuk SELURUH tabel di database sekaligus
padi ga

# Buat komponen individual
padi make:controller Product
padi make:model products
padi make:resource Product
padi make:migration create_products_table

# Migrasi Database
padi migrate
padi migrate:rollback
padi migrate:status

# Background Queue Worker
padi queue:work
```

---

## 🔄 Update Framework Core (`padi_core`)

Jika ada pembaruan pada library framework core (`padi_core`), perbarui dependensi dengan perintah:
```bash
cargo update -p padi_core
```

---

## 📄 License

Licensed under the [MIT License](LICENSE).
