use std::fs;

#[derive(Debug, Copy, Clone)]
enum NodeType {
    Member,
    Number(u32),
    Boolean(bool),
    Null,
    Array,
}

const NONE: &str = "NONE";
const NUMBER: &str = "NUMBER";

struct Parser {
    code: String,
    index: usize,
    current_token: str,
}

fn get_next_token(parser: &mut Parser) -> Option<NodeType> {
    for current_char in parser.code[parser.index..parser.code.len()].chars() {
        if current_char == ' ' || current_char == '\n' {
            continue;
        }

        if current_char.is_digit(10) {
            return Some(NodeType::Number(current_char.to_digit(10).unwrap()));
        }

        // Skip whitespace + returns
        // Check {}
        // :
        // null
        // true/false
        // string
        // int
        // float

        parser.index += 1;
    }

    return None;
}

fn eat(parser: &mut Parser, node_type: NodeType) -> Result<NodeType, String> {
    let token = get_next_token(parser);

    return match token {
        None => Err(String::from("Could not find a token")),
        _ => {
            if std::mem::discriminant(&token.unwrap()) == std::mem::discriminant(&node_type) {
                Ok(token.unwrap())
            } else {
                Err(String::from("Wrong token type"))
            }
        }
    };

    // return match token {
    //     node_type => Ok(token.unwrap()),
    //     _ => Err(String::from("qsdqsd")),
    // };
}

fn json(parser: &mut Parser) {
    if parser.current_token == NONE {}
    // if std::mem::discriminant(&parser.current_token) == std::mem::discriminant(&NodeType::Null) {}

    // if std::mem::discriminant(&parser.current_token) == std::mem::discriminant(&NodeType::Boolean) {
    // }
}

fn go_down(parser: &mut Parser) {
    // let token = match get_next_token(parser) {
    //     None => return,
    //     _ => (),
    // };
    let token = eat(parser);
    while token.is_some() {
        // eat
        // token = get_next_token(parser);
        println!("{:?}", token)
    }
}

fn main() {
    let contents =
        fs::read_to_string("./input.json").expect("Something went wrong reading the file");

    // println!("{}", contents);

    let mut parser = Parser {
        code: contents,
        index: 0,
    };

    go_down(&mut parser)
}
