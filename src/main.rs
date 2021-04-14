use std::str::Chars;
mod file_reader;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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
    SHIFTLEFT(String),
    SHIFTRIGHT(String),
    STARTSTAR(String),
    STRING(String),
}

struct Lexer<'a> {
    iter: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(code: &str) -> Lexer {
        Lexer { iter: code.chars() }
    }

    fn next(
        &mut self,
        mapping: &HashMap<&str, &str>,
        append: &mut String,
        token_map: &mut Vec<Token>,
    ) -> Option<Token> {
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
                ',' => Some(Token::COMMA(String::from(chr))),
                '\'' => Some(Token::APOSTROPHE(String::from(chr))),
                '(' => Some(Token::OPENPARENTHESES(String::from(chr))),
                ')' => Some(Token::CLOSEPARENTHESES(String::from(chr))),
                '[' => Some(Token::STARTSQUAREBRACKETS(String::from(chr))),
                ']' => Some(Token::ENDSQUAREBRACKETS(String::from(chr))),
                '{' => Some(Token::STARTBRACES(String::from(chr))),
                '}' => Some(Token::ENDBRACES(String::from(chr))),
                ';' => Some(Token::SEMICOLON(String::from(chr))),
                ':' => Some(Token::COLON(String::from(chr))),
                '"' => {
                    token_map.push(Token::INVERTEDCOMMAS(String::from("\"")));
                    let mut end = self.iter.as_str();
                    while let Some(c) = self.iter.next() {
                        if c == '\"' {
                            break;
                        }
                        end = self.iter.as_str();
                    }
                    let len = start.len() - end.len();
                    let word = String::from(start[1..len].trim().to_string());
                    append.push_str(&word);
                    let temp = String::from(append.as_mut_str());
                    append.clear();
                    token_map.push(Token::STRING(temp));
                    Some(Token::INVERTEDCOMMAS(String::from("\"")))
                }
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
                        } else if c == '*' {
                            Some(Token::STARTSTAR(String::from("**")))
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
                            *append = String::from(c);
                            Some(Token::LESSTHAN(String::from(chr)))
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
                    let mut checker = false;
                    let mut end = self.iter.as_str();
                    while let Some(c) = self.iter.next() {
                        if !c.is_ascii_digit() {
                            if c == ';' {
                                checker = true;
                                let len = start.len() - end.len();
                                let word = String::from(start[0..len].trim().to_string());
                                append.push_str(&word);
                                let temp = String::from(append.as_mut_str());
                                append.clear();
                                let temp_slice: &str = &temp[..];
                                if mapping.contains_key(&temp_slice) {
                                    token_map.push(Token::KEYWORD(temp));
                                } else {
                                    token_map.push(Token::NUMBER(temp));
                                }
                            }
                            break;
                        }
                        end = self.iter.as_str();
                    }
                    if !checker {
                        let len = start.len() - end.len();
                        let word = start[0..len].trim().to_string();
                        Some(Token::NUMBER(word))
                    } else {
                        Some(Token::SEMICOLON(String::from(";")))
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut library = false;
                    let mut end = self.iter.as_str();
                    while let Some(c) = self.iter.next() {
                        if !c.is_ascii_alphanumeric() && c != '_' {
                            if c != '.' {
                                if c == '>' {
                                    library = true;
                                    let len = start.len() - end.len();
                                    let word = String::from(start[0..len].trim().to_string());
                                    append.push_str(&word);
                                    let temp = String::from(append.as_mut_str());
                                    append.clear();
                                    let temp_slice: &str = &temp[..];
                                    if mapping.contains_key(&temp_slice) {
                                        token_map.push(Token::KEYWORD(temp));
                                    }
                                }
                                break;
                            }
                        }
                        end = self.iter.as_str();
                    }
                    if !library {
                        let len = start.len() - end.len();
                        let word = String::from(start[0..len].trim().to_string());
                        append.push_str(&word);
                        let temp = String::from(append.as_mut_str());
                        append.clear();
                        let temp_slice: &str = &temp[..];
                        if mapping.contains_key(&temp_slice) {
                            if library == true {
                                Some(Token::GREATERTHAN(String::from(">")))
                            } else {
                                Some(Token::KEYWORD(temp))
                            }
                        } else {
                            Some(Token::IDENTIFIER(start[0..len].trim().to_string()))
                        }
                    } else {
                        Some(Token::GREATERTHAN(String::from(">")))
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
    while let Some(token) = lex.next(&token_map, &mut append, &mut generated_tokens) {
        generated_tokens.push(token);
    }

    /*
    for i in 0..generated_tokens.len() {
        match &generated_tokens[i] {
            Token::KEYWORD(val) => {
                println!("{}", val);
            },
            _=> continue,
        }
    }
    */

    println!("\nGENERATED TOKENS\n {:?}", generated_tokens);
}
