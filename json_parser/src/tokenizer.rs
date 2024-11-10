use crate::parser::ParseError;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

// convert json string into a list of tokens
pub fn tokenize(json: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = json.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // println!("At {}th position the char is {} ", i, chars[i]);
        match chars[i] {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '"' => {
                // Handle strings
                let mut s = String::new();
                i += 1; // Skip the opening quote

                while i < chars.len() {
                    match chars[i] {
                        '\\' => {
                            // Handle escape sequences
                            i += 1;
                            if i < chars.len() {
                                match chars[i] {
                                    '"' => s.push('"'),
                                    '\\' => s.push('\\'),
                                    'n' => s.push('\n'),
                                    't' => s.push('\t'),
                                    'r' => s.push('\r'),
                                    _ => {
                                        return Err(ParseError::new(
                                            &format!(
                                                "Invalid escape sequence '\\{}' at position {}",
                                                chars[i], i
                                            ),
                                            i,
                                        ));
                                    }
                                }
                            } else {
                                return Err(ParseError::new(
                                    "Unfinished escape sequence at end of string",
                                    i,
                                ));
                            }
                        }
                        '"' => break, // End of string
                        _ => s.push(chars[i]),
                    }
                    i += 1;
                }

                if i >= chars.len() || chars[i] != '"' {
                    // Unterminated string
                    return Err(ParseError::new(
                        &format!("Unterminated string starting at position {}", i),
                        i,
                    ));
                }
                tokens.push(Token::String(s));
            }
            't' => {
                if json[i..].starts_with("true") {
                    tokens.push(Token::True);
                    i += 3; // Skip over 'rue'
                } else {
                    return Err(ParseError::new("Invalid character sequence", i));
                }
            }
            'f' => {
                if json[i..].starts_with("false") {
                    tokens.push(Token::False);
                    i += 4; // Skip over 'alse'
                } else {
                    return Err(ParseError::new(
                        &format!(
                            "Invalid character sequence starting with '{}' at position {}",
                            chars[i], i
                        ),
                        i,
                    ));
                }
            }
            'n' => {
                if json[i..].starts_with("null") {
                    tokens.push(Token::Null);
                    i += 3; // Skip over 'ull'
                } else {
                    return Err(ParseError::new("Invalid character sequence", i));
                }
            }
            '0'..='9' | '-' => {
                let mut num_str = String::new();
                let mut has_decimal = false;

                while i < chars.len()
                    && (chars[i].is_numeric() || chars[i] == '.' || chars[i] == '-')
                {
                    if chars[i] == '.' {
                        if has_decimal {
                            return Err(ParseError::new("Invalid number format", i));
                        }
                        has_decimal = true;
                    }
                    num_str.push(chars[i]);
                    i += 1;
                }

                if let Ok(number) = num_str.parse::<f64>() {
                    tokens.push(Token::Number(number));
                    i -= 1; // Because the outer loop will increment i, so we do not want to miss processing the next character.
                } else {
                    return Err(ParseError::new("Invalid number format", i - num_str.len()));
                }
            }
            ' ' | '\n' | '\t' => {} // Ignore whitespace
            _ => {
                return Err(ParseError::new("Unexpected character", i));
            }
        }
        i += 1;
    }
    Ok(tokens)
}
