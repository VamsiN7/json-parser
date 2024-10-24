use std::collections::HashMap;
use crate::tokenizer::{tokenize, Token};

/// Object: A JSON object is represented by a HashMap<String, JsonValue>.
/// Array: A JSON array is represented by a Vec<JsonValue>.
/// String, Number, Bool, Null: Represent JSON primitive types.
#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

pub fn parse(tokens : &[Token]) -> Result<JsonValue, String> {
    let mut i = 0;
    parse_value(tokens, &mut i)
}

fn parse_object(tokens: &[Token], i: &mut usize) -> Result<JsonValue, String> {
    let mut object = HashMap::new();
    *i += 1; // skip the '{' token

    loop {
        match tokens.get(*i) {
            Some(Token::String(key)) => {
                *i += 1; // skip the string (key) token
                match tokens.get(*i) {
                    Some(Token::Colon) => {
                        *i += 1; // skip the ':' token
                        let value = parse_value(tokens, i)?;
                        object.insert(key.clone(), value);
                        match tokens.get(*i) {
                            Some(Token::Comma) => *i += 1, // skip the ',' token and continue
                            Some(Token::RightBrace) => {
                                *i += 1; // skip the '}' token and end
                                return Ok(JsonValue::Object(object));
                            }
                            _ => return Err(format!("Expected ',' or '}}' at position {}", *i)),
                        }
                    }
                    _ => return Err(format!("Expected ':' after key at position {}", *i)),
                }
            }
            Some(Token::RightBrace) => {
                *i += 1; // skip the '}' token
                return Ok(JsonValue::Object(object));
            }
            _ => return Err(format!("Unexpected token in object at position {}", *i)),
        }
    }
}

fn parse_value(tokens: &[Token], i: &mut usize) -> Result<JsonValue, String> {
    match tokens.get(*i) {
        Some(Token::LeftBrace) => parse_object(tokens, i),
        Some(Token::LeftBracket) => parse_array(tokens, i),
        Some(Token::String(s)) => {
            *i += 1; // Consume the string token
            Ok(JsonValue::String(s.clone()))
        }
        Some(Token::Number(n)) => {
            *i += 1; 
            Ok(JsonValue::Number(*n))
        }
        Some(Token::True) => {
            *i += 1;
            Ok(JsonValue::Bool(true))
        }
        Some(Token::False) => {
            *i += 1; 
            Ok(JsonValue::Bool(false))
        }
        Some(Token::Null) => {
            *i += 1;
            Ok(JsonValue::Null)
        }
        _ => Err(format!("Unexpected token at position {}", *i)),
    }
}

fn parse_array(tokens: &[Token], i: &mut usize) -> Result<JsonValue, String> {
    let mut array = Vec::new();
    *i += 1; // Consume the '[' token

    loop {
        match tokens.get(*i) {
            Some(Token::RightBracket) => {
                *i += 1; // Consume the ']' token
                return Ok(JsonValue::Array(array));
            }
            _ => {
                let value = parse_value(tokens, i)?;
                array.push(value);
                match tokens.get(*i) {
                    Some(Token::Comma) => *i += 1, // Consume the ',' token and continue
                    Some(Token::RightBracket) => {
                        *i += 1; // Consume the ']' token and end
                        return Ok(JsonValue::Array(array));
                    }
                    _ => return Err(format!("Expected ',' or ']' at position {}", *i)),
                }
            }
        }
    }
}


