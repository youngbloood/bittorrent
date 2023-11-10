use serde_json::{self, json, Value};
use std::{collections::HashMap, usize};

// Example: "5:hello" -> "hello"
fn decode_string(encoded_value: &str) -> (usize, serde_json::Value) {
    let colon_index = encoded_value.find(':').unwrap();
    let number = &encoded_value[..colon_index].parse::<usize>().unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + number + 1];
    return (
        encoded_value.len(),
        serde_json::Value::String(string.to_string()),
    );
}

// Example: i52e -> 52
fn decode_integer(encoded_value: &str) -> (usize, serde_json::Value) {
    return (
        encoded_value.len(),
        serde_json::Value::Number(
            encoded_value[1..encoded_value.len() - 1]
                .parse::<i64>()
                .unwrap()
                .into(),
        ),
    );
}

// Example: l5:helloi52ee -> ["hello", 52]
pub fn decode_list(encoded_value: &str) -> (usize, serde_json::Value) {
    let all_len = encoded_value.len();
    let mut content = &encoded_value[1..encoded_value.len() - 1];
    let mut list = Vec::<Value>::new();
    while content.len() != 0 {
        let first = content.chars().next().unwrap();
        // 是字符串类型
        if first.is_digit(10) {
            let colon = content.find(":").unwrap();
            let str_len = content[..colon].parse::<usize>().unwrap();
            let (remove_len, value) = decode_string(&content[..str_len + 1 + colon]);
            content = &content[remove_len..];
            list.append(&mut vec![value]);
            continue;
        }
        // 是数字类型
        if first == 'i' {
            let mut pos = 0;
            for i in content.chars() {
                pos += 1;
                if i.eq(&'e') {
                    break;
                }
            }
            let (remove_len, value) = decode_integer(&content[..pos]);
            content = &content[remove_len..];
            list.append(&mut vec![value]);
            continue;
        }
    }
    return (all_len, json!(list));
}

// fn decode_dict(encoded_value: &str) -> serde_json::Value {
//     // decode dictionaries: Example: d3:foo3:bar5:helloi52ee -> {"foo":"bar","hello":52}
//     let mut content = &encoded_value[1..encoded_value.len() - 1];

//     let mut map = HashMap::new();
//     while content.len() != 0 {
//         let find = |c: &str| -> (usize, serde_json::Value) {
//             // 如果povit==0，则表示非string类型
//             let povit = c.find(":");

//             let match_integer = |c: &str| -> (usize, serde_json::Value) {
//                 let int_reg = regex::Regex::new("i*e").unwrap();
//                 if int_reg.is_match(c) {
//                     let length = int_reg.find(c).unwrap().end();
//                     return (length, decode_bencoded_value(&c[..length]));
//                 }
//                 return (0, json!([]));
//             };

//             match povit {
//                 Some(index) => {
//                     // let length = c[..index].parse::<usize>().unwrap();
//                     let len = c[..index].parse::<usize>();
//                     if len.is_ok() {
//                         let length = len.unwrap();
//                         return (
//                             index + 1 + length,
//                             decode_bencoded_value(&c[..index + 1 + length]),
//                         );
//                     }
//                     return match_integer(&c[..index]);
//                 }
//                 None => {
//                     return match_integer(c);
//                 }
//             }
//         };

//         let (length, key) = find(&content);
//         content = &content[length..];

//         println!("content = {:#?}", content);
//         let first = content.chars().next();
//         println!("first = {:#?}", first);
//         let mut value = serde_json::Value::Null;
//         let mut len;
//         match first {
//             Some('i') => {
//                 value = decode_integer(content);
//                 match value {
//                     String(s) => {
//                         len = s.len() + 2;
//                     }
//                 }
//             }
//             Some('l') => value = decode_list(content),
//             Some('d') => value = decode_dict(content),
//             Some(_usize) => {
//                 if _usize.is_digit(10) {
//                     value = decode_string(content);
//                 }
//             }
//             None => todo!(),
//         }

//         println!("key = {:#?}", key);
//         println!("value = {:#?}\n", value);
//         // println!("content = {:#?}", content);

//         map.insert(key.as_str().unwrap().to_owned(), value);
//     }
//     return json!(map);
// }

// #[allow(dead_code)]
// pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
//     // If encoded_value starts with a digit, it's a number

//     let first = encoded_value.chars().next();
//     match first {
//         Some('i') => return decode_integer(encoded_value),
//         Some('l') => return decode_list(encoded_value),
//         Some('d') => return decode_dict(encoded_value),
//         Some(_usize) => {
//             if _usize.is_digit(10) {
//                 return decode_string(encoded_value);
//             }
//         }
//         None => panic!("Unhandled encoded value: {}", encoded_value),
//     }
//     panic!("Unhandled encoded value: {}", encoded_value);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string() {
        let (size, value) = decode_string("5:hello");
        assert_eq!(7, size);
        assert_eq!("hello", value);
    }

    #[test]
    fn test_decode_integer() {
        let (size, value) = decode_integer("i52e");
        assert_eq!(4, size);
        assert_eq!(52, value);
    }

    #[test]
    fn test_decode_list() {
        struct Case {
            encoded_value: String,
            size: usize,
            value: Value,
        }
        // test case slice
        let cases: &[Case] = &[
            Case {
                encoded_value: String::from("l5:helloi52ee"),
                size: 13,
                value: json!(["hello", 52]),
            },
            Case {
                encoded_value: String::from("li52e5:helloe"),
                size: 13,
                value: json!([52, "hello",]),
            },
            Case {
                encoded_value: String::from("li52ei53e5:helloe"),
                size: 17,
                value: json!([52, 53, "hello",]),
            },
        ];

        for cs in cases {
            let (size, value) = decode_list(cs.encoded_value.as_str());
            assert_eq!(cs.size, size);
            assert_eq!(cs.value, value);
        }
    }
}
