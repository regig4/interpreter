use repl::start_repl;

pub mod lexer_test;
pub mod repl;
pub mod lexer;


fn main() {
    println!("Hello from REPL");
    start_repl();
}
