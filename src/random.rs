#![cfg(feature = "rand")]

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use crate::{Color, GenericMana, Mana, SingleMana, SplitMana};

impl Distribution<Color> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let i = rng.random_range(0u8..5);
        Color::from_usize(i as usize)
    }
}

impl Distribution<SingleMana> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SingleMana {
        let c: Color = rng.sample(StandardUniform);
        if rng.random_bool(0.5) { SingleMana::Normal(c) } else { SingleMana::Phyrexian(c) }
    }
}

impl Distribution<SplitMana> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SplitMana {
        let sort = rng.random_range(0..3);
        let c1: Color = rng.sample(StandardUniform);
        match sort {
            0 => {
                let value = rng.random_range(0..20);
                SplitMana::Mono { value, color: c1 }
            }
            1 => SplitMana::Colorless(c1),
            2 => {
                let c2: Color = rng.sample(StandardUniform);
                let phyrexian = rng.random_bool(0.5);
                SplitMana::Duo { a: c1, b: c2, phyrexian }
            }
            _ => unreachable!(),
        }
    }
}

impl Distribution<Mana> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mana {
        let sort = rng.random_range(0..5);
        match sort {
            0 => Mana::Single(rng.sample(StandardUniform)),
            1 => Mana::Generic(rng.sample(StandardUniform)),
            2 => Mana::Split(rng.sample(StandardUniform)),
            3 => Mana::Colorless,
            4 => Mana::Snow,
            _ => unreachable!(),
        }
    }
}

impl Distribution<GenericMana> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GenericMana {
        let sort = rng.random_range(0..4);
        match sort {
            0 => {
                let value = rng.random_range(0..20);
                GenericMana::Number(value)
            }
            1 => GenericMana::X,
            2 => GenericMana::Y,
            3 => GenericMana::Z,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mana() {
        let mut rng = rand::rng();
        let _: Mana = rng.sample(StandardUniform);
    }
}
