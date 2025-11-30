use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use crate::{Color, GenericMana, SingleMana, SplitMana};

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

    pub fn left_half_color(&self) -> Option<Color> {
        match self {
            Mana::Single(single_mana) => Some(single_mana.color()),
            Mana::Generic(_) => None,
            Mana::Split(split_mana) => split_mana.left_half_color(),
            Mana::Colorless => None,
            Mana::Snow => None,
        }
    }

    pub fn right_half_color(&self) -> Option<Color> {
        match self {
            Mana::Single(single_mana) => Some(single_mana.color()),
            Mana::Generic(_) => None,
            Mana::Split(split_mana) => split_mana.right_half_color(),
            Mana::Colorless => None,
            Mana::Snow => None,
        }
    }
}
