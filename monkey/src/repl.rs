use crate::{lexer, token};
use std::io;

const PROMPT: &str = ">> ";


pub fn start() {
    loop {
        let mut input = String::new();
        eprint!("{}", PROMPT);
        io::stdin().read_line(&mut input);
        if input.len() == 0 {
            return
        } else {
            let mut lexer = lexer::Lexer::new(&input);
            loop {
                let token = lexer.next_token();
                if token.token_type == token::TokenKind::EOF {
                    break;
                }
                else {
                    println!("Type: {:?} Literal: {:?}", token.token_type, token.literal);
                }
            }
        }
    }
}