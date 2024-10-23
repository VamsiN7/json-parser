mod tokenizer;

use tokenizer::tokenize;

fn main(){
    let json = r#"{"key": "value"}"#;
    let tokens = tokenize(json);
    println!("{:?}", tokens);
}