# Siren Language

I'm trying to make a simple programming language called Siren Language.

This language will become a graphic and text programming language.

## example
```
let a = 123 - 12 / 4;
let b = (-a + 42) / 2;
```
REPL output: `cargo r`
```
> let a = 123 - 12 / 4
> a
120
> let b = (-a + 42) / 2
> b
-39
>
```
File output: `cargo r test1.siren`
```
Content:
let a = 123 - 12 / 4;
let b = (-a + 42) / 2;
Done.
Env:
a = 120
b = -39
```

## Todolist

1. minimum REPL
  - [x] minimum amount of datatype: int
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
  - [ ] variable table type system
  - [ ] fn (function):
    - [x] fn parser
    - [ ] return statement
    - [ ] fn interpreter
      - [ ] bind function