use std::fmt::Display;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while,
    character::complete::char,
    combinator::verify,
    sequence::{preceded, separated_pair, terminated},
};

use crate::{
    Color,
    color_set::ColorSet,
    parsing::{Case, parse_char},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitMana {
    Mono { value: usize, color: Color },
    Colorless(Color),
    Duo { a: Color, b: Color, phyrexian: bool },
}

impl Display for SplitMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mono { value, color } => write!(f, "{value}/{color}"),
            Self::Colorless(color) => write!(f, "C/{color}"),
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
            Self::Mono { color, .. } | Self::Colorless(color) => *color,
            Self::Duo { b, .. } => *b,
        }
    }

    pub fn parse(case: Case, input: &str) -> IResult<&str, Self> {
        let color_parser = |x| Color::parse(case, x);

        // If it starts with "C/", then it's colorless hybrid mana
        let co = preceded((parse_char(case, 'c'), char('/')), color_parser).map(Self::Colorless);

        // If it starts with a number it's generic mana
        let number = take_while(char::is_numeric).map_res(|s: &str| s.parse::<usize>());
        let ge = separated_pair(number, char('/'), color_parser).map(|(n, c)| Self::generic(n, c));

        // Every other hybrid mana has two colors seperated by '/'
        let color_split = |x| separated_pair(color_parser, char('/'), color_parser).parse(x);

        // The two colors are not allowed to be equal
        let color_split_ne = |x| verify(color_split, |(a, b)| a != b).parse(x);

        // If it has "/P" after that it's phyrexian mana
        let ph = terminated(color_split_ne, (char('/'), parse_char(case, 'p')))
            .map(|(a, b)| Self::phyrexian(a, b));

        // Else it's normal hybrid mana
        let no = color_split_ne.map(|(a, b)| Self::normal(a, b));

        // Then we check if any of them matches
        alt((ph, no, ge, co)).parse(input)
    }

    const fn normal(a: Color, b: Color) -> Self {
        Self::Duo { a, b, phyrexian: false }
    }

    const fn phyrexian(a: Color, b: Color) -> Self {
        Self::Duo { a, b, phyrexian: true }
    }

    const fn generic(value: usize, color: Color) -> Self {
        Self::Mono { value, color }
    }
}
