extern crate cedict;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("cedict.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|l| l.ok());
    let entries = lines.map(cedict::parse_line);

    for entry in entries {
        println!("{:?}", entry);
    }
}
