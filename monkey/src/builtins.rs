use super::object::{Object};
use super::errors::{Errors};
use std::collections::HashMap;

pub fn new() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("len"), Object::Builtin{func: len});
    builtins
}

pub fn len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(Errors::InvalidNumberOfArguments{got: args.len(), want:1});
    }

    match &args[0] {
        Object::String(string) => Object::Integer(string.len() as i32),
        _ => {
            Object::Error(Errors::LenInvalidTypeError(Box::new(args[0].clone())))
            },
    }
}