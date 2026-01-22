use std::{fs, io::Error, path::Path};

pub struct FileData {
    bytes: Vec<u8>,
    utf8valid: bool,
    total_bytes: usize,
    line_count: Option<usize>,
    word_count: Option<usize>,
    non_ascii_count: i32,
}

pub fn stats(fd: &FileData, file_name: &str) {
    println!();
    println!("Dataset Analysis Report");
    println!("-----------------------");
    println!("Source file        : {}", file_name);
    println!("Total bytes        : {}", fd.total_bytes);
    println!(
        "UTF-8 valid        : {}",
        if fd.utf8valid { "yes" } else { "no" }
    );

    match fd.line_count {
        Some(lines) => println!("Text lines         : {}", lines),
        None => println!("Text lines         : N/A"),
    }

    match fd.word_count {
        Some(words) => println!("Word count         : {}", words),
        None => println!("Word count         : N/A"),
    }

    println!("Non-ASCII bytes    : {}", fd.non_ascii_count);
}

pub fn analyze(file_name: &str) -> Result<FileData, Error> {
    if !Path::new(file_name).exists() {
        println!("File not found!");

        // return Ok();
    }
    let mut fd = FileData {
        bytes: fs::read(file_name)?,
        utf8valid: true,
        total_bytes: 0,
        line_count: None,
        word_count: None,
        non_ascii_count: 0,
    };

    fd.non_ascii_count = 0;

    // let bytes = fs::read(file_name)?;   // <-- FIXED
    fd.total_bytes = fd.bytes.len();
    fd.utf8valid = std::str::from_utf8(&fd.bytes).is_ok();

    for byte in &fd.bytes {
        if *byte > 127 {
            fd.non_ascii_count += 1;
        }
    }

    match std::str::from_utf8(&fd.bytes) {
        Ok(text) => {
            // utf8_valid = true;
            fd.line_count = Some(text.lines().count());
            fd.word_count = Some(text.split_whitespace().count());
            // println!("UTF-8: {}", if fd.utf8valid { "yes" } else { "no" });
            // println!("Lines: {}", text.lines().count());
            // println!("Words: {}", text.split_whitespace().count());
        }
        Err(_) => {
            // fd.utf8valid = false
        }
    }

    stats(&fd, file_name);

    Ok(fd)
}

fn main() {
    // let p
    let _ = analyze("data.txt").expect("An error occurred");
}
