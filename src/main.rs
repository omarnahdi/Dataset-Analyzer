use std::env::args;
mod csv_analyzer;
mod file_analyzer;

use csv_analyzer::csv_analyze;
use file_analyzer::analyze;
use csv_analyzer::data_validation;

fn main() {
    let args: Vec<String> = args().collect();

    if args.get(1).map(|s| s.as_str()) == Some("version") {
        println!("RustSight v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.get(1).map(|s| s.as_str()) == Some("help") || args.len() < 2 {
        println!("RustSight v{} 🦀", env!("CARGO_PKG_VERSION"));
        println!("Fast, safe dataset analysis and validation CLI — built in Rust.\n");
        println!("USAGE:");
        println!("  rustsight <COMMAND> <FILE>\n");
        println!("COMMANDS:");
        println!("  csv <file>       Analyze a CSV file — column types, missing values, min/max/mean");
        println!("  validate <file>  Validate a CSV file for data quality issues before ML/AI training");
        println!("  analyze <file>   Inspect any file — bytes, UTF-8 validity, line & word counts");
        println!("  version          Print the current version");
        println!("  help             Show this help message\n");
        println!("EXAMPLES:");
        println!("  rustsight csv dataset.csv");
        println!("  rustsight validate dataset.csv");
        println!("  rustsight analyze report.txt\n");
        println!("More info: https://github.com/omarnahdi/Dataset-Analyzer");
        return;
    }

    if args.len() < 3 {
        println!("Usage: rustsight <COMMAND> <FILE>");
        println!("Run 'rustsight help' for available commands.");
        return;
    }

    let command = &args[1];
    let file_name = &args[2];
    let d_v = || {
        let d = csv_analyze(file_name, Some(0));
        data_validation(&d);
    };
    match command.as_str() {
        "analyze" => {
            if let Err(e) = analyze(file_name) {
                eprintln!("Error: {}", e);
            }
        }
        "csv" => {
            let _csv = csv_analyze(file_name, Some(1));
        }
        "validate" => {
            d_v();
        }
        _ => {
            println!("Command '{}' not found.", &args[1]);
            println!("Run 'rustsight help' for available commands.");
        }
    }
}