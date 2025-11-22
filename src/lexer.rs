use crate::token::{Range, Token, TokenType};

pub struct Lexer {
    lines: String,
    cur_pos: usize,
}

impl Lexer {
    pub fn new(lines: String) -> Self {
        Lexer {
            lines: lines,
            cur_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();
        if let Some(eof) = self.parse_eof() {
            return eof;
        }
        if let Some(token) = self.parse_token_started_with_alphabetic() {
            return token;
        }
        if let Some(token) = self.parse_operation() {
            return token;
        }
        if let Some(token) = self.parse_number_literal() {
            return token;
        }
        if let Some(token) = self.parse_punctuators() {
            return token;
        }
        panic!("Unknown token");
    }

    pub fn get_peek(&mut self) -> Token {
        let pos = self.cur_pos;
        let token = self.next_token();
        self.cur_pos = pos;
        return token;
    }

    fn skip_whitespaces(&mut self) {
        while self.cur_pos < self.lines.len()
            && self.lines.as_bytes()[self.cur_pos].is_ascii_whitespace()
        {
            self.cur_pos += 1;
        }
    }

    fn parse_eof(&self) -> Option<Token> {
        if self.cur_pos < self.lines.as_bytes().len() {
            return None;
        }
        Some(Token::new(
            TokenType::Eof,
            Range::new(self.cur_pos, self.cur_pos),
        ))
    }

    fn start_with_digit(&mut self) -> bool {
        let bytes = self.lines.as_bytes();
        (self.cur_pos < bytes.len()) && bytes[self.cur_pos].is_ascii_digit()
    }

    fn start_with_alphabetic(&mut self) -> bool {
        let bytes = self.lines.as_bytes();
        (self.cur_pos < bytes.len()) && bytes[self.cur_pos].is_ascii_alphabetic()
    }

    fn parse_token_started_with_alphabetic(&mut self) -> Option<Token> {
        if !self.start_with_alphabetic() {
            return None;
        }
        if let Some(range) = self.eat_full("var") {
            return Some(Token::new(TokenType::KwVar, range));
        }
        if let Some(range) = self.eat_full("true") {
            return Some(Token::new(TokenType::BoolLiteral(true), range));
        }
        if let Some(range) = self.eat_full("false") {
            return Some(Token::new(TokenType::BoolLiteral(false), range));
        }
        if let Some(range) = self.eat_full("if") {
            return Some(Token::new(TokenType::KwIf, range));
        }
        if let Some(range) = self.eat_full("else") {
            return Some(Token::new(TokenType::KwElse, range));
        }
        if let Some(range) = self.eat_full("for") {
            return Some(Token::new(TokenType::KwFor, range));
        }
        if let Some(range) = self.eat_full("while") {
            return Some(Token::new(TokenType::KwWhile, range));
        }
        if let Some(range) = self.eat_full("fn") {
            return Some(Token::new(TokenType::KwFn, range));
        }
        Some(self.eat_alphanumeric())
    }

    fn parse_operation(&mut self) -> Option<Token> {
        if let Some(range) = self.eat_word("==") {
            return Some(Token::new(TokenType::EqEq, range));
        }
        if let Some(range) = self.eat_word("=") {
            return Some(Token::new(TokenType::Eq, range));
        }
        if let Some(range) = self.eat_word("!=") {
            return Some(Token::new(TokenType::BangEq, range));
        }
        if let Some(range) = self.eat_word("!=") {
            return Some(Token::new(TokenType::Bang, range));
        }
        if let Some(range) = self.eat_word(">=") {
            return Some(Token::new(TokenType::Ge, range));
        }
        if let Some(range) = self.eat_word(">") {
            return Some(Token::new(TokenType::Gt, range));
        }
        if let Some(range) = self.eat_word("<=") {
            return Some(Token::new(TokenType::Le, range));
        }
        if let Some(range) = self.eat_word("<") {
            return Some(Token::new(TokenType::Lt, range));
        }
        if let Some(range) = self.eat_word("+=") {
            return Some(Token::new(TokenType::PlusEq, range));
        }
        if let Some(range) = self.eat_word("+") {
            return Some(Token::new(TokenType::Plus, range));
        }
        if let Some(range) = self.eat_word("-=") {
            return Some(Token::new(TokenType::MinusEq, range));
        }
        if let Some(range) = self.eat_word("-") {
            return Some(Token::new(TokenType::Minus, range));
        }
        if let Some(range) = self.eat_word("*=") {
            return Some(Token::new(TokenType::StarEq, range));
        }
        if let Some(range) = self.eat_word("*") {
            return Some(Token::new(TokenType::Star, range));
        }
        if let Some(range) = self.eat_word("/=") {
            return Some(Token::new(TokenType::SlashEq, range));
        }
        if let Some(range) = self.eat_word("/") {
            return Some(Token::new(TokenType::Slash, range));
        }
        None
    }

    fn parse_number_literal(&mut self) -> Option<Token> {
        if !self.start_with_digit() {
            return None;
        }
        let bytes = self.lines.as_bytes();
        let start_pos = self.cur_pos;
        let cur_pos = &mut self.cur_pos;

        while *cur_pos < bytes.len() && bytes[*cur_pos].is_ascii_alphanumeric() {
            if bytes[*cur_pos].is_ascii_alphabetic() {
                *cur_pos = start_pos;
                return None;
            }
            *cur_pos += 1;
        }

        let token = std::str::from_utf8(&bytes[start_pos..*cur_pos]).unwrap();
        if let Ok(literal) = token.parse::<i128>() {
            return Some(Token::new(
                TokenType::IntLiteral(literal),
                Range::new(start_pos, *cur_pos),
            ));
        }
        None
    }

    fn parse_punctuators(&mut self) -> Option<Token> {
        if let Some(range) = self.eat_word("(") {
            return Some(Token::new(TokenType::LParen, range));
        }
        if let Some(range) = self.eat_word(")") {
            return Some(Token::new(TokenType::RParen, range));
        }
        if let Some(range) = self.eat_word("{") {
            return Some(Token::new(TokenType::LBrace, range));
        }
        if let Some(range) = self.eat_word("}") {
            return Some(Token::new(TokenType::RBrace, range));
        }
        if let Some(range) = self.eat_word(";") {
            return Some(Token::new(TokenType::Semicolon, range));
        }
        if let Some(range) = self.eat_word(",") {
            return Some(Token::new(TokenType::Comma, range));
        }
        if let Some(range) = self.eat_word("//") {
            return Some(Token::new(TokenType::Comment, range));
        }
        None
    }

    fn eat_alphanumeric(&mut self) -> Token {
        let start_pos = self.cur_pos;
        let bytes = self.lines.as_bytes();
        while self.cur_pos < bytes.len() && bytes[self.cur_pos].is_ascii_alphanumeric() {
            self.cur_pos += 1;
        }
        Token::new(
            TokenType::Identifier(
                std::str::from_utf8(&bytes[start_pos..self.cur_pos])
                    .unwrap()
                    .to_string(),
            ),
            Range::new(start_pos, self.cur_pos),
        )
    }

    fn peek_symbol(&self) -> Option<u8> {
        if self.cur_pos >= self.bytes().len() {
            return None;
        }
        Some(self.bytes()[self.cur_pos])
    }

    fn eat_word(&mut self, word: &str) -> Option<Range> {
        let bytes = self.lines.as_bytes();
        if bytes.len() - self.cur_pos < word.len() {
            return None;
        }
        let start_pos = self.cur_pos;
        for i in 0..word.len() as usize {
            if bytes[self.cur_pos] != word.as_bytes()[i] {
                self.cur_pos = start_pos;
                return None;
            }
            self.cur_pos += 1;
        }
        Some(Range::new(start_pos, self.cur_pos))
    }

    fn eat_full(&mut self, word: &str) -> Option<Range> {
        if let Some(range) = self.eat_word(word) {
            if let Some(symbol) = self.peek_symbol() {
                if symbol.is_ascii_alphanumeric() {
                    return None;
                }
            }
            return Some(range);
        }
        None
    }

    fn bytes(&self) -> &[u8] {
        self.lines.as_bytes()
    }
}
