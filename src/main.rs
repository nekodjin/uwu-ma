mod expr;
mod parser;
mod token;
mod value;

use std::io::{self, BufRead, Write};

use parser::parser;
use token::Token;

use ariadne::{Label, Report, ReportKind, Source};
use chumsky::Parser as _;
use logos::Logos;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("UwU Math Assistant v{VERSION}");

    print!("-% ");
    io::stdout().flush().unwrap();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        let mut spans = Vec::new();
        let mut tokens = Vec::new();
        let mut last_was_err = false;
        let mut start = 0;
        let mut end = 0;

        for (token, span) in Token::lexer(&line).spanned() {
            if token != Token::Error {
                if last_was_err {
                    spans.push(start..end);
                    tokens.push(Token::Error);
                }

                last_was_err = false;
                spans.push(span);
                tokens.push(token);
                continue;
            }

            if last_was_err {
                end = span.end;
                continue;
            }

            last_was_err = true;
            start = span.start;
        }

        if last_was_err {
            spans.push(start..end);
            tokens.push(Token::Error);
        }

        if tokens.iter().any(|t| *t == Token::Error) {
            let mut report = Report::build(ReportKind::Error, (), 0);
            let mut errors = 0;

            for (idx, token) in tokens.iter().enumerate() {
                if *token != Token::Error {
                    continue;
                }

                errors += 1;

                let span = spans[idx].clone();

                report =
                    report.with_label(Label::new(span).with_message("Invalid token found here"));
            }

            report
                .with_message(format!(
                    "Invalid token{}",
                    if errors == 1 { "" } else { "s" }
                ))
                .finish()
                .eprint(Source::from(&line))
                .unwrap();
        }

        let (tree, errs) = parser().parse_recovery(tokens.as_slice());

        if errs.len() != 0 {
            // todo!()
            println!("{errs:#?}");
        }

        if let Some(tree) = tree {
            println!("{}", tree.eval());
        }

        print!("-% ");
        io::stdout().flush().unwrap();
    }

    println!();
}
