// Based on: https://www.reddit.com/r/custommagic/comments/1nhtr3w/guide_for_formatting_mana_costs/

mod color;
mod color_set;

pub use color::Color;

use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use crate::color_set::ColorSet;

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

    fn left_half_color(&self) -> Option<Color> {
        match self {
            SplitMana::Mono { .. } => None,
            SplitMana::Colorless { .. } => None,
            SplitMana::Duo { a, .. } => Some(*a),
        }
    }

    fn right_half_color(&self) -> Option<Color> {
        match self {
            SplitMana::Mono { color, .. } => Some(*color),
            SplitMana::Colorless { color } => Some(*color),
            SplitMana::Duo { b, .. } => Some(*b),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleMana {
    Normal(Color),
    Phyrexian(Color),
}

impl Display for SingleMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleMana::Normal(color) => color.fmt(f),
            SingleMana::Phyrexian(color) => write!(f, "{}/P", color),
        }
    }
}

impl FromStr for SingleMana {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(color) = Color::from_str(s).ok() {
            Ok(Self::Normal(color))
        } else if let Some((first, second)) = s.split_once('/')
            && second == "P"
        {
            let color = Color::from_str(first)?;
            Ok(Self::Phyrexian(color))
        } else {
            Err(())
        }
    }
}

impl SingleMana {
    fn color(&self) -> Color {
        match self {
            SingleMana::Normal(color) | SingleMana::Phyrexian(color) => *color,
        }
    }
}

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

    fn left_half_color(&self) -> Option<Color> {
        match self {
            Mana::Single(single_mana) => Some(single_mana.color()),
            Mana::Generic(_) => None,
            Mana::Split(split_mana) => split_mana.left_half_color(),
            Mana::Colorless => None,
            Mana::Snow => None,
        }
    }

    fn right_half_color(&self) -> Option<Color> {
        match self {
            Mana::Single(single_mana) => Some(single_mana.color()),
            Mana::Generic(_) => None,
            Mana::Split(split_mana) => split_mana.right_half_color(),
            Mana::Colorless => None,
            Mana::Snow => None,
        }
    }
}

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
        let manas: Result<Result<Vec<Mana>, ()>, ()> = GroupIterator::new(s)
            .map(|x| x.map(Mana::from_str))
            .collect();
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

        let (colorless_hybrid, rest) = take_while(rest, |x| {
            matches!(x, Mana::Split(SplitMana::Colorless { .. }))
        });
        println!("{:?}", colorless_hybrid);
        sort_by_colors(colorless_hybrid, |x| x.right_half_color().unwrap());

        let (colored, snow) = take_while(rest, |x| {
            matches!(x, Mana::Single(_) | Mana::Split(SplitMana::Duo { .. }))
        });

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

/// Every element in the first array will satisfy `pred` and the first element of
/// the second array will not satisfy `pred` (or it will be empty).
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
    use crate::color::ALL_COLORS;

    use super::*;

    fn sort_colors(colors: &mut [Color], goal: &[Color]) {
        let mut color_set = ColorSet::new();
        for &c in colors.iter() {
            color_set.set_color(c);
        }

        let order = color_set.order_values();
        colors.sort_by_key(|x| order[*x as usize]);

        assert_eq!(colors, goal);
    }

    #[test]
    fn sort_five() {
        let mut colors = ALL_COLORS;
        sort_colors(&mut colors, &ALL_COLORS);
    }

    #[test]
    fn sort_two() {
        let mut unsorted = [Color::Green, Color::Red];
        let sorted = [Color::Red, Color::Green];
        sort_colors(&mut unsorted, &sorted);

        let mut unsorted = [Color::Green, Color::Black];
        let sorted = [Color::Black, Color::Green];
        sort_colors(&mut unsorted, &sorted);
    }

    #[test]
    fn wubrg() {
        assert!(Color::White.le(&Color::Blue));
        assert!(Color::Blue.le(&Color::Black));
        assert!(Color::Black.le(&Color::Red));
        assert!(Color::Red.le(&Color::Green));
    }

    #[test]
    fn empty() {
        assert!(Manas::from_str("{}").is_err());
    }

    #[test]
    fn hybrid() {
        assert!(Manas::from_str("{W}{U}").is_ok());
    }

    // https://scryfall.com/card/hop/96/arsenal-thresher
    #[test]
    fn arsenal_thresher() {
        let s = "{2}{W/B}{U}";
        let manas = Manas::from_str(s).unwrap();
        assert_eq!(s.to_string(), manas.to_string());
    }

    // From [RDDT]
    #[test]
    fn long_example() {
        let s = "{X}{Y}{4}{2/B}{2/R}{C}{C/U}{B}{B/R/P}{R/P}{R/W}{G}{G/W/P}{W}{W/U}{S}";
        let manas = Manas::from_str(s).unwrap();
        assert_eq!(s.to_string(), manas.to_string());
    }

    // From [RDDT]
    #[test]
    fn sort_long() {
        let before = "{R/P}{X}{C/U}{2/B}{W}{W/U}{B}{B/R/P}{2/R}{G}{C}{G/W/P}{S}{4}{Y}{R/W}";
        let after = "{X}{Y}{4}{2/B}{2/R}{C}{C/U}{B}{B/R/P}{R/P}{R/W}{G}{G/W/P}{W}{W/U}{S}";
        let mut manas_before = Manas::from_str(before).unwrap();

        manas_before.normalize();
        assert_eq!(manas_before.to_string(), after);
    }
}
