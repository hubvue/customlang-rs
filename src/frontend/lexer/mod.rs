use std::{process::exit, str::Chars};

#[derive(Debug)]
pub enum TokenType {
    Number,       // 数字
    Identifier,   // 标识符
    Equals,       // =
    Semicolon,    // ;
    Commoa,       // ,
    Colon,        // :
    Dot,          // .
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBarce,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    BinaryOperator,
    //keywords
    Let,   // let
    Const, // const
    Fn,    // fn
    If,    // if
    For,   // for
    While, // while

    EOF, // 文件结束标识
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    value: String,
    start: usize,
    end: usize,
}

impl Token {
    pub fn new(kind: TokenType, value: impl Into<String>, start: usize, end: usize) -> Self {
        Self {
            kind,
            value: value.into(),
            start,
            end,
        }
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    cur_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
            cur_char: source.chars().next(),
        }
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
        while let Some(s) = self.chars.next() {
            let kind: TokenType;
            let value: String;
            match s {
                '(' => {
                    kind = TokenType::OpenParen;
                    value = s.to_string()
                }
                ')' => {
                    kind = TokenType::CloseParen;
                    value = s.to_string();
                }
                '[' => {
                    kind = TokenType::OpenBracket;
                    value = s.to_string();
                }
                ']' => {
                    kind = TokenType::CloseBracket;
                    value = s.to_string();
                }
                '{' => {
                    kind = TokenType::OpenBrace;
                    value = s.to_string();
                }
                '+' | '-' | '*' | '/' | '%' => {
                    kind = TokenType::BinaryOperator;
                    value = s.to_string();
                }
                '=' => {
                    kind = TokenType::Equals;
                    value = s.to_string();
                }
                ';' => {
                    kind = TokenType::Semicolon;
                    value = s.to_string();
                }
                ',' => {
                    kind = TokenType::Commoa;
                    value = s.to_string();
                }
                ':' => {
                    kind = TokenType::Colon;
                    value = s.to_string();
                }
                '.' => {
                    kind = TokenType::Dot;
                    value = s.to_string();
                }
                // skip
                ' ' | '\n' | '\t' | '\r' => continue,
                _ => {
                    if self.is_number(s) {
                        let mut num = String::new();
                        num.push(s);
                        while let Some(s) = self.chars.next() {
                            if self.is_number(s) {
                                num.push(s);
                            } else {
                                break;
                            }
                        }
                        kind = TokenType::Number;
                        value = num;
                    } else if self.is_ident(s) {
                        let mut ident = String::new();
                        ident.push(s);
                        // while let Some()
                    }
                    panic!("Unreconized character found in source {}", s);
                }
            };
            return Some((kind, value));
        }
        None
    }
    fn is_number(&self, c: char) -> bool {
        '0' <= c && c <= '9'
    }
    fn is_ident(&self, c: char) -> bool {
        !c.to_lowercase().eq(c.to_uppercase())
    }
    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
}
