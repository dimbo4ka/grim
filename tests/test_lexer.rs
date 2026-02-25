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
        var x: int = 10; // initalize
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
        TokenType::Colon,
        TokenType::KwInt,
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

#[test]
fn test_fn_statement() {
    let code = "
        fn func(x: int) -> int {
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
        TokenType::Arrow,
        TokenType::KwInt,
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
        sum := 0;
        foreach i in 0..10 {
            sum += i;
        }
    "
    .to_string();
    let expected = vec![
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
        sum := 0;
        for i := 0; i < 10; i++ {
            sum += i;
        }
    "
    .to_string();
    let expected = vec![
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
        TokenType::Identifier("i".to_string()),
        TokenType::PlusPlus,
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
        sum := 0;
        var i: int = 0;
        while i < 10 {
            sum += i;
            i++;
        }
    "
    .to_string();
    let expected = vec![
        TokenType::Identifier("sum".to_string()),
        TokenType::ColonEq,
        TokenType::IntLiteral(0),
        TokenType::Semicolon,
        TokenType::KwVar,
        TokenType::Identifier("i".to_string()),
        TokenType::Colon,
        TokenType::KwInt,
        TokenType::Eq,
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
        TokenType::Identifier("i".to_string()),
        TokenType::PlusPlus,
        TokenType::Semicolon,
        TokenType::RBrace,
    ];
    assert_eq!(parse_token_types(code), expected);
}

#[test]
fn test_loop() {
    let code = "
        sum := 0;
        var i: int = 0;
        loop {
            if i >= 10 {
                break;
            }
            sum += i;
            i++;
        }
    "
    .to_string();
    let expected = vec![
        TokenType::Identifier("sum".to_string()),
        TokenType::ColonEq,
        TokenType::IntLiteral(0),
        TokenType::Semicolon,
        TokenType::KwVar,
        TokenType::Identifier("i".to_string()),
        TokenType::Colon,
        TokenType::KwInt,
        TokenType::Eq,
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
        TokenType::Identifier("i".to_string()),
        TokenType::PlusPlus,
        TokenType::Semicolon,
        TokenType::RBrace,
    ];
    assert_eq!(parse_token_types(code), expected);
}

#[test]
fn test_struct_definition() {
    let code = "pub type W = struct(x: int, y: str, z: bool);".to_string();

    let expected = vec![
        TokenType::KwPub,
        TokenType::KwType,
        TokenType::Identifier("W".to_string()),
        TokenType::Eq,
        TokenType::KwStruct,
        TokenType::LParen,
        TokenType::Identifier("x".to_string()),
        TokenType::Colon,
        TokenType::KwInt,
        TokenType::Comma,
        TokenType::Identifier("y".to_string()),
        TokenType::Colon,
        TokenType::KwString,
        TokenType::Comma,
        TokenType::Identifier("z".to_string()),
        TokenType::Colon,
        TokenType::KwBool,
        TokenType::RParen,
        TokenType::Semicolon,
    ];
    assert_eq!(parse_token_types(code), expected);
}

#[test]
fn test_module() {
    let code = "module test;".to_string();

    let expected = vec![
        TokenType::KwModule,
        TokenType::Identifier("test".to_string()),
        TokenType::Semicolon
    ];
    assert_eq!(parse_token_types(code), expected);
}

#[test]
fn test_method() {
    let code = "
    pub fn (self: MyStruct) method_name(x: int, y: int) {
	    // ...
    }"
    .to_string();

    let expected = vec![
        TokenType::KwPub,
        TokenType::KwFn,

        TokenType::LParen,
        TokenType::KwSelf,
        
        TokenType::Colon,
        TokenType::Identifier("MyStruct".to_string()),

        TokenType::RParen,

        TokenType::Identifier("method_name".to_string()),
        TokenType::LParen,
        TokenType::Identifier("x".to_string()),
        TokenType::Colon,
        TokenType::KwInt,
        TokenType::Comma,
        TokenType::Identifier("y".to_string()),
        TokenType::Colon,
        TokenType::KwInt,
        TokenType::RParen,

        TokenType::LBrace,
        TokenType::RBrace,
    ];
    assert_eq!(parse_token_types(code), expected);
}