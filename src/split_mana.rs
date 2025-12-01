use std::fmt::Display;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::char,
    sequence::{preceded, separated_pair, terminated},
};

use crate::{Color, color_set::ColorSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitMana {
    Mono { value: usize, color: Color },
    Colorless { color: Color },
    Duo { a: Color, b: Color, phyrexian: bool },
}

impl Display for SplitMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mono { value, color } => write!(f, "{value}/{color}"),
            Self::Colorless { color } => write!(f, "C/{color}"),
            Self::Duo { a, b, phyrexian } => {
                if *phyrexian {
                    write!(f, "{a}/{b}/P")
                } else {
                    write!(f, "{a}/{b}")
                }
            }
        }
    }
}

impl SplitMana {
    pub const fn normalize(&mut self) {
        if let Self::Duo { a, b, phyrexian } = self {
            // We sort hybrid mana with two colors
            let mut color_set = ColorSet::new();
            color_set.set_color(*a);
            color_set.set_color(*b);
            let order = color_set.order_values();
            if order[*a as usize] > order[*b as usize] {
                *self = Self::Duo { a: *b, b: *a, phyrexian: *phyrexian }
            }
        }
    }

    pub const fn left_half_color(&self) -> Option<Color> {
        match self {
            Self::Mono { .. } | Self::Colorless { .. } => None,
            Self::Duo { a, .. } => Some(*a),
        }
    }

    pub const fn right_half_color(&self) -> Color {
        match self {
            Self::Mono { color, .. } | Self::Colorless { color } => *color,
            Self::Duo { b, .. } => *b,
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let colorless = preceded(tag("C/"), Color::parse).map(|color| Self::Colorless { color });
        let phyrexian =
            terminated(separated_pair(Color::parse, char('/'), Color::parse), tag("/P"))
                .map(|(a, b)| Self::Duo { a, b, phyrexian: true });
        let normal = separated_pair(Color::parse, char('/'), Color::parse)
            .map(|(a, b)| Self::Duo { a, b, phyrexian: false });

        let number = take_while(char::is_numeric).map_res(|s: &str| s.parse::<usize>());
        let generic = separated_pair(number, char('/'), Color::parse)
            .map(|(n, color)| Self::Mono { value: n, color });
        alt((colorless, phyrexian, normal, generic)).parse(input)
    }
}
