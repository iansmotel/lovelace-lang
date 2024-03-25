# lovelace-lang
## Context and Introduction
I am a Computer Science student interested in functional programming and theory
of programming languages. So I decided to write a tree-walk interpreter for a
functional programming language of my making using the world's most loved
programming language, Rust!
## What will the language look like?
Here you can see some of the basic functionality
```
def some_number := 5

def some_string := "hello"

def add := (num1, num2) -> (num1 + num2)

def factorial := (n) -> (
    n = 0? 1: self(n - 1)
)

def is_larger := (num1, num2) -> (
    num1 > num2
)

def get_based_on_comparaison := (comparaison, num1, num2) -> (
    comparaison(num1, num2)?
        num1:
        num2
)

get_based_on_comparaison(is_larger, 2, some_number)
//this expression would evaluate to 5

get_based_on_comparaison((num1, num2) -> (num1 < num2),
                         2,
                         some_number)
//this expression would evaluate to 2
```
