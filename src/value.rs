use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    UwU,
    OwO,
    NwN,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Value::*;

        write!(
            f,
            "{}",
            match self {
                UwU => "uwu",
                OwO => "owo",
                NwN => "nwn",
            }
        )
    }
}
