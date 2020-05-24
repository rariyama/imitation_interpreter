use super::ast::{Precedence};

#[derive(Debug,Eq,PartialEq, Clone, Copy)]
pub enum TokenKind {
     ILLEGAL,     // ILLEGAL
     EOF,         // EOF

    // identifier and literal
     IDENT,       // IDENT
     INT,         // 123...

    // operator
     ASSIGN,      // =
     PLUS,        // +
     MINUS,       // -
     BANG,        // !
     ASTERISK,    // *
     SLASH,       // /
     LT,          // <
     GT,          // >
     EQ,          // ==
     NotEq,      // !=

     // delimiter
     COMMA,       // ,
     SEMICOLON,   // ;

     LPAREN,      // (
     RPAREN,      // )
     LBRACE,      // {
     RBRACE,      // }

     // keyword
     FUNCTION,    // FUNCTION
     LET,         // LET
     TRUE,        // true
     FALSE,       // false
     IF,          // if
     ELSE,        // else
     RETURN,      // return

     DEFAULT,
}

pub fn get_keyword(ident: &str) -> TokenKind {
    match ident {
        "let" => {
            TokenKind::LET
        }
        "fn" => {
            TokenKind::FUNCTION
        }
        "true" => {
            TokenKind::TRUE
        }
        "false" => {
            TokenKind::FALSE
        }
        "if" => {
            TokenKind::IF
        }
        "else" => {
            TokenKind::ELSE
        }
        "return" => {
            TokenKind::RETURN
        }
        _ => {
            TokenKind::IDENT
        }
    }
}

// if other module refers to Token, pub is needed to write 
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenKind,
    pub literal: String
}

impl Token {
    pub fn get_precedence(&mut self) -> Precedence {
        match self.token_type {
            TokenKind::EQ => Precedence::EQUALS,
            TokenKind::NotEq => Precedence::EQUALS,
            TokenKind::LT => Precedence::LESSGREATER,        
            TokenKind::GT => Precedence::LESSGREATER,        
            TokenKind::PLUS => Precedence::SUM,
            TokenKind::MINUS => Precedence::SUM,
            TokenKind::SLASH => Precedence::PRODUCT,
            TokenKind::ASTERISK => Precedence::PRODUCT,
            TokenKind::LPAREN => Precedence::CALL,
            _                   => Precedence::LOWEST
        }
    }
}