
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
