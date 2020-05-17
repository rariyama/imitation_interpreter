use super::lexer;
use super::token::{Token};


pub trait Node {
    fn token_literal(&self) -> String;
}

//pub trait Statement: Node {
//    fn statement_node(&self);
//}

//pub trait Expression: Node {
//    fn expression_node(&self);
//}

#[derive(Debug,PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>
}

#[derive(Debug,PartialEq)]
pub enum Statement {
    LetStatement(LetStatement)
}

#[derive(Debug,PartialEq)]
pub struct LetStatement {
    pub identifier: Identifier
}

pub enum Expression {
    Identifier(Identifier)
}

#[derive(Debug,PartialEq)]
pub struct Identifier {
    pub value: String
}

