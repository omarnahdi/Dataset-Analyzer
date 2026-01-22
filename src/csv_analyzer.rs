// mod boilerplate;

use std::fs;
// use std::path::Path;
use csv::Reader;

struct ColumnProfiler {
    name: String,
    numeric_count: usize,
    missing_count: usize,
}

// const PATH: Path = Path::new(p);
pub fn csv_analyze(f: &str) {
    let file_name = f;
    if !Path::new(&file_name).exists() {
        println!("File not found!");
        // return
    }

    let mut data = Reader::from_path(&file_name).expect("Error opening csv file");
    println!("File: {}", file_name);
    let header = data.headers().expect("Error reading").clone();
    let header_length = header.len();

    let mut column: Vec<ColumnProfiler> = header
        .iter()
        .map(|h| ColumnProfiler {
            name: h.to_string(),
            numeric_count: 0,
            missing_count: 0,
        })
        .collect();
    // let mut rows = 0;
    // println!("Data: ");

    let mut total_rows = 0;
    // let mut scoreboard = vec![0; header_length];
    for record in data.records() {
        let re = record.unwrap();
        total_rows += 1;
        for (i, val) in re.iter().enumerate() {
            // println!("{}: {}", i + 1, val);
            if val.trim().is_empty() {
                column[i].missing_count += 1;
            }
            if val.parse::<f64>().is_ok() {
                // println!("\t is numeric {}", 1);
                // println!("{}: {}", i + 1, val);
                // scoreboard[i] += 1;
                column[i].numeric_count += 1;
            }
        }
    }
    // for (i, val) in scoreboard.iter().enumerate() {
    //     if *val > (total_rows / 2) {
    //         // let num = data.headers().unwrap().get(i).unwrap();
    //         let num = &header[i];
    //
    //         println!("Column: {} -> Numeric:  ", num);
    //     } else {
    //         let cat = data.headers().unwrap().get(i).unwrap();
    //         // let cat = &header[i + 1];
    //         println!("Column: {} -> categorial", cat)
    //     }
    // }
    let mut report = String::new();

    report.push_str(&format!("File: {}\n\n", file_name));
    report.push_str("Column                              | Missing | Type\n");
    report.push_str("----------------------------------------------------------\n");

    for c in &column {
        let col_type = if c.numeric_count > total_rows / 2 {
            "numeric"
        } else {
            "categorical"
        };

        report.push_str(&format!(
            "{:<35} | {:>7} | {}\n",
            c.name, c.missing_count, col_type
        ));
    }

    report.push_str(&format!("\nTotal rows: {}\n", total_rows));

    report.push_str(&format!("Columns: {}", header_length));
    use std::path::{Path, PathBuf};

    let input_path = Path::new(file_name);

    // 1️⃣ Get file name without extension
    let stem = input_path
        .file_stem()
        .expect("Invalid file name")
        .to_string_lossy();

    // 2️⃣ Build new file name with _report
    let output_file: PathBuf = input_path.with_file_name(format!("{}_report.txt", stem));

    // 3️⃣ Write report
    fs::write(&output_file, report).expect("Error writing report");

    println!("Saved report to {:?}", output_file);
}

fn main() {
    csv_analyze("CVD Dataset.csv");
}
