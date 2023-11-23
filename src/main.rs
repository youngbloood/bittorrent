use serde_bencode::value::Value;
use std::fs::File;
use std::io::Read;
use std::{env, fs};

mod parse;
// Available if you need it!
// use serde_bencode

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let result = parse::Cell::new(encoded_value).unwrap();
        println!("result = {:?}", result);
    } else if command == "info" {
        let filename = &args[2];

        let content = fs::read(filename).unwrap();
        let info: Value = serde_bencode::from_bytes::<Value>(&content).unwrap();

        print_value(&info);
    } else {
        println!("unknown command: {}", args[1])
    }
}

fn print_value(info: &Value) {
    match &info {
        Value::Bytes(bts) => {
            println!("bytes = {:#?}, ", bts);
        }
        Value::Int(i) => {
            println!("int = {}, ", i);
        }
        Value::List(list) => {
            println!("list:");
            for v in list {
                print_value(v);
            }
        }
        Value::Dict(dict) => {
            for (k, v) in dict {
                println!(
                    "key = {}, \n=========value is dict:",
                    &std::str::from_utf8(k).unwrap()
                );
                print_value(v);
                println!("=========")
            }
        }
    }
}
