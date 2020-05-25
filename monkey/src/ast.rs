use std::fmt;


pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug,PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for stmt in self.statements.iter() {
                write!(f, "{}\n", stmt)?;  
            }
            Ok(())  
    }
}

#[derive(Debug,PartialEq)]
pub enum Statement {
    LetStatement{identifier: Expression,
                 value: Expression},
    Return(Expression),
    ExpressionStatement(Expression),
    Block(Vec<Statement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::LetStatement{
                               identifier,
                               value
                                    } =>write!(f, "let {} = {};",identifier,  value),
            Statement::Return(Expression) =>write!(f, "return {}", Expression),
            Statement::ExpressionStatement(Expression) =>write!(f, "{}", Expression),
            Statement::Block(Statements) => {
                                             for stmt in Statements.iter()
                                                 {
                                                  write!(f, "{}", stmt)?;
                                                 }
                                            Ok(())  
                                            },
           _ => write!(f, "none")
                    }
                }
            }

#[derive(Debug,PartialEq)]
pub enum Expression {
    Identifier(String),
    Integer(i32),
    LParen(String),
    Bool(bool),
    PrefixExpression{operator: String,
                     right_expression: Box<Expression>
                     },
    InfixExpression{left_expression: Box<Expression>,
                    operator: String,
                    right_expression: Box<Expression>
                   },
    IfExpression{condition: Box<Expression>,
                 consequence: Box<Statement>,
                 alternative: Box<Statement>
                },
    FunctionLiteral{parameters: Vec<Expression>,
                    body: Box<Statement>,
                   },
    CallExpression{function: Box<Expression>,
                    body: Vec<Expression>
                  }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(value) => write!(f, "{}",&value),
            Expression::Integer(value) => write!(f, "{}",value),
            Expression::LParen(value) => write!(f, "{}",value),
            Expression::Bool(value) => write!(f, "{}",value),
            Expression::PrefixExpression{operator,right_expression} => write!(f, "{}{}",operator, right_expression),
            Expression::InfixExpression{left_expression,operator,right_expression} => write!(f, "{} {} {}",left_expression, operator, right_expression),
            Expression::IfExpression{condition, consequence, alternative} => write!(f, "if ({}) {{{}}} else {{{}}}",condition, consequence, alternative),
            Expression::FunctionLiteral{parameters, body} => write!(f, "fn ({}) {{{}}}",parameters.iter().map(|expression| -> &str {
                                                                                                                        match expression {
                                                                                                                            Expression::Identifier(identifier) => identifier,
                                                                                                                            _ => unreachable!(),
                                                                                                                        }}).collect::<Vec<_>>().join(", ")
                                                                                                                      , body),
            Expression::CallExpression{function, body} => write!(f, "{}({});",
                                                                function,
                                                                body.iter().map(|expression| format!("{}", &expression)).collect::<Vec<_>>().join(", "),
                                                                ),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,      
    EQUALS,       // ==
    LESSGREATER,  // > or <
    SUM,          // +
    PRODUCT,      // *
    PREFIX,       // -X or !X
    CALL          // my_cunction(x){}
}
