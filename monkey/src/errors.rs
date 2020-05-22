use super::token::{Token};
use super::parser;
use super::lexer;

#[derive(Debug)]
pub enum Errors {
    TokenInvalid(Token)
}