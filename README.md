# cedict - Rust library for parsing CC-CEDICT

[![Crates.io](https://img.shields.io/crates/v/cedict.svg)](https://crates.io/crates/cedict)
[![Docs.rs](https://img.shields.io/badge/docs-cedict-brightgreen.svg)](https://docs.rs/cedict)

Welcome to `cedict`! This Rust crate provides functionality for reading and writing the CC-CEDICT Chinese-English
dictionary format. You can use it to implement Chinese dictionaries in Rust or
automate maintenance tasks for the CEDICT project.

## What is CC-CEDICT?
[CC-CEDICT](https://cc-cedict.org/) is a freely available Chinese-English
dictionary. This library allows you to parse it.

## Usage

Here's a simple example of parsing a single dictionary entry:

```rust
let line = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/";
let parsed = cedict::parse_line(line).unwrap();

println!("{}", parsed.definitions[0]); // Prints "Hello!"
```

Parse a dictionary file and search for "Hello".

```rust
extern crate cedict;

use std::fs::File;

fn main() {
    let file = File::open("cedict.txt").unwrap();

    for definition in cedict::parse_reader(file) {
        if definition.definitions().next().unwrap().contains("Hello") {
            println!("{}", definition.simplified());
        }
    }
}
```
