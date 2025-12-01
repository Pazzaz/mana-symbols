use std::fmt::Display;

use nom::{IResult, Parser, branch::alt, bytes::complete::tag, sequence::terminated};

use crate::Color;

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

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let phyrexian = terminated(Color::parse, tag("/P")).map(Self::Phyrexian);
        let normal = Color::parse.map(Self::Normal);
        alt((phyrexian, normal)).parse(input)
    }
}
