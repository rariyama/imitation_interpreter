extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::{lexer, parser, ast, errors, evaluator, object};

pub fn start() {
    let mut rl = Editor::<()>::new();
    // if environment is defined outside loop,
    // initialize it per iterator, and can't contain variables.
    let mut environment = evaluator::Environment::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line == "" {
                    continue;
                }
                // the command to exit
                if line == "exit()" {
                    println!("Bye!");
                    break;
                }

                else if line == "exit" {
                    println!("if you would like to exit, please use exit(), ctrl-c, or ctrl-d");
                    continue;
                }

                let lexer = lexer::Lexer::new(&line);
                let mut parser = parser::Parser::new(lexer);
                let program = parser.parse_program().unwrap();
                let evaluated = environment.evaluate(&program);
                println!("{}", evaluated.unwrap());
            },
            Err(ReadlineError::Interrupted) => {
                println!("ctrl-c");
                break
            },
        Err(ReadlineError::Eof) => {
            println!("ctrl-d");
            break
        },
        Err(err) => {
            println!("error: {:?}", err);
            break
           }
        }
    }
}
