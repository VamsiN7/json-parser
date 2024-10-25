mod tokenizer;
mod parser;

use tokenizer::tokenize;
use parser::{parse, JsonValue};

impl JsonValue {
    pub fn pretty_print(&self, indent: usize) -> String {
        match self {
            JsonValue::Object(map) => {
                let mut result = String::new();
                result.push('{');
                if !map.is_empty() {
                    result.push('\n');
                    for (i, (key, value)) in map.iter().enumerate() {
                        if i > 0 {
                            result.push(',');
                            result.push('\n');
                        }
                        result.push_str(&" ".repeat(indent + 2));
                        result.push_str(&format!("\"{}\": {}", key, value.pretty_print(indent + 2)));
                    }
                    result.push('\n');
                    result.push_str(&" ".repeat(indent));
                }
                result.push('}');
                result
            }
            JsonValue::Array(arr) => {
                let mut result = String::new();
                result.push('[');
                if !arr.is_empty() {
                    result.push('\n');
                    for (i, value) in arr.iter().enumerate() {
                        if i > 0 {
                            result.push(',');
                            result.push('\n');
                        }
                        result.push_str(&" ".repeat(indent + 2));
                        result.push_str(&value.pretty_print(indent + 2));
                    }
                    result.push('\n');
                    result.push_str(&" ".repeat(indent));
                }
                result.push(']');
                result
            }
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Null => "null".to_string(),
        }
    }
}

fn main() {
    let json = r#"{"key": "value, "num": 123, "bool": true, "arr": [1, 2, "test"], "nullVal": null}"#;
    
    match tokenize(json) {
        Ok(tokens) => {
            match parse(&tokens) {
                Ok(value) => println!("{}", value.pretty_print(0)),
                Err(err) => println!("Error parsing JSON: {}", err),
            }
        }
        Err(err) => println!("Error tokenizing JSON: {}", err),
    }
}