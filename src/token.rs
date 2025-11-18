#[derive(Debug, Clone, Copy)]
struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub TokenType: token_type,
    pub Range: range,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Identifier(String),
    IntLiteral(i128),
    BoolLiteral(bool),
    Var,
    LBrace, RBrace,
    LParen, RParen,
    Semicolon,
    Comma,
    Comment,

    Assign,
    Plus, Minus,
    Star, Slash,

    Eq, LogicalEq,
    Bang, BangEq,
    Lt, Gt, Le, Ge,
    
    KwIf, KwElse,
    KwFor, KwWhile,
    KwFn, KwVar, 

    Eof
}



