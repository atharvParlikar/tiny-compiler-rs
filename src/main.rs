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
    params: Option<Vec<ASTnode>>
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

fn parser(tokens: Vec<Token>) -> Result<ASTnode, String> {
    let mut current: usize = 0;
    fn walk(tokens: &Vec<Token>, current: &mut usize) -> Result<ASTnode, String> {
        let mut token = &tokens[*current];
        match token {
            t if token.type_ == "number" => {
                *current += 1;
                return Ok(ASTnode {
                    type_: "NumberLiteral".to_string(),
                    name: t.value.clone(),
                    params: None
                })
            },
            t if token.type_ == "string" => {
                *current += 1;
                return Ok(ASTnode {
                    type_: "StringLiteral".to_string(),
                    name: t.value.clone(),
                    params: None
                });
            },
            _ if token.type_ == "paren" && token.value == "(" => {
                *current += 1;
                token = &tokens[*current];
                let mut node = ASTnode {
                    type_: "CallExpression".to_string(),
                    name: token.value.clone(),
                    params: Some(Vec::new())
                };

                *current += 1;
                token = &tokens[*current];

                while token.type_ != "paren".to_string() || (token.type_ == "paren" && token.value != ")") {
                    match &mut node.params {
                        Some(params) => {
                            params.push(walk(tokens, current).unwrap());
                            token = &tokens[*current];
                        }
                        None => {}
                    };
                }
                *current += 1;
                return Ok(node);
            },
            _ => {
                return Err(String::from("Invalid character"))
            }
        }
    }
    return walk(&tokens, &mut current);
}

fn print_ast(tree: ASTnode) {
    let mut tabs :usize = 0;
    fn recur(tree: ASTnode, tabs: &mut usize) {
        match tree.params {
            Some(params) => {
                println!("{}{} :- {}", " ".to_string().repeat(*tabs * 4), tree.type_, tree.name);
                *tabs += 1;
                for i in params {
                    recur(i, tabs);
                }
            }
            None => {
                println!("{}{} :- {}", " ".to_string().repeat(*tabs * 4), tree.type_, tree.name);
            }
        }
    }
    recur(tree, &mut tabs);
}

fn main() {
    let code = "(add 4 (substract 3 8))".to_string();
    println!("\nLisp style function calls := {}\n", code);
    let tokens = tokanize(code);
    println!("Abstract syntax tree (shitty representation):");
    print_ast(parser(tokens).unwrap());
}
