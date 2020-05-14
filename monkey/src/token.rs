
#[derive(Debug,Eq,PartialEq)]
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
}

pub fn get_keyword(ident: &str) -> TokenKind {
//    let _let = "let".to_string();
//    let _fn = "fn".to_string();
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
#[derive(Debug)]
pub struct Token {
    pub Type: TokenKind,
    pub Literal: String
}
