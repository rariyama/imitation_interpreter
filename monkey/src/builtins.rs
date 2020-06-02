use super::object::{Object};
use super::errors::{Errors};
use std::collections::HashMap;

pub fn new() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("len"), Object::Builtin{func: len});
    builtins.insert(String::from("first"), Object::Builtin{func: first});
    builtins.insert(String::from("last"), Object::Builtin{func: last});
    builtins.insert(String::from("rest"), Object::Builtin{func: rest}); 
    builtins.insert(String::from("push"), Object::Builtin{func: push}); 
    builtins
}

fn len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(Errors::InvalidNumberOfArguments{got: args.len(), want:1});
    }

    match &args[0] {
        Object::Array(value) => {
            Object::Integer(value.len() as i32)
        }
        Object::String(string) => Object::Integer(string.len() as i32),
        _ => {
            Object::Error(Errors::LenInvalidTypeError(Box::new(args[0].clone())))
            },
    }
}

fn first(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(value) => {
            value[0].clone()
        },
        _ =>  Object::Error(Errors::FirstTypeError(Box::new(args[0].clone())))
    }
}

fn last(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(value) => {
            let length = value.len();
            value[length-1].clone()
        },
        _ =>  Object::Error(Errors::LastTypeError(Box::new(args[0].clone())))
    }
}

fn rest(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(value) => {
            let length = value.len();
            let mut array: Vec<Object> = Vec::new();
            for (i, v) in value.iter().enumerate(){
                array.push(v.clone());
                if i == length -2 {
                    return Object::Array(array)
                }
            }
            value[length-1].clone()
        },
        _ =>  Object::Error(Errors::RestTypeError(Box::new(args[0].clone())))
    }
}

fn push(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(value) => {
            let mut array = value.clone();
            array.push(args[1].clone());
            Object::Array(array.clone())
        },
        _ =>  Object::Error(Errors::PushTypeError(Box::new(args[0].clone())))
    }
}