use crate::{lexer, parser, ast, errors, evaluator, object};
use std::io;
use std::io::prelude::*;


const PROMPT: &str = ">> ";


pub fn start() {
    // if environment is defined outside loop,
    // initialize it per iterator, and can't contain variables.
    let mut environment = evaluator::Environment::new();
    loop {
        // display prompt symbol
        io::stdout().flush().unwrap();
        eprint!("{}", PROMPT);

        //read and parse input data 
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let lexer = lexer::Lexer::new(&input);
        
        // the command to exit
        if input == "exit()\n" {
            println!("Bye!");
            break
        }
        else if input == "exit\n" {
            println!("if you would like to exit, please use exit()");
            continue
        }
        
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let evaluated = environment.evaluate(&program);

        // if input is invalid, display error message and retry.
        if let Err(_error) = &evaluated {
            println!("invalid syntax");
            println!("{:?}", evaluated);
            continue;
        }

        // if input is null, display nothing and retry.
        if input.len() == 1 {
            continue;
        } else {
            // if correctly input, display parsed result.
            println!("{}", evaluated.unwrap());
            }
        }
        }