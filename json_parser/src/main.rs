use tokenizer::tokenize;
use parser::{parse, JsonValue};
use clap::Parser;
use std::fs;
use std::path::Path;

mod tokenizer;
mod parser;
#[derive(Parser, Debug)]
#[command(name = "json_parser")]
#[command(about = "A simple CLI tool to parse and pretty print JSON files", long_about = None)]
struct Cli {
    /// Path to the JSON file to parse
    #[arg(short, long)]
    file: String,

    /// Indentation level for pretty printing
    #[arg(short, long, default_value_t = 2)]
    indent: usize,
}


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
    let cli = Cli::parse();

    let file_path = Path::new(&cli.file);
    if !file_path.exists() {
        eprintln!("Error: File not found: {}", cli.file);
        std::process::exit(1);
    }

    let json_content = fs::read_to_string(file_path).expect("Unable to read file");
    match tokenize(&json_content) {
        Ok(tokens) => match parse(&tokens) {
            Ok(json_value) => {
                let pretty_output = json_value.pretty_print(cli.indent);
                println!("{}", pretty_output);
            }
            Err(e) => {
                eprintln!("Error while parsing JSON: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error while tokenizing JSON: {}", e);
            std::process::exit(1);
        }
    }
}