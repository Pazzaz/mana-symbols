use std::fmt::{Display, Write};

use nom::{IResult, Parser, branch::alt, bytes::complete::take_while, combinator::value};

use crate::{Case, parsing::parse_char};

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
    pub fn parse(case: Case, input: &str) -> IResult<&str, Self> {
        let x = value(Self::X, parse_char(case, 'x'));
        let y = value(Self::Y, parse_char(case, 'y'));
        let z = value(Self::Z, parse_char(case, 'z'));
        let number =
            take_while(|c: char| c.is_numeric()).map_res(|s: &str| s.parse().map(Self::Number));
        alt((x, y, z, number)).parse(input)
    }
}
