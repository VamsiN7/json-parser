#[cfg(test)]
mod tests {
    use crate::tokenizer::{tokenize, Token};

    #[test]
    fn test_simple_json() {
        let json = r#"{"key": "value", "num": 123, "bool": true, "nullVal": null}"#;
        let tokens = tokenize(json);
        assert_eq!(tokens, vec![
            Token::LeftBrace,
            Token::String("key".to_string()), Token::Colon, Token::String("value".to_string()), Token::Comma,
            Token::String("num".to_string()), Token::Colon, Token::Number(123.0), Token::Comma,
            Token::String("bool".to_string()), Token::Colon, Token::True, Token::Comma,
            Token::String("nullVal".to_string()), Token::Colon, Token::Null,
            Token::RightBrace,
        ]);
    }
}
