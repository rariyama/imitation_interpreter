use std::fmt;


pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug,PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>
}

#[derive(Debug,PartialEq)]
pub enum Statement {
    LetStatement{identifier: Expression,
                 value: Expression},
    Return(Expression),
    ExpressionStatement(Expression),
    Block(Vec<Statement>),
    Parameter(Vec<Statement>),
    Arguments(Vec<Statement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::LetStatement{
                               identifier, value
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
            Statement::Parameter(Statements) => {
                for stmt in Statements.iter()
                    {
                     write!(f, "{}", stmt)?;
                    }
               Ok(())  
               },
            Statement::Arguments(Statements) => {
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
    Identifier(Identifier),
    Integer(Integer),
    LParen(LParen),
    Bool(Bool),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(Identifier{value}) => write!(f, "{}",&value),
            Expression::Integer(Integer{value}) => write!(f, "{}",value),
            Expression::LParen(LParen{value}) => write!(f, "{}",value),
            Expression::Bool(Bool{value}) => write!(f, "{}",value),
            Expression::PrefixExpression(PrefixExpression{operator,right_expression}) => write!(f, "{}{}",operator, right_expression),
            Expression::InfixExpression(InfixExpression{left_expression,operator,right_expression}) => write!(f, "{} {} {}",left_expression, operator, right_expression),
            Expression::IfExpression(IfExpression{condition, consequence, alternative}) => write!(f, "{} {} {}",condition, consequence, alternative),
            Expression::FunctionLiteral(FunctionLiteral{parameters, body}) => write!(f, "{} {} ",parameters, body),
            Expression::CallExpression(CallExpression{function, body}) => write!(f, "{} {:?} ",function, body),
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Identifier {
    pub value: String
}

#[derive(Debug,PartialEq)]
pub struct Integer {
    pub value: i32
}

#[derive(Debug,PartialEq)]
pub struct LParen {
    pub value: String
}

#[derive(Debug,PartialEq)]
pub struct Bool {
    pub value: bool
}

#[derive(Debug,PartialEq)]
pub struct PrefixExpression {
    pub operator: String,
    pub right_expression: Box<Expression>
}

#[derive(Debug,PartialEq)]
pub struct InfixExpression {
    pub left_expression: Box<Expression>,
    pub operator: String,
    pub right_expression: Box<Expression>
}

#[derive(Debug,PartialEq)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: Box<Statement>,
    pub alternative: Box<Statement>
}

#[derive(Debug,PartialEq)]
pub struct FunctionLiteral {
    pub parameters: Box<Statement>,
    pub body: Box<Statement>,
}

#[derive(Debug,PartialEq)]
pub struct CallExpression {
    pub function: Box<Expression>,
    pub body: Vec<Expression>,
}

#[derive(Debug,PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>
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
