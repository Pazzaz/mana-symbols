use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use nom::{
    IResult, Parser, branch::alt, character::complete::char, combinator::value, sequence::delimited,
};

use crate::{Color, GenericMana, SingleMana, SplitMana};

/// A mana symbol.
///
/// Any symbol that could be used as part of a mana cost.
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
            Mana::Single(single_mana) => single_mana.fmt(f),
            Mana::Generic(generic_mana) => generic_mana.fmt(f),
            Mana::Split(split_mana) => split_mana.fmt(f),
            Mana::Colorless => f.write_char('C'),
            Mana::Snow => f.write_char('S'),
        }
    }
}

impl FromStr for Mana {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(single) = SingleMana::from_str(s) {
            Ok(Mana::Single(single))
        } else if let Ok(generic_mana) = GenericMana::from_str(s) {
            Ok(Mana::Generic(generic_mana))
        } else if let Ok(split) = SplitMana::from_str(s) {
            Ok(Mana::Split(split))
        } else if s == "C" {
            Ok(Mana::Colorless)
        } else if s == "S" {
            Ok(Mana::Snow)
        } else {
            Err(())
        }
    }
}

impl Mana {
    pub fn mana_value(&self) -> usize {
        match self {
            Mana::Generic(GenericMana::Number(v)) => *v,
            Mana::Generic(GenericMana::X)
            | Mana::Generic(GenericMana::Y)
            | Mana::Generic(GenericMana::Z) => 0,
            Mana::Split(SplitMana::Mono { value, .. }) => *value,
            Mana::Split(SplitMana::Duo { .. }) => 1,
            Mana::Split(SplitMana::Colorless { .. }) => 1,
            Mana::Single { .. } | Mana::Colorless | Mana::Snow => 1,
        }
    }

    /// Normalize left/right side of a hybrid mana symbol (does nothing if it's
    /// not a hybrid mana symbol).
    pub fn normalize_hybrid(&mut self) {
        match self {
            Mana::Split(split_mana) => split_mana.normalize(),
            Mana::Single(_) | Mana::Generic(_) | Mana::Colorless | Mana::Snow => {}
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
    pub fn left_half_color(&self) -> Option<Color> {
        match self {
            Mana::Single(single_mana) => Some(single_mana.color()),
            Mana::Generic(_) => None,
            Mana::Split(split_mana) => split_mana.left_half_color(),
            Mana::Colorless => None,
            Mana::Snow => None,
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
    pub fn right_half_color(&self) -> Option<Color> {
        match self {
            Mana::Single(single_mana) => Some(single_mana.color()),
            Mana::Generic(_) => None,
            Mana::Split(split_mana) => split_mana.right_half_color(),
            Mana::Colorless => None,
            Mana::Snow => None,
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let single = SingleMana::parse.map(Self::Single);
        let generic = GenericMana::parse.map(Self::Generic);
        let split = SplitMana::parse.map(Self::Split);
        let colorless = value(Self::Colorless, char('C'));
        let snow = value(Self::Snow, char('S'));

        // We put the "longer" types first, to avoid matching prefixes
        alt((split, generic, single, colorless, snow)).parse(input)
    }

    pub fn parse_possible_brackets(input: &str) -> IResult<&str, Self> {
        let brackets = delimited(char('{'), Self::parse, char('}'));
        alt((brackets, Self::parse)).parse(input)
    }
}
