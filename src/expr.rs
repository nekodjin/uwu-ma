use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Atom(Value),
    Hug(Box<Expr>, Box<Expr>),
    Boop(Box<Expr>, Box<Expr>),
}
