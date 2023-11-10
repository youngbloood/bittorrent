use std::env;
mod decode;

// Available if you need it!
// use serde_bencode

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let (_, decoded_value) = decode::decode_list(encoded_value);
        println!("{}", decoded_value.to_string());
    } else if command == "info" {
        let encoded_value = &args[2];
        let (_, decoded_value) = decode::decode_list(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
