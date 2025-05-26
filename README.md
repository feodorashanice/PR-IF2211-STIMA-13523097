# 📌 TSP Solver (Rust)

Proyek ini merupakan implementasi penyelesaian Travelling Salesman Problem (TSP) menggunakan bahasa pemrograman Rust.

## Author

| Nama                    | NIM      |
|-------------------------|----------|
| Shanice Feodora Tjahjono | 13523097 |

## How to Compile and Run

### Di Linux/macOS (termasuk WSL)

1. Pastikan sudah menginstal Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Kompilasi file:
```bash
rustc src/tsp.rs -o tsp
```

3. Jalankan program:
```bash
./tsp test/test1.txt
```

### Di Windows (CMD atau PowerShell)

1. Unduh dan install Rust dari [https://rustup.rs](https://rustup.rs)
2. Kompilasi file:
```powershell
rustc src\tsp.rs -o tsp.exe
```

3. Jalankan program:
```powershell
tsp.exe test\test1.txt
```

## Struktur Folder

- `src/`: berisi kode sumber utama (`tsp.rs`)
- `test/`: berisi file uji (`test1.txt`, `test2.txt`, dst.)
- `test/output/`: menyimpan hasil visualisasi atau output program