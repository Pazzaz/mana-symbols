// Based on: https://www.reddit.com/r/custommagic/comments/1nhtr3w/guide_for_formatting_mana_costs/

use std::fmt::{Display, Write};

/// The five main colors:
/// - White (W)
/// - Blue (U),
/// - Black (B),
/// - Red (R),
/// - Green (G),
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Color {
    White = 0,
    Blue = 1,
    Black = 2,
    Red = 3,
    Green = 4,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => f.write_char('W'),
            Color::Blue => f.write_char('U'),
            Color::Black => f.write_char('B'),
            Color::Red => f.write_char('R'),
            Color::Green => f.write_char('G'),
        }
    }
}

impl Color {
    fn from_inner(s: &str) -> Option<Self> {
        let color = match s {
            "W" => Color::White,
            "U" => Color::Blue,
            "B" => Color::Black,
            "R" => Color::Red,
            "G" => Color::Green,
            _ => return None,
        };
        Some(color)
    }

    const fn next_color(&self) -> Color {
        match self {
            Color::White => Color::Blue,
            Color::Blue => Color::Black,
            Color::Black => Color::Red,
            Color::Red => Color::Green,
            Color::Green => Color::White,
        }
    }
}

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

impl GenericMana {
    fn from_inner(s: &str) -> Option<Self> {
        let mana = match s {
            "X" => GenericMana::X,
            "Y" => GenericMana::Y,
            "Z" => GenericMana::Z,
            s => {
                if let Ok(n) = s.parse::<usize>() {
                    GenericMana::Number(n)
                } else {
                    return None;
                }
            }
        };
        Some(mana)
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

impl SplitMana {
    fn from_inner(s: &str) -> Option<Self> {
        if let Some((first, mut second)) = s.split_once('/') {
            let phyrexian: bool;
            (second, phyrexian) = {
                if let Some((new_second, third)) = second.split_once('/') {
                    (new_second, third == "P")
                } else {
                    (second, false)
                }
            };

            let b = Color::from_inner(second)?;
            if phyrexian {
                let a = Color::from_inner(first)?;
                Some(SplitMana::Duo {
                    a,
                    b,
                    phyrexian: true,
                })
            } else {
                if first == "C" {
                    Some(SplitMana::Colorless { color: b })
                } else if let Ok(value) = first.parse::<usize>() {
                    Some(SplitMana::Mono { value, color: b })
                } else {
                    let a = Color::from_inner(first)?;
                    Some(SplitMana::Duo {
                        a,
                        b,
                        phyrexian: false,
                    })
                }
            }
        } else {
            None
        }
    }

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

impl SingleMana {
    pub fn from_inner(s: &str) -> Option<Self> {
        if let Some(color) = Color::from_inner(s) {
            Some(Self::Normal(color))
        } else if let Some((first, second)) = s.split_once('/')
            && second == "P"
        {
            let color = Color::from_inner(first)?;
            Some(Self::Phyrexian(color))
        } else {
            None
        }
    }

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

    pub fn from_inner(s: &str) -> Result<Self, ()> {
        if let Some(single) = SingleMana::from_inner(s) {
            Ok(Mana::Single(single))
        } else if let Some(generic_mana) = GenericMana::from_inner(s) {
            Ok(Mana::Generic(generic_mana))
        } else if let Some(split) = SplitMana::from_inner(s) {
            Ok(Mana::Split(split))
        } else if s == "C" {
            Ok(Mana::Colorless)
        } else if s == "S" {
            Ok(Mana::Snow)
        } else {
            Err(())
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

impl Manas {
    pub fn mana_value(&self) -> usize {
        self.manas.iter().map(Mana::mana_value).sum()
    }

    pub fn from_str(s: &str) -> Result<Self, ()> {
        let manas: Result<Result<Vec<Mana>, ()>, ()> = GroupIterator::new(s)
            .map(|x| x.map(Mana::from_inner))
            .collect();
        manas.flatten().map(|manas| Manas { manas })
    }

    /// Normalizes hybrid mana symbols and sorts the mana symbols
    pub fn normalize(&mut self) {
        // Normalize hybird mana
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
            sort_by_colors(hybrid_phyrexian,     |x| x.right_half_color().unwrap());
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

struct ColorSet {
    bitset: u8,
}

const VALUES: usize = 0b11111 + 1;

const ALL_COLORS: [Color; 5] = [
    Color::White,
    Color::Blue,
    Color::Black,
    Color::Red,
    Color::Green,
];

// We precompute the order of each color combination
const ORDER_ARRAY: [[u8; 5]; VALUES] = {
    let mut array = [[0; 5]; VALUES];

    // when we have zero and one active bits we just return zeros
    // so we start at two values
    let mut color_i = 0;
    while color_i != 5 {
        let color = ALL_COLORS[color_i];
        let next1 = color.next_color();
        let next2 = next1.next_color();

        // Adjacent color
        {
            let mut i = ColorSet::new();
            i.set_color(color);
            i.set_color(next1);
            array[i.bitset as usize][next1 as usize] = 1;
        }

        // Two steps away
        {
            let mut i = ColorSet::new();
            i.set_color(color);
            i.set_color(next2);
            array[i.bitset as usize][next2 as usize] = 1;
        }

        color_i += 1;
    }

    // Three colors
    let mut color_i = 0;
    while color_i != 5 {
        let color = ALL_COLORS[color_i];
        let next1 = color.next_color();
        let next2 = next1.next_color();
        let next3 = next2.next_color();

        // Three adjacent colors
        {
            let mut i = ColorSet::new();
            i.set_color(color);
            i.set_color(next1);
            i.set_color(next2);
            array[i.bitset as usize][next1 as usize] = 1;
            array[i.bitset as usize][next2 as usize] = 2;
        }

        // Two adjacent and one opposite
        {
            let mut i = ColorSet::new();
            i.set_color(color);
            i.set_color(next1);
            i.set_color(next3);
            array[i.bitset as usize][next3 as usize] = 1;
            array[i.bitset as usize][color as usize] = 2;
        }

        color_i += 1;
    }

    // Four colors
    let mut color_i = 0;
    while color_i != 5 {
        let color = ALL_COLORS[color_i];
        let next1 = color.next_color();
        let next2 = next1.next_color();
        let next3 = next2.next_color();

        {
            let mut i = ColorSet::new();
            i.set_color(color);
            i.set_color(next1);
            i.set_color(next2);
            i.set_color(next3);
            array[i.bitset as usize][next1 as usize] = 1;
            array[i.bitset as usize][next2 as usize] = 2;
            array[i.bitset as usize][next3 as usize] = 3;
        }

        color_i += 1;
    }

    // Five colors
    array[0b11111] = [0, 1, 2, 3, 4];

    array
};

impl ColorSet {
    const fn new() -> Self {
        Self { bitset: 0 }
    }

    const fn set_color(&mut self, color: Color) {
        self.bitset |= 1 << color as u8;
    }

    fn order_values(&self) -> &[u8] {
        &ORDER_ARRAY[self.bitset as usize]
    }
}

#[cfg(test)]
mod tests {
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
