use nom::{AsChar, Input, Parser, character::complete::char, error::ParseError};

/// Configuration value for the case of string representations
///
/// Used by parsing functions such as [`Manas::parse`](crate::Manas::parse)
/// and [`Mana::parse`](crate::Mana::parse).
///
/// The default value is [`Either`](Case::Either).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Case {
    /// Lower case (e.g. `wubrg`)
    Lower,
    /// Upper case (e.g. `WUBRG`)
    Upper,
    /// Any case is valid (e.g. `wUrBG`)
    #[default]
    Either,
}

pub fn parse_char<I, Error: ParseError<I>>(
    case: Case,
    c: char,
) -> impl Parser<I, Output = char, Error = Error>
where
    I: Input,
    <I as Input>::Item: AsChar,
{
    let c_lo = c.to_ascii_lowercase();
    let c_up = c.to_ascii_uppercase();
    move |input| match case {
        Case::Lower => char(c_lo).parse(input),
        Case::Upper => char(c_up).parse(input),
        Case::Either => char(c_lo).or(char(c_up)).parse(input),
    }
}
