mod expr;
mod parser;
mod token;
mod value;

use std::io::{self, BufRead};

use chumsky::Parser as _;
use logos::Logos;

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        let toks = token::Token::lexer(&line).collect::<Vec<_>>();
        let tree = parser::parser().parse(toks);

        println!("{tree:#?}");
    }
}
