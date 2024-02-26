use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // name of the file to read. if none is provided, read from stdin
    #[arg(long, default_value = "")]
    file: String,

    // where to output the results. if none is provided, write to stdout
    #[arg(long, default_value = "")]
    output: String,
}

mod parser;

fn main() {
    let args = Args::parse();

    let lines = match read_input(&args.file) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("could not read input: {}", e);
            std::process::exit(1);
        }
    };

    let parsed = match parser::parse(lines) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("could not parse input: {}", e);
            std::process::exit(1);
        }
    };

    let mut games_map = std::collections::HashMap::new();
    parsed
        .iter()
        .enumerate()
        .for_each(|(i, game)| { games_map.insert(format!("game_{}", i), game); });

    write_output(&args.output, serde_json::to_string(&games_map).unwrap()).unwrap();
}

fn read_input(file: &str) -> Result<Vec<String>, String> {
    if file == "" {
        let reader = BufReader::new(std::io::stdin());
        let lines = reader
            .lines()
            .filter(|e| e.is_ok())
            .map(|e| e.unwrap())
            .collect();
        return Ok(lines);
    }
    let f = match File::open(file) {
        Ok(f) => f,
        Err(e) => return Err(format!("could not open file: {}", e)),
    };
    let reader = BufReader::new(f).lines();
    Ok(reader.filter(|e| e.is_ok()).map(|e| e.unwrap()).collect())
}

fn write_output(output: &str, content: String) -> Result<(), String> {
    if output == "" {
        println!("{}", content);
        return Ok(());
    }

    let mut f = match File::create(output) {
        Ok(f) => f,
        Err(e) => return Err(format!("could not create file: {}", e)),
    };

    match f.write(content.as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(format!("could not write to file: {}", e)),
    }

    Ok(())
}
