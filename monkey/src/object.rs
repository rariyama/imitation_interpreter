use std::fmt;
use std::collections::BTreeMap;

use super::evaluator::{Environment};
use super::ast::{Expression, Statement};
use super::errors::{Errors};

#[derive(Debug,PartialEq, Clone, Eq, Ord, PartialOrd)]
pub enum Object {
    Identifier(String),
    String(String),
    Integer(i32),
    Boolean(bool),
    Return(Box<Object>),
    Let(Box<Object>),
    Array(Vec<Object>),
    Hash(BTreeMap<Box<HashKey>, Box<HashPair>>),
    Function{params: Vec<Expression>,
             body: Statement,
             env: Environment
            },
    Builtin{
        func: fn(Vec<Object>) -> Object
       },
    Error(Errors),
    Null,
    Default
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           Object::Identifier(value) => write!(f, "{}", value),
           Object::String(value) => write!(f, "{}", value),
           Object::Integer(value) => write!(f, "{}", value),
           Object::Boolean(value) => write!(f, "{}", value),
           Object::Return(value) => write!(f, "{}", value),
           Object::Let(value) => write!(f, "{}", value),
           Object::Hash(tree) => {
            match tree {
                key => write!(f, "{{{}}}", tree.iter().map(|(key, value)| format!("{}: {}", key, value)).collect::<Vec<_>>().join(", ")),
                _ =>  unreachable!()}
            },
           Object::Array(value) => write!(f, "[{}]", value.iter().map(|expression| format!("{}", &expression)).collect::<Vec<_>>().join(", ")),
           Object::Function{params, body, env} => write!(f, "{:?} {} {:?}", params, body, env),
           Object::Builtin{func: _} => write!(f, "builtin functions"),
           Object::Null => write!(f, ""),
           Object::Default => write!(f, "default"),
           Object::Error(value) => write!(f, "{}", value)
       }
    }
}

#[derive(Debug,PartialEq, Clone, Eq, Ord, PartialOrd)]
pub struct HashPair {
    pub key: Object,
    pub value: Object,
}

impl fmt::Display for HashPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug,PartialEq, Clone, Eq, Ord, PartialOrd)]
pub enum HashKey {
    Integer(i32),
    String(String),
    Boolean(bool),
    Null
}

impl fmt::Display for HashKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           HashKey::Integer(value) => write!(f, "{}", value),
           HashKey::String(value) => write!(f, "{}", value),
           HashKey::Boolean(value) => write!(f, "{}", value),
           HashKey::Null => write!(f, "null"),
       }
    }
}

impl HashKey {
    pub fn get_hashkey(key: &Object) -> Self {
        match key {
            Object::Integer(key) => HashKey::Integer(*key),
            Object::String(key) => HashKey::String(key.clone()),
            Object::Boolean(key) => HashKey::Boolean(*key),
            _ => HashKey::Null
        }
    }
}

