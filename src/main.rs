use std::fs;

#[derive(Debug)]
enum NodeType {
    String(String),
    Number(u32),
    Object(Object),
    Member(Member),
    Json(Json),
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

const STRING: &str = "STRING";
const NUMBER: &str = "NUMBER";
const L_BRACKET: &str = "{";
const R_BRACKET: &str = "}";
const COMMA: &str = ",";
const COLON: &str = ":";

struct Parser {
    code: String,
    index: usize,
    current_token: Token,
}

impl Parser {
    fn get_next_token(&mut self) -> Option<Token> {
        for current_char in self.code[self.index..self.code.len()].chars() {
            if current_char == ' ' || current_char == '\n' {
                self.index += 1;
                continue;
            }

            // FIXME Need to handle mutiple digit numbers
            if current_char.is_digit(10) {
                self.index += 1;
                return Some(Token {
                    token_type: String::from(NUMBER),
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
                            token_type: String::from(STRING),
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

            if current_char == '{' {
                self.index += 1;
                return Some(Token {
                    token_type: String::from(L_BRACKET),
                    value: String::from(current_char),
                });
            }

            if current_char == '}' {
                self.index += 1;
                return Some(Token {
                    token_type: String::from(R_BRACKET),
                    value: String::from(current_char),
                });
            }

            if current_char == ':' {
                self.index += 1;
                return Some(Token {
                    token_type: String::from(COLON),
                    value: String::from(current_char),
                });
            }

            if current_char == ',' {
                self.index += 1;
                return Some(Token {
                    token_type: String::from(COMMA),
                    value: String::from(current_char),
                });
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
        if self.current_token.token_type == NUMBER {
            let value = self.current_token.value.parse::<u32>().unwrap();
            self.eat(NUMBER);
            return Some(NodeType::Number(value));
        }
        if self.current_token.token_type == STRING {
            let value = self.current_token.value.to_string();
            self.eat(STRING);
            return Some(NodeType::String(value));
        }
        return None;
    }

    fn member(&mut self) -> Option<NodeType> {
        if self.current_token.token_type == STRING {
            let key = self.current_token.value.clone();
            self.eat(STRING);
            self.eat(COLON);
            let value = self.json();
            return Some(NodeType::Member(Member {
                key: key,
                value: Box::new(value),
            }));
        }
        return None;
    }

    fn object(&mut self) -> Option<NodeType> {
        if self.current_token.token_type == L_BRACKET {
            self.eat(L_BRACKET);
            let obj = self.member();
            self.eat(R_BRACKET);
            match obj {
                Some(obj) => return Some(NodeType::Object(Object { members: vec![obj] })),
                None => return None,
            }
        }
        return None;
    }

    fn json(&mut self) -> NodeType {
        match self.unit() {
            Some(unit) => {
                println!("is unit {:?}", unit);
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

        return NodeType::Json(Json {
            value: Box::new(NodeType::Null),
        });
    }

    fn go_down(&mut self) {
        let result = self.json();
        println!("{}", print_json(&result, 0));
    }
}

fn print_json(json: &NodeType, depth: i8) -> String {
    match &*json {
        NodeType::Json(json) => return print_json(&json.value, depth + 1),
        NodeType::Member(value) => {
            return format!(
                "\tkey: \"{}\", value: {}",
                value.key,
                print_json(&value.value, depth + 1)
            );
        }
        NodeType::Object(object) => {
            let mut a = String::from("{\n");
            for mem in &object.members {
                let mem_str = print_json(&mem, 1);
                a = a + &mem_str;
            }
            a = a + &String::from("\n}\n");
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
    let contents =
        fs::read_to_string("./input.json").expect("Something went wrong reading the file");

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
