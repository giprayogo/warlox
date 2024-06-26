use crate::expr::Expr;
use crate::token::Token;

pub trait StmtVisitor {
    type Output;

    fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output {
        println!("{stmt:?}");
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum Stmt {
    Expr {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}
