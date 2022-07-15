use std::env;
use std::fs;

pub mod tokens;

#[derive(Debug)]
enum NodeType {
    String(String),
    Number(u32),
    Object(Object),
    Member(Member),
    Json(Json),
    Array(Vec<NodeType>),
    Null,
}

#[derive(Debug)]
struct Token {
    token_type: String,
    value: String,
}

#[derive(Debug)]
struct Json {
    value: Box<NodeType>,
}

#[derive(Debug)]
struct Member {
    key: String,
    value: Box<NodeType>,
}

#[derive(Debug)]
struct Object {
    members: Vec<NodeType>,
}

struct Parser {
    code: String,
    index: usize,
    current_token: Token,
}

impl Parser {
    pub fn get_next_token(&mut self) -> Option<Token> {
        for current_char in self.code[self.index..self.code.len()].chars() {
            if current_char == ' ' || current_char == '\n' {
                self.index += 1;
                continue;
            }

            // FIXME Need to handle mutiple digit numbers
            if current_char.is_digit(10) {
                self.index += 1;
                return Some(Token {
                    token_type: String::from(tokens::NUMBER),
                    value: String::from(current_char),
                });
            }

            if current_char == '"' {
                let mut new_string = String::new();
                self.index += 1;
                for looped_char in self.code[self.index..self.code.len()].chars() {
                    if looped_char == '"' {
                        self.index += 1;
                        return Some(Token {
                            token_type: String::from(tokens::STRING),
                            value: String::from(new_string),
                        });
                    } else if looped_char == '\n' {
                        panic!("Wrong string format")
                    } else {
                        self.index += 1;
                        new_string.push(looped_char);
                    }
                }
            }

            for token in [
                tokens::L_CURLY,
                tokens::R_CURLY,
                tokens::L_BRACKET,
                tokens::R_BRACKET,
                tokens::COLON,
                tokens::COMMA,
            ] {
                if String::from(current_char) == token {
                    self.index += 1;
                    return Some(Token {
                        token_type: String::from(current_char),
                        value: String::from(current_char),
                    });
                }
            }
        }

        return None;
    }

    fn eat(&mut self, node_type: &str) {
        if self.current_token.token_type == node_type {
            let token = self.get_next_token();
            match token {
                Some(token) => self.current_token = token,
                // None => panic!("Invalid syntax: {}", &self.code[self.index..self.index + 1]),
                None => (),
            }
        } else {
            panic!(
                "Wrong token type! Expected: {}, got: {}",
                node_type, self.current_token.token_type
            );
        }
    }

    fn unit(&mut self) -> Option<NodeType> {
        if self.current_token.token_type == tokens::NUMBER {
            let value = self.current_token.value.parse::<u32>().unwrap();
            self.eat(tokens::NUMBER);
            return Some(NodeType::Number(value));
        }
        if self.current_token.token_type == tokens::STRING {
            let value = self.current_token.value.to_string();
            self.eat(tokens::STRING);
            return Some(NodeType::String(value));
        }
        return None;
    }

    fn member(&mut self) -> Option<NodeType> {
        if self.current_token.token_type == tokens::STRING {
            let key = self.current_token.value.clone();
            self.eat(tokens::STRING);
            self.eat(tokens::COLON);
            let value = self.json();
            return Some(NodeType::Member(Member {
                key: key,
                value: Box::new(value),
            }));
        }
        return None;
    }

    fn array(&mut self) -> Option<Vec<NodeType>> {
        if self.current_token.token_type == tokens::L_BRACKET {
            let mut arr = vec![];
            self.eat(tokens::L_BRACKET);
            loop {
                let json = self.json();
                arr.push(json);
                if self.current_token.token_type == tokens::R_BRACKET {
                    break;
                }
                self.eat(tokens::COMMA);
            }
            self.eat(tokens::R_BRACKET);
            return Some(arr);
        }
        return None;
    }

    fn object(&mut self) -> Option<NodeType> {
        if self.current_token.token_type == tokens::L_CURLY {
            let mut arr = vec![];
            self.eat(tokens::L_CURLY);

            loop {
                match self.member() {
                    Some(member) => arr.push(member),
                    None => (),
                }
                if self.current_token.token_type == tokens::R_CURLY {
                    break;
                }
                self.eat(tokens::COMMA);
            }
            self.eat(tokens::R_CURLY);
            return Some(NodeType::Object(Object { members: arr }));
        }
        return None;
    }

    fn json(&mut self) -> NodeType {
        match self.unit() {
            Some(unit) => {
                return NodeType::Json(Json {
                    value: Box::new(unit),
                });
            }
            None => (),
        }

        match self.object() {
            Some(object) => {
                return NodeType::Json(Json {
                    value: Box::new(object),
                })
            }
            None => (),
        }

        match self.array() {
            Some(array) => {
                return NodeType::Json(Json {
                    value: Box::new(NodeType::Array(array)),
                })
            }
            None => (),
        }

        return NodeType::Json(Json {
            value: Box::new(NodeType::Null),
        });
    }

    fn go_down(&mut self) {
        let result = self.json();
        println!("{}", print_json(&result, 0));
    }
}

fn print_json(json: &NodeType, depth: usize) -> String {
    match &*json {
        NodeType::Json(json) => return print_json(&json.value, depth + 1),
        NodeType::Member(value) => {
            return format!(
                "{}\"{}\": {}",
                "  ".repeat(depth),
                value.key,
                print_json(&value.value, depth + 1)
            );
        }
        NodeType::Object(object) => {
            let mut a = String::from("{\n");
            for mem in &object.members {
                let mem_str = print_json(&mem, depth);
                a = a + &mem_str + ",\n";
            }
            a = a + &String::from(format!("{}}}\n", "  ".repeat(depth - 1)));
            return a;
        }
        NodeType::Array(array) => {
            let mut a = String::from("[");
            for item in array {
                a = a + &print_json(&item, depth) + ",";
            }
            a = a + &String::from("]");
            return a;
        }
        NodeType::String(string) => {
            return format!("\"{}\"", string.to_string());
        }
        NodeType::Number(num) => {
            return num.to_string();
        }
        _ => return String::from(""),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Missing filename.\nUsage: cargo run file.json");
        return;
    }

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut parser = Parser {
        code: contents,
        index: 0,
        current_token: Token {
            token_type: String::from(""),
            value: String::from(""),
        },
    };

    let token = parser.get_next_token();
    match token {
        Some(token) => parser.current_token = token,
        None => panic!("Invalid syntax"),
    }
    parser.go_down()
}
