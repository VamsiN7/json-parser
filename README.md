# json-parser

A custom JSON parser built from scratch in Rust, designed to enhance understanding of JSON parsing fundamentals and Rust's memory safety, concurrency, and performance capabilities. This project takes inspiration from the importance of robust JSON parsing, motivated by the desire to comprehend JSON processing at a lower level and provide a functional, efficient JSON parser.

## Features

- Tokenization: Parses JSON data into tokens, providing a base for understanding the structure of JSON documents.
- Parsing: Transforms tokenized JSON input into a structured, nested data representation.
- Error Handling: Implements error-checking to handle malformed JSON inputs gracefully.
- Support for Complex JSON Structures: Supports various JSON structures, including nested objects, arrays, strings, numbers, booleans, and null values.

## Example

Here's an example JSON input:

```json
{
    "name": "John",
    "age": 25,
    "is_student": true,
    "skills": ["Rust", "Java", "Python"],
    "address": {
        "city": "Boston",
        "zip_code": "01234"
    }
}
```

## Usage

1. Clone the repository:
   ```bash
   git clone <repo_url>
   cd <repo_name>
   ```

2. Run the parser:
   ```bash
   cargo run -- <path_to_json_file>
   ```

3. Test the parser:
   Run the tests to validate functionality:
   ```bash
   cargo test
   ```

## Project Structure

- `main.rs`: Handles command-line interface and user interaction.
- `parser.rs`: Core JSON parsing logic, transforming tokens into data structures.
- `tokenizer.rs`: Tokenizes JSON input for parsing.
- `lib.rs`: Provides a library interface for using the parser as a dependency in other projects.
- `Cargo.toml`: Manages dependencies and configuration for the Rust project.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests for improvements, bug fixes, or feature additions.

---