use crate::frontend::lexer::{Token, TokenType};

mod ast;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}
