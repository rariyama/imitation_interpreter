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
    NodeError,
    InvalidNumberOfArguments{got: usize,
                             want: usize
                            },
    LenInvalidTypeError(Box<Object>),
    FirstTypeError(Box<Object>),
    LastTypeError(Box<Object>),
    RestTypeError(Box<Object>),
    PushTypeError(Box<Object>),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::TokenInvalid(value) => write!(f, "invalid token: {:?}", value),
            Errors::InvalidOperator(value) => write!(f, "invalid operator: {}", value),
            Errors::InvalidInteger(value) => write!(f, "invalid integer: {}", value),
            Errors::InvalidIdentifier(value) => write!(f, "invalid identifier: {}", value),
            Errors::InvalidInfix => write!(f, "invalid_infix"),
            Errors::NodeError => write!(f, "node_error"),
            Errors::InvalidNumberOfArguments{got, want} => write!(f, "wrong number of arguments. got={}, want={}",got, want),
            Errors::LenInvalidTypeError(value) => write!(f, "argument to len not supported got {}", value),
            Errors::FirstTypeError(value) => write!(f, "argument to 'first' must be array, got {}", value),
            Errors::LastTypeError(value) => write!(f, "argument to 'last' must be array, got {}", value),
            Errors::RestTypeError(value) => write!(f, "argument to 'rest' must be array, got {}", value),
            Errors::PushTypeError(value) => write!(f, "argument to 'push' must be array, got {}", value)
        }
    }
}