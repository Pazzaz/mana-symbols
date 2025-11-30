mod color;
mod color_set;
mod generic_mana;
mod mana;
mod single_mana;
mod split_mana;

use std::{fmt::Display, str::FromStr};

pub use color::Color;
pub use generic_mana::GenericMana;
pub use mana::Mana;
pub use single_mana::SingleMana;
pub use split_mana::SplitMana;

use crate::color_set::ColorSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manas {
    manas: Vec<Mana>,
}

impl Display for Manas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for mana in &self.manas {
            write!(f, "{{{mana}}}")?;
        }
        Ok(())
    }
}

impl FromStr for Manas {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let manas: Result<Result<Vec<Mana>, ()>, ()> =
            GroupIterator::new(s).map(|x| x.map(Mana::from_str)).collect();
        manas.flatten().map(|manas| Manas { manas })
    }
}

impl Manas {
    pub fn mana_value(&self) -> usize {
        self.manas.iter().map(Mana::mana_value).sum()
    }

    /// Normalizes hybrid mana symbols and sorts the mana symbols
    pub fn normalize(&mut self) {
        // Normalize hybrid mana
        for mana in &mut self.manas {
            match mana {
                Mana::Split(split_mana) => split_mana.normalize(),
                Mana::Single(_) | Mana::Generic(_) | Mana::Colorless | Mana::Snow => {}
            }
        }

        // Sort the colors into 6 categories:
        // 0. Generic
        // 0.0 X
        // 0.1 Y
        // 0.2 Z
        // 0.3 Number
        // 1. Hybrid Generic
        // 2. Colorless
        // 3. Hybrid Colorless
        // 4. Colored
        // 5. Snow

        self.manas.sort_by_key(|k| match k {
            Mana::Generic(GenericMana::X) => 0,
            Mana::Generic(GenericMana::Y) => 1,
            Mana::Generic(GenericMana::Z) => 2,
            Mana::Generic(GenericMana::Number(_)) => 3,
            Mana::Split(SplitMana::Mono { .. }) => 4,
            Mana::Colorless => 5,
            Mana::Split(SplitMana::Colorless { .. }) => 6,
            Mana::Single(_) | Mana::Split(SplitMana::Duo { .. }) => 7,
            Mana::Snow => 8,
        });

        let (_, rest) = take_while(&mut self.manas, |x| matches!(x, Mana::Generic(_)));

        let (generic_hybrid, rest) =
            take_while(rest, |x| matches!(x, Mana::Split(SplitMana::Mono { .. })));
        sort_by_colors(generic_hybrid, |x| x.right_half_color().unwrap());

        let (_, rest) = take_while(rest, |x| matches!(x, Mana::Colorless));

        let (colorless_hybrid, rest) =
            take_while(rest, |x| matches!(x, Mana::Split(SplitMana::Colorless { .. })));

        sort_by_colors(colorless_hybrid, |x| x.right_half_color().unwrap());

        let (colored, snow) =
            take_while(rest, |x| matches!(x, Mana::Single(_) | Mana::Split(SplitMana::Duo { .. })));

        sort_by_colors(colored, |x| x.left_half_color().unwrap());

        // Go through each run of equal colors
        for chunk in colored.chunk_by_mut(|a, b| a.left_half_color() == b.left_half_color()) {
            chunk.sort_by_key(|x| match x {
                Mana::Single(SingleMana::Normal(_)) => 0,
                Mana::Single(SingleMana::Phyrexian(_)) => 1,
                Mana::Split(SplitMana::Duo { phyrexian, .. }) => {
                    if !*phyrexian {
                        2
                    } else {
                        3
                    }
                }
                Mana::Split(SplitMana::Colorless { .. })
                | Mana::Split(SplitMana::Mono { .. })
                | Mana::Colorless
                | Mana::Generic(_)
                | Mana::Snow => unreachable!(),
            });

            // Discard non-hybrid mana
            let (_, rest) = take_while(chunk, |x| matches!(x, Mana::Single(_)));

            let (hybrid_non_phyrexian, hybrid_phyrexian) = take_while(rest, |x| {
                if let Mana::Split(SplitMana::Duo { phyrexian, .. }) = x {
                    !*phyrexian
                } else {
                    false
                }
            });

            sort_by_colors(hybrid_non_phyrexian, |x| x.right_half_color().unwrap());
            sort_by_colors(hybrid_phyrexian, |x| x.right_half_color().unwrap());
        }

        for mana in snow {
            debug_assert_eq!(*mana, Mana::Snow);
        }
    }
}

fn sort_by_colors<T, F: Fn(&T) -> Color>(a: &mut [T], pred: F) {
    let mut color_set = ColorSet::new();
    for v in &*a {
        color_set.set_color(pred(v));
    }
    let order = color_set.order_values();

    a.sort_by_key(|x| order[pred(x) as usize]);
}

/// Every element in the first array will satisfy `pred` and the first element
/// of the second array will not satisfy `pred` (or it will be empty).
fn take_while<T, F: Fn(&T) -> bool>(a: &mut [T], pred: F) -> (&mut [T], &mut [T]) {
    let mut i = 0;
    while i < a.len() {
        if !pred(&a[i]) {
            break;
        } else {
            i += 1;
        }
    }
    a.split_at_mut(i)
}

struct GroupIterator<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> GroupIterator<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, pos: 0 }
    }
}

impl<'a> Iterator for GroupIterator<'a> {
    type Item = Result<&'a str, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s.len() == self.pos {
            return None;
        }

        if self.s.as_bytes()[self.pos] != b'{' {
            return Some(Err(()));
        }

        if let Some(end_past) = self.s[self.pos..].find('}') {
            let end = self.pos + end_past;
            let start = self.pos + 1;
            self.pos = end + 1;

            return Some(Ok(&self.s[start..end]));
        } else {
            return Some(Err(()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        assert!(Manas::from_str("{}").is_err());
    }

    #[test]
    fn parse_hybrid() {
        assert!(Manas::from_str("{W/U}").is_ok());
    }

    // https://scryfall.com/card/hop/96/arsenal-thresher
    #[test]
    fn arsenal_thresher() {
        let s = "{2}{W/B}{U}";
        let manas = Manas::from_str(s).unwrap();
        assert_eq!(s.to_string(), manas.to_string());
    }

    #[test]
    fn parse_long() {
        let s = "{X}{Y}{4}{2/B}{2/R}{C}{C/U}{B}{B/R/P}{R/P}{R/W}{G}{G/W/P}{W}{W/U}{S}";
        let manas = Manas::from_str(s).unwrap();
        assert_eq!(s.to_string(), manas.to_string());
    }

    #[test]
    fn sort_long() {
        let before = "{R/P}{X}{C/U}{2/B}{W}{W/U}{B}{B/R/P}{2/R}{G}{C}{G/W/P}{S}{4}{Y}{R/W}";
        let after = "{X}{Y}{4}{2/B}{2/R}{C}{C/U}{B}{B/R/P}{R/P}{R/W}{G}{G/W/P}{W}{W/U}{S}";
        let mut manas_before = Manas::from_str(before).unwrap();

        manas_before.normalize();
        assert_eq!(manas_before.to_string(), after);
    }
}
