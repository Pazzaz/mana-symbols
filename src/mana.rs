use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use nom::{
    Finish, IResult, Parser,
    branch::alt,
    character::complete::char,
    combinator::{eof, value},
    sequence::{delimited, terminated},
};

use crate::{Color, GenericMana, SingleMana, SplitMana};

/// A mana symbol
///
/// Any symbol that could be used as part of a [mana cost](https://mtg.wiki/page/Mana_cost).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mana {
    Single(SingleMana),
    Generic(GenericMana),
    Split(SplitMana),
    Colorless,
    Snow,
}

impl Display for Mana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(single_mana) => single_mana.fmt(f),
            Self::Generic(generic_mana) => generic_mana.fmt(f),
            Self::Split(split_mana) => split_mana.fmt(f),
            Self::Colorless => f.write_char('C'),
            Self::Snow => f.write_char('S'),
        }
    }
}

impl FromStr for Mana {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = terminated(Self::parse, eof).parse(s).finish();

        match p {
            Ok((_, mana)) => Ok(mana),
            Err(_) => Err(()),
        }
    }
}

impl Mana {
    /// The [mana value](https://mtg.wiki/page/Mana_value).
    #[must_use]
    pub const fn mana_value(&self) -> usize {
        match self {
            Self::Generic(GenericMana::Number(v)) => *v,
            Self::Generic(GenericMana::X | GenericMana::Y | GenericMana::Z) => 0,
            Self::Split(SplitMana::Mono { value, .. }) => *value,
            Self::Split(SplitMana::Duo { .. } | SplitMana::Colorless { .. })
            | Self::Single { .. }
            | Self::Colorless
            | Self::Snow => 1,
        }
    }

    /// Normalize left/right side of a hybrid mana symbol (does nothing if it's
    /// not a hybrid mana symbol).
    pub const fn normalize_hybrid(&mut self) {
        match self {
            Self::Split(split_mana) => split_mana.normalize(),
            Self::Single(_) | Self::Generic(_) | Self::Colorless | Self::Snow => {}
        }
    }

    /// The left half color of a mana symbol.
    ///
    /// ```
    /// use mana_symbols::{Color, Mana};
    ///
    /// let u: Mana = "U".parse().unwrap();
    /// let c: Mana = "C".parse().unwrap();
    /// let rg_phyrexian: Mana = "R/G/P".parse().unwrap();
    ///
    /// assert_eq!(u.left_half_color(), Some(Color::Blue));
    /// assert_eq!(c.left_half_color(), None);
    /// assert_eq!(rg_phyrexian.left_half_color(), Some(Color::Red));
    /// ```
    #[must_use]
    pub const fn left_half_color(&self) -> Option<Color> {
        match self {
            Self::Single(single_mana) => Some(single_mana.color()),
            Self::Split(split_mana) => split_mana.left_half_color(),
            Self::Generic(_) | Self::Colorless | Self::Snow => None,
        }
    }

    /// The right half color of a mana symbol.
    ///
    /// ```
    /// use mana_symbols::{Color, Mana};
    ///
    /// let u: Mana = "U".parse().unwrap();
    /// let c: Mana = "C".parse().unwrap();
    /// let rg_phyrexian: Mana = "R/G/P".parse().unwrap();
    ///
    /// assert_eq!(u.right_half_color(), Some(Color::Blue));
    /// assert_eq!(c.right_half_color(), None);
    /// assert_eq!(rg_phyrexian.right_half_color(), Some(Color::Green));
    /// ```
    #[must_use]
    pub const fn right_half_color(&self) -> Option<Color> {
        match self {
            Self::Single(single_mana) => Some(single_mana.color()),
            Self::Split(split_mana) => Some(split_mana.right_half_color()),
            Self::Generic(_) | Self::Colorless | Self::Snow => None,
        }
    }

    fn parse_inner(input: &str) -> IResult<&str, Self> {
        let single = SingleMana::parse.map(Self::Single);
        let generic = GenericMana::parse.map(Self::Generic);
        let split = SplitMana::parse.map(Self::Split);
        let colorless = value(Self::Colorless, char('C'));
        let snow = value(Self::Snow, char('S'));

        // We put the "longer" types first, to avoid matching prefixes
        alt((split, generic, single, colorless, snow)).parse(input)
    }

    /// Parse `Mana` using [`nom`]. If you just want to parse normally, use
    /// [`Mana::from_str`].
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let brackets = delimited(char('{'), Self::parse_inner, char('}'));
        alt((brackets, Self::parse_inner)).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        assert!(Mana::from_str("{}").is_err());
    }

    #[test]
    fn parse_u() {
        assert!(Mana::from_str("U").is_ok());
    }

    #[test]
    fn parse_with_whitespace() {
        assert!(Mana::from_str("U ").is_err());
        assert!(Mana::from_str(" U").is_err());
    }

    #[test]
    fn parse_with_brackets() {
        assert!(Mana::from_str("{U}").is_ok());
    }
}
