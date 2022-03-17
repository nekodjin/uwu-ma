use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Atom(Value),
    Hug(Box<Expr>, Box<Expr>),
    Boop(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> Value {
        use Expr::*;

        match self {
            Atom(v) => v.clone(),
            Hug(l, r) => hug(l.eval(), r.eval()),
            Boop(l, r) => boop(l.eval(), r.eval()),
        }
    }
}

fn hug(l: Value, r: Value) -> Value {
    use Value::*;

    match (l, r) {
        (OwO, _) => OwO,
        (_, OwO) => OwO,
        (UwU, _) => UwU,
        (NwN, UwU) => UwU,
        (NwN, _) => NwN,
    }
}

fn boop(l: Value, r: Value) -> Value {
    use Value::*;

    match (l, r) {
        (NwN, _) => NwN,
        (_, NwN) => NwN,
        (OwO, _) => OwO,
        (UwU, _) => UwU,
    }
}
