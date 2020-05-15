use crate::{lexer, token};

#[derive(Debug,Eq,PartialEq)]
pub enum Node {
    token_literal()
}

pub enum Statement {
    Node
}



pub struct Program {
    pub statements: Statement

}

impl Program {
    pub fn token_literal(&mut self)  -> String {
        if self.statements.len() > {
            return self.statements[0].
        }
        else {
            return ""
        }
    }
}

pub struct LetStatement {
    Token token.Token, // let token
    Name   *Identifier,
    Value  Expression
}

impl LetStatement {
    pub fn statement_node(&mut self) {}
    pub fn token_literal(&mut self) -> String {
        return self.Token.literal
    }
}


pub struct Identifier {
    Token token.Token, // token.IDENT token
    Value String
}

impl Identifier {
    pub fn expression_node(&mut self) {}
    pub fn token_literal(&mut self) -> String {
        return self.Token.literal
    } 
}





