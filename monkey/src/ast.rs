use super::lexer;
use super::token::{Token};


pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug,PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>
}

#[derive(Debug,PartialEq)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement)
}

#[derive(Debug,PartialEq)]
pub struct LetStatement {
    pub identifier: Identifier
}

#[derive(Debug,PartialEq)]
pub struct ReturnStatement {
    pub identifier: Identifier
}

pub enum Expression {
    Identifier(Identifier)
}

#[derive(Debug,PartialEq)]
pub struct Identifier {
    pub value: String
}

