#![warn(dead_code)]
use regex::Regex;

struct Token {
    type_: String,
    value: String 
}

#[derive(Default)]
struct ASTnode {
    type_: String,
    name: String,
    params: Option<Vec<Token>>
}

fn tokanize(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current = 0;
    let number_pattern = Regex::new(r"[0-9]").unwrap();
    let alphabet_pattern = Regex::new(r"[a-z]").unwrap();
    while current < code.len() {
        let char = code.chars().nth(current).unwrap();
        match char {
            '(' => {
                tokens.push(Token {
                    type_: "paren".to_string(),
                    value: "(".to_string()
                });
                current += 1;
            },
            ')' => {
                tokens.push(Token {
                    type_: "paren".to_string(),
                    value: ")".to_string()
                });
                current += 1;
            },
            char if number_pattern.is_match(&char.to_string().as_str()) => {
                let mut number: String = "".to_string();
                number.push(char);
                while current + 1 < code.len() && number_pattern.is_match(code.chars().nth(current + 1).unwrap().to_string().as_str()) {
                    number.push(code.chars().nth(current + 1).unwrap());
                    current += 1;
                }
                tokens.push(Token {
                    type_: "number".to_string(),
                    value: number
                });
                current += 1;
            },
            '"' => {
                current += 1;
                let mut string = "".to_string();
                while current < code.len() && code.chars().nth(current).unwrap() != '"' {
                    string.push(code.chars().nth(current).unwrap());
                    current += 1;
                }
                tokens.push(Token {
                    type_: "string".to_string(),
                    value: string
                });
                current += 1;
            },
            char if alphabet_pattern.is_match(code.chars().nth(current).unwrap().to_string().as_str()) => {
                let mut name = "".to_string();
                name.push(char);
                while current + 1 < code.len() && alphabet_pattern.is_match(code.chars().nth(current + 1).unwrap().to_string().as_str()) {
                    name.push(code.chars().nth(current + 1).unwrap());
                    current += 1;
                }
                tokens.push(Token {
                    type_: "name".to_string(),
                    value: name
                });
                current += 1;
            },
            _ => current += 1
        }
    }
    return tokens;
}

fn parser(tokens: Vec<Token>) {
    let mut current = 0;
    let walk = || -> Result<ASTnode, String> {
        let mut token = &tokens[current];
        match token {
            t if token.type_ == "number" => {
                current += 1;
                return Ok(ASTnode {
                    type_: "NumberLiteral".to_string(),
                    name: t.value.clone(),
                    params: None
                })
            },
            t if token.type_ == "string" => {
                current += 1;
                return Ok(ASTnode {
                    type_: "StringLiteral".to_string(),
                    name: t.value.clone(),
                    params: None
                });
            },
            _ if token.type_ == "param" && token.value == "(" => {
                current += 1;
                token = &tokens[current];
                let node = ASTnode {
                    type_: "CallExpression".to_string(),
                    name: token.value.clone(),
                    params: Some(Vec::new())
                };

                current += 1;
                token = &tokens[current];

                while token.type_ != "paran".to_string() || (token.type_ == "param" && token.value != ")") {
                    // TODO := find a way to implement this walk recursion as a closure cannot be
                    //         recursively called.
                    node.params.unwrap().push(walk());
                }

                return Ok(node);
            }
            _ => {
                return Err(String::from("Invalid character"))
            }
        }
    };
}

fn main() {
    let tokens = tokanize(" 1234 (keyword\"a string\")".to_string());
    let mut token_str = "".to_string();
    for i in tokens {
        token_str.push_str(format!("{}, ", i.value.as_str()).as_str());
    }
    println!("[{}]", &token_str[0..token_str.chars().count() - 2]);
}
