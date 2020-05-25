use crate::ast;
use super::object::{Object};
use super::errors::{Errors};



pub fn evaluate(program: &ast::Program) -> Result<Object, Errors> {
    let mut result = Object::Default;

    for statement in program.statements.iter() {
        result = evaluate_statement(statement)?;
    }
    Ok(result)
}

fn evaluate_statement(statement: &ast::Statement) -> Result<Object, Errors> {
    match statement {
        ast::Statement::ExpressionStatement(expression) => evaluate_expression(expression),
        _ => Err(Errors::NodeError)
        }
    }

fn evaluate_expression(expression: &ast::Expression) -> Result<Object, Errors> {
    match expression {
        ast::Expression::Integer(value) => Ok(Object::Integer(*value)),
        ast::Expression::Bool(bool) => Ok(Object::Boolean(*bool)),
        ast::Expression::PrefixExpression{operator, right_expression} => {
            let right = evaluate_expression(&right_expression);
            evaluate_prefix_expression(operator, right.unwrap())
        },
        ast::Expression::InfixExpression{left_expression, operator, right_expression} => {
            let left = evaluate_expression(&left_expression);
            let right = evaluate_expression(&right_expression);
            evaluate_infix_expression(left.unwrap(), operator, right.unwrap())
        },
        _ =>  Err(Errors::NodeError)
    }
}

fn evaluate_prefix_expression(operator: &str, right: Object) -> Result<Object, Errors> {
    match operator {
        "!" => evaluate_bang_operation_expression(right),
        "-" => evaluate_minus_prefix_operator_expression(right),
        _ => Ok(Object::Null)
    }
}

fn evaluate_bang_operation_expression(right: Object) -> Result<Object, Errors> {
    match right {
        Object::Boolean(true) => Ok(Object::Boolean(false)),
        Object::Boolean(false) => Ok(Object::Boolean(true)),
        Object::Boolean(Null) => Ok(Object::Boolean(true)),
        _ => Ok(Object::Boolean(false))
    }
}

fn evaluate_minus_prefix_operator_expression(right: Object) -> Result<Object, Errors> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(-value)),
        _ =>Ok(Object::Null)
    }
}

fn evaluate_infix_expression(left: Object,operator: &str, right: Object) -> Result<Object, Errors> {
    match (left, right) {
        (Object::Integer(left),Object::Integer(right)) => {
            match operator {
                "+" => Ok(Object::Integer(left + right)),
                "-" => Ok(Object::Integer(left - right)),
                "*" => Ok(Object::Integer(left * right)),
                "/" => Ok(Object::Integer(left / right)),
                "<" => Ok(Object::Boolean(left < right)),
                ">" => Ok(Object::Boolean(left > right)),
                "==" => Ok(Object::Boolean(left == right)),
                "!=" => Ok(Object::Boolean(left != right)),
                _ => Ok(Object::Null)
            }
        },
        (Object::Boolean(left), Object::Boolean(right)) => {
            match operator {
                "==" => Ok(Object::Boolean(left == right)),
                "!=" => Ok(Object::Boolean(left != right)),
                _ => Ok(Object::Null)
            }
        }
        _ =>Ok(Object::Null)
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
                        ("10", 10),
                        ("-5", -5),
                        ("-10", -10),
                        ("5 + 5 + 5 + 5 - 10", 10),
                        ("2 * 2 * 2 * 2 * 2", 32),
                        ("-50 + 100 + -50", 0),
                        ("5 * 2 + 10", 20),
                        ("5 + 2 * 10", 25),
                        ("20 + 2 * -10", 0),
                        ("50 / 2 * 2 + 10", 60),
                        ("2 * (5 + 10)", 30),
                        ("3 * 3 * 3 + 10", 37),
                        ("3 * (3 * 3) + 10", 37),
                        ("(5 + 10 * 2 + 15 /3) * 2 + -10", 50),
                        ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let integer = format!("{}", evaluated);
            assert_eq!(integer.parse::<i32>().unwrap(), test.1)
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = vec![
                        ("true", true),
                        ("false", false),
                        ("1 < 2", true),
                        ("1 > 2", false),
                        ("1 < 1", false),
                        ("1 > 1", false),
                        ("1 == 1", true),
                        ("1 != 1", false),
                        ("1 == 2", false),
                        ("1 != 2", true),
                        ("true == true", true),
                        ("false == false", true),
                        ("true == false", false),
                        ("true != false", true),
                        ("false != true", true),
                        ("(1 < 2) == true", true),
                        ("(1 < 2) == false", false),
                        ("(1 > 2) == true", false),
                        ("(1 > 2) == false", true),
                        ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let boolean = format!("{}", evaluated);
            assert_eq!(FromStr::from_str(&boolean.to_string()[..]), Ok(test.1));
        }
    }

    #[test]
    fn test_bang_operator_expression() {
        let tests = vec![
                        ("!true", false),
                        ("!false", true),
                        ("!5", false),
                        ("!!true", true),
                        ("!!false", false),
                        ("!!5", true),
                        ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let boolean = format!("{}", evaluated);
            assert_eq!(FromStr::from_str(&boolean.to_string()[..]), Ok(test.1));
        }
    }
}