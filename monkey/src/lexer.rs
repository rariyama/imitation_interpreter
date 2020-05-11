use super::token::{Token, TokenKind};


pub struct Lexer<'a>  {
    input:        &'a str,
    position:     usize, // inputに対する現在の位置
    readPosition: usize, // inputに対する次の読み込みの位置
    ch:           u8, // 現在検査中の文字
}

impl<'a>  Lexer<'a>  {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer{
                      input,
                      position: 0,
                      readPosition: 0,
                      ch: 0
                    };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
    if self.readPosition >= self.input.len() {
        self.ch = 0;
    } else {
        self.ch = self.input.as_bytes()[self.readPosition];
        }
    self.position = self.readPosition;
    self.readPosition += 1;
    }

    pub fn new_token(Type: TokenKind, ch: u8)-> Token { //返り値にtoken.Tokenと指定するとダメ...
        Token {
              Type,
              Literal: String::from_utf8(vec![ch]).unwrap(), //ch.to_string()
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token;
        match self.ch {
            b'=' => {
                token = Self::new_token(TokenKind::ASSIGN, self.ch);
            }
            b';' => {
                token = Self::new_token(TokenKind::SEMICOLON, self.ch);
            }
            b'(' => {
                token = Self::new_token(TokenKind::LPAREN, self.ch);
            }
            b')' => {
                token = Self::new_token(TokenKind::RPAREN, self.ch);
            }
            b',' => {
                token = Self::new_token(TokenKind::COMMA, self.ch);
            }
            b'+' => {
                token = Self::new_token(TokenKind::PLUS, self.ch);
            }
            b'{' => {
                token = Self::new_token(TokenKind::LBRACE, self.ch);
            }
            b'}' => {
                token = Self::new_token(TokenKind::RBRACE, self.ch);
            }
            0 => {
                token = Token {
                       Type:  TokenKind::EOF,
                       Literal: String::from(""),
                };
            }
            _   => {
                token = Self::new_token(TokenKind::ILLEGAL, self.ch);
            }
        }
        self.read_char();
        println!("{:?}", token);
        return token;
    }
}

// if cfg(test) is written, test code is compiled only when test runs
#[cfg(test)]// test runs only when execute cargo run
mod testing {
    use crate::lexer::Lexer;
    use crate::token::TokenKind;
    //    use lexer::Lexer;//if you need to refer the code inside this file, you can use 'super'
//    use token::TokenKind;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = vec![
               (TokenKind::ASSIGN, String::from("=")),
               (TokenKind::PLUS, String::from("+")),
               (TokenKind::LPAREN, String::from("(")),
               (TokenKind::RPAREN, String::from(")")),
               (TokenKind::LBRACE, String::from("{")),
               (TokenKind::RBRACE, String::from("}")),
               (TokenKind::COMMA, String::from(",")),
               (TokenKind::SEMICOLON, String::from(";")),
               (TokenKind::EOF, String::from("")),
                ];

//    let mut l = Lexer::new(input.to_string());
    let mut lexer = Lexer::new(input);
//    println!("{:?}", lexer.position);
    for test in tests.iter() {
        let _token = lexer.next_token();
        assert_eq!(_token.Type,  test.0);
        assert_eq!(_token.Literal, test.1);
        }
    }
}