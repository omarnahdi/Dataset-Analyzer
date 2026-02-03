use std::env::args;
mod csv_analyzer;
mod file_analyzer;

use csv_analyzer::csv_analyze;
use file_analyzer::analyze;
use csv_analyzer::data_validation;
use crate::csv_analyzer::CsvAnalysis;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run -- analyze filename");
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
            let csv = csv_analyze(file_name, Some(1));
        }
        "validate" => {
            d_v();
        }
        _ => {
            println!("Command not found");
            println!("Available commands: analyze, csv, validate",)
        }
    }
}
