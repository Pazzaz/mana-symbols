use std::{
    fmt::{Display, Write},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMana {
    Number(usize),
    X,
    Y,
    Z,
}

impl Display for GenericMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericMana::Number(x) => x.fmt(f),
            GenericMana::X => f.write_char('X'),
            GenericMana::Y => f.write_char('Y'),
            GenericMana::Z => f.write_char('Z'),
        }
    }
}

impl FromStr for GenericMana {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mana = match s {
            "X" => Self::X,
            "Y" => Self::Y,
            "Z" => Self::Z,
            s => {
                if let Ok(n) = s.parse::<usize>() {
                    Self::Number(n)
                } else {
                    return Err(());
                }
            }
        };
        Ok(mana)
    }
}
