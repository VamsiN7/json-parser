mod tokenizer;
mod parser;

use tokenizer::tokenize;
use parser::{parse, JsonValue};

fn main() {
    let json = r#"{"key": "value", "num": 123, "bool": true, "arr": [1, 2, 3], "nullVal": null}"#;
    let tokens = tokenize(json);
    match parse(&tokens) {
        Ok(value) => println!("{:?}", value),
        Err(err) => println!("Error parsing JSON: {}", err),
    }
}
