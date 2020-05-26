use std::fmt;


use super::token::{Token};
use super::parser;
use super::lexer;
use super::evaluator;
use super::object::{Object};

#[derive(Debug, PartialEq, Clone)]
pub enum Errors {
    TokenInvalid(Token),
    InvalidOperator(String),
    InvalidInteger(Box<Object>),
    InvalidIdentifier(Box<Object>),
    InvalidInfix,
    NodeError
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::TokenInvalid(value) => write!(f, "invalid token: {:?}", value),
            Errors::InvalidOperator(value) => write!(f, "invalid operator: {}", value),
            Errors::InvalidInteger(value) => write!(f, "invalid integer: {}", value),
            Errors::InvalidIdentifier(value) => write!(f, "invalid identifier: {}", value),
            Errors::InvalidInfix => write!(f, "invalid_infix"),
            Errors::NodeError => write!(f, "node_error")
        }
    }
}