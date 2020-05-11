use super::token::{Token, TokenKind};

pub struct Lexer {
    input:        String,
    position:     usize, // inputに対する現在の位置
    readPosition: usize, // inputに対する次の読み込みの位置
    ch:           u8, // 現在検査中の文字
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer{input: input.to_string(),
                      position: 0,
                      readPosition: 0,
                      ch: 0
                    };
        l.read_char();
        return l
    }

    fn read_char(&mut self) {
    if self.readPosition == self.input.len() {
        self.ch = 0;
    } else {
        self.ch = self.input.as_bytes()[self.readPosition];
        }
    self.position = self.readPosition;
    self.readPosition += 1;
    }

    pub fn new_token(TokenKind: TokenKind, ch: u8)-> Token { //返り値にtoken.Tokenと指定するとダメ...
        Token{Type: TokenKind,
              Literal: ch.to_string()
              }
    }

    fn next_token(&mut self) -> Token {
        let tok: Token = match self.ch {
            b'=' => Lexer::new_token(TokenKind::ASSIGN, Self.ch),
            b';' => Lexer::new_token(TokenKind::SEMICOLON, Self.ch),
            b'(' => Lexer::new_token(TokenKind::LPAREN, Self.ch),
            b')' => Lexer::new_token(TokenKind::RPAREN, Self.ch),
            b',' => Lexer::new_token(TokenKind::COMMA, Self.ch),
            b'+' => Lexer::new_token(TokenKind::PLUS, Self.ch),
            b'{' => Lexer::new_token(TokenKind::LBRACE, Self.ch),
            b'}' => Lexer::new_token(TokenKind::RBRACE, Self.ch),
            _   => Token{Type: TokenKind::EOF,
                               Literal: "".to_string()
                        }
        };
    self.read_char();
    return tok
    }
}




// if cfg(test) is written, test code is compiled only when test runs
#[cfg(test)]// test runs only when execute cargo run
mod testing {
    use super::Lexer;//if you need to refer the code inside this file, you can use 'super'
    use super::TokenKind;

    #[test]
    fn test_next_token() {
        let input = r#"=+(){},;"#.to_string();

        struct TokenTest {
            expectedToken: TokenKind,
            expectedLiteral: String
        }
        let tests = [
               TokenTest {expectedToken: TokenKind::ASSIGN, expectedLiteral: "=".to_string()},
               TokenTest {expectedToken: TokenKind::PLUS, expectedLiteral: "+".to_string()},
               TokenTest {expectedToken: TokenKind::LPAREN, expectedLiteral: "(".to_string()},
               TokenTest {expectedToken: TokenKind::RPAREN, expectedLiteral: ")".to_string()},
               TokenTest {expectedToken: TokenKind::LBRACE, expectedLiteral: "{".to_string()},
               TokenTest {expectedToken: TokenKind::RBRACE, expectedLiteral: "}".to_string()},
               TokenTest {expectedToken: TokenKind::COMMA, expectedLiteral: ",".to_string()},
               TokenTest {expectedToken: TokenKind::SEMICOLON, expectedLiteral: ";".to_string()},
               TokenTest {expectedToken: TokenKind::EOF, expectedLiteral: "".to_string()},
                ];

    let mut l = Lexer::new(input.to_string());
    for test in &tests {
        let token = l.next_token();
        assert_eq!(token.Type,  test.expectedToken);
        assert_eq!(token.Literal,  test.expectedLiteral);
        }
    }
}