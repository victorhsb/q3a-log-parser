use clap::Parser;
use std::fs::File;
use std::io::BufReader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // name of the file to read. if none is provided, read from stdin
    #[arg(short, long, default_value = "")]
    file: String,
}

mod parser;
use parser::parser::{ parse, group };

fn main() {
    let args = Args::parse();
    let f = File::open(args.file).unwrap();
    let reader = BufReader::new(f);

    let raw_games = group( parse(reader) );

    // for r in raw_games {
    //     println!("{}", serde_json::to_string(&r).unwrap());
    // }

    let games = raw_games.iter().map(|g| {
        serde_json::to_string(&g.to_game()).unwrap()
    }).collect::<Vec<_>>();

    for g in games {
        println!("{}", g);
    }
}
