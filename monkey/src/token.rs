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

// if other module refers to Token, pub is needed to write 
#[derive(Debug)]
pub struct Token {
    pub Type: TokenKind,
    pub Literal: String
}
