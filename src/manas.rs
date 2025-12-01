use std::{fmt::Display, str::FromStr};

use nom::{Finish, IResult, Parser, combinator::eof, multi::many0, sequence::terminated};

use crate::{Color, GenericMana, Mana, SingleMana, SplitMana, color_set::ColorSet};

/// Collection of mana symbols.
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
        let p = terminated(Self::parse, eof).parse(s).finish();

        match p {
            Ok((_, mana)) => Ok(mana),
            Err(_) => Err(()),
        }
    }
}

impl Manas {
    /// The total [mana value](https://mtg.wiki/page/Mana_value) of the mana symbols.
    pub fn mana_value(&self) -> usize {
        self.manas.iter().map(Mana::mana_value).sum()
    }

    /// Normalize left/right side of hybrid mana symbols (see
    /// [`Mana::normalize_hybrid`]).
    pub fn normalize_hybrid(&mut self) {
        for mana in &mut self.manas {
            mana.normalize_hybrid();
        }
    }

    /// Sorts the mana symbols in groups, then sorts those groups, in the
    /// following order:
    /// 1. Generic mana
    ///     1. X
    ///     2. Y
    ///     3. Z
    ///     4. Number
    /// 2. Hybrid Generic mana (then based on their right half color)
    /// 3. Colorless mana
    /// 4. Hybrid Colorless mana (then based on their right half color)
    /// 5. Colored mana (then based on their left half color, then on right half
    ///    color)
    /// 6. Snow mana
    pub fn sort(&mut self) {
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

        let rest = skip(&mut self.manas, |x| matches!(x, Mana::Generic(_)));

        let (generic_hybrid, rest) =
            take_while(rest, |x| matches!(x, Mana::Split(SplitMana::Mono { .. })));
        sort_by_colors(generic_hybrid, |x| x.right_half_color().unwrap());

        let rest = skip(rest, |x| matches!(x, Mana::Colorless));

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
                    if *phyrexian {
                        3
                    } else {
                        2
                    }
                }
                Mana::Split(SplitMana::Colorless { .. } | SplitMana::Mono { .. })
                | Mana::Colorless
                | Mana::Generic(_)
                | Mana::Snow => unreachable!(),
            });

            // Discard non-hybrid mana
            let rest = skip(chunk, |x| matches!(x, Mana::Single(_)));

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

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, res) = many0(Mana::parse).parse(input)?;
        Ok((rest, Self { manas: res }))
    }
}

impl From<Manas> for Vec<Mana> {
    fn from(value: Manas) -> Self {
        value.manas
    }
}

impl From<Vec<Mana>> for Manas {
    fn from(value: Vec<Mana>) -> Self {
        Self { manas: value }
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
        if pred(&a[i]) {
            i += 1;
        } else {
            break;
        }
    }
    a.split_at_mut(i)
}

fn skip<T, F: Fn(&T) -> bool>(a: &mut [T], pred: F) -> &mut [T] {
    let (_, rest) = take_while(a, pred);
    rest
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

        manas_before.sort();
        assert_eq!(manas_before.to_string(), after);
    }

    #[test]
    fn nom_parse_long_1() {
        let unsorted_long = "{R/P}{X}{C/U}{2/B}{W}{W/U}{B}{B/R/P}{2/R}{G}{C}{G/W/P}{S}{4}{Y}{R/W}";
        if let Ok((res, manas)) = Manas::parse(unsorted_long) {
            assert_eq!(res, "");
            let simple_parser = Manas::from_str(unsorted_long).unwrap();
            assert_eq!(manas, simple_parser);
        } else {
            panic!();
        }
    }

    #[test]
    fn nom_parse_long_2() {
        let unsorted_long = "R/PXC/U2/BWW/UBB/R/P2/RGCG/W/PS4YR/W";
        if let Ok((res, _manas)) = Manas::parse(unsorted_long) {
            assert_eq!(res, "");
        } else {
            panic!();
        }
    }
}
