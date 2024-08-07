use crate::error::RuntimeError;
use crate::expr::Expr;
use crate::token::Token;

pub trait StmtVisitor {
    type Output;

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<Self::Output, RuntimeError> {
        println!("{stmt:?}");
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expression: Expr,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expression: Expr,
    },
    VarDecl {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}
