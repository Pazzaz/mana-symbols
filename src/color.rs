use std::{fmt::{Display, Write}, str::FromStr};

/// The five [colors](https://mtg.wiki/page/Color) of the color pie.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
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

pub(crate) const ALL_COLORS: [Color; 5] = [
    Color::White,
    Color::Blue,
    Color::Black,
    Color::Red,
    Color::Green,
];

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

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = match s {
            "W" => Self::White,
            "U" => Self::Blue,
            "B" => Self::Black,
            "R" => Self::Red,
            "G" => Self::Green,
            _ => return Err(()),
        };
        Ok(color)
    }
}

impl Color {
    pub(crate) const fn next_color(&self) -> Color {
        match self {
            Self::White => Self::Blue,
            Self::Blue => Self::Black,
            Self::Black => Self::Red,
            Self::Red => Self::Green,
            Self::Green => Self::White,
        }
    }
}