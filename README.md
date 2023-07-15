# Siren Language

I'm trying to make a simple programming language called Siren Language.

This language will become a graphic and text programming language.

## example
REPL: `cargo r`
```
> let a = 123 - 12 / 4
> a
120
> let b = (-a + 42) / 2
> b
-39
> !false
true
> true
true
```



File interpret: `cargo r examples/fib.siren`:
```
let fib = fn (n) {
    if n <= 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
};

let a = fib(5);
```
output:
```
let fib = fn (n) {
    if n <= 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
};

let a = fib(5);
Done.
Env:
a = 8
fib = fn (n) { return if (n <= 1) { [return 1] } else { [return (fib.call((n - 1)) + fib.call((n - 2)))] }; }
```

## Todolist

1. minimum REPL
  - [x] minimum amount of datatype: int, fn, bool
  - [x] expressions:
    - Infix: `+ - * /`
    - Prefix: `- !`
  - [x] variables(identifier): `abc foo bar`
    - let statement
    - set statement

2. minimum interpreter for file
  - [x] variable type system
  - [x] fn (function):
    - [x] fn parser
    - [x] bind function
    - [x] fn call
    - [x] return statement
  - [x] boolean operator
    - [x] not operator
    - [x] eql operator
    - [x] not eql operator
  - [x] if expression
    - [x] if parser
    - [x] block statement
    - [ ] side effect