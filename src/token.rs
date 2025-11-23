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
    LBracket,
    RBracket,
    Arrow,
    ColonEq,
    Colon,
    Semicolon,
    Comma,
    Dot,
    DotDot,
    Question,

    Eq,
    Plus,
    PlusPlus,
    PlusEq,
    Minus,
    MinusMinus,
    MinusEq,
    Star,
    StarEq,
    Slash,
    SlashEq,
    Percent,
    PercentEq,

    Amp,
    Pipe,
    Caret,
    Tilde,
    LtLt,
    GtGt,

    AmpAmp,
    PipePipe,
    EqEq,
    Bang,
    BangEq,
    Lt,
    Gt,
    Le,
    Ge,

    KwVar,
    KwConst,
    KwIf,
    KwElse,
    KwLoop,
    KwFor,
    KwForEach,
    KwWhile,
    KwContinue,
    KwBreak,
    KwFn,
    KwReturn,
    KwIn,

    KwBool,
    KwInt,
    KwString,

    Eof,
}
