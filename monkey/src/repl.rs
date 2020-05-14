use std::io::{Stdin, Stdout, Write};
use crate::{lexer, token};
use std::io;

const PROMPT: &str = ">> ";

pub fn start(stdin: Stdin, stdout: Stdout) {
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
                if token.Type == token::TokenKind::EOF {
                    break;
                }
                else {
                    println!("Type: {:?} Literal: {:?}", token.Type, token.Literal);
                }
            }
        }
    }
}