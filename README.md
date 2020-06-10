# Imitation Interpreter

## Overview
This is an interpreter written in rust.
It is originally written in [Writing An Interpreter In Go](https://interpreterbook.com/#the-monkey-programming-language).  
This interpreter is working for Monkey programming language, which is created for this book.

## Environment
### Local Environment
```
$ git clone git@github.com:rariyama/imitation_interpreter.git
$ cd imitation_interpreter
$ cargo run
```

### Use Dockerfile
```
$ git clone git@github.com:rariyama/imitation_interpreter.git
$ docker build -t monkey_repl ./imitation_interpreter
$ docker run --rm -e USER=$USER -it -v $PWD/imitation_interpreter:/monkey -w /monkey monkey_repl cargo run
```

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
