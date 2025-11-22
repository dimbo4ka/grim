use grim::lexer::Lexer;
use grim::token::{self, TokenType};

fn parse_token_types(lines: String) -> Vec<TokenType> {
    let mut lexer = Lexer::new(lines);
    let mut tokens = Vec::<TokenType>::new();
    loop {
        let token = lexer.next_token();
        if let TokenType::Eof = token.token_type {
            break;
        }
        tokens.push(token.token_type);
    }
    tokens
}

#[test]
fn test_if_statement() {
    let code = "
        var x = 10;
        if x > 5 {
            true;
        } else {
            false;
        }
    "
    .to_string();
    let expected = vec![
        TokenType::KwVar,
        TokenType::Identifier("x".to_string()),
        TokenType::Eq,
        TokenType::IntLiteral(10),
        TokenType::Semicolon,
        TokenType::KwIf,
        TokenType::Identifier("x".to_string()),
        TokenType::Gt,
        TokenType::IntLiteral(5),
        TokenType::LBrace,
        TokenType::BoolLiteral(true),
        TokenType::Semicolon,
        TokenType::RBrace,
        TokenType::KwElse,
        TokenType::LBrace,
        TokenType::BoolLiteral(false),
        TokenType::Semicolon,
        TokenType::RBrace,
    ];
    assert_eq!(parse_token_types(code), expected);
}
