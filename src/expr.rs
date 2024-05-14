use crate::token::{LoxLiteral, Token};

// NOTE: Juggling between
// Type parameter (type = ...) and generic <T>
// Trait vs enum
pub trait ExprVisitor {
    type Output;

    fn visit(&self, expr: &Expr) -> Self::Output {
        println!("{expr:?}");
        unimplemented!()
    }
    // fn visit_binary(&self, expr: &Binary) -> Self::Item {}
    // fn visit_grouping(&self, expr: &Grouping) {}
    // fn visit_literal(&self, expr: &Literal) {}
    // fn visit_unary(&self, expr: &Unary) {}
}

// NOTE: Juggling between Enum and trait implementation for Expr
#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Option<LoxLiteral>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
// trait Expr {
//     fn accept<T>(&self, visitor: &dyn ExprVisitor<Item = T>) -> T;
// }
// impl Expr {
//     fn accept<T>(&self, visitor: &dyn ExprVisitor<Output = T>) -> T {
//         visitor.visit(self)
//     }
// }

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let inner = exprs
            .iter()
            .map(|e| self.visit(e))
            .reduce(|acc, e| acc + " " + &e)
            .unwrap_or("".to_string());
        format!("({name} {inner})")
    }

    pub fn print(&self, expr: &Expr) -> String {
        self.visit(expr)
    }
}

impl ExprVisitor for AstPrinter {
    type Output = String;

    // # NOTE: At the end of the day not so much of a visitor?? ¯\_(ツ)_/¯
    // Perhaps will be revisited (got it?) when I added statements
    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(&operator.lexeme, &[left, right]),
            Expr::Grouping { expression } => self.parenthesize("group", &[expression]),
            Expr::Literal { value } => match value {
                Some(v) => format!("{v}"),
                None => "".to_string(),
            },
            Expr::Unary { operator, right } => self.parenthesize(&operator.lexeme, &[right]),
        }
    }
}
