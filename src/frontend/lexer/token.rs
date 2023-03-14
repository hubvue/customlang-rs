#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub start: usize,
    pub end: usize,
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
