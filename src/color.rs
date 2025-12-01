use std::fmt::{Display, Write};

use nom::{IResult, Parser, branch::alt, character::complete::char, combinator::value};

/// One of the five [colors](https://mtg.wiki/page/Color) of the color pie.
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

pub(crate) const ALL_COLORS: [Color; 5] =
    [Color::White, Color::Blue, Color::Black, Color::Red, Color::Green];

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => f.write_char('W'),
            Color::Blue => f.write_char('U'),
            Color::Black => f.write_char('B'),
            Color::Red => f.write_char('R'),
            Color::Green => f.write_char('G'),
        }
    }
}

impl Color {
    const fn from_usize(n: usize) -> Color {
        match n % 5 {
            0 => Self::White,
            1 => Self::Blue,
            2 => Self::Black,
            3 => Self::Red,
            4 => Self::Green,
            _ => unreachable!(),
        }
    }

    pub(crate) const fn next(&self, i: usize) -> Color {
        Self::from_usize((*self as usize).wrapping_add(i))
    }

    pub fn parse(input: &str) -> IResult<&str, Color> {
        let w = value(Color::White, char('W'));
        let u = value(Color::Blue, char('U'));
        let b = value(Color::Black, char('B'));
        let r = value(Color::Red, char('R'));
        let g = value(Color::Green, char('G'));
        alt((w, u, b, r, g)).parse(input)
    }
}
