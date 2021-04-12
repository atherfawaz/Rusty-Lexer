use std::str::Chars;
mod file_reader;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    PLUS(String),
    MINUS(String),
    NUMBER(String),
    IDENTIFIER(String),
    MOD(String),
    MULTIPLY(String),
    LESSTHAN(String),
    GREATERTHAN(String),
    DOT(String),
    COMMA(String),
    OPENPARENTHESES(String),
    CLOSEPARENTHESES(String),
    STARTBRACES(String),
    ENDBRACES(String),
    STARTSQUAREBRACKETS(String),
    ENDSQUAREBRACKETS(String),
    INVERTEDCOMMAS(String),
    APOSTROPHE(String),
    EQUALS(String),
    ASSIGNMENT(String),
    KEYWORD(String),
    REFERENCE(String),
    SEMICOLON(String),
    COLON(String),
    PLUSEQUALS(String),
    MINUSEQUALS(String),
    MULTIPLYEQUALS(String),
    MODEQUALS(String),
    AND(String),
    OR(String),
    NOT(String),
    ABSOLUTEVALUE(String),
    LESSTHANEQUALS(String),
    GREATERTHANEQUALS(String),
    PLUSPLUS(String),
    MINUSMINUS(String),
    SHIFTLEFT(String),
    SHIFTRIGHT(String),
}

struct Lexer<'a> {
    iter: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(code: &str) -> Lexer {
        Lexer { iter: code.chars() }
    }

    fn next(&mut self, mapping: &HashMap<&str, &str>, append: &mut String) -> Option<Token> {
        let mut start: &str = self.iter.as_str();
        let mut index = self.iter.next();
        while let Some(chr) = index {
            if !chr.is_whitespace() {
                break;
            }
            start = self.iter.as_str();
            index = self.iter.next();
        }

        if let Some(chr) = index {
            match chr {
                '.' => Some(Token::DOT(String::from(chr))),
                '!' => Some(Token::NOT(String::from(chr))),
                '+' => {
                    if let Some(c) = self.iter.next() {
                        if c == '=' {
                            Some(Token::PLUSEQUALS(String::from("+=")))
                        } else {
                            Some(Token::PLUS(String::from(chr)))
                        }
                    } else {
                        Some(Token::PLUS(String::from(chr)))
                    }
                }
                '-' => {
                    if let Some(c) = self.iter.next() {
                        if c == '=' {
                            Some(Token::MINUSEQUALS(String::from("-=")))
                        } else {
                            Some(Token::MINUS(String::from(chr)))
                        }
                    } else {
                        Some(Token::MINUS(String::from(chr)))
                    }
                }
                '*' => {
                    if let Some(c) = self.iter.next() {
                        if c == '=' {
                            Some(Token::MULTIPLYEQUALS(String::from("*=")))
                        } else {
                            Some(Token::MULTIPLY(String::from(chr)))
                        }
                    } else {
                        Some(Token::MULTIPLY(String::from(chr)))
                    }
                }
                '%' => {
                    if let Some(c) = self.iter.next() {
                        if c == '=' {
                            Some(Token::MODEQUALS(String::from("%=")))
                        } else {
                            Some(Token::MOD(String::from(chr)))
                        }
                    } else {
                        Some(Token::MOD(String::from(chr)))
                    }
                }
                '<' => {
                    if let Some(c) = self.iter.next() {
                        if c == '<' {
                            Some(Token::SHIFTLEFT(String::from("<<")))
                        } else if c == '=' {
                            Some(Token::LESSTHANEQUALS(String::from("<=")))
                        } else {
                            println!("Found a library import!");
                            *append = String::from(c);
                            Some(Token::LESSTHANEQUALS(String::from(chr)))
                        }
                    } else {
                        Some(Token::LESSTHAN(String::from(chr)))
                    }
                }
                '>' => {
                    if let Some(c) = self.iter.next() {
                        if c == '>' {
                            Some(Token::SHIFTRIGHT(String::from(">>")))
                        } else if c == '=' {
                            Some(Token::GREATERTHANEQUALS(String::from(">=")))
                        } else {
                            Some(Token::GREATERTHAN(String::from(chr)))
                        }
                    } else {
                        Some(Token::GREATERTHAN(String::from(chr)))
                    }
                }
                '#' => {
                    let mut end = self.iter.as_str();
                    while let Some(c) = self.iter.next() {
                        if !c.is_ascii_alphanumeric() && c != '_' {
                            break;
                        }
                        end = self.iter.as_str();
                    }
                    let len = start.len() - end.len();
                    let word = String::from(start[0..len].trim().to_string());
                    if mapping.contains_key(&start[0..len]) {
                        Some(Token::KEYWORD(word))
                    } else {
                        Some(Token::IDENTIFIER(start[0..len].trim().to_string()))
                    }
                }
                ',' => Some(Token::COMMA(String::from(chr))),
                '\'' => Some(Token::APOSTROPHE(String::from(chr))),
                '(' => Some(Token::OPENPARENTHESES(String::from(chr))),
                ')' => Some(Token::CLOSEPARENTHESES(String::from(chr))),
                '[' => Some(Token::STARTSQUAREBRACKETS(String::from(chr))),
                ']' => Some(Token::ENDSQUAREBRACKETS(String::from(chr))),
                '{' => Some(Token::STARTBRACES(String::from(chr))),
                '}' => Some(Token::ENDBRACES(String::from(chr))),
                '"' => Some(Token::INVERTEDCOMMAS(String::from(chr))),
                '&' => {
                    if let Some(c) = self.iter.next() {
                        if c == '&' {
                            Some(Token::AND(String::from("&&")))
                        } else {
                            Some(Token::REFERENCE(String::from(chr)))
                        }
                    } else {
                        Some(Token::REFERENCE(String::from(chr)))
                    }
                }
                '|' => {
                    if let Some(c) = self.iter.next() {
                        if c == '|' {
                            Some(Token::OR(String::from("||")))
                        } else {
                            Some(Token::ABSOLUTEVALUE(String::from(chr)))
                        }
                    } else {
                        Some(Token::ABSOLUTEVALUE(String::from(chr)))
                    }
                }
                ';' => Some(Token::SEMICOLON(String::from(chr))),
                ':' => Some(Token::COLON(String::from(chr))),
                '=' => {
                    if let Some(c) = self.iter.next() {
                        if c == '=' {
                            Some(Token::EQUALS(String::from("=")))
                        } else {
                            Some(Token::ASSIGNMENT(String::from("=")))
                        }
                    } else {
                        Some(Token::ASSIGNMENT(String::from("=")))
                    }
                }
                '0'..='9' => {
                    let mut end = self.iter.as_str();
                    while let Some(c) = self.iter.next() {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        end = self.iter.as_str();
                    }
                    let len = start.len() - end.len();
                    let word = start[0..len].trim().to_string();
                    Some(Token::NUMBER(word))
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut end = self.iter.as_str();
                    while let Some(c) = self.iter.next() {
                        if !c.is_ascii_alphanumeric() && c != '_' {
                            if c != '.' {
                                break;
                            }
                        }
                        end = self.iter.as_str();
                    }
                    let len = start.len() - end.len();
                    let word = String::from(start[0..len].trim().to_string());
                    append.push_str(&word);
                    let temp = String::from(append.as_mut_str());
                    let temp_slice: &str = &temp[..];
                    if mapping.contains_key(&temp_slice) {
                        Some(Token::KEYWORD(temp))
                    } else {
                        Some(Token::IDENTIFIER(start[0..len].trim().to_string()))
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

fn build_tokens(token_vector: Vec<&str>) -> HashMap<&str, &str> {
    let mut token_map = HashMap::new();
    for &elem in &token_vector {
        let one_tok: Vec<&str> = elem.split(",").collect();
        token_map.insert(one_tok[0], one_tok[1]);
    }
    return token_map;
}

fn main() {
    let tokens_str = String::from(file_reader::read_tokens("src/TOKENS.txt"));
    let token_map: HashMap<&str, &str> = build_tokens(tokens_str.split(" ").collect());
    let code = String::from(file_reader::read_source_code("src/source_code.c")).to_owned();
    let code_slice: &str = &code[..];

    let mut generated_tokens = Vec::new();

    let mut lex = Lexer::new(code_slice);
    let mut append = String::from("");
    while let Some(token) = lex.next(&token_map, &mut append) {
        generated_tokens.push(token);
    }

    for i in 0..generated_tokens.len() {
        if generated_tokens[i] == Token::IDENTIFIER(String::from(">")) {
            generated_tokens.push(Token::GREATERTHAN(String::from(">")));
        }
    }

    println!("\nGENERATED TOKENS: \n\n{:?}", generated_tokens);
}
