use std::collections::HashMap;
use crate::ast;
use super::object::{Object};
use super::errors::{Errors};
use super::ast::{Expression};

#[derive(Debug,PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>
}

impl Environment {
    pub fn new() -> Environment{
        let env = HashMap::new();
        return Environment{store: env}
    }

    pub fn get(&mut self, name: String) -> Object {
        let object = self.store[&name].clone();
        return object
    }

    pub fn set(&mut self, name: String, value: Object) -> Object {
        self.store.insert(name, value.clone());
        return value;
    }


    pub fn evaluate(&mut self, program: &ast::Program) -> Result<Object, Errors> {
        let mut result = Object::Default;
        // evaluate sentence per semicolon.
        for statement in program.statements.iter() {
            result = self.evaluate_statement(statement)?;
            // if statement contains 'return', process should be broken in order to return value.
            if let Object::Return(value) = result {
                return Ok(*value)
            }
            // if the result of evaluation, process should be broken.
            if result == Object::Error(Errors::InvalidInfix){
                return Ok(result)
            }
        }
        Ok(result)
    }

    fn evaluate_statement(&mut self, statement: &ast::Statement) -> Result<Object, Errors> {
        match statement {
            ast::Statement::ExpressionStatement(expression) => self.evaluate_expression(expression),
            ast::Statement::Block(stmt) => self.evaluate_block_statements(stmt),
            ast::Statement::Return(expression) => {
                                    let return_expression = self.evaluate_expression(expression)?;
                                    Ok(Object::Return(Box::new(return_expression)))
                                    },
            ast::Statement::LetStatement{identifier ,value} => {
                                                 if let Expression::Identifier(identifier) = identifier {
                                                    // if expression is identifier, evaluate value, and 
                                                    // append identifier as variable.
                                                    let evaluated_value = self.evaluate_expression(&value)?;
                                                    let value = self.set(identifier.to_owned(), evaluated_value);
                                                    return Ok(value)
                                                 }
                                                 Ok(Object::Null)
                                                },
            _ => Err(Errors::NodeError),
            }
        }

    fn evaluate_block_statements(&mut self, statements: &Vec<ast::Statement>) -> Result<Object, Errors> {
        let mut result = Object::Default;
        for statement in statements.iter() {
            result = self.evaluate_statement(statement)?;
            // if 'return' is in nested block, the value should be returned.
            if let Object::Return(_) = result {
                return Ok(result);
            }
        }
        Ok(result)
    }

    fn evaluate_expression(&mut self, expression: &ast::Expression) -> Result<Object, Errors> {
        match expression {
            ast::Expression::Identifier(value) => {
                // if a key exists in Environment map,
                // get value which is equivalent to it.
                match self.get(value.clone()) {
                    value => {
                        Ok(value)},
                    _ => Ok(Object::Null)
                    }
                },
            ast::Expression::Integer(value) => Ok(Object::Integer(*value)),
            ast::Expression::Bool(bool) => Ok(Object::Boolean(*bool)),
            ast::Expression::PrefixExpression{operator, right_expression} => {
                let right = self.evaluate_expression(&right_expression);
                evaluate_prefix_expression(operator, right.unwrap())
            },
            ast::Expression::InfixExpression{left_expression, operator, right_expression} => {
                // if there are more than two calculations, left expression should be a calculation.
                // it is firstly evaluated, and then the result and right_expression are calculated.
                // for example, the whole sentence is 1 + 2 + 5. firstly, 1 + 2 is evaluated and
                // the result is 3. After that the result and 5 is evaluated.
                let left = self.evaluate_expression(&left_expression);
                let right = self.evaluate_expression(&right_expression);
                evaluate_infix_expression(left.unwrap(), operator, right.unwrap())
            },
            ast::Expression::IfExpression{condition, consequence, alternative} => {
                let condition = self.evaluate_expression(&condition);
                if is_truthy(condition?) {
                    self.evaluate_statement(consequence)
                } else {
                    match alternative {
                        Some(alternative) => self.evaluate_statement(alternative),
                        None => Ok(Object::Null)
                    }
                }
            }
            _ =>  Err(Errors::NodeError)
        }
    }
}

fn evaluate_prefix_expression(operator: &str, right: Object) -> Result<Object, Errors> {
    match operator {
        "!" => evaluate_bang_operation_expression(right),
        "-" => evaluate_minus_prefix_operator_expression(right),
        _ => Ok(Object::Error(Errors::InvalidOperator(operator.to_string())))
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
        _ =>Ok(Object::Error(Errors::InvalidInteger(Box::new(right))))
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
                _ => Ok(Object::Error(Errors::InvalidOperator(operator.to_string())))
            }
        },
        (Object::Boolean(left), Object::Boolean(right)) => {
            match operator {
                "==" => Ok(Object::Boolean(left == right)),
                "!=" => Ok(Object::Boolean(left != right)),
                _ => Ok(Object::Error(Errors::InvalidOperator(operator.to_string())))
            }
        },
        _ => {
            Ok(Object::Error(Errors::InvalidInfix))}
    }
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Null => false,
        Object::Boolean(true) => true,
        Object::Boolean(false) => false,
        _ => true
    }
}

#[cfg(test)]// test runs only when execute cargo run
mod testing {
    use crate::lexer::Lexer;
    use crate::evaluator::Environment;
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
        let program = p.parse_program();
        let mut environment = Environment::new();
        environment.evaluate(&program.unwrap()).unwrap()
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

    #[test]
    fn test_if_else_expression() {
        let tests = vec![
                        ("if (true) {10}", "10"),
                        ("if (false) {10}", "null"),
                        ("if (1) {10}", "10"),
                        ("if (1 < 2) { 10 }", "10"),
                        ("if (1 > 2) {10}", "null"),
                        ("if (1 > 2) {10} else {20}", "20"),
                        ("if (1 < 2) {10} else {20}", "10"),
                        ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let conclusion = format!("{}", evaluated);
            assert_eq!(conclusion, test.1);
        }
    }

    #[test]
    fn test_return_statement() {
        let tests = vec![
                        ("return 10;", "10"),
                        ("return 10; 9;", "10"),
                        ("return 2 * 5; 9;", "10"),
                        ("9; return 2 * 5;", "10"),
                        ("if (1 > 2) {10}", "null"),
                        ("if (10 > 1){
                             if (10 > 1){
                                return 10;
                                        }
                                      }", "10")
                        ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let return_value = format!("{}", evaluated);
            assert_eq!(return_value, test.1);
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
                        ("5 + true", "invalid_infix"),
                        ("5 + true; 5;", "invalid_infix"),
                        ("-true", "invalid integer: true"),
                        ("true + false;", "invalid operator: +"),
                        ("5; true + false;", "invalid operator: +"),
                        ("if (10 > 1) {true + false;}", "invalid operator: +"),
                        ("if (10 > 1){
                            if (10 > 1) {
                               return true + false;
                                        }
                                return 1;
                                     }", "invalid operator: +")
                        ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let return_value = format!("{}", evaluated);
            println!("{}", return_value);
            assert_eq!(return_value, test.1);
        }
    }

    #[test]
    fn test_let_statements() {
        let tests = vec![
                        ("let a = 5; a;", "5"),
                        ("let a = 5 * 5; a;", "25"),
                        ("let a = 5 ; let b = a; b;", "5"),
                        ("let a = 5 ; let b = a; let c = a + b + 5; c;", "15"), 
                        ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let return_value = format!("{}", evaluated);
            assert_eq!(return_value, test.1);
        }
    }
}