use grim::lexer::Lexer;
use grim::token::TokenType;

fn parse_token_types(lines: String) -> Vec<TokenType> {
    let mut lexer = Lexer::new(lines);
    let mut tokens = Vec::<TokenType>::new();
    loop {
        let token = lexer.next_token();
        if token.is_none() {
            break;
        }
        if let TokenType::Eof = token.clone().unwrap().token_type {
            break;
        }
        tokens.push(token.unwrap().token_type);
    }
    tokens
}

#[test]
fn test_if_statement() {
    let code = "
        var x := 10; // initalize
        if x > 5 {   // condition
            true;
        } else {
            false;
        }
    "
    .to_string();
    let expected = vec![
        TokenType::KwVar,
        TokenType::Identifier("x".to_string()),
        TokenType::ColonEq,
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

#[test]
fn test_fn_statement() {
    let code = "
        fn func(x: int) {
            return x;
        }
    "
    .to_string();
    let expected = vec![
        TokenType::KwFn,
        TokenType::Identifier("func".to_string()),
        TokenType::LParen,
        TokenType::Identifier("x".to_string()),
        TokenType::Colon,
        TokenType::KwInt,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::KwReturn,
        TokenType::Identifier("x".to_string()),
        TokenType::Semicolon,
        TokenType::RBrace,
    ];
    assert_eq!(parse_token_types(code), expected);
}

#[test]
fn test_function_call() {
    let code = "
        func(/*async_call=*/true);
    "
    .to_string();
    let expected = vec![
        TokenType::Identifier("func".to_string()),
        TokenType::LParen,
        TokenType::BoolLiteral(true),
        TokenType::RParen,
        TokenType::Semicolon,
    ];
    assert_eq!(parse_token_types(code), expected);
}

#[test]
fn test_foreach() {
    let code = "
        var sum := 0;
        foreach i in 0..10 {
            sum += i;
        }
    "
    .to_string();
    let expected = vec![
        TokenType::KwVar,
        TokenType::Identifier("sum".to_string()),
        TokenType::ColonEq,
        TokenType::IntLiteral(0),
        TokenType::Semicolon,
        TokenType::KwForEach,
        TokenType::Identifier("i".to_string()),
        TokenType::KwIn,
        TokenType::IntLiteral(0),
        TokenType::DotDot,
        TokenType::IntLiteral(10),
        TokenType::LBrace,
        TokenType::Identifier("sum".to_string()),
        TokenType::PlusEq,
        TokenType::Identifier("i".to_string()),
        TokenType::Semicolon,
        TokenType::RBrace,
    ];
    assert_eq!(parse_token_types(code), expected);
}

#[test]
fn test_for() {
    let code = "
        var sum := 0;
        for i := 0; i < 10; ++i {
            sum += i;
        }
    "
    .to_string();
    let expected = vec![
        TokenType::KwVar,
        TokenType::Identifier("sum".to_string()),
        TokenType::ColonEq,
        TokenType::IntLiteral(0),
        TokenType::Semicolon,
        TokenType::KwFor,
        TokenType::Identifier("i".to_string()),
        TokenType::ColonEq,
        TokenType::IntLiteral(0),
        TokenType::Semicolon,
        TokenType::Identifier("i".to_string()),
        TokenType::Lt,
        TokenType::IntLiteral(10),
        TokenType::Semicolon,
        TokenType::PlusPlus,
        TokenType::Identifier("i".to_string()),
        TokenType::LBrace,
        TokenType::Identifier("sum".to_string()),
        TokenType::PlusEq,
        TokenType::Identifier("i".to_string()),
        TokenType::Semicolon,
        TokenType::RBrace,
    ];
    assert_eq!(parse_token_types(code), expected);
}

#[test]
fn test_while() {
    let code = "
        var sum := 0;
        var i := 0;
        while i < 10 {
            sum += i;
            ++i;
        }
    "
    .to_string();
    let expected = vec![
        TokenType::KwVar,
        TokenType::Identifier("sum".to_string()),
        TokenType::ColonEq,
        TokenType::IntLiteral(0),
        TokenType::Semicolon,
        TokenType::KwVar,
        TokenType::Identifier("i".to_string()),
        TokenType::ColonEq,
        TokenType::IntLiteral(0),
        TokenType::Semicolon,
        TokenType::KwWhile,
        TokenType::Identifier("i".to_string()),
        TokenType::Lt,
        TokenType::IntLiteral(10),
        TokenType::LBrace,
        TokenType::Identifier("sum".to_string()),
        TokenType::PlusEq,
        TokenType::Identifier("i".to_string()),
        TokenType::Semicolon,
        TokenType::PlusPlus,
        TokenType::Identifier("i".to_string()),
        TokenType::Semicolon,
        TokenType::RBrace,
    ];
    assert_eq!(parse_token_types(code), expected);
}

#[test]
fn test_loop() {
    let code = "
        var sum := 0;
        var i := 0;
        loop {
            if i >= 10 {
                break;
            }
            sum += i;
            ++i;
        }
    "
    .to_string();
    let expected = vec![
        TokenType::KwVar,
        TokenType::Identifier("sum".to_string()),
        TokenType::ColonEq,
        TokenType::IntLiteral(0),
        TokenType::Semicolon,
        TokenType::KwVar,
        TokenType::Identifier("i".to_string()),
        TokenType::ColonEq,
        TokenType::IntLiteral(0),
        TokenType::Semicolon,
        TokenType::KwLoop,
        TokenType::LBrace,
        TokenType::KwIf,
        TokenType::Identifier("i".to_string()),
        TokenType::Ge,
        TokenType::IntLiteral(10),
        TokenType::LBrace,
        TokenType::KwBreak,
        TokenType::Semicolon,
        TokenType::RBrace,
        TokenType::Identifier("sum".to_string()),
        TokenType::PlusEq,
        TokenType::Identifier("i".to_string()),
        TokenType::Semicolon,
        TokenType::PlusPlus,
        TokenType::Identifier("i".to_string()),
        TokenType::Semicolon,
        TokenType::RBrace,
    ];
    assert_eq!(parse_token_types(code), expected);
}
