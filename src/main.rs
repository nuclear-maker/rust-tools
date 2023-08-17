use encoding_rs::{GBK, UTF_8};
use std::env;
use std::fs;
use std::io::{self, Read, Write};

fn convert_file_to_utf8(file_path: &str) -> io::Result<()> {
    let mut file = fs::File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let (cow, _, had_errors) = GBK.decode(&content);
    if had_errors {
        eprintln!("Failed to decode file: {}", file_path);
        return Ok(());
    }

    let (converted, _, _) = UTF_8.encode(&cow);
    let mut output_file = fs::File::create(file_path)?;
    output_file.write_all(&converted)?;

    Ok(())
}

fn process_directory(directory_path: &str) -> io::Result<()> {
    let entries = fs::read_dir(directory_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            process_directory(path.to_str().unwrap())?;
        } else if path.is_file() {
            convert_file_to_utf8(path.to_str().unwrap())?;
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./program <directory_path>");
        return;
    }

    let directory_path = &args[1];

    if let Err(err) = process_directory(directory_path) {
        eprintln!("Error: {}", err);
    } else {
        println!("Conversion completed successfully!");
    }
}
