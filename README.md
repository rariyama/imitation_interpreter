# Imitation Interpreter

## Overview
This is an interpreter written in rust.
It is originally written in [Writing An Interpreter In Go](https://interpreterbook.com/#the-monkey-programming-language).  
This interpreter is working for Monkey programming language, which is created for this book.

## Environment
### Local
```
$ git clone git@github.com:rariyama/imitation_interpreter.git
$ cd imitation_interpreter
$ cargo run
```

### Dockerfile
```
$ git clone git@github.com:rariyama/imitation_interpreter.git
$ docker build -t monkey_repl ./imitation_interpreter
$ docker run --rm -e USER=$USER -it -v $PWD/imitation_interpreter:/monkey -w /monkey monkey_repl cargo run
```
if you would like to exit from console. input exit(), CTRL-C or CTRL-D.

## Usage
### Example
note: This console doesn't support new line, so please write your code one by line.
```
let five = 5;
let ten = 10;
let add = fn(x, y){x + y;};
let result = add (five, ten);
if (result < 10) { return true; } else { return false;}
let array = [1, 2];
let pushed = push(array, 3);
print(pushed)
print(len(pushed))
let map = {"foo": "bar"};
let value = map["foo"];
```
### Grammer
#### print
you can show your output by using `print function`.

#### Variable definition
you can bind literals with variables. 
```
let integer = 5;
let identifier = "Hello monkey.";
```
#### Function definition
you can bind functions with variables.
```
let add = fn(x, y){x + y;};
let result = add (1, 2);
```
#### Function definition
you can bind functions with variables.
```
let add = fn(x, y){x + y;};
let result = add (1, 2);
```
#### IF Statement
you can write conditional sentences by using `if` or `else`, but if you can't use `else if` or `elif`.
```
let x = 5;
let y = 10;
if (x < y) { print("y is larger than x") } else { print("y is smaller than x")}
```
#### map
This supports key value map literal. you can declare key-value and slice it.  
This is alphabetically ordered.
```
let my_profile = {"first_name": "ryo", "last_name": "ariyama", "age": 29, "sex": "male"}
print(my_profile["first_name"])
```
#### Array
you can use array and can manipulate by using some functions.
```
let arr = [1,2,"three"];
# you can get a part of values by slicing it.
print(arr[0]);
# the length of an array.
print(len(arr))
# the first value of an array.
print(first(arr))
# the last value of an array.
print(last(arr))
# append a value into an array.
print(push(arr, 4))
# remove the last value from an array.
print(rest(arr))
```
