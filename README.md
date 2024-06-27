# README

Yet another Rust implementation for Crafting Interpreter's
tree walk interpreter for Lox language.

## Grammar

```bnf
program     = declaration* EOF
declaration = varDecl | statement
varDecl     = "var" IDENTIFIER ( "=" expression )? ";"
statement   = exprStmt | printStmt
exprStmt    = expression
printStmt   = "print" expression
expression  = comma
comma       = ternary ( "," ternary )*
ternary     = equality "?" ternary ":" ternary
equality    = comparison ( ( "!=" | "==" ) comparison )*
comparison  = term ( ( ">" | ">=" | "<" | "<=" ) term )*
term        = factor ( ( "-" | "+" ) factor )*
factor      = unary ( ( "/" | "*" ) unary )*
unary       = ( "!" | "-" ) unary | primary
primary     = NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER
```
