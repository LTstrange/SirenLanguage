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
>
```
test1.siren
```
let a = 123 - 12 / 4;
let b = (-a + 42) / 2;

let add = fn (a, b) {
    return a + b;
};

let c = add(a, b);
```

File output: `cargo r test1.siren`
```
Content:
let a = 123 - 12 / 4;
let b = (-a + 42) / 2;
[...]
let c = add(a, b);
Done.
Env:
c = 81
b = -39
a = 120
add = fn (a, b) { return (a + b); }
```

## Todolist

1. minimum REPL
  - [x] minimum amount of datatype
    - [x] int
    - [x] fn
    - [ ] bool
  - [x] expressions:
    - Infix: `+ - * /`
    - Prefix: `-`
  - [x] variables(identifier): `abc foo bar`
    - [x] identifier parser
    - [x] variable table
    - [x] let statement
    - [x] assign statement

2. minimum interpreter for file
  - [x] read and interprete file
  - [x] variable table type system
  - [x] fn (function):
    - [x] fn parser
    - [x] bind function
    - [x] fn call
    - [x] return statement