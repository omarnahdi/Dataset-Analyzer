# Dataset Analyzer (Rust)

A fast, safe, and extensible **dataset analysis CLI tool written in Rust**.  
This project focuses on **data validation and exploratory analysis** — the exact step that comes *before* AI/ML model training.

It works on **any CSV file** and can also analyze **binary files** to determine text properties.

---

## ✨ Features

### CSV Dataset Analysis
- Detects **numeric vs categorical columns**
- Counts **missing values per column**
- Handles large CSV files efficiently (streaming)
- Generates a **clean, readable report**
- Saves analysis results to a `_report.txt` file

### File Analysis
- Counts total bytes
- Detects UTF-8 validity
- Counts lines and words (if text)
- Counts non-ASCII bytes (for binaries)

---

## 📂 Example Datasets Included

- `stockdata.csv` — financial dataset
- `CVD Dataset.csv` — cardiovascular health dataset

You can analyze **any CSV**, not just these.

---

## 🚀 How to Run

### 1️⃣ Analyze a CSV dataset
```bash
cargo run -- csv stockdata.csv
cargo run -- csv "CVD Dataset.csv"
```

This will:
- Print a column-wise analysis
- Save a report like: `stockdata_report.txt`

### 2️⃣ Analyze any file (text or binary)
```bash
cargo run -- analyze stockdata.csv
cargo run -- analyze target\debug\dataset_analyzer.exe
```

This detects:
- Total bytes
- UTF-8 validity
- Line & word counts (if text)
- Non-ASCII bytes (if binary)

---

## 📦 Installation
```bash
# Clone the repository
git clone https://github.com/omarnahdi/Dataset-Analyzer.git
cd dataset-analyzer

# Build the project
cargo build --release

# Run
cargo run -- csv <your-file.csv>
```

---

## 🛠️ Tech Stack

- **Rust** — for performance and safety
- **csv crate** — for efficient CSV parsing

---

## 📝 License

MIT License

---

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
