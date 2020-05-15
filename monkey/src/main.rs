mod lexer;
mod repl;
mod token;


fn main() {
    println!("Hello, world!");
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    println!("");
    repl::start();
}
