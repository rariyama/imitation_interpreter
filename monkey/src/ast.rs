use crate::{lexer, token};


pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self) -> String;
}

pub trait Expression: Node {
    fn expression_node(&self) -> String;
}


#[derive(Debug,Eq,PartialEq)]
pub struct Program<T: Statement> {
    pub statements: Vec<T>
}

// Progtramの引数にStatementTを指定する。
// Goのソースコードがinterface型なため<T>にしてみる。
impl <T: Statement> Node for Program<T> {
    fn token_literal(& self)  -> String {
        if self.statements.len() > 0 {
            return self.token_literal()
        }
        else {
            return "".to_string()
        }
    }
}
// valueはinterface型にするため<T>にする。
pub struct LetStatement<T: Expression> {
    Token: token::Token, // let token
    Name:   Identifier,
    Value:  T
}

impl <T: Expression> Node for LetStatement<T> {
    fn token_literal(&self) -> String {
        return self.Token.literal.clone() //cloneを使うことで借用権エラーを防ぐ。
    }
}


pub struct Identifier {
    Token: token::Token, // token.IDENT token
    Value: String
}

impl Identifier {
    pub fn expression_node(&mut self) {}
    pub fn token_literal(&mut self) -> String {
        return self.Token.literal.clone()
    } 
}





