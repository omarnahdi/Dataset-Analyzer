// mod boilerplate;

use std::path::Path;
use std::fs;
// use std::path::Path;
use csv::Reader;

struct ColumnProfiler {
    name: String,
    numeric_count: usize,
    missing_count: usize,
}

#[derive(Debug)]
#[derive(Clone)]
struct Statistics {
    min: f64,
    max: f64,
    sum: f64,
    count: usize,
}

pub struct CsvAnalysis {
    header: csv::StringRecord,
    // header_length
    columns: Vec<ColumnProfiler>,
    stats_col: Vec<Option<Statistics>>,
    total_rows: usize,
}


// const PATH: Path = Path::new(p);
pub fn csv_analyze(f: &str, report_gen: Option<i32>) -> CsvAnalysis {
    let file_name = f;

    if !file_name.ends_with(".csv") {
        eprintln!("❌ Error: Not a CSV file (must end with .csv)");
        std::process::exit(1);
    }
    if !Path::new(&file_name).exists() {
        println!("❌ File not found!");
        std::process::exit(1);
    }

    let mut data = Reader::from_path(&file_name).expect("Error opening csv file");
    println!("File: {}", file_name);
    let header = data.headers().expect("Error reading").clone();
    let header_length = header.len();
    let mut stats_col: Vec<Option<Statistics>> = vec![None; header_length];

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
            if let Ok(x) = val.parse::<f64>() {
                // println!("\t is numeric {}", 1);
                // println!("{}: {}", i + 1, val);
                // scoreboard[i] += 1;
                // println!("Stats: {:?}", stats);
                column[i].numeric_count += 1;

                if stats_col[i].is_none() {
                    stats_col[i] = Some(Statistics {
                        min: x,
                        max: x,
                        sum: x,
                        count: 1,
                    })
                } else {
                    let stats = stats_col[i].as_mut().unwrap();

                    if x < stats.min {
                        stats.min = x;
                    }

                    if x > stats.max {
                        stats.max = x;
                    }

                    stats.count += 1;
                    stats.sum += x;
                }
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

    for (i, stats_opt) in stats_col.iter().enumerate() {
        if let Some(stats) = stats_opt {
            let mean = stats.sum / stats.count as f64;

            // println!("Column Statistics {}:", &header[i]);
            // println!("min: {}", stats.min);
            // println!("max: {}", stats.max);
            // println!("mean: {}", mean);
            // println!("")
        }
    }
    if report_gen == Option::from(1) {
        let mut report = String::new();

        report.push_str(&format!("File: {}\n\n", file_name));
        report.push_str(
            "Column                          | Missing | Type        | Min     | Max     | Mean\n"
        );
        report.push_str(
            "------------------------------------------------------------------------------------\n"
        );

        for i in 0..header_length {
            let col = &column[i];

            let col_type = if col.numeric_count > total_rows / 2 {
                "numeric"
            } else {
                "categorical"
            };

            match &stats_col[i] {
                Some(stats) => {
                    let mean = stats.sum / stats.count as f64;

                    report.push_str(&format!(
                        "{:<30} | {:>7} | {:<11} | {:>7.2} | {:>7.2} | {:>7.2}\n",
                        &header[i],
                        col.missing_count,
                        col_type,
                        stats.min,
                        stats.max,
                        mean
                    ));
                }
                None => {
                    report.push_str(&format!(
                        "{:<30} | {:>7} | {:<11} | {:>7} | {:>7} | {:>7}\n",
                        &header[i],
                        col.missing_count,
                        col_type,
                        "N/A",
                        "N/A",
                        "N/A"
                    ));
                }
            }
        }

        report.push_str(&format!("\nTotal rows: {}\n", total_rows));
        report.push_str(&format!("Columns: {}\n", header_length));

        use std::path::{Path, PathBuf};
        let input_path = Path::new(file_name);
        // 1️⃣ Get file name without extension
        let stem = input_path.file_stem().expect("Invalid file name").to_string_lossy();
        // 2️⃣ Build new file name with _report
        let output_file: PathBuf = input_path.with_file_name(format!("{}_report.txt", stem));
        // 3️⃣ Write report
        fs::write(&output_file, report).expect("Error writing report");
        println!("Saved report to {:?}", output_file);
    }
    CsvAnalysis {
        header: header,
        columns: column,
        stats_col: stats_col,
        total_rows: total_rows,
    }
    // (header, header_length, column, stats_col);
}
pub fn data_validation (analysis: &CsvAnalysis) {
    let mut warnings = 0;

    for i in 0..analysis.header.len() {
        let col = &analysis.columns[i];

        let missing_ratio =
            col.missing_count as f64 / analysis.total_rows as f64;

        if missing_ratio > 0.01 {
            warnings += 1;
            println!(
                "⚠ Column '{}' has {:.1}% missing values",
                &analysis.header[i],
                missing_ratio * 100.0
            );
        }

        if let Some(stats) = &analysis.stats_col[i] {
            let mean = stats.sum / stats.count as f64;

            if (stats.max - stats.min) == 0.0 && stats.count > 1 {
                warnings += 1;
                println!(
                    "⚠ Column '{}' has no variance",
                    &analysis.header[i]
                );
            }

            if stats.max > mean * 10.0 {
                warnings += 1;
                println!(
                    "⚠ Column '{}' may contain outliers",
                    &analysis.header[i]
                );
            }
        }

        let numeric_ratio =
            col.numeric_count as f64 / analysis.total_rows as f64;

        if numeric_ratio > 0.2 && numeric_ratio < 0.8 {
            warnings += 1;
            println!(
                "⚠ Column '{}' has mixed data types",
                &analysis.header[i]
            );
        }
    }

    if warnings == 0 {
        println!("Data is clean you are good to go!\n");
    }
}
    fn main() {
    csv_analyze("CVD Dataset.csv", Some(1));
}