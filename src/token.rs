#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Range {
            start: start,
            end: end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub range: Range,
}

impl Token {
    pub fn new(token_type: TokenType, range: Range) -> Self {
        Token {
            token_type: token_type,
            range: range,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier(String),
    IntLiteral(i128),
    BoolLiteral(bool),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicolon,
    Comma,
    Comment,

    Eq,
    Plus,
    PlusEq,
    Minus,
    MinusEq,
    Star,
    StarEq,
    Slash,
    SlashEq,

    EqEq,
    Bang,
    BangEq,
    Lt,
    Gt,
    Le,
    Ge,

    KwIf,
    KwElse,
    KwFor,
    KwWhile,
    KwFn,
    KwVar,

    Eof,
}
