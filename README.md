<h1 align="center">nyvo</h1>

<p align="center">A lightweight, fast and secure GUI client for PostgreSQL</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/backend-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="Windows" src="https://img.shields.io/badge/platform-Windows-0078D6?style=flat-square&logo=windows&logoColor=white">
  <img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square">
<img alt="Status" src="https://img.shields.io/badge/status-early%20alpha-d69e2e?style=flat-square">
</p>


---

## Features

- Fast · Lightweight: nyvo weighs `~15 MB` and consumes minimal RAM
- Security: Credentials and connection strings are managed entirely within compiled Rust code, keeping your data safe.
- Dynamic Table Rendering: Automatically generates clean virtual tables for any custom `SELECT` query results.
- Persistent Connection Pool: Powered by Rust's async runtime and deadpool, keeping database connections alive without needing to reconnect on every query.

---

## Made with

<p align="center">
  <img src="https://skillicons.dev/icons?i=rust,postgresql&theme=dark" alt="core skills" />
</p>

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) stable toolchain
- [PostgreSQL](https://www.postgresql.org/download/) server for testing

### Run from source

```bash
git clone https://github.com/gemushka-dev/nyvo.git
cd nyvo

cargo run --release
```

## Status

Nyvo is in early alpha. The UI, database layer and configuration format may change
between releases.
