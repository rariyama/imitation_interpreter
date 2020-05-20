use super::token::{Token, TokenKind};
use super::lexer;
use super::ast::{Program, Statement, LetStatement, ReturnStatement, Expression, ExpressionStatement, ExpressionKind, Identifier, Integer};

#[derive(Debug, Clone)]
pub struct Parser<'a>  {
    lexer: lexer::Lexer<'a>,
    current_token: Token,
    next_token: Token,
}

impl<'a>  Parser<'a>  {
    pub fn new(l: lexer::Lexer<'a>) -> Self {
        // Goだと初期化時に省略するが、rustではできないためはじめにやる。

        let mut p = Parser{
            lexer: l,
            current_token: Token{token_type: TokenKind::DEFAULT, literal: "default".to_string() },
            next_token: Token{token_type: TokenKind::DEFAULT, literal: "default".to_string() },
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.next_token.clone(); //借用権問題でcloneする。
        self.next_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program{
        let mut statements: Vec<Statement> = vec![];

        // eofになるまでstatementsを配列に入れる。
        while !self.is_current_token(TokenKind::EOF){
            let statement = self.parse_statement();
//            println!("statement is : {:?}", statement);
            statements.push(statement);
            self.next_token();
        };
        // 読んだ式をprogramに入れて返す。
        Program {
            statements: statements
        }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token.token_type {
            TokenKind::LET => {
                Statement::LetStatement(self.parse_let_statement())
            },
            TokenKind::RETURN => {
                Statement::ReturnStatement(self.parse_return_statement())
            },
            _ => {
                Statement::ExpressionStatement(self.parse_expression_statement())
            }
        }
    }

    fn parse_let_statement(&mut self) -> LetStatement {
//        let current_token = self.current_token.clone();
        let identifier = self.next_token.clone();

        if !self.expect_next_token(TokenKind::IDENT) {
            return panic!()
        }

//        let name = Identifier {
//            value: self.current_token.literal.clone(),
//        };

        if !self.expect_next_token(TokenKind::ASSIGN) {
            return panic!()
        }

        // セミコロンまでの読み飛ばしをしてからstatementを定義して返す。
        while !self.is_current_token(TokenKind::SEMICOLON) {
            self.next_token()
        }
        let stmt = LetStatement {
            identifier: Identifier{
                value: identifier.literal
            }
        };
        return stmt
    }

    fn parse_return_statement(&mut self) -> ReturnStatement {
        let identifier = self.next_token.clone();
        let stmt = ReturnStatement {
            identifier: Identifier{
                value: identifier.literal
            }
        };
        // セミコロンまでの読み飛ばしをしてからstatementを定義して返す。
        while !self.is_current_token(TokenKind::SEMICOLON) {
            self.next_token()
        }
        return stmt
    }

    fn parse_expression_statement(&mut self) -> ExpressionStatement {
        let expression = self.parse_expression();

        if self.is_next_token(TokenKind::SEMICOLON) {
            self.next_token()
        }
        return ExpressionStatement{expression: expression}
    }

    fn parse_expression(&mut self) -> Expression {
        match self.current_token.token_type {
            TokenKind::IDENT => Expression::Identifier(self.parse_identifier()),
            TokenKind::INT => Expression::Integer(self.parse_integer()),
            _ => panic!()
        }
    }

    fn parse_identifier(&mut self) -> Identifier {
        return Identifier{value: self.current_token.literal.to_string()}
    }

    fn parse_integer(&mut self) -> Integer {
        return Integer{value: self.current_token.literal.to_string()}
    }

    fn is_current_token(&self, token_kind: TokenKind) -> bool {
        self.current_token.token_type == token_kind
    }

    fn is_next_token(&self, token_kind: TokenKind) -> bool {
        self.next_token.token_type == token_kind
    }

    fn expect_next_token(&mut self, token_kind: TokenKind) -> bool {
        let expect_token = self.is_next_token(token_kind);
        if expect_token {
            self.next_token();
        } else {
            println!("expect_token is {:?} but accually got {:?}", token_kind, self.next_token.token_type);
        }
        expect_token
    }
}

// if cfg(test) is written, test code is compiled only when test runs
#[cfg(test)]// test runs only when execute cargo run
mod testing {
    use crate::ast::ReturnStatement;
    use crate::lexer::Lexer;
    use crate::token::TokenKind;
    use crate::ast::Statement;
    use crate::ast::Expression;
    use crate::ast::ExpressionStatement;
    use crate::ast::LetStatement;
    use crate::ast::Identifier;
    use crate::ast::Integer;
    use crate::parser::Parser;

    #[test]
    fn test_let_statement() {
        let input = r#"let x = 5;
                       let y = 10;
                       let foobar = 838383;"#;
        
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
//        println!("{:?}", program);
        assert_eq!(program.statements.len(), 3);

        let tests = vec![
            "x",
            "y",
            "foobar",
        ];

        for (i, test) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
//            println!("{:?}", stmt);
            assert_eq!(stmt, &Statement::LetStatement(LetStatement{identifier: Identifier{value: test.to_string()}}));
        }
    }

    #[test]
    fn test_return_statement() {
        let input = r#"return 5;
                       return 10;
                       return 993322;"#;
        
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
//        println!("{:?}", program);
        assert_eq!(program.statements.len(), 3);

        let tests = vec![
            "5",
            "10",
            "993322",
        ];

        for (i, test) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            assert_eq!(stmt, &Statement::ReturnStatement(ReturnStatement{identifier: Identifier{value: test.to_string()}}));
        }
    }
    #[test]
    fn test_identifier_expression() {
        let input = "foobar;".to_string();
        
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 1); // 識別子が一つであること
//        println!("{:?}", program);
//        println!("{:?}", program.statements);
//        println!("{:?}", Statement::ExpressionStatement(ExpressionStatement{expression: Expression::Identifier(Identifier{value: "foobar".to_string()})}));
        let test_ident = Statement::ExpressionStatement(ExpressionStatement{expression: Expression::Identifier(Identifier{value: "foobar".to_string()})});
        assert_eq!(program.statements[0], test_ident)
        // let test_identifier = program.statements[0].ExpressionStatement;
        }

        #[test]
        fn test_interger_expression() {
            let input = "5;".to_string();
            
            let lexer = Lexer::new(&input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            assert_eq!(program.statements.len(), 1); // 識別子が一つであること
    //        println!("{:?}", program);
    //        println!("{:?}", program.statements);
    //        println!("{:?}", Statement::ExpressionStatement(ExpressionStatement{expression: Expression::Identifier(Identifier{value: "foobar".to_string()})}));
            let test_ident = Statement::ExpressionStatement(ExpressionStatement{expression: Expression::Integer(Integer{value: "5".to_string()})});
            assert_eq!(program.statements[0], test_ident)
            // let test_identifier = program.statements[0].ExpressionStatement;
            }
    }
