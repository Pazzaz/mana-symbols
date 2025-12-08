use std::{
    f64,
    fmt::{Display, Write},
    str::FromStr,
};

use base64::{Engine, prelude::BASE64_STANDARD};
use nom::{
    Finish, IResult, Parser,
    branch::alt,
    character::complete::char,
    combinator::{eof, value},
    sequence::{delimited, terminated},
};
use svg::{
    Document,
    node::element::{Circle, Path, SVG, path::Data},
};

use crate::{
    Color, GenericMana, SVG_WIDTH, SingleMana, SplitMana,
    color::HEX_C,
    symbols::{
        color_symbol, colorless_symbol, number_symbol, phyrexian_symbol, snow_symbol, x_symbol,
        y_symbol, z_symbol,
    },
};

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

    /// Display the mana symbol as an [SVG](https://en.wikipedia.org/wiki/SVG).
    #[must_use]
    pub fn as_svg(&self) -> SVG {
        let shadow_offset = 1.5;
        let mut document = Document::new().set(
            "viewBox",
            (
                -shadow_offset,
                -shadow_offset,
                SVG_WIDTH + 2.0 * shadow_offset,
                SVG_WIDTH + 2.0 * shadow_offset,
            ),
        );

        document = with_shadow(document, shadow_offset);

        document = match self {
            Mana::Single(SingleMana::Normal(color)) => {
                document = with_circle(document, color.hex());
                with_symbol(document, color_symbol(*color), SVG_WIDTH, 0.8125)
            }
            Mana::Single(SingleMana::Phyrexian(color)) => {
                let document = with_circle(document, color.hex());
                with_symbol(document, phyrexian_symbol(), SVG_WIDTH, 0.8125)
            }
            Mana::Generic(GenericMana::Number(n)) => {
                document = with_circle(document, HEX_C);
                if let Some(symbol) = number_symbol(*n) {
                    with_symbol(document, symbol, SVG_WIDTH, 0.8125)
                } else {
                    document
                }
            }
            Mana::Generic(GenericMana::X) => {
                let document = with_circle(document, HEX_C);
                with_symbol(document, x_symbol(), SVG_WIDTH, 0.8125)
            }
            Mana::Generic(GenericMana::Y) => {
                let document = with_circle(document, HEX_C);
                with_symbol(document, y_symbol(), SVG_WIDTH, 0.8125)
            }
            Mana::Generic(GenericMana::Z) => {
                let document = with_circle(document, HEX_C);
                with_symbol(document, z_symbol(), SVG_WIDTH, 0.8125)
            }
            Mana::Split(SplitMana::Colorless { color }) => {
                document = with_split_circle(document, HEX_C, color.hex());
                with_symbols(document, colorless_symbol(), color_symbol(*color), SVG_WIDTH, 0.875)
            }
            Mana::Split(SplitMana::Mono { color, value }) => {
                document = with_split_circle(document, HEX_C, color.hex());
                if let Some(number) = number_symbol(*value) {
                    with_symbols(document, number, color_symbol(*color), SVG_WIDTH, 0.875)
                } else {
                    document
                }
            }
            Mana::Split(SplitMana::Duo { a, b, phyrexian }) => {
                document = with_split_circle(document, a.hex(), b.hex());
                if *phyrexian {
                    with_symbols(document, phyrexian_symbol(), phyrexian_symbol(), SVG_WIDTH, 0.875)
                } else {
                    with_symbols(document, color_symbol(*a), color_symbol(*b), SVG_WIDTH, 0.875)
                }
            }
            Mana::Colorless => {
                document = with_circle(document, HEX_C);
                with_symbol(document, colorless_symbol(), SVG_WIDTH, 0.8125)
            }
            Mana::Snow => {
                document = with_circle(document, HEX_C);
                with_symbol(document, snow_symbol(), SVG_WIDTH, 1.0)
            }
        };

        document
    }

    /// Display the mana symbol as a [`String`] of [HTML](https://en.wikipedia.org/wiki/HTML), where the image is an SVG (see [`Mana::as_svg`]).
    #[must_use]
    pub fn as_html(&self, include_css: bool) -> String {
        let mut out = String::new();
        self.write_html(&mut out, include_css).unwrap();
        out
    }

    /// Display the mana symbol as [HTML](https://en.wikipedia.org/wiki/HTML) written to `output` (see [`Mana::as_html`]).
    pub fn write_html<W: Write>(&self, output: &mut W, include_css: bool) -> std::fmt::Result {
        let svg = self.as_svg();
        let base64 = BASE64_STANDARD.encode(svg.to_string());
        let css = if include_css {
            r#" style="height: 1.5em; width: 1.7em; vertical-align: middle""#
        } else {
            ""
        };

        write!(
            output,
            r#"<img{css} alt="{{{self}}}" title="{}" src="data:image/svg+xml;base64,{base64}">"#,
            self.name()
        )
    }

    fn name(&self) -> String {
        match self {
            Mana::Single(SingleMana::Normal(color)) => format!("{} mana", color.name_capitalized()),
            Mana::Single(SingleMana::Phyrexian(color)) => {
                format!("Phyrexian {} mana", color.name())
            }
            Mana::Generic(GenericMana::Number(n)) => format!("{n} generic mana"),
            Mana::Generic(GenericMana::X) => "X generic mana".to_string(),
            Mana::Generic(GenericMana::Y) => "Y generic mana".to_string(),
            Mana::Generic(GenericMana::Z) => "Z generic mana".to_string(),
            Mana::Split(SplitMana::Mono { value, color }) => {
                format!("Hybrid mana: {value} generic or {}", color.name())
            }
            Mana::Split(SplitMana::Duo { a, b, phyrexian }) => {
                if *phyrexian {
                    format!("Phyrexian hybrid mana: {} or {}", a.name(), b.name())
                } else {
                    format!("Hybrid mana: {} or {}", a.name(), b.name())
                }
            }
            Mana::Split(SplitMana::Colorless { color }) => {
                format!("Hybrid mana: colorless or {}", color.name())
            }
            Mana::Colorless => "Colorless mana".to_string(),
            Mana::Snow => "Snow mana".to_string(),
        }
    }
}

#[must_use]
fn with_symbol(document: SVG, symbol: SVG, width: f64, size: f64) -> SVG {
    let symbol_width = width * size;
    let x_pos = width / 2.0;
    let y_pos = width / 2.0;
    let symbol = symbol
        .set("width", symbol_width)
        .set("height", symbol_width)
        .set("x", x_pos - symbol_width / 2.0)
        .set("y", y_pos - symbol_width / 2.0);
    document.add(symbol)
}

#[must_use]
fn with_symbols(
    mut document: SVG,
    symbol_left: SVG,
    symbol_right: SVG,
    width: f64,
    size: f64,
) -> SVG {
    let pi = f64::consts::PI;
    let x_right = f64::cos(pi / 4.0) * (width / 4.0) + (width / 2.0);
    let y_right = f64::sin(pi / 4.0) * (width / 4.0) + (width / 2.0);

    let x_left = f64::cos(pi / 4.0 + pi) * (width / 4.0) + (width / 2.0);
    let y_left = f64::sin(pi / 4.0 + pi) * (width / 4.0) + (width / 2.0);

    let symbol_width = (width / 2.0) * size;
    let symbol = symbol_right
        .set("width", symbol_width)
        .set("height", symbol_width)
        .set("x", x_right - symbol_width / 2.0)
        .set("y", y_right - symbol_width / 2.0);

    document = document.add(symbol);

    let symbol = symbol_left
        .set("width", symbol_width)
        .set("height", symbol_width)
        .set("x", x_left - symbol_width / 2.0)
        .set("y", y_left - symbol_width / 2.0);

    document.add(symbol)
}

#[must_use]
fn with_circle(document: SVG, fill: &str) -> SVG {
    let circle = Circle::new()
        .set("fill", fill)
        .set("stroke", "none")
        .set("r", SVG_WIDTH / 2.0)
        .set("cx", SVG_WIDTH / 2.0)
        .set("cy", SVG_WIDTH / 2.0);
    document.add(circle)
}

#[must_use]
fn with_shadow(document: SVG, offset: f64) -> SVG {
    let circle = Circle::new()
        .set("fill", "black")
        .set("stroke", "none")
        .set("r", SVG_WIDTH / 2.0)
        .set("cx", SVG_WIDTH / 2.0 - offset)
        .set("cy", SVG_WIDTH / 2.0 + offset);
    document.add(circle)
}

#[must_use]
fn with_split_circle(mut document: SVG, fill_left: &str, fill_right: &str) -> SVG {
    let pi = f64::consts::PI;
    let x_right = f64::cos(pi / 4.0) * 16.0 + 16.0;
    let y_right = -f64::sin(pi / 4.0) * 16.0 + 16.0;

    let x_left = f64::cos(pi / 4.0 + pi) * 16.0 + 16.0;
    let y_left = -f64::sin(pi / 4.0 + pi) * 16.0 + 16.0;

    let data = Data::new()
        .move_to((x_right, y_right))
        .elliptical_arc_to((16, 16, 0, 0, 1, x_left, y_left))
        .close();

    let path = Path::new().set("d", data).set("fill", fill_right);
    document = document.add(path);

    let data = Data::new()
        .move_to((x_right, y_right))
        .elliptical_arc_to((16, 16, 0, 0, 0, x_left, y_left))
        .close();

    let path = Path::new().set("d", data).set("fill", fill_left);
    document.add(path)
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
