use crate::expr::Expr;
use crate::token::Token;

pub trait StmtVisitor {
    type Output;

    fn visit_stmt(&self, stmt: &Stmt) -> Self::Output {
        println!("{stmt:?}");
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum Stmt {
    Expr { expression: Box<Expr> },
    Print { expression: Box<Expr> },
    Var { name: Token },
}
