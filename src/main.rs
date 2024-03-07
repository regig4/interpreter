use repl::start_repl;
use crate::parser_test::test_let_statements;

pub mod lexer_test;
pub mod repl;
pub mod lexer;
pub mod ast;
mod parser;
mod parser_test;


fn main() {
    //println!("Hello from REPL");
    //start_repl();
    test_let_statements();
}
