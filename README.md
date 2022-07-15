# Rusty Json Parser

A **unfinished** toy JSON parser written in Rust.  
It only parses the JSON and prints it, you can't do anything useful with it. The goal of this project was to play around and learn a bit of Rust.  
If you want a real library use https://github.com/serde-rs/json.

## Usage

To parse a file: `cargo run file.json`  
To build `cargo build .`

## Grammar

```
<json> ::= <object> | <array> | <number> | <string> | <boolean> | <null>
<object> ::= '{' [ <member> *(', ' <member>) ] '}'
<member> ::= <string> ': ' <json>
<array> ::= '[' [ <json> *(', ' <json>) ] ']'
```

## Todo

- [ ] Add tests
- [ ] Improve output
- [ ] Add error handling
- [ ] Understand what `Box` does instead of randomly using it to fix errors
- [ ] Make it useful?
