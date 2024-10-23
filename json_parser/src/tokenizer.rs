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
pub fn tokenize(json: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = json.chars().collect();
    let mut i = 0;

    while i < chars.len() {
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
                i += 1; // Skip the opening quote because it is " 
                while i < chars.len() && chars[i] != '"' {
                    s.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::String(s));
            }
            't' => {
                if json[i..].starts_with("true") {
                    tokens.push(Token::True);
                    i += 3; // Skip over 'rue'
                }
            }
            'f' => {
                if json[i..].starts_with("false") {
                    tokens.push(Token::False);
                    i += 4; // Skip over 'alse'
                }
            }
            'n' => {
                if json[i..].starts_with("null") {
                    tokens.push(Token::Null);
                    i += 3; // Skip over 'ull'
                }
            }
            '0'..='9' | '-' => {
                let mut num_str = String::new();
                while i < chars.len() && (chars[i].is_numeric() || chars[i] == '.') {
                    num_str.push(chars[i]);
                    i += 1;
                }
                if let Ok(number) = num_str.parse::<f64>() {
                    tokens.push(Token::Number(number));
                    i -= 1; // Because the outer loop will increment i and we don't want to miss processing a non numeric char 
                }
            }
            ' ' | '\n' | '\t' => {} // Ignore whitespace
            _ => {
                println!("Unexpected character: {}", chars[i]);
            }
        }
        i += 1;
    }
    tokens
}