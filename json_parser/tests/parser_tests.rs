#[cfg(test)]
mod tests {
    use json_parser::tokenizer::tokenize;
    use json_parser::parser::{parse, JsonValue};
    use std::collections::HashMap;

    #[test]
    fn test_valid_simple_object() {
        let json = r#"{"key": "value"}"#;
        let tokens = tokenize(json).expect("Tokenization failed");
        let parsed_value = parse(&tokens).expect("Parsing failed");

        let mut expected_object = HashMap::new();
        expected_object.insert("key".to_string(), JsonValue::String("value".to_string()));
        assert_eq!(parsed_value, JsonValue::Object(expected_object));
    }

    #[test]
    fn test_valid_complex_object() {
        let json = r#"{"key1": "value1", "key2": 42, "key3": true, "key4": null, "key5": [1, "two", false]}"#;
        let tokens = tokenize(json).expect("Tokenization failed");
        let parsed_value = parse(&tokens).expect("Parsing failed");

        let mut expected_object = HashMap::new();
        expected_object.insert("key1".to_string(), JsonValue::String("value1".to_string()));
        expected_object.insert("key2".to_string(), JsonValue::Number(42.0));
        expected_object.insert("key3".to_string(), JsonValue::Bool(true));
        expected_object.insert("key4".to_string(), JsonValue::Null);
        expected_object.insert(
            "key5".to_string(),
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::String("two".to_string()),
                JsonValue::Bool(false),
            ]),
        );
        assert_eq!(parsed_value, JsonValue::Object(expected_object));
    }

    #[test]
    fn test_valid_array() {
        let json = r#"["item1", 2, false, null, {"nested_key": "nested_value"}]"#;
        let tokens = tokenize(json).expect("Tokenization failed");
        let parsed_value = parse(&tokens).expect("Parsing failed");

        let mut expected_object = HashMap::new();
        expected_object.insert("nested_key".to_string(), JsonValue::String("nested_value".to_string()));

        let expected_array = JsonValue::Array(vec![
            JsonValue::String("item1".to_string()),
            JsonValue::Number(2.0),
            JsonValue::Bool(false),
            JsonValue::Null,
            JsonValue::Object(expected_object),
        ]);
        assert_eq!(parsed_value, expected_array);
    }

    #[test]
    fn test_unterminated_string() {
        let json = r#"{"key": "unterminated}"#;
        let result = tokenize(json);
        let error_position = 22;
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.message, format!("Unterminated string starting at position {}", error_position));
            assert_eq!(err.position, error_position);
        }
    }

    #[test]
    fn test_missing_comma_in_object() {
        let json = r#"{"key1": "value1" "key2": "value2"}"#;
        let tokens = tokenize(json).expect("Tokenization failed");
        let result = parse(&tokens);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.message, "Expected ',' or '}'");
            assert_eq!(err.position, 5); //5th token should've been ","
        }
    }

    #[test]
    fn test_invalid_number_format() {
        let json = r#"{"key": 12.34.56}"#;
        let result = tokenize(json);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.message, "Invalid number format");
            assert_eq!(err.position, 13);
        }
    }

    #[test]
    fn test_unexpected_character() {
        let json = r#"{"key": @}"#;
        let result = tokenize(json);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.message, "Unexpected character");
            assert_eq!(err.position, 8);
        }
    }

    #[test]
    fn test_empty_object() {
        let json = r#"{}"#;
        let tokens = tokenize(json).expect("Tokenization failed");
        let parsed_value = parse(&tokens).expect("Parsing failed");
        assert_eq!(parsed_value, JsonValue::Object(HashMap::new()));
    }

    #[test]
    fn test_empty_array() {
        let json = r#"[]"#;
        let tokens = tokenize(json).expect("Tokenization failed");
        let parsed_value = parse(&tokens).expect("Parsing failed");
        assert_eq!(parsed_value, JsonValue::Array(Vec::new()));
    }
}
