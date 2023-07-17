# Siren Language

I'm trying to make a simple programming language called Siren Language.

This language will become a graphic and text programming language.

## features
- REPL
- Datatypes:
  - int, bool, fn, unit
- Arithmetic expressions:
  - `+ - * /`
- compare operators:
  - `== != < > <= >=`
- let and set statements
- Functions:
  - They are first-class, meaning they can be treated as values and passed as arguments, returned from other functions, and assigned to variables.
  - All functions are pure from the outside, having no side effects on the external environment where they are called.
  - But, functions may have side effects internally. Like perform I/O operations.
  - Recursion is supported.



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
fib = fn (n) { return if (n <= 1) { [return 1] } else { [return (fib(n - 1) + fib(n - 2))] }; }
```

## Todolist

1. REPL
  - [x] datatype: int, fn, bool, unit
    - [ ] float
    - [ ] string
  - [x] expressions:
    - Infix: `+ - * / == != < <= > >=`
    - Prefix: `- !`
  - [x] variables(identifier): `abc foo bar`

2. interpreter for file
  - [x] variable type system
    - [ ] type annotations
    - [ ] type checking
  - [x] fn (function)
  - [x] if expression
  - [ ] for loop
  - [ ] basic struct and enum
  - [ ] input output