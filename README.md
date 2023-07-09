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
File interpret: `cargo r test1.siren`
test1.siren:
```
let a = 123 - 12 / 4;
let b = (-a + 42) / 2;

let add = fn (a, b) {
    return a + b;
};

let one = fn () {
    1
};

let c = add(a, b);
```
output:
```
Content:
let a = 123 - 12 / 4;
let b = (-a + 42) / 2;
[...]
let c = add(a, b);
Done.
Env:
add = fn (a, b) { return (a + b); }
one = fn () { return 1; }
c = 81
a = 120
b = -39
```

## Todolist

1. minimum REPL
  - [x] minimum amount of datatype: int, fn
    - [ ] bool
  - [x] expressions:
    - Infix: `+ - * /`
    - Prefix: `-`
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