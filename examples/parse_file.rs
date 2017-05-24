extern crate cedict;

use std::fs::File;

fn main() {
    let file = File::open("cedict.txt").unwrap();

    for definition in cedict::parse_reader(file) {
        if definition.definitions[0].contains("Hello") {
            println!("{:?}", definition);
        }
    }
}
