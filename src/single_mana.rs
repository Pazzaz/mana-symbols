use std::{fmt::Display, str::FromStr};

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
            SingleMana::Normal(color) => color.fmt(f),
            SingleMana::Phyrexian(color) => write!(f, "{}/P", color),
        }
    }
}

impl FromStr for SingleMana {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(color) = Color::from_str(s).ok() {
            Ok(Self::Normal(color))
        } else if let Some((first, second)) = s.split_once('/')
            && second == "P"
        {
            let color = Color::from_str(first)?;
            Ok(Self::Phyrexian(color))
        } else {
            Err(())
        }
    }
}

impl SingleMana {
    pub fn color(&self) -> Color {
        match self {
            SingleMana::Normal(color) | SingleMana::Phyrexian(color) => *color,
        }
    }

    pub fn parse(input: &str) -> IResult<&str, SingleMana> {
        let phyrexian = terminated(Color::parse, tag("/P")).map(SingleMana::Phyrexian);
        let normal = Color::parse.map(SingleMana::Normal);
        alt((phyrexian, normal)).parse(input)
    }
}
