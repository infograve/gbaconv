use std::fs::File;
use std::io::{self, Read, Write};
use std::env;
use std::process;

fn reverse_bytes(input: &[u8]) -> Vec<u8> {
    let mut reversed = input.to_vec();
    reversed.reverse();
    reversed
}

fn process_file(input_file: &str, output_file: &str) -> io::Result<()> {
    let mut input = File::open(input_file)?;
    let mut output = File::create(output_file)?;

    let mut buffer = [0; 8];  // 8バイト毎のバッファ

    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let reversed_bytes = reverse_bytes(&buffer[..bytes_read]);
        output.write_all(&reversed_bytes)?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    if let Err(e) = process_file(input_file, output_file) {
        eprintln!("Error processing file: {}", e);
        process::exit(1);
    }

    println!("File processed successfully.");
}
