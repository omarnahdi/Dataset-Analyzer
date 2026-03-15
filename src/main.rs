use std::env::args;
use std::time::Instant;
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
        println!("  stats <file>     Analyze a CSV file — column types, missing values, min/max/mean");
        println!("  validate <file>  Validate a CSV file for data quality issues before ML/AI training");
        println!("  inspect <file>   Inspect any file — bytes, UTF-8 validity, line & word counts");
        println!("  version          Print the current version");
        println!("  help             Show this help message\n");
        println!("EXAMPLES:");
        println!("  rustsight csv dataset.csv");
        println!("  rustsight validate dataset.csv");
        println!("  rustsight analyze report.txt\n");
        println!("More info: https://github.com/omarnahdi/Dataset-Analyzer");
        println!("Learn more: https://omarnahdi.dev/writing/rustsight-cli-csv-analyzer");
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
        "inspect" => {
            let start = Instant::now();
            if let Err(e) = analyze(file_name) {
                eprintln!("Error: {}", e);
            } else {
                print_elapsed(start.elapsed());
            }
        }
        "stats" => {
            let start = Instant::now();
            let _csv = csv_analyze(file_name, Some(1));
            print_elapsed(start.elapsed());
        }
        "validate" => {
            let start = Instant::now();
            d_v();
            print_elapsed(start.elapsed());
        }
        _ => {
            println!("Command '{}' not found.", &args[1]);
            println!("Run 'rustsight help' for available commands.");
        }
    }
}

fn print_elapsed(elapsed: std::time::Duration) {
    if elapsed.as_secs() > 0 {
        println!("\n🕛  Analysis completed in {:.2}s", elapsed.as_secs_f64());
    } else {
        println!("\n🕛  Analysis completed in {}ms", elapsed.as_millis());
    }
}