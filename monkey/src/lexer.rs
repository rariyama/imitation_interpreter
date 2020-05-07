use super::token::{Token,TokenKind};

struct Lexer<'a> {
    input: &'a String,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    ch: char,
    eof: bool,
}


// if cfg(test) is written, test code is compiled only when test runs
#[cfg(test)]// test runs only when execute cargo run
mod testing {
    use super::Lexer;//if you need to refer the code inside this file, you can use 'super'
    use super::Token;
}
    #[test]
    fn test_next_token() {
        let input = r#"=+(){},;"#;

        struct TokenTest {
            expectedToken: TokenType,
            expectedLiteral: String
        }
    }