use serde_json::{self, json, Value};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

// MatchStack:
// Match the list or dict length
struct MatchStack {
    raw: String,
    stack: VecDeque<u8>,
}

impl MatchStack {
    fn new(str: &str) -> Self {
        Self {
            raw: str.to_owned(),
            stack: VecDeque::<u8>::new(),
        }
    }

    fn match_len(&mut self) -> usize {
        let chars = self.raw.chars();
        let mut len = 0;
        for i in chars {
            if len == 0 && (i.eq(&'d') || i.eq(&'l')) {
                self.stack.push_back(i as u8);
            }
            if i.eq(&'i') {
                self.stack.push_back(i as u8);
            }
            len += 1;
            if i.eq(&'e') {
                self.stack.pop_back();
                if self.stack.len() == 0 {
                    break;
                }
            }
        }
        return len;
    }
}

pub struct Cell {
    t: u8, // 1:digit; 2:string; 3:list; 4:dict
    raw: String,
    value: Value,
    children: Option<Vec<Cell>>, // 仅list和dict才有该值
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell")
            .field("t", &self.t)
            .field("raw", &self.raw)
            .field("value", &self.value)
            .field("children", &self.children)
            .finish()
    }
}

impl Cell {
    pub fn new(encoded_value: &str) -> Option<Self> {
        if !Self::validate(encoded_value) {
            return None;
        }
        let first = encoded_value.chars().next().unwrap();
        if first.is_digit(10) {
            // 字符串
            return Some(decode_string(encoded_value));
        } else if first.eq(&'i') {
            // 数字
            return Some(decode_integer(encoded_value));
        } else if first.eq(&'l') {
            // 列表
            return Some(decode_list_or_dict(encoded_value));
        } else if first.eq(&'d') {
            //  字典
            return Some(decode_list_or_dict(encoded_value));
        }

        return None;
    }

    // 该函数有问题
    fn validate(encoded_value: &str) -> bool {
        let first = encoded_value.chars().next().unwrap();
        match first {
            'i' | 'l' | 'd' => {
                let mut content = &encoded_value[1..encoded_value.len() - 1];
                while content.len() != 0 {
                    let left_first = content.chars().next().unwrap();
                    if left_first.is_digit(10) {
                        let colon_index = content.find(":").unwrap();
                        let len = content[..colon_index].parse::<usize>().unwrap();
                        let str_content = &content[..colon_index + 1 + len];
                        content = &content[colon_index + 1 + len..];
                        if !Self::validate(str_content) {
                            return false;
                        }
                    }
                    if !Self::validate(content) {
                        return false;
                    }
                }
                return true;
            }
            _ => {
                let colon_index = encoded_value.find(":").unwrap();
                let str_len = encoded_value[..colon_index].parse::<usize>().unwrap();
                return colon_index + 1 + str_len == encoded_value.len();
            }
        }
    }

    fn is_digit(&self) -> bool {
        return self.t == 1;
    }
    fn is_str(&self) -> bool {
        return self.t == 2;
    }
    fn is_list(&self) -> bool {
        return self.t == 3;
    }
    fn is_dict(&self) -> bool {
        return self.t == 4;
    }

    pub fn get_values(&self) -> Value {
        if self.is_list() {
            let mut list = Vec::<Value>::new();
            for child in self.children.as_ref().unwrap() {
                list.append(&mut vec![Self::get_value(child)])
            }
            return serde_json::Value::Array(list);
        }
        if self.is_dict() {
            let mut map = HashMap::<String, Value>::new();
            let mut iter = 0;
            while iter < self.children.as_ref().unwrap().len() - 1 {
                let key = Self::get_value(&self.children.as_ref().unwrap()[iter]);
                let value = Self::get_value(&self.children.as_ref().unwrap()[iter + 1]);
                map.insert(key.to_string(), value);
                iter += 2;
            }
            return json!(map);
        }
        if self.is_str() || self.is_digit() {
            return self.value.clone();
        }
        return json!([]);
    }

    fn get_value(cell: &Cell) -> Value {
        let mut list = Vec::<Value>::new();
        if cell.is_list() || cell.is_dict() {
            for child in cell.children.as_ref().unwrap() {
                list.append(&mut vec![Self::get_value(child)])
            }
            return serde_json::Value::Array(list);
        }
        if cell.is_str() || cell.is_digit() {
            return cell.value.clone();
        }
        return json!([]);
    }
}

// Example: "5:hello" -> "hello"
fn decode_string(encoded_value: &str) -> Cell {
    let colon_index = encoded_value.find(':').unwrap();
    let number = &encoded_value[..colon_index].parse::<usize>().unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + number + 1];

    return Cell {
        t: 1,
        raw: encoded_value.to_owned(),
        value: json!(string),
        children: None,
    };
}

// Example: i52e -> 52
fn decode_integer(encoded_value: &str) -> Cell {
    return Cell {
        t: 2,
        raw: encoded_value.to_owned(),
        value: json!(encoded_value[1..encoded_value.len() - 1]
            .parse::<i64>()
            .unwrap()),
        children: None,
    };
}

// Example: l5:helloi52ee -> ["hello", 52]
fn decode_list_or_dict(encoded_value: &str) -> Cell {
    let typ = encoded_value.chars().next().unwrap();
    let mut content = &encoded_value[1..encoded_value.len() - 1];
    let mut children = Vec::<Cell>::new();
    let mut _t = 0;

    match typ {
        'i' => _t = 1,
        'l' => _t = 3,
        'd' => _t = 4,
        _ => _t = 2,
    }

    while content.len() != 0 {
        let first = content.chars().next().unwrap();
        // 是字符串类型
        if first.is_digit(10) {
            let colon = content.find(":").unwrap();
            let str_len = content[..colon].parse::<usize>().unwrap();
            let child = decode_string(&content[..str_len + 1 + colon]);
            content = &content[child.raw.len()..];
            children.append(&mut vec![child]);
            // values.append(&mut vec![child.value]);
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
            let child = decode_integer(&content[..pos]);
            content = &content[child.raw.len()..];
            children.append(&mut vec![child]);
            continue;
        }
        // 是list类型
        if first == 'l' {
            let list_len = MatchStack::new(&content).match_len();
            let child = decode_list_or_dict(&content[..list_len]);
            content = &content[list_len..];
            children.append(&mut vec![child]);
            continue;
        }
        // 是dict类型
        if first == 'd' {
            let dict_len = MatchStack::new(&content).match_len();
            let child = decode_list_or_dict(&content[..dict_len]);
            content = &content[dict_len..];
            children.append(&mut vec![child]);
            continue;
        }
    }
    return Cell {
        t: _t,
        raw: encoded_value.to_owned(),
        value: json!([]),
        children: Some(children),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string() {
        let result = decode_string("5:hello");
    }

    #[test]
    fn test_decode_integer() {
        let result = decode_integer("i52e");
    }

    #[test]
    fn test_decode_list() {
        let result = decode_list_or_dict("l5:helloi52ee");
    }

    #[test]
    fn test_decode_dict() {
        let result = decode_list_or_dict("d3:fool3:bare5:helloli52eee");

        let result = decode_list_or_dict("d3:foo3:bar5:helloli52eee");
    }

    #[test]
    fn test_cell_new() {
        let list = vec![
            "5:hello",
            "i52e",
            "l5:helloi52ee",
            "d3:foo3:bar5:helloli52eee",
        ];
        for encoded_value in list {
            let cell = Cell::new(encoded_value);
        }
    }

    #[test]
    fn test_cell_new_invalid() {
        struct Case {
            encoded: String,
            valid: bool,
        }
        let list = vec![
            Case {
                encoded: String::from("5:helloi"),
                valid: false,
            },
            Case {
                encoded: String::from("i52ee"),
                valid: false,
            },
            // Case {
            //     encoded: String::from("l5:helloni52ee"),
            //     valid: false,
            // },
            // Case {
            //     encoded: String::from("d3:foo3:bar5:helloli52eeee"),
            //     valid: false,
            // },
            // Case {
            //     encoded: String::from("i52e"),
            //     valid: true,
            // },
        ];
        for cs in list {
            assert_eq!(cs.valid, Cell::new(cs.encoded.as_str()).is_some());
        }
    }

    #[test]
    fn test_cell_get_values() {
        let list = vec![
            "5:hello",
            "i52e",
            "l5:helloi52ee",
            "d3:foo3:bar5:helloli52eee",
        ];
        for encoded_value in list {
            let cell = Cell::new(encoded_value).unwrap();
            let value = cell.get_values();
        }
    }
}
