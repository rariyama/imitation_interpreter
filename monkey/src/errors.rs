use super::token::{Token};
use super::parser;
use super::lexer;
use super::evaluator;

#[derive(Debug)]
pub enum Errors {
    TokenInvalid(Token),
    NodeError
}