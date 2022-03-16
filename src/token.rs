use logos::Logos;

#[derive(Logos, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Token {
    #[error]
    #[regex("[ \t\r\n]", logos::skip)]
    Error,

    #[token("(")]
    LPar,
    #[token(")")]
    RPar,

    #[token("uwu", ignore(case))]
    UwU,
    #[token("owo", ignore(case))]
    OwO,
    #[token("nwn", ignore(case))]
    NwN,

    #[regex("hugs?", ignore(case))]
    Hug,
    #[regex("boops?", ignore(case))]
    Boop,
}
