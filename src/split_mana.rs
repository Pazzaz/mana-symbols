use std::{fmt::Display, str::FromStr};

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
            SplitMana::Mono { value, color } => write!(f, "{}/{}", value, color),
            SplitMana::Colorless { color } => write!(f, "C/{}", color),
            SplitMana::Duo { a, b, phyrexian } => {
                if *phyrexian {
                    write!(f, "{a}/{b}/P")
                } else {
                    write!(f, "{a}/{b}")
                }
            }
        }
    }
}

impl FromStr for SplitMana {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((first, mut second)) = s.split_once('/') {
            let phyrexian: bool;
            (second, phyrexian) = {
                if let Some((new_second, third)) = second.split_once('/') {
                    (new_second, third == "P")
                } else {
                    (second, false)
                }
            };

            let b = Color::from_str(second)?;
            if phyrexian {
                let a = Color::from_str(first)?;
                Ok(SplitMana::Duo {
                    a,
                    b,
                    phyrexian: true,
                })
            } else {
                if first == "C" {
                    Ok(SplitMana::Colorless { color: b })
                } else if let Ok(value) = first.parse::<usize>() {
                    Ok(SplitMana::Mono { value, color: b })
                } else {
                    let a = Color::from_str(first)?;
                    Ok(SplitMana::Duo {
                        a,
                        b,
                        phyrexian: false,
                    })
                }
            }
        } else {
            Err(())
        }
    }
}

impl SplitMana {
    pub fn normalize(&mut self) {
        match self {
            // We sort hybrid mana with two colors
            SplitMana::Duo { a, b, phyrexian } => {
                let mut color_set = ColorSet::new();
                color_set.set_color(*a);
                color_set.set_color(*b);
                let order = color_set.order_values();
                if order[*a as usize] > order[*b as usize] {
                    *self = SplitMana::Duo {
                        a: *b,
                        b: *a,
                        phyrexian: *phyrexian,
                    }
                }
            }
            _ => {}
        }
    }

    pub fn left_half_color(&self) -> Option<Color> {
        match self {
            SplitMana::Mono { .. } => None,
            SplitMana::Colorless { .. } => None,
            SplitMana::Duo { a, .. } => Some(*a),
        }
    }

    pub fn right_half_color(&self) -> Option<Color> {
        match self {
            SplitMana::Mono { color, .. } => Some(*color),
            SplitMana::Colorless { color } => Some(*color),
            SplitMana::Duo { b, .. } => Some(*b),
        }
    }
}
