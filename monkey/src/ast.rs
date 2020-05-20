use std::fmt;

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
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement)
}

#[derive(Debug,PartialEq)]
pub struct LetStatement {
    pub identifier: Identifier
}

impl LetStatement {
    pub fn string(&mut self) -> String {
        if self.identifier.value.len() != 0 {
            let mut statement = "let ".to_string() + &self.identifier.value.to_string() + ";";
            return statement
        } else {
            return "".to_string()
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct ReturnStatement {
    pub identifier: Identifier
}

impl ReturnStatement {
    pub fn string(&mut self) -> String {
        if self.identifier.value.len() != 0 {
            let mut statement = "return ".to_string()+ &self.identifier.value.to_string() + ";";
            return statement
        } else {
            return "return; ".to_string()            
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression
}

//impl ExpressionStatement {
//    pub fn string(&mut self) -> String{
//        if self.expression.Expression.Identifier.value.len() != 0{
//            return self.identifier.value.clone()
//        }
//        else {
//            return "".to_string()
//        }
//    }
//}

#[derive(Debug,PartialEq)]
pub enum Expression {
    Identifier(Identifier)
}

#[derive(Debug,PartialEq)]
pub struct Identifier {
    pub value: String
}

pub enum ExpressionKind {
    LOWEST,      
    EQUALS,       // ==
    LESSGREATER,  // > or <
    SUM,          // +
    PRODUCT,      // *
    PREFIX,       // -X or !X
    CALL          // my_cunction(x){}
}
