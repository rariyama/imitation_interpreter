struct Lexer {
    input char
    position usize
    readPosition usize
    ch u8

}

impl Lexer {
    fn New(input: char) -> *Lexer {
        l := &Lexer{input: input}
        return l
    }
    
    fn readChar(Lexer: *Lexer) {
        if 
    }
}


#[cfg(test)]// test runs only when execute cargo run
mod testing {
    use lexer::Lexer;
    use token::Token;
}
    #[test]
    fn TestNextToken() {
        let input = r#"=+(){},;"#;

        tests = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
        ];
    }
}