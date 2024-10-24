#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenize;
    use crate::parser::{parse, JsonValue};
    use std::collections::HashMap;

    #[test]
    fn test_parse_object() {
        let json = r#"{"key": "value", "num": 123}"#;
        let tokens = tokenize(json);
        let parsed = parse(&tokens).unwrap();

        let mut expected_object = HashMap::new();
        expected_object.insert("key".to_string(), JsonValue::String("value".to_string()));
        expected_object.insert("num".to_string(), JsonValue::Number(123.0));

        assert_eq!(parsed, JsonValue::Object(expected_object));
    }

    #[test]
    fn test_parse_array() {
        let json = r#"[1, 2, 3, null, true]"#;
        let tokens = tokenize(json);
        let parsed = parse(&tokens).unwrap();

        assert_eq!(
            parsed,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
                JsonValue::Null,
                JsonValue::Bool(true)
            ])
        );
    }
}
