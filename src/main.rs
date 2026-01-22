use std::env::args;
mod csv_analyzer;
mod file_analyzer;
use csv::Reader;
use csv_analyzer::csv_analyze;
use file_analyzer::analyze;
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run -- analyze filename");
        return;
    }

    let command = &args[1];
    let file_name = &args[2];
    // println!("Command: {}", command);
    // println!("Filename: {}", args[2]);

    match command.as_str() {
        "analyze" => {
            if let Err(e) = analyze(file_name) {
                eprintln!("Error: {}", e);
            }
        }
        "csv" => csv_analyze(file_name),
        _ => {
            println!("Command not found");
            println!("Avaliable commands: analyze",)
        }
    }
}
