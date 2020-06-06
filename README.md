# Imitation Interpreter

## Overview
This is an interpreter written in rust.  
It is originally written in [Writing An Interpreter In Go](https://interpreterbook.com/#the-monkey-programming-language).  
This interpreter is working for Monkey programming language, which is created for this book.

## How to use
### Local Environment
```
git clone git@github.com:rariyama/imitation_interpreter.git
cd imitation_interpreter
cargo run
```

### Use Dockerfile
```
git clone git@github.com:rariyama/imitation_interpreter.git
docker build -t monkey_repl ./imitation_interpreter
docker run --rm -it -v $PWD/imitation_interpreter:/monkey -w /monkey rust cargo run
```

