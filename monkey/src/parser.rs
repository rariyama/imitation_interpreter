use super::token;
use super::lexer;
use super::ast;


pub struct Parser {
    lexer: *lexer::Lexer,
    current_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn new(&mut self) -> *Parser {
        p = *Parser{lexer: lexer} 

        p.next_token();
        p.next_token();

        return p
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token
        self.peek_token = self.lexer.next_token()
    }

    fn parse_program(&mut self) ->*ast::Program {
        return nil
    }
}