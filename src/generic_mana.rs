use std::{
    fmt::{Display, Write},
    str::FromStr,
};

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
            GenericMana::Number(x) => x.fmt(f),
            GenericMana::X => f.write_char('X'),
            GenericMana::Y => f.write_char('Y'),
            GenericMana::Z => f.write_char('Z'),
        }
    }
}

impl FromStr for GenericMana {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mana = match s {
            "X" => Self::X,
            "Y" => Self::Y,
            "Z" => Self::Z,
            s => {
                if let Ok(n) = s.parse::<usize>() {
                    Self::Number(n)
                } else {
                    return Err(());
                }
            }
        };
        Ok(mana)
    }
}

impl GenericMana {
    pub fn parse(input: &str) -> IResult<&str, GenericMana> {
        let x = value(GenericMana::X, char('X'));
        let y = value(GenericMana::Y, char('Y'));
        let z = value(GenericMana::Z, char('Z'));
        let number = take_while(|c: char| c.is_numeric())
            .map_res(|s: &str| s.parse().map(GenericMana::Number));
        alt((x, y, z, number)).parse(input)
    }
}
