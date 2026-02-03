# Dataset Analyzer (Rust)

A fast, safe, and extensible **dataset analysis CLI tool written in Rust**.  
This project focuses on **data validation and exploratory analysis** — the exact step that comes *before* AI/ML model training.

It works on **any CSV file** and can also analyze **binary or text files** to extract useful properties.

---

## ✨ Features

### CSV Dataset Analysis
- Detects **numeric vs categorical columns**
- Counts **missing values per column**
- Computes **basic statistics** (min, max, mean) for numeric columns
- Handles large CSV files efficiently (streaming)
- Generates a **clean, readable analysis report**
- Saves results to a `_report.txt` file

### Data Validation (NEW)
- Detects columns with **high missing value ratios**
- Flags **no-variance columns** (min == max)
- Detects **potential outliers**
- Identifies **mixed-type columns**
- Prints clear **validation warnings** before ML usage

### File Analysis
- Counts total bytes
- Detects UTF-8 validity
- Counts lines and words (if text)
- Counts non-ASCII bytes (for binaries)

---

## 📂 Example Datasets

Used during development (not required):

- `stockdata.csv` — financial dataset
- `CVD Dataset.csv` — cardiovascular health dataset

> ⚠ Large datasets are **not bundled**.  
You can analyze **any CSV file**.

---

## 🚀 How to Run

### 1️⃣ Analyze a CSV dataset
```bash
cargo run -- csv stockdata.csv
cargo run -- csv "CVD Dataset.csv"
```

This will:
- Print column-wise analysis
- Save a report like: `stockdata_report.txt`

---

### 2️⃣ Validate a dataset (NEW)

```bash
cargo run -- validate insta_data.csv
```

Example output:
```
File: insta_data.csv
⚠ Column 'followers_count' may contain outliers
⚠ Column 'user_engagement_score' may contain outliers
```

This helps detect **data quality issues before ML training**.

---

### 3️⃣ Analyze any file (text or binary)

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

## 📦 Installation (From Source)

```bash
git clone https://github.com/omarnahdi/Dataset-Analyzer.git
cd dataset-analyzer

cargo build --release
```

Run using:
```bash
./target/release/dataset_analyzer csv your_file.csv
```

---

## 🪟 Using the Windows `.exe` (Recommended)

1. Go to the **Releases** section on GitHub
2. Download: `dataset_analyzer.exe`
3. Run from terminal:

```bash
dataset_analyzer.exe csv your_file.csv
dataset_analyzer.exe validate your_file.csv
```

No Rust installation required.

---

## 🛠️ Tech Stack

- **Rust** — performance, memory safety
- **csv crate** — efficient CSV parsing
- **CLI first design** — easy automation & scripting

---

## 📝 License

MIT License

---

## 🤝 Contributing

Contributions are welcome!  
Feel free to open issues or submit pull requests.