use crate::lexer::{Lexer, TokenType};

#[test]
fn lexer_test() {
    let input = "=+(){},;";
    let mut l = Lexer::new(String::from(input));

    let tests: Vec<(TokenType, String)> = vec![
        (TokenType::Assign, String::from("=")),
        (TokenType::Plus, String::from("+")),
        (TokenType::LParen, String::from("(")),
        (TokenType::RParen, String::from(")")),
        (TokenType::LBrace, String::from("{")),
        (TokenType::RBrace, String::from("}")),
        (TokenType::Comma, String::from(",")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::Eof, String::from("\0")),
    ];

    for item in tests {
        let token = l.next_token();
        assert!(token.Type == item.0);
        assert!(token.Literal == item.1);
    }
}

#[test]
fn test_next_token() {
    let input = "let five = 5;
              let ten = 10;
              
              let add = fn(x, y) {
                  x + y;
              };
              
              let result = add(five, ten);
              !-/*5;
              5 < 10 > 5;

              if (5 < 10) {
                return true;
              } else {
                return false;
              }

              10 == 10;
              10 != 9;
              ";
    let mut l = Lexer::new(String::from(input));

    let tests: Vec<(TokenType, String)> = vec![
        (TokenType::Let, String::from("let")),
        (TokenType::Ident, String::from("five")),
        (TokenType::Assign, String::from("=")),
        (TokenType::Int, String::from("5")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::Let, String::from("let")),
        (TokenType::Ident, String::from("ten")),
        (TokenType::Assign, String::from("=")),
        (TokenType::Int, String::from("10")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::Let, String::from("let")),
        (TokenType::Ident, String::from("add")),
        (TokenType::Assign, String::from("=")),
        (TokenType::Function, String::from("fn")),
        (TokenType::LParen, String::from("(")),
        (TokenType::Ident, String::from("x")),
        (TokenType::Comma, String::from(",")),
        (TokenType::Ident, String::from("y")),
        (TokenType::RParen, String::from(")")),
        (TokenType::LBrace, String::from("{")),
        (TokenType::Ident, String::from("x")),
        (TokenType::Plus, String::from("+")),
        (TokenType::Ident, String::from("y")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::RBrace, String::from("}")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::Let, String::from("let")),
        (TokenType::Ident, String::from("result")),
        (TokenType::Assign, String::from("=")),
        (TokenType::Ident, String::from("add")),
        (TokenType::LParen, String::from("(")),
        (TokenType::Ident, String::from("five")),
        (TokenType::Comma, String::from(",")),
        (TokenType::Ident, String::from("ten")),
        (TokenType::RParen, String::from(")")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::Bang, String::from("!")),
        (TokenType::Minus, String::from("-")),
        (TokenType::Slash, String::from("/")),
        (TokenType::Asteriks, String::from("*")),
        (TokenType::Int, String::from("5")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::Int, String::from("5")),
        (TokenType::Lt, String::from("<")),
        (TokenType::Int, String::from("10")),
        (TokenType::Gt, String::from(">")),
        (TokenType::Int, String::from("5")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::If, String::from("if")),
        (TokenType::LParen, String::from("(")),
        (TokenType::Int, String::from("5")),
        (TokenType::Lt, String::from("<")),
        (TokenType::Int, String::from("10")),
        (TokenType::RParen, String::from(")")),
        (TokenType::LBrace, String::from("{")),
        (TokenType::Return, String::from("return")),
        (TokenType::True, String::from("true")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::RBrace, String::from("}")),
        (TokenType::Else, String::from("else")),
        (TokenType::LBrace, String::from("{")),
        (TokenType::Return, String::from("return")),
        (TokenType::False, String::from("false")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::RBrace, String::from("}")),
        (TokenType::Int, String::from("10")),
        (TokenType::Equals, String::from("==")),
        (TokenType::Int, String::from("10")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::Int, String::from("10")),
        (TokenType::NotEquals, String::from("!=")),
        (TokenType::Int, String::from("9")),
        (TokenType::Semicolon, String::from(";")),
        (TokenType::Eof, String::from("\0")),
    ];

    for item in tests {
        let token = l.next_token();
        println!("{} {}", token.Literal, item.1);
        assert!(token.Type == item.0);
        assert!(token.Literal == item.1);
    }
}
