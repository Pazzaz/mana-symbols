use std::fmt::Display;

use nom::{IResult, Parser, branch::alt, character::complete::char, sequence::terminated};

use crate::{
    Color,
    parsing::{Case, parse_char},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleMana {
    Normal(Color),
    Phyrexian(Color),
}

impl Display for SingleMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal(color) => color.fmt(f),
            Self::Phyrexian(color) => write!(f, "{color}/P"),
        }
    }
}

impl SingleMana {
    pub const fn color(self) -> Color {
        match self {
            Self::Normal(color) | Self::Phyrexian(color) => color,
        }
    }

    pub fn parse(case: Case, input: &str) -> IResult<&str, Self> {
        let color_parser = |x| Color::parse(case, x);
        let phyrexian_tag = (char('/'), parse_char(case, 'p'));
        let phyrexian = terminated(color_parser, phyrexian_tag).map(Self::Phyrexian);
        let normal = color_parser.map(Self::Normal);
        alt((phyrexian, normal)).parse(input)
    }
}
