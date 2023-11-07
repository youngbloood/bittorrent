use serde_json::{self, json};
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    if encoded_value.chars().next().unwrap().is_digit(10) {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        return serde_json::Value::String(string.to_string());
    } else if encoded_value.starts_with("i") && encoded_value.ends_with("e") {
      let number: &str =   encoded_value.trim_start_matches("i").trim_end_matches("e");
      return serde_json::Value::Number(number.parse::<i64>().unwrap().into());
    } else if encoded_value.starts_with("l") && encoded_value.ends_with("e") {
      let content  = &encoded_value[1..encoded_value.len()-1];
      let povit = content.find(":").unwrap();
      let length = content[..povit].parse::<usize>().unwrap();

      let first = decode_bencoded_value(&content[..povit+1+length]);
      let second = decode_bencoded_value(&content[povit+1+length..]);

      return json!([first,second]);
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}


// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
      let encoded_value=&args[2];
      let decoded_value = decode_bencoded_value(encoded_value);
      println!("{}",decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
