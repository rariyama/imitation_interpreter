use crate::ast;
use super::object;
use super::errors::{Errors};



fn evaluate(program: &ast::Program) -> Result<object::Object, Errors> {
    let mut result = object::Object::Default;

    for statement in program.statements.iter() {
        result = evaluate_statement(statement)?;
    }
    Ok(result)
}

fn evaluate_statement(statement: &ast::Statement) -> Result<object::Object, Errors> {
    match statement {
        ast::Statement::ExpressionStatement(expression) => evaluate_expression(expression),
        _ => Err(Errors::NodeError)
        }
    }

fn evaluate_expression(expression: &ast::Expression) -> Result<object::Object, Errors> {
    match expression {
        ast::Expression::Integer(value) => Ok(object::Object::Integer(*value)),
        _ =>  Err(Errors::NodeError)
    }
}

#[cfg(test)]// test runs only when execute cargo run
mod testing {
    use crate::lexer::Lexer;
    use crate::token::TokenKind;
    use crate::ast::Statement::Block;
    use crate::ast::Statement;
    use crate::ast::Expression;
    use crate::parser::Parser;
    use crate::evaluator;
    use crate::object::Object;
    use std::str::FromStr;

    fn test_evaluate(input: &str) -> Object {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        evaluator::evaluate(&program).unwrap()
    }

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
                        ("5", 5),
                        ("10", 10)
                        ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let integer = format!("{}", evaluated);
            assert_eq!(integer.parse::<i32>().unwrap(), test.1)
        }
    }

}