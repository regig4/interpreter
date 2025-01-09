use crate::lexer::{Lexer, TokenType};
use std::io::{self, BufRead};

const PROMPT: &str = ">>";

pub fn start_repl() {
    loop { 
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        
        let mut lex = Lexer::new(buffer);
        let mut tok = lex.next_token();

        while tok.Type != TokenType::Eof {
            println!("Token literal {}", &tok.Literal);
            tok = lex.next_token();
        }
    }
}
