# RustSight

RustSight is a fast, safe, and extensible **dataset analysis CLI tool written in Rust**.  
This project focuses on **data validation and exploratory analysis** — the exact step that comes *before* AI/ML model training.

It works on **any CSV file** and can also analyze **binary or text files** to extract useful properties.

[![Crates.io](https://img.shields.io/crates/v/rustsight)](https://crates.io/crates/rustsight)
[![Downloads](https://img.shields.io/crates/d/rustsight)](https://crates.io/crates/rustsight)

---

## 📦 Installation

### From crates.io (Recommended)

```bash
cargo install rustsight
```

👉 [crates.io/crates/rustsight](https://crates.io/crates/rustsight)

### From Source

```bash
git clone https://github.com/omarnahdi/Dataset-Analyzer.git
cd dataset-analyzer
cargo build --release
./target/release/rustsight csv your_file.csv
```

---

## ✨ Features

### CSV Dataset Analysis
- Detects **numeric vs categorical columns**
- Counts **missing values per column**
- Computes **basic statistics** (min, max, mean) for numeric columns
- Handles large CSV files efficiently (streaming)
- Generates a **clean, readable analysis report**
- Saves results to a `_report.txt` file

### Data Validation
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

## 🚀 Usage

### Analyze a CSV dataset
```bash
rustsight stats your_dataset.csv
```

### Validate a dataset
```bash
rustsight validate your_dataset.csv
```
Example output:
```
File: insta_data.csv
⚠ Column 'followers_count' may contain outliers
⚠ Column 'user_engagement_score' may contain outliers
```

### Analyze any file (text or binary)
```bash
rustsight inspect your_file.txt
```

### Help & Version
```bash
rustsight help
rustsight version
```

---

## 📂 Example Datasets

Used during development (not required):

- `stockdata.csv` — financial dataset
- `CVD Dataset.csv` — cardiovascular health dataset

> ⚠ Large datasets are **not bundled**. You can analyze **any CSV file**.

---

## 🪟 Windows `.exe`

1. Go to the **Releases** section on GitHub
2. Download `dataset_analyzer.exe`
3. Run from terminal:

```bash
rustsight stats your_file.csv
rustsight validate your_file.csv
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

Contributions are welcome! Feel free to open issues or submit pull requests.

Portfolio: https://omarnahdi.dev  
RustSight: https://omarnahdi.dev/work/dataset-analyzer  
crates.io: https://crates.io/crates/rustsight  
Learn more: https://omarnahdi.dev/writing/rustsight-cli-csv-analyzer
