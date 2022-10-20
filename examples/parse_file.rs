extern crate cedict;

use std::fs::File;

fn main() {
    let file = File::open("cedict.txt").unwrap();

    for definition in cedict::parse_reader(file) {
        // Join all definitions with a comma
        let definitions = definition.definitions().collect::<Vec<_>>().join(", ");

        // Make it lowercase
        let definitions = definitions.to_lowercase();

        if definitions.contains("hello") {
            println!("{} {} {}", definition.simplified(), definition.pinyin(), definitions);
        }
    }
}