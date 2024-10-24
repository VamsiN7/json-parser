mod tokenizer;
mod parser;

use tokenizer::tokenize;
use parser::parse;

fn main() {
    let json = r#"{"key": "value", "num": 123, "bool": true, "arr": [1, 2, "test"], "nullVal": null}"#;
    
    match tokenize(json) {
        Ok(tokens) => {
            match parse(&tokens) {
                Ok(value) => println!("{:?}", value),
                Err(err) => println!("Error parsing JSON: {}", err),
            }
        }
        Err(err) => println!("Error tokenizing JSON: {}", err),
    }
}