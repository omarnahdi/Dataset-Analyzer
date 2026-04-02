
# RustSight

RustSight is a fast, safe, and extensible **dataset analysis CLI tool written in Rust**.  
This project focuses on **data validation and exploratory analysis** — the exact step that comes *before* AI/ML model training.
It works on **any CSV file** and can also analyze **binary or text files** to extract useful properties.


[![Official Website & Docs](https://img.shields.io/badge/Official_Website-rustsight.omarnahdi.dev-rust?style=for-the-badge)](https://rustsight.omarnahdi.dev)

*Read the official interactive documentation, watch video demos, and view detailed benchmarks at [rustsight.omarnahdi.dev](https://rustsight.omarnahdi.dev).* 

[![Crates.io](https://img.shields.io/crates/v/rustsight)](https://crates.io/crates/rustsight)
[![Downloads](https://img.shields.io/crates/d/rustsight)](https://crates.io/crates/rustsight)
[![gitcgr](https://gitcgr.com/badge/omarnahdi/Dataset-Analyzer.svg)](https://gitcgr.com/omarnahdi/Dataset-Analyzer)

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
./target/release/rustsight stats your_file.csv
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

### Inspect any file (text or binary)

```bash
rustsight inspect your_file.txt
```

### Help & Version

```bash
rustsight help
rustsight version
```

---

## ⚡ Benchmark

Tested on **chicago crimes.csv** — 8,500,901 rows, 22 columns.

| Tool | Time | vs Pandas |
|------|------|-----------|
| 🐻‍❄️ Polars (Python) | 1.42s | 22.2× faster |
| 🦆 DuckDB CLI | 4.33s | 7.3× faster |
| 🦀 **RustSight** | **5.21s** | **6.1× faster** |
| 🐼 Pandas (Python) | 31.53s | baseline |
| ❌ csvkit | DNF | unusably slow |

> Benchmarked on Windows, release build (`cargo build --release`), 20 threads.  
> RustSight outperforms Pandas by **6.1×** and runs within 1.3 seconds of DuckDB — a production C++ query engine.

Dataset source: [Chicago Crime Dataset 2024–2026](https://www.kaggle.com/datasets/aliafzal9323/chicago-crime-dataset-2024-2026) via Kaggle.

---

## 📂 Example Datasets

Used during development (not required):

- `stockdata.csv` — financial dataset
- `CVD Dataset.csv` — cardiovascular health dataset

> ⚠ Large datasets are **not bundled**. You can analyze **any CSV file**.

---

## 🪟 Windows `.exe`

1. Go to the **Releases** section on GitHub
2. Download `rustsight.exe`
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
RustSight: https://rustsight.omarnahdi.dev/
crates.io: https://crates.io/crates/rustsight  
```
