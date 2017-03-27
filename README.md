# cedict - Rust library for parsing CC-CEDICT

## What is CC-CEDICT
CC-CEDICT, or formerly CEDICT, is a freely available Chinese-English dictionary. This library allows you to parse it.

## Usage

```rust
let line = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/";
let parsed = cedict::parse_line(line).unwrap();

println!("{}", parsed.definitions[0]); // Prints "Hello!"
```
