# Bisa Dihitung - Tauri + React

Aplikasi desktop dan web yang dibangun dengan Tauri dan React untuk menghitung berbagai kalkulasi.

## 📋 Prerequisites

Sebelum menjalankan aplikasi, pastikan Anda sudah menginstal:

### Untuk Web & Desktop
- **Node.js** (v18+) - [Download](https://nodejs.org/)
- **pnpm** (v10+) - Install dengan: `npm install -g pnpm`
- **Git**

### Untuk Desktop (Tauri)
Selain di atas, diperlukan:
- **Rust** (latest stable) - [Download](https://www.rust-lang.org/tools/install)
- **Visual Studio Build Tools 2022** (Windows) atau gcc (Linux/Mac)
- **PostgreSQL** (untuk development) - [Download](https://www.postgresql.org/)

## 🚀 Quick Start

### 1. Clone & Install Dependencies

```bash
# Clone repository
git clone <repository-url>
cd bisa_dihitung

# Install dependencies
pnpm install
```

## 🌐 Menjalankan Web Version

### Development Mode
```bash
# Jalankan development server
pnpm dev
```

Server akan berjalan di `http://localhost:5173`

Fitur:
- Hot reload otomatis saat ada perubahan code
- Development tools dari Vite

### Build untuk Production
```bash
pnpm build
```

Output akan berada di folder `dist/`

### Preview Build Production
```bash
pnpm preview
```

## 🖥️ Menjalankan Tauri (Desktop App)

### Development Mode
```bash
# Jalankan Tauri development
pnpm tauri dev
```

Proses:
1. Vite dev server akan berjalan di background (port 1420)
2. Aplikasi desktop akan terbuka otomatis
3. Hot reload bekerja untuk frontend changes
4. Rust code changes membutuhkan recompile

### Build untuk Production
```bash
# Build aplikasi desktop
pnpm tauri build
```

Output akan berada di `src-tauri/target/release/`

### Perintah Tauri Lainnya

```bash
# Lihat info aplikasi
pnpm tauri info

# Android development
pnpm tauri android dev

# iOS development
pnpm tauri ios dev
```

## 📁 Project Structure

```
bisa_dihitung/
├── src/                    # Frontend React
│   ├── components/        # React components
│   ├── pages/            # Pages
│   ├── const/            # Constants
│   ├── assets/           # Static assets
│   ├── App.jsx           # Main App component
│   ├── main.jsx          # Entry point
│   └── theme.js          # Theme configuration
│
├── src-tauri/            # Tauri backend (Rust)
│   ├── src/
│   │   ├── main.rs       # Main Tauri entry
│   │   └── lib.rs        # Library code
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
│
├── public/               # Public assets
├── package.json          # Node dependencies
└── vite.config.js        # Vite configuration
```

## 🔧 Development Workflow

### Frontend Development
```bash
# Mode web biasa
pnpm dev

# atau dengan Tauri untuk testing desktop
pnpm tauri dev
```

### Backend Development (Rust)
File: `src-tauri/src/main.rs` dan `src-tauri/src/lib.rs`

Perubahan Rust code akan recompile saat menjalankan `pnpm tauri dev`

### Dependencies

**Frontend:**
- React 19.1.0
- React Router 7.12.0
- Ant Design 6.1.4
- Tauri API 2.x

**Backend:**
- Tauri 2.x
- Tokio (async runtime)
- SQLx (database)
- PostgreSQL driver

## 📱 Platform Support

- ✅ Windows
- ✅ macOS
- ✅ Linux
- ✅ Android
- ✅ iOS

## 🐛 Troubleshooting

### Port 1420 sudah digunakan
```bash
# Ubah port di vite.config.js atau kill process yang menggunakan port
# Windows
netstat -ano | findstr :1420
taskkill /PID <PID> /F

# Linux/Mac
lsof -i :1420
kill -9 <PID>
```

### Rust compilation error
```bash
# Clear Rust cache
cargo clean

# Update Rust
rustup update
```

### PostgreSQL connection error
Pastikan PostgreSQL service sudah berjalan dan environment variables sudah di-set dengan benar di file `.env`

## 🎨 IDE Setup (Recommended)

- [VS Code](https://code.visualstudio.com/)
  - Extension: [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - Extension: [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - Extension: [ES7+ React/Redux/React-Native snippets](https://marketplace.visualstudio.com/items?itemName=dsznajder.es7-react-js-snippets)
