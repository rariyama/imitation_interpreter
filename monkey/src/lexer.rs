use super::token::{Token, TokenKind, get_keyword};


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

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Self::is_letter(&self.ch) {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap().to_string()
//        self.input[..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Self::is_digit(&self.ch) {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap().to_string()
    }

pub    fn is_letter(ch: &u8) -> bool {
        let ch = char::from(*ch);
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

pub    fn is_digit(ch: &u8) -> bool {
        let ch = char::from(*ch);
        '0' <= ch && ch <= '9'
    }

    fn skip_whitespace(&mut self) {
//        while self.ch == b' ' || self.ch == b'\n' {
    while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
        self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
//        println!("{}", self.ch.to_string());
//        println!("{}", b' ');
//        println!("char is {}",c);
        self.skip_whitespace();
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
                    if Self::is_letter(&self.ch) {
                        let ident = self.read_identifier();
                        let ident_token = get_keyword(&ident);
//                        println!("{:?}", ident);
//                        println!("{:?}", ident_token);
                            token =  Token {
                            Type: ident_token,
                            Literal: ident
                     };//ここでreturnしないと文字が一つ読み飛ばされる。
                     return token
                    } else if Self::is_digit(&self.ch) {
                        token =  Token {
                            Type: TokenKind::INT,
                            Literal: self.read_number()
                        };
                        return token
                    } else {
                    token = Self::new_token(TokenKind::ILLEGAL, self.ch);
                           }
                    }
                }
        self.read_char();
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
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y){
   x + y;
};
let result = add (five, ten);"#;
        let tests = vec![
               (TokenKind::LET, String::from("let")),
               (TokenKind::IDENT, String::from("five")),
               (TokenKind::ASSIGN, String::from("=")),
               (TokenKind::INT, String::from("5")),
               (TokenKind::SEMICOLON, String::from(";")),
               (TokenKind::LET, String::from("let")),
               (TokenKind::IDENT, String::from("ten")),
               (TokenKind::ASSIGN, String::from("=")),
               (TokenKind::INT, String::from("10")),
               (TokenKind::SEMICOLON, String::from(";")),
               (TokenKind::LET, String::from("let")),
               (TokenKind::IDENT, String::from("add")),
               (TokenKind::ASSIGN, String::from("=")),
               (TokenKind::FUNCTION, String::from("fn")),
               (TokenKind::LPAREN, String::from("(")),
               (TokenKind::IDENT, String::from("x")),
               (TokenKind::COMMA, String::from(",")),
               (TokenKind::IDENT, String::from("y")),
               (TokenKind::RPAREN, String::from(")")),
               (TokenKind::LBRACE, String::from("{")),
               (TokenKind::IDENT, String::from("x")),
               (TokenKind::PLUS, String::from("+")),
               (TokenKind::IDENT, String::from("y")),
               (TokenKind::SEMICOLON, String::from(";")),
               (TokenKind::RBRACE, String::from("}")),
               (TokenKind::SEMICOLON, String::from(";")),
               (TokenKind::LET, String::from("let")),
               (TokenKind::IDENT, String::from("result")),
               (TokenKind::ASSIGN, String::from("=")),
               (TokenKind::IDENT, String::from("add")),
               (TokenKind::LPAREN, String::from("(")),
               (TokenKind::IDENT, String::from("five")),
               (TokenKind::COMMA, String::from(",")),
               (TokenKind::IDENT, String::from("ten")),
               (TokenKind::RPAREN, String::from(")")),
               (TokenKind::SEMICOLON, String::from(";")),
               (TokenKind::EOF, String::from("")),
                ];

//    let mut l = Lexer::new(input.to_string());
    let mut lexer = Lexer::new(input);
//    println!("{:?}", lexer.position);
    for test in tests.iter() {
//        println!("{:?}", test.1);
        let _token = lexer.next_token();
        println!("{:?}", _token);
//        println!("{:?}", test.0);
//        println!("{:?}", test.1);
        assert_eq!(_token.Type,  test.0);
        assert_eq!(_token.Literal, test.1);
        }
    }
}