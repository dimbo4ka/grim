use crate::token::{Range, Token, TokenType};

pub struct Lexer {
    lines: String,
    cur_pos: usize,
}

macro_rules! try_eat_word {
    ($self:expr, $(($word:expr, $token:expr)),+ $(,)?) => {
        $(
            if let Some(range) = $self.eat_word($word) {
                return Some(Token::new($token, range));
            }
        )+
    };
}

macro_rules! try_eat_full {
    ($self:expr, $(($word:expr, $token:expr)),+ $(,)?) => {
        $(
            if let Some(range) = $self.eat_full($word) {
                return Some(Token::new($token, range));
            }
        )+
    };
}

impl Lexer {
    pub fn new(lines: String) -> Self {
        Self {
            lines: lines,
            cur_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_comments_and_whitespaces();
        self.parse_eof()
            .or_else(|| self.parse_token_started_with_alphabetic())
            .or_else(|| self.parse_operation())
            .or_else(|| self.parse_i128_literal())
            .or_else(|| self.parse_punctuators())
    }

    pub fn get_peek(&mut self) -> Option<Token> {
        let pos = self.cur_pos;
        let token = self.next_token();
        self.cur_pos = pos;
        token
    }

    fn skip_whitespaces(&mut self) {
        while !self.is_at_end() && self.current_is_whitespace() {
            self.cur_pos += 1;
        }
    }

    fn skip_until_word(&mut self, word: &str) {
        while !self.is_at_end() {
            if self.eat_word(word).is_some() {
                self.skip_comments_and_whitespaces();
                return;
            }
            self.cur_pos += 1;
        }
    }

    fn skip_comments_and_whitespaces(&mut self) {
        self.skip_whitespaces();
        if self.eat_word("//").is_some() {
            self.skip_until_word("\n");
        }
        if self.eat_word("/*").is_some() {
            self.skip_until_word("*/");
        }
    }

    fn parse_eof(&self) -> Option<Token> {
        if !self.is_at_end() {
            return None;
        }
        Some(Token::new(
            TokenType::Eof,
            Range::new(self.cur_pos, self.cur_pos),
        ))
    }

    fn parse_token_started_with_alphabetic(&mut self) -> Option<Token> {
        if !self.current_is_alphabetic() {
            return None;
        }
        try_eat_full!(
            self,
            ("module", TokenType::KwModule),
            ("self", TokenType::KwSelf),
            ("type", TokenType::KwType),
            ("struct", TokenType::KwStruct),
            ("pub", TokenType::KwPub),
            ("var", TokenType::KwVar),
            ("const", TokenType::KwConst),
            ("true", TokenType::BoolLiteral(true)),
            ("false", TokenType::BoolLiteral(false)),
            ("if", TokenType::KwIf),
            ("else", TokenType::KwElse),
            ("loop", TokenType::KwLoop),
            ("foreach", TokenType::KwForEach),
            ("for", TokenType::KwFor),
            ("while", TokenType::KwWhile),
            ("continue", TokenType::KwContinue),
            ("break", TokenType::KwBreak),
            ("fn", TokenType::KwFn),
            ("return", TokenType::KwReturn),
            ("int", TokenType::KwInt),
            ("bool", TokenType::KwBool),
            ("str", TokenType::KwString),
            ("in", TokenType::KwIn),
        );
        self.parse_identifier()
    }

    fn parse_operation(&mut self) -> Option<Token> {
        try_eat_word!(
            self,
            ("^", TokenType::Caret),
            ("~", TokenType::Tilde),
            ("&&", TokenType::AmpAmp),
            ("&", TokenType::Amp),
            ("||", TokenType::PipePipe),
            ("|", TokenType::Pipe),
            ("==", TokenType::EqEq),
            ("=", TokenType::Eq),
            ("!=", TokenType::BangEq),
            ("!", TokenType::Bang),
            (">>", TokenType::GtGt),
            (">=", TokenType::Ge),
            (">", TokenType::Gt),
            ("<<", TokenType::LtLt),
            ("<=", TokenType::Le),
            ("<", TokenType::Lt),
            ("+=", TokenType::PlusEq),
            ("++", TokenType::PlusPlus),
            ("+", TokenType::Plus),
            ("->", TokenType::Arrow),
            ("-=", TokenType::MinusEq),
            ("--", TokenType::MinusMinus),
            ("-", TokenType::Minus),
            ("*=", TokenType::StarEq),
            ("*", TokenType::Star),
            ("/=", TokenType::SlashEq),
            ("/", TokenType::Slash),
            (":=", TokenType::ColonEq),
            ("%=", TokenType::PercentEq),
            ("%", TokenType::Percent),
        );
        None
    }

    fn parse_i128_literal(&mut self) -> Option<Token> {
        if !self.current_is_digit() {
            return None;
        }
        let start_pos = self.cur_pos;

        while !self.is_at_end() && self.current_is_alphanumeric() {
            if self.current_is_alphabetic() {
                self.cur_pos = start_pos;
                return None;
            }
            self.cur_pos += 1;
        }

        let token = std::str::from_utf8(self.get_slice(start_pos)).ok()?;
        let literal = token.parse::<i128>().ok()?;
        Some(Token::new(
            TokenType::IntLiteral(literal),
            Range::new(start_pos, self.cur_pos),
        ))
    }

    fn parse_punctuators(&mut self) -> Option<Token> {
        try_eat_word!(
            self,
            ("(", TokenType::LParen),
            (")", TokenType::RParen),
            ("{", TokenType::LBrace),
            ("}", TokenType::RBrace),
            ("[", TokenType::LBracket),
            ("]", TokenType::RBracket),
            (":", TokenType::Colon),
            (";", TokenType::Semicolon),
            (",", TokenType::Comma),
            ("..", TokenType::DotDot),
            (".", TokenType::Dot),
            ("?", TokenType::Question),
        );
        None
    }

    fn parse_identifier(&mut self) -> Option<Token> {
        let start_pos = self.cur_pos;
        while !self.is_at_end() && (self.current_is_alphanumeric() || self.current_is_underscore()) {
            self.cur_pos += 1;
        }

        let identifier = std::str::from_utf8(self.get_slice(start_pos)).ok()?;
        Some(Token::new(
            TokenType::Identifier(identifier.to_string()),
            Range::new(start_pos, self.cur_pos),
        ))
    }

    fn eat_word(&mut self, word: &str) -> Option<Range> {
        let start_pos = self.cur_pos;
        let end_pos = start_pos.checked_add(word.len())?;

        if self.bytes().get(start_pos..end_pos)? != word.as_bytes() {
            return None;
        }
        self.cur_pos = end_pos;
        Some(Range::new(start_pos, self.cur_pos))
    }

    fn eat_full(&mut self, word: &str) -> Option<Range> {
        if let Some(range) = self.eat_word(word) {
            if self.current_is_alphanumeric() {
                return None;
            }
            return Some(range);
        }
        None
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.cur_pos >= self.lines.as_bytes().len()
    }

    #[inline]
    fn get_slice(&self, start_pos: usize) -> &[u8] {
        &self.bytes()[start_pos..self.cur_pos]
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        self.lines.as_bytes()
    }

    #[inline]
    fn current_byte(&self) -> Option<u8> {
        self.bytes().get(self.cur_pos).copied()
    }

    #[inline]
    fn current_is_whitespace(&self) -> bool {
        let byte = self.current_byte();
        byte.map_or(false, |b| b.is_ascii_whitespace())
    }

    #[inline]
    fn current_is_digit(&self) -> bool {
        let byte = self.current_byte();
        byte.map_or(false, |b| b.is_ascii_digit())
    }

    #[inline]
    fn current_is_alphabetic(&self) -> bool {
        let byte = self.current_byte();
        byte.map_or(false, |b| b.is_ascii_alphabetic())
    }

    #[inline]
    fn current_is_alphanumeric(&self) -> bool {
        let byte = self.current_byte();
        byte.map_or(false, |b| b.is_ascii_alphanumeric())
    }

    #[inline]
    fn current_is_underscore(&self) -> bool {
        let byte = self.current_byte();
        byte.map_or(false, |b| b == '_' as u8)
    }
}
