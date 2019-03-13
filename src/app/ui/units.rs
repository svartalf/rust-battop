use std::str::FromStr;

use crate::Error;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Units {
    Human,
    Si,
}

impl Units {
    // Why do you even need this, when clap provides `arg_enum!` macro?
    // I just do not like that results are capitalized.
    // Who the hell want to write manually arguments like `-u Human`?
    // `-u human` is much prettier.
    pub fn arg_variants() -> [&'static str; 2] {
        ["human", "si"]
    }
}

impl FromStr for Units {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match () {
            _ if s.eq_ignore_ascii_case("human") => Ok(Units::Human),
            _ if s.eq_ignore_ascii_case("si") => Ok(Units::Si),
            _ => Err(Error::ParseError),
        }
    }
}
