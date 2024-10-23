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
                i += 1; // Skip the opening quote
                while i < chars.len() && chars[i] != '"' {
                    s.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::String(s));
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
