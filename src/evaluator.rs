use std::fmt;
use std::collections::BTreeMap;

use crate::ast;
use super::object::{Object, HashKey, HashPair};
use super::errors::{Errors};
use super::ast::{Expression};
use super::builtins;

#[derive(Debug,PartialEq, Clone, Eq, Ord, PartialOrd)]
pub struct Environment {
    store: BTreeMap<String, Object>,
    outer: Option<Box<Environment>>,
    builtin: BTreeMap<String, Object>
}

impl Environment {
    pub fn new() -> Environment{
        let env = BTreeMap::new();
        let builtins = builtins::new();
        return Environment{store: env, outer: None, builtin: builtins}
    }

    pub fn new_outer(self) -> Environment {
        return Environment{store: BTreeMap::new(), outer: Some(Box::new(self.clone())), builtin: builtins::new()}
    }

    pub fn get(& self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(value) => Some(value.clone()),
            None => match &self.outer {
                Some(outer) => outer.get(name),
                None => None
            }
        }
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
            // if statement contains 'return', process should be broken and return value.
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
                match self.get(value) {
                    Some(value) => {
                        Ok(value)},
                    _ => {
                        match self.builtin.get(value) {
                            Some(value) => {
                                Ok(value.to_owned())
                            },
                            None => {
                                Ok(Object::Null)}
                        }
                        }
                    }
                },
            ast::Expression::String(value) => Ok(Object::String(value.to_owned())),
            ast::Expression::Integer(value) => Ok(Object::Integer(*value)),
            ast::Expression::Bool(bool) => Ok(Object::Boolean(*bool)),
            ast::Expression::Array(value) =>{
                let array = self.evaluate_arguments(value.to_vec())?;
                Ok(Object::Array(array))
            },
            ast::Expression::IndexExpression{array, subscript} => {
                                                        let array = self.evaluate_expression(array)?;
                                                        let index = self.evaluate_expression(subscript)?;
                                                        Ok(evaluate_index_expression(array, index))
                                                        },
            ast::Expression::Hashmap(value) => {
                let mut pairs = BTreeMap::new();
                for (key, value) in value {
                    let mut key = self.evaluate_expression(key)?;
                    let hash_key = match HashKey::get_hashkey(&key) {
                        key => key,
                        _ => HashKey::Null
                    };
                    let mut value = self.evaluate_expression(value)?;
                    pairs.insert(Box::new(hash_key), Box::new(HashPair{key: key.to_owned(), value: value}));
                }               
                Ok(Object::Hash(pairs))
            }
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
            },
            ast::Expression::FunctionLiteral{parameters, body} => {
                let obj = Object::Function{params: parameters.clone(),
                                           body: *body.clone(),
                                           env: Environment{store: self.store.clone(), outer:None, builtin: builtins::new()}
                                          };
                Ok(obj)
            },
            ast::Expression::CallExpression{function, body} => {
                match self.evaluate_expression(function) {
                    Ok(value) =>{
                        let func = self.evaluate_expression(function)?;
                        let args = self.evaluate_arguments(body.to_vec())?;
                        let res = apply_function(func, args);
                        return res
                    },
                    Err(_) => return Ok(Object::Null)
                }

                Ok(Object::Null)
            },
            _ =>  Err(Errors::NodeError)
        }
    }

    fn evaluate_arguments(&mut self, expressions: Vec<Expression>) -> Result<Vec<Object>, Errors> {
        let mut results: Vec<Object> = Vec::new();
        for expression in expressions.iter() {
            match self.evaluate_expression(expression) {
                Ok(value) => {
                    results.push(value)
                             },
                Err(_) => return Ok(results)
            }
        }
        Ok(results)    
    }
}

fn apply_function(func: Object, args: Vec<Object>) -> Result<Object, Errors> {
    match func {
        Object::Function{params, body, env} => {
            // the value of parameter is inserted in outer when function is called.
            let mut outer = env.new_outer();
            for (i, param) in params.iter().enumerate() {
                if let Expression::Identifier(param) = param {
                    outer.set(param.to_string(), args[i].clone());
                }
            }
            match outer.evaluate_statement(&body)? {
                Object::Return(expression) => {
                    return Ok(*expression)
                },
                other_expression => return Ok(other_expression)
            }
            Ok(Object::Null)
        }
        Object::Builtin{func} => {
            Ok(func(args))
        }
        _ => {
            Ok(Object::Null)}
    }
}

fn evaluate_index_expression(left: Object, index: Object) -> Object {
    match left {
        Object::Array(left) => {
            if let Object::Integer(index) = index {
                let target_array = left.to_vec();
                evaluate_array_index_expression(target_array, index as i32)
            } else {
                Object::Null
            }
        },
        Object::Hash(left) => {
            let hash_key = match HashKey::get_hashkey(&index) {
                key => key,
                _ => HashKey::Null
            };
            if let Some(hash_pair) = left.get(&hash_key) {
                return hash_pair.value.clone()
            } else {
                return Object::Null
            }
            left[&hash_key].value.clone()
        }
        _ => Object::Null
    }
}


fn evaluate_array_index_expression(array: Vec<Object>, index: i32) -> Object {
    let max = array.len() as i32;
    if index < 0 || index > max {
        return Object::Null
    } else {
        array[index as usize].clone()
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

fn evaluate_infix_expression(left: Object, operator: &str, right: Object) -> Result<Object, Errors> {
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
        (Object::String(left), Object::String(right)) => {
            if operator != "+" {
                Ok(Object::Null)
            } else {
                let concatenated = format!("{}{}", left, right);
                Ok(Object::String(concatenated))                
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
                        ("if (false) {10}", ""),
                        ("if (1) {10}", "10"),
                        ("if (1 < 2) { 10 }", "10"),
                        ("if (1 > 2) {10}", ""),
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
                        ("if (1 > 2) {10}", ""),
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

    #[test]
    fn test_function_statements() {
        let tests = vec![
                ("let identity = fn(x) {x;}; identity(5);", "5"),
                ("let identity = fn(x) {return x;}; identity(5);", "5"),
                ("let double = fn(x) {x * 2;}; double(5);", "10"),
                ("let add = fn(x, y) {x + y;}; add(5, 5);", "10"),
                ("let add = fn(x, y) {x + y;}; add(5 + 5, add(5, 5));", "20"),
                ("fn(x) {x;}(5)", "5"),
                    ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let return_value = format!("{}", evaluated);
            assert_eq!(return_value, test.1);
        }
    }

    #[test]
    fn test_closures() {
        let input = "let new_adder = fn(x) {fn(y) {x + y};}; 
                     let add_two = new_adder(2);
                     add_two(2);";
        let evaluated = test_evaluate(input);
        let return_value = format!("{}", evaluated);
        assert_eq!(return_value.parse::<i32>().unwrap(), 4);
        }

    #[test]
    fn test_string() {
        let input = r#""Hello world;""#;
        let evaluated = test_evaluate(input);
        let return_value = format!("{}", evaluated);
        assert_eq!(return_value, "Hello world;");
        }
    #[test]
    fn test_string_concatnation() {
        let input = r#""Hello"+ " " + "world;""#;
        let evaluated = test_evaluate(input);
        let return_value = format!("{}", evaluated);
        assert_eq!(return_value, "Hello world;");
        }

    #[test]
    fn test_builtin_functions() {
        let tests = vec![
            ("len(\"\");", "0"),
            ("len(\"four\");", "4"),
            ("len(\"hello world\");", "11"),
            ("len(1);", "argument to len not supported got 1"),
            ("len(\"one\", \"two\");", "wrong number of arguments. got=2, want=1"),
            ("first([\"a\",\"b\"]);", "a"),
            ("first(\"ab\");", "argument to 'first' must be array, got ab"),
            ("last([\"a\",\"b\"]);", "b"),
            ("last(\"ab\");", "argument to 'last' must be array, got ab"),
            ("rest([\"a\",\"b\",\"c\",\"d\"]);", "[a, b, c]"),
            ("rest(\"ab\");", "argument to 'rest' must be array, got ab"),
            ("push([\"a\",\"b\",\"c\",\"d\"], \"e\");", "[a, b, c, d, e]"),
            ("push(\"ab\");", "argument to 'push' must be array, got ab"),
            ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let return_value = format!("{}", evaluated);
            assert_eq!(return_value, test.1);
        }
    }

    #[test]
    fn test_array_literal() {
        let input = "[1, 2 * 2, 3 + 3]";
        let evaluated = test_evaluate(input);
        let return_value = format!("{}", evaluated);
        assert_eq!(return_value, "[1, 4, 6]");
        }

    #[test]
    fn test_array_index_expressions() {
        let tests = vec![
            ("[1, 2, 3][0]", "1"),
            ("[1, 2, 3][1]", "2"),
            ("[1, 2, 3][2]", "3"),
            ("let i = 0; [1][i]", "1"),
            ("[1, 2, 3][1 + 1]", "3"),
            ("let my_array = [1, 2, 3]; let i = my_array[2]", "3"),
            ("let my_array = [1, 2, 3];my_array[0] + my_array[1]", "3"),
            ("let my_array = [1, 2, 3]; let i = my_array[0]; my_array[i]", "2"),
            ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let return_value = format!("{}", evaluated);
            assert_eq!(return_value, test.1);
        }
    }

    #[test]
    fn test_hash_literals() {
        let tests = vec![
            ("{\"one\": 10-9}", "{one: 1}"),
            ("{\"two\": 1 + 1}", "{two: 2}"),
            ("{\"thr\"+\"ee\":6 / 2}", "{three: 3}"),
            ("{4: 4}", "{4: 4}"),
            ("{true: 5}", "{true: 5}"),
            ("{false: 6}", "{false: 6}"),
            ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let return_value = format!("{}", evaluated);
            assert_eq!(return_value, test.1);
        }
    }

    #[test]
    fn test_hash_index_expression() {
        let tests = vec![
            ("{\"foo\": 5}[\"foo\"]", "5"),
            ("{\"foo\": 10}[\"bar\"]", ""),
            ("let key = \"foo\"; {\"foo\": 5}[key]", "5"),
            ("{}[\"foo\"]", ""),
            ("{5: 5}[5]", "5"),
            ("{true: 5}[true]", "5"),
            ("{false: 5}[false]", "5"),
            ];
        for test in tests.iter() {
            let evaluated = test_evaluate(test.0);
            let return_value = format!("{}", evaluated);
            assert_eq!(return_value, test.1);
        }
    }
}