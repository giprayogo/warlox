# README

Yet another Rust implementation for Crafting Interpreter's
tree walk interpreter for Lox language.

## Grammar

Note: on the precedence between assignment and ternary
(extra materials from the book), I (roughly) follow C++'s grammar,
with one difference: I don't know how to restrict left hand side
to `equality` instead of `ternary` in this sytle of parser.
See [StackOverflow](https://stackoverflow.com/a/13515505).

```bnf
program     = declaration* EOF
declaration = varDecl | statement
varDecl     = "var" IDENTIFIER ( "=" expression )? ";"
statement   = exprStmt | printStmt | block
block       = "{" declaration* "}"
exprStmt    = expression
printStmt   = "print" expression
expression  = comma
comma       = assignment ( "," assignment )*
assignment  = ternary "=" assignment | ternary
ternary     = equality "?" expression ":" assignment
equality    = comparison ( ( "!=" | "==" ) comparison )*
comparison  = term ( ( ">" | ">=" | "<" | "<=" ) term )*
term        = factor ( ( "-" | "+" ) factor )*
factor      = unary ( ( "/" | "*" ) unary )*
unary       = ( "!" | "-" ) unary | primary
primary     = NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER
```
