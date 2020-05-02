enum TokenKind {
     ILLEGAL
    ,EOF
    ,IDENT
    ,Number(u64)
    ,ASSIGN
    ,PLUS
    ,COMMA
    ,SEMICOLON
    ,LPAREN
    ,RPAREN
    ,LBRACE
    ,RBRACE
    ,FUNCTION
    ,LET

}

struct Token {
    Type: TokenKind
    Literal: string
}

//impl Token {
//    fn number(n: u64, literal: Literal) -> Self {
//        Self::new(TokenKind::Number(n))
//    }
//
//    fn plus(literal: Literal) -> Self {
//        Self::new(TokenKind::PLUS, Literal)
//    }
//
//    fn plus(literal: Literal) -> Self {
//        Self::new(TokenKind::PLUS, Literal)
//    }
//}

