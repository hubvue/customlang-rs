use std::{collections::HashMap, str::Chars};
mod token;
pub use token::{Token, TokenType};

lazy_static::lazy_static! {
  static ref KEYSWORDS: HashMap<String, TokenType> = {
    let mut keywords = HashMap::new();
    keywords.insert("let".to_string(), TokenType::Let);
    keywords.insert("const".to_string(), TokenType::Const);
    keywords.insert("fn".to_string(), TokenType::Fn);
    keywords.insert("if".to_string(), TokenType::If);
    keywords.insert("for".to_string(), TokenType::For);
    keywords.insert("while".to_string(), TokenType::While);

    keywords
  };
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    cur_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut L = Self {
            source,
            chars: source.chars(),
            cur_char: None,
        };
        L.cur_char = L.chars.next();
        L
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            } else {
                let offset = self.offset();
                tokens.push(Token::new(TokenType::EOF, "EOF", offset, offset));
                break;
            }
        }
        tokens
    }
    fn next_token(&mut self) -> Option<Token> {
        let start = self.offset();
        if let Some((kind, value)) = self.next_kind() {
            let end = self.offset();
            return Some(Token::new(kind, value, start, end));
        }
        None
    }
    fn next_kind(&mut self) -> Option<(TokenType, String)> {
        while let Some(c) = self.cur() {
            let kind: TokenType;
            let value: String;
            match c {
                '(' => {
                    kind = TokenType::OpenParen;
                    value = c.to_string();
                    self.next();
                }
                ')' => {
                    kind = TokenType::CloseParen;
                    value = c.to_string();
                    self.next();
                }
                '[' => {
                    kind = TokenType::OpenBracket;
                    value = c.to_string();
                    self.next();
                }
                ']' => {
                    kind = TokenType::CloseBracket;
                    value = c.to_string();
                    self.next();
                }
                '{' => {
                    kind = TokenType::OpenBrace;
                    value = c.to_string();
                    self.next();
                }
                '+' | '-' | '*' | '/' | '%' => {
                    kind = TokenType::BinaryOperator;
                    value = c.to_string();
                    self.next();
                }
                '=' => {
                    kind = TokenType::Equals;
                    value = c.to_string();
                    self.next();
                }
                ';' => {
                    kind = TokenType::Semicolon;
                    value = c.to_string();
                    self.next();
                }
                ',' => {
                    kind = TokenType::Commoa;
                    value = c.to_string();
                    self.next();
                }
                ':' => {
                    kind = TokenType::Colon;
                    value = c.to_string();
                    self.next();
                }
                '.' => {
                    kind = TokenType::Dot;
                    value = c.to_string();
                    self.next();
                }
                ' ' | '\n' | '\t' | '\r' => {
                    self.next();
                    continue;
                }
                _ => {
                    if is_number(c) {
                        let mut num = String::new();
                        while let Some(c) = self.cur() {
                            if is_number(c) {
                                num.push(c);
                                self.next();
                            } else {
                                break;
                            }
                        }
                        kind = TokenType::Number;
                        value = num;
                    } else if is_ident(c) {
                        let mut ident = String::new();
                        while let Some(c) = self.cur() {
                            if is_ident(c) {
                                ident.push(c);
                                self.next();
                            } else {
                                break;
                            }
                        }
                        if let Some(&k) = KEYSWORDS.get(&ident) {
                            kind = k;
                        } else {
                            kind = TokenType::Identifier;
                        }
                        value = ident;
                    } else {
                        panic!("Unreconized character found in source {}", c);
                    }
                }
            };
            println!("{:?}, {}, {:?}", kind, value, self.cur());
            return Some((kind, value));
        }
        None
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
    fn cur(&self) -> Option<char> {
        self.cur_char
    }
    fn next(&mut self) -> Option<char> {
        self.cur_char = self.chars.next();
        self.cur_char
    }
}

fn is_number(c: char) -> bool {
    '0' <= c && c <= '9'
}

fn is_ident(c: char) -> bool {
    !c.to_lowercase().eq(c.to_uppercase())
}

mod test {
    use super::{Lexer, Token, TokenType};
    #[test]
    fn test_let() {
        let mut lexer = Lexer::new("let");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenType::Let);
    }
    #[test]
    fn test_const() {
        let mut lexer = Lexer::new("const");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenType::Const);
    }
    #[test]
    fn test_fn() {
        let mut lexer = Lexer::new("fn");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenType::Fn)
    }
    #[test]
    fn test_if() {
        let mut lexer = Lexer::new("if");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenType::If);
    }
    #[test]
    fn test_for() {
        let mut lexer = Lexer::new("for");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenType::For);
    }
    #[test]
    fn test_while() {
        let mut lexer = Lexer::new("while");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenType::While);
    }
    #[test]
    fn test() {
        let mut lexer = Lexer::new("let a = 10;");
        let tokens = lexer.tokenize();
        println!("{:?}", tokens);
    }
}
