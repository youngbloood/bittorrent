use std::env;
mod parse;
// Available if you need it!
// use serde_bencode

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let result = parse::Cell::new(encoded_value);
        println!("result = {:?}", result);
    } else if command == "info" {
        let encoded_value = &args[2];
        let result = parse::Cell::new(encoded_value);
        println!("result = {:?}", result);
    } else {
        println!("unknown command: {}", args[1])
    }
}
