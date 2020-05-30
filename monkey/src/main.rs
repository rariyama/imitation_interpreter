mod lexer;
mod repl;
mod token;
mod parser;
mod ast;
mod errors;
mod evaluator;
mod object;
mod builtins;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    println!("");
    repl::start();
}
