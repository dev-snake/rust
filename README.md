# 🛠️ ftools - A Powerful CLI Toolkit for File Operations

**ftools** là một công cụ command-line mạnh mẽ được viết bằng Rust, cung cấp nhiều tiện ích để làm việc với files và thư mục. Công cụ này nhanh, an toàn và dễ sử dụng.

## ✨ Tính năng

| Command  | Mô tả                                         |
| -------- | --------------------------------------------- |
| `dupes`  | 🔍 Tìm file trùng lặp bằng hash SHA256        |
| `search` | 🔎 Tìm kiếm text/regex trong files (như grep) |
| `rename` | ✏️ Đổi tên hàng loạt với regex pattern        |
| `size`   | 📊 Phân tích dung lượng ổ đĩa                 |
| `hash`   | #️⃣ Tính hash file (SHA256, SHA512, MD5)       |
| `diff`   | 📁 So sánh hai thư mục                        |
| `empty`  | 🧹 Tìm và xóa file/thư mục trống              |
| `list`   | 📋 Liệt kê files với thông tin chi tiết       |
| `large`  | 📦 Tìm file lớn                               |
| `recent` | 🕐 Tìm file mới được sửa đổi                  |
| `stats`  | 📈 Thống kê thư mục                           |

## 🚀 Cài đặt

### Build từ source

```bash
git clone https://github.com/yourusername/ftools.git
cd ftools
cargo build --release

# Binary sẽ ở target/release/ftools.exe (Windows) hoặc target/release/ftools (Linux/macOS)
```

### Thêm vào PATH (Windows)

```powershell
# Copy vào thư mục trong PATH
Copy-Item .\target\release\ftools.exe C:\Users\$env:USERNAME\.cargo\bin\
```

## 📖 Cách sử dụng

### Xem help

```bash
ftools --help
ftools <command> --help
```

### 🔍 Tìm file trùng lặp

```bash
# Tìm duplicates trong thư mục hiện tại
ftools dupes .

# Chỉ tìm file ảnh
ftools dupes ~/Pictures --extensions jpg,png,gif

# Xuất kết quả ra JSON
ftools dupes . --output duplicates.json

# Xóa duplicates (giữ lại file đầu tiên)
ftools dupes . --delete
```

### 🔎 Tìm kiếm text

```bash
# Tìm kiếm đơn giản
ftools search "TODO" ./src

# Tìm kiếm regex
ftools search "fn\s+\w+" ./src --extensions rs

# Case-insensitive
ftools search "error" . --ignore-case

# Chỉ hiện tên file
ftools search "import" . --files-only

# Hiện context xung quanh
ftools search "function" . --context 3
```

### ✏️ Đổi tên hàng loạt

```bash
# Xem trước thay đổi (dry run)
ftools rename ./photos --find "IMG_(\d+)" --replace "photo_$1"

# Áp dụng thay đổi
ftools rename ./photos --find "IMG_" --replace "vacation_" --dry-run=false

# Đổi tên trong thư mục con
ftools rename . --find "old" --replace "new" --recursive
```

### 📊 Phân tích dung lượng

```bash
# Top 20 thư mục lớn nhất
ftools size .

# Nhóm theo loại file
ftools size . --by-type

# Xuất ra CSV
ftools size . --csv disk_usage.csv

# Chỉ hiện item >= 10MB
ftools size . --min 10MB
```

### #️⃣ Tính hash file

```bash
# SHA256 (mặc định)
ftools hash file.txt

# SHA512
ftools hash file.txt --algorithm sha512

# Verify hash
ftools hash file.txt --verify abc123...

# Nhiều file, xuất JSON
ftools hash *.zip --format json
```

### 📁 So sánh thư mục

```bash
# So sánh cơ bản (theo tên & size)
ftools diff folder1 folder2

# So sánh nội dung (bằng hash)
ftools diff folder1 folder2 --content

# Chỉ hiện khác biệt
ftools diff folder1 folder2 --diff-only
```

### 🧹 Tìm items trống

```bash
# Tìm tất cả file và thư mục trống
ftools empty .

# Chỉ thư mục trống
ftools empty . --dirs

# Xóa items trống
ftools empty . --delete
```

### 📋 Liệt kê files

```bash
# Liệt kê đơn giản
ftools list .

# Chi tiết với size và date
ftools list . --long

# Sắp xếp theo size
ftools list . --sort size --long

# Lọc theo pattern
ftools list . --pattern "*.rs" --recursive
```

### 📦 Tìm file lớn

```bash
# Tìm file >= 100MB
ftools large .

# Tìm file >= 1GB
ftools large / --size 1GB

# Top 10 file lớn nhất
ftools large . --size 1MB --top 10
```

### 🕐 Tìm file mới sửa đổi

```bash
# Files sửa trong 24h qua
ftools recent .

# Files sửa trong 1 giờ qua
ftools recent . --within 1h

# Files sửa trong 7 ngày qua
ftools recent . --within 7d
```

### 📈 Thống kê thư mục

```bash
# Xem thống kê
ftools stats .

# Bao gồm hidden files
ftools stats . --hidden
```

## ⚡ Performance

- **Multi-threaded**: Sử dụng Rayon để xử lý song song
- **Efficient hashing**: Buffer 1MB, streaming hash
- **Smart filtering**: Bỏ qua binary files khi search
- **Memory efficient**: Không load toàn bộ file vào RAM

## 🔧 Dependencies chính

- `clap` - CLI argument parsing
- `walkdir` - Directory traversal
- `rayon` - Parallel processing
- `sha2` - Cryptographic hashing
- `regex` - Pattern matching
- `indicatif` - Progress bars
- `colored` - Terminal colors
- `serde_json` - JSON serialization

## 📝 License

MIT License

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first.
