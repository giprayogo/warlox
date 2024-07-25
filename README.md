# README

Yet another Rust implementation for Crafting Interpreter's
tree walk interpreter for Lox language.

## Grammar

Note: Precedence for optional implementations
(extra materials from the book) roughly follows C's (not C++) grammar,
with one difference: I'm not sure how to restrict assignments's left hand side
to `equality` instead of `ternary` in a recursive descent parser.

See [StackOverflow](https://stackoverflow.com/a/13515505)
and [The Syntax of C in Backus-Naur form](https://cs.wmich.edu/~gupta/teaching/cs4850/sumII06/The%20syntax%20of%20C%20in%20Backus-Naur%20form.htm).

```bnf
program     ::= declaration* EOF
declaration ::= varDecl | statement
varDecl     ::= "var" IDENTIFIER ( "=" expression )? ";"
statement   ::= exprStmt | ifStmt | printStmt | whileStmt | block
ifStmt      ::= "if" "(" expression ")" statement ( "else" statement )?
whileStmt   ::= "while" "(" expression ")" statement
forStmt     ::= "for" "(" varDecl | exprStmt ";" expression? ";" expression? ")" statement
block       ::= "{" declaration* "}"
exprStmt    ::= expression
printStmt   ::= "print" expression
expression  ::= comma
comma       ::= assignment ( "," assignment )*
assignment  ::= ternary "=" assignment | ternary
ternary     ::= logic_or "?" expression ":" ternary
logic_or    ::= logic_and ( "or" logic_and )*
logic_and   ::= equality ( "and" equality )*
equality    ::= comparison ( ( "!=" | "==" ) comparison )*
comparison  ::= term ( ( ">" | ">=" | "<" | "<=" ) term )*
term        ::= factor ( ( "-" | "+" ) factor )*
factor      ::= unary ( ( "/" | "*" ) unary )*
unary       ::= ( "!" | "-" ) unary | primary
primary     ::= NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER
```
