use repl::start_repl;

pub mod lexer_test;
pub mod repl;
pub mod lexer;
pub mod ast;
mod parser;
mod parser_test;


fn main() {
    println!("Hello from REPL");
    start_repl();
}
