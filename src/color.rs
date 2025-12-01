use std::fmt::{Display, Write};

use nom::{IResult, Parser, branch::alt, character::complete::char, combinator::value};

/// One of the five [colors](https://mtg.wiki/page/Color) of the color pie
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// [White](https://mtg.wiki/page/White) (W)
    White = 0,
    /// [Blue](https://mtg.wiki/page/Blue) (U)
    Blue = 1,
    /// [Black](https://mtg.wiki/page/Black) (B)
    Black = 2,
    /// [Red](https://mtg.wiki/page/Red) (R)
    Red = 3,
    /// [Green](https://mtg.wiki/page/Green) (G)
    Green = 4,
}

pub const ALL_COLORS: [Color; 5] =
    [Color::White, Color::Blue, Color::Black, Color::Red, Color::Green];

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => f.write_char('W'),
            Self::Blue => f.write_char('U'),
            Self::Black => f.write_char('B'),
            Self::Red => f.write_char('R'),
            Self::Green => f.write_char('G'),
        }
    }
}

impl Color {
    const fn from_usize(n: usize) -> Self {
        match n % 5 {
            0 => Self::White,
            1 => Self::Blue,
            2 => Self::Black,
            3 => Self::Red,
            4 => Self::Green,
            _ => unreachable!(),
        }
    }

    pub(crate) const fn next(self, i: usize) -> Self {
        Self::from_usize((self as usize).wrapping_add(i))
    }

    pub(crate) fn parse(input: &str) -> IResult<&str, Self> {
        let w = value(Self::White, char('W'));
        let u = value(Self::Blue, char('U'));
        let b = value(Self::Black, char('B'));
        let r = value(Self::Red, char('R'));
        let g = value(Self::Green, char('G'));
        alt((w, u, b, r, g)).parse(input)
    }
}
