use std::fs;

// #[derive(Debug)]
// enum NodeType {
//     Member,
//     Number(u32),
//     Boolean(bool),
//     String(String),
//     Null,
//     Array,
// }

#[derive(Debug)]
struct Token<'a> {
    token_type: &'a str,
    value: String,
}

#[derive(Debug)]
struct Node<T> {
    token_type: String,
    value: T,
}

// #[derive(Debug)]
// enum NodeType {
//     Number(Node<u32>),
//     String(Node<String>),
// }

struct JsonNumber {}

struct JsonString {}

// const NONE: &str = "NONE";
const STRING: &str = "STRING";
const NUMBER: &str = "NUMBER";

struct Parser<'a> {
    code: String,
    index: usize,
    current_token: Token<'a>,
}

impl Parser<'_> {
    fn get_next_token(&mut self) -> Token {
        for current_char in self.code[self.index..self.code.len()].chars() {
            println!("{}", current_char);
            if current_char == ' ' || current_char == '\n' {
                continue;
            }

            if current_char.is_digit(10) {
                // return (NodeType::Number(current_char.to_digit(10).unwrap()), NUMBER);
                return Token {
                    token_type: NUMBER,
                    value: String::from(current_char),
                };
            }

            // if current_char == '"' {
            //     let mut new_string = String::new();
            //     for looped_char in self.code[self.index..self.code.len()].chars() {
            //         if looped_char == '"' {
            //             self.index += 1;
            //             return (NodeType::String(new_string), STRING);
            //         } else if looped_char == '\n' {
            //             panic!("Wront string format")
            //         } else {
            //             new_string.push(looped_char);
            //         }
            //     }
            // }

            // Skip whitespace + returns
            // Check {}
            // :
            // null
            // true/false
            // string
            // int
            // float

            self.index += 1;
        }

        panic!("oh no")
    }

    fn eat(&mut self, node_type: &str) {
        self.current_token = self.get_next_token();

        if self.current_token.token_type == node_type {
        } else {
            panic!(
                "Wrong token type! Expected: {}, got: {}",
                node_type, self.current_token.token_type
            )
        }

        // if matches!(self.current_token, node_type) {} // == std::mem::discriminant(&node_type) {}

        // return match self.current_token {
        //     None => Err(String::from("Could not find a token")),
        //     _ => {
        //         if token.unwrap() == node_type {
        //             Ok(token.unwrap())
        //         } else {
        //             Err(String::from("Wrong token type"))
        //         }
        //     }
        // };

        // return match token {
        //     node_type => Ok(token.unwrap()),
        //     _ => Err(String::from("qsdqsd")),
        // };
    }

    // fn json(&mut self) {
    //     self.eat(NONE);
    //     if self.current_token == NONE {}
    //     // if std::mem::discriminant(&parser.current_token) == std::mem::discriminant(&NodeType::Null) {}

    //     // if std::mem::discriminant(&parser.current_token) == std::mem::discriminant(&NodeType::Boolean) {
    //     // }
    // }

    fn go_down(&mut self) {
        self.eat(NUMBER);
        let node = Node {
            value: self.current_token.value.to_ascii_lowercase(),
            token_type: String::from(self.current_token.token_type),
        };
        // let token = match get_next_token(parser) {
        //     None => return,
        //     _ => (),
        // };
        // let token = eat(parser);
        // while token.is_some() {
        //     // eat
        //     // token = get_next_token(parser);
        //     println!("{:?}", token)
        // }
    }
}

fn main() {
    let contents =
        fs::read_to_string("./input.json").expect("Something went wrong reading the file");

    // println!("{}", contents);

    let mut parser = Parser {
        code: contents,
        index: 0,
        current_token: Token {
            token_type: String::from(""),
            value: String::from(""),
        },
    };

    parser.go_down()
}
