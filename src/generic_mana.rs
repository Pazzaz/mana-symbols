use std::fmt::{Display, Write};

use nom::{
    IResult, Parser, branch::alt, bytes::complete::take_while, character::complete::char,
    combinator::value,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMana {
    Number(usize),
    X,
    Y,
    Z,
}

impl Display for GenericMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(x) => x.fmt(f),
            Self::X => f.write_char('X'),
            Self::Y => f.write_char('Y'),
            Self::Z => f.write_char('Z'),
        }
    }
}

impl GenericMana {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let x = value(Self::X, char('X'));
        let y = value(Self::Y, char('Y'));
        let z = value(Self::Z, char('Z'));
        let number =
            take_while(|c: char| c.is_numeric()).map_res(|s: &str| s.parse().map(Self::Number));
        alt((x, y, z, number)).parse(input)
    }
}
