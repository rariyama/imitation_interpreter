use std::fmt;

use super::ast::{Expression};

#[derive(Debug,PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Null,
    Default
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           Object::Integer(value) => write!(f, "{}", value),
           Object::Boolean(value) => write!(f, "{}", value),
           Object::Null => write!(f, "null"),
           Object::Default => write!(f, "default")
       }
    }
}

pub enum ObjectType {
    INTEGER_OBJ,
    BOOLEAN_OBJ,
    NULL_OBJ,
    ILLEGAL
}

pub fn get_object_type(object_type: &str) -> ObjectType {
    match object_type {
        "INTEGER" => ObjectType::INTEGER_OBJ,
        "BOOLEAN" => ObjectType::BOOLEAN_OBJ,
        "NULL" => ObjectType::NULL_OBJ,
        _ => ObjectType::ILLEGAL
    }
}

#[derive(Debug,PartialEq)]
pub struct Integer {
    value: i32
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(write!(f, "{}", self.value))?
    }
}

impl Integer {
    pub fn inspect(&mut self) -> i32 {
        self.value
    }
    pub fn fetch_type(&mut self) -> ObjectType {
        ObjectType::INTEGER_OBJ
    }
}

pub struct Boolean {
    value: bool
}

impl Boolean {
    pub fn inspect(&mut self) -> bool {
        self.value
    }
    pub fn fetch_type(&mut self) -> ObjectType {
        ObjectType::BOOLEAN_OBJ
    }
}

pub struct Null {
}

impl Null {
    pub fn inspect(&mut self) -> String {
        "null".to_string()
    }
    pub fn fetch_type(&mut self) -> ObjectType {
        ObjectType::NULL_OBJ
    }
}
