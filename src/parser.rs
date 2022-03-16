use crate::expr::Expr;
use crate::token::Token;
use crate::value::Value;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|expr| {
        let expr = || expr.clone();

        let atom = || {
            choice((
                just(Token::UwU).to(Expr::Atom(Value::UwU)),
                just(Token::OwO).to(Expr::Atom(Value::OwO)),
                just(Token::NwN).to(Expr::Atom(Value::NwN)),
                expr().delimited_by(just(Token::LPar), just(Token::RPar)),
            ))
        };

        let op = || {
            choice((
                just(Token::Hug).to(Expr::Hug as fn(_, _) -> _),
                just(Token::Boop).to(Expr::Boop as fn(_, _) -> _),
            ))
        };

        atom()
            .then(op().then(atom()).repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
    })
}
