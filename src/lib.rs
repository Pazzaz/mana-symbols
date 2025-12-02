//! # Mana Symbols
//! This crate models [Mana costs][mw:mc] from [Magic the Gathering][wp:mtg]
//! cards. It can parse text representations of mana (e.g. `{5}{U}{U/B}`), sort
//! mana costs and calculate [mana values][mw:mv].
//!
//! ## Supported mana
//!
//! The types of mana supported by this library (as [`Mana`]) are:
//! - [Generic mana][mw:gm]
//! - [Colorless mana][mw:clm]
//! - [Colored][mw:c] mana (including [phyrexian][mw:pm])
//! - [Hybrid mana][mw:hm] (including generic, colorless and phyrexian)
//! - [Snow mana][mw:sc]
//!
//! [mw:mc]:  https://mtg.wiki/page/Mana_cost
//! [mw:mv]:  https://mtg.wiki/page/Mana_value
//! [mw:gm]:  https://mtg.wiki/page/Generic_mana
//! [mw:clm]: https://mtg.wiki/page/Colorless#Colorless_mana
//! [mw:pm]:  https://mtg.wiki/page/Phyrexian_mana
//! [mw:c]:   https://mtg.wiki/page/Color
//! [mw:hm]:  https://mtg.wiki/page/Hybrid_mana
//! [mw:sc]:  https://mtg.wiki/page/Snow#Snow_costs
//!
//! [wp:mtg]:  https://en.wikipedia.org/wiki/Magic:_The_Gathering
//! [wp:wotc]: https://en.wikipedia.org/wiki/Wizards_of_the_Coast
//!
//! [reddit:user]: https://www.reddit.com/user/Mean-Government1436
//! [reddit:post]: https://www.reddit.com/r/custommagic/comments/1nhtr3w/guide_for_formatting_mana_costs/

mod color;
mod color_set;
mod generic_mana;
mod mana;
mod manas;
mod single_mana;
mod split_mana;
mod symbols;

pub use color::Color;
pub(crate) use generic_mana::GenericMana;
pub use mana::Mana;
pub use manas::Manas;
pub(crate) use single_mana::SingleMana;
pub(crate) use split_mana::SplitMana;
