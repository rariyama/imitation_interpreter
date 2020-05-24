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
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
    Block(Vec<Statement>),
    Parameter(Vec<Statement>),
    Arguments(Vec<Statement>),
}

//impl fmt::Display for Statement {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        match self {
//            Statement::LetStatement(
//                           LetStatement{
//                               identifier: Expression::Identifier(
//                                                           Identifier{value}
//                                                    )
//                                        }) =>write!(f, "let {};", value),
//            Statement::ReturnStatement(
//                ReturnStatement{
//                    identifier: Expression::Identifier(
//                                                Identifier{value}
//                                         )
//                             }) =>write!(f, "return {};", value),
//            Statement::ExpressionStatement(
//                ExpressionStatement{
//                   expression: Expression::Identifier(
//                                               Identifier{value}
//                                        )
//                            }) =>write!(f, "{};", value),
//        _ => write!(f, "none")
//                    }
//                }
//            }


#[derive(Debug,PartialEq)]
pub struct ReturnStatement {
    pub identifier: Expression
}

#[derive(Debug,PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression
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
