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
        f.write_char(self.char())
    }
}

impl Color {
    #[must_use]
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

    /// The letter representing each color. `White -> W`, `Blue -> U`, etc.
    #[must_use]
    pub const fn char(self) -> char {
        match self {
            Self::White => 'W',
            Self::Blue => 'U',
            Self::Black => 'B',
            Self::Red => 'R',
            Self::Green => 'G',
        }
    }

    #[must_use]
    pub(crate) const fn next(self, i: usize) -> Self {
        Self::from_usize((self as usize).wrapping_add(i))
    }

    pub(crate) fn parse(input: &str) -> IResult<&str, Self> {
        let parsers = ALL_COLORS.map(|c| value(c, char(c.char())));
        alt(parsers).parse(input)
    }

    #[must_use]
    pub const fn hex(self) -> &'static str {
        match self {
            Color::White => HEX_W,
            Color::Blue => HEX_U,
            Color::Black => HEX_B,
            Color::Red => HEX_R,
            Color::Green => HEX_G,
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        match self {
            Color::White => "white",
            Color::Blue => "blue",
            Color::Black => "black",
            Color::Red => "red",
            Color::Green => "green",
        }
    }

    pub(crate) fn name_capitalized(&self) -> &'static str {
        match self {
            Color::White => "White",
            Color::Blue => "Blue",
            Color::Black => "Black",
            Color::Red => "Red",
            Color::Green => "Green",
        }
    }
}

// Colors of the five main colors
pub const HEX_W: &str = "#fffbd5";
pub const HEX_U: &str = "#aae0fa";
pub const HEX_B: &str = "#cbc2bf";
pub const HEX_R: &str = "#f9aa8f";
pub const HEX_G: &str = "#9bd3ae";

// Generic and colorless color
pub const HEX_C: &str = "#cbc2bf";
