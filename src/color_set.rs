use crate::{Color, color::ALL_COLORS};

pub(crate) struct ColorSet {
    bitset: u8,
}

/// There are 2 ^ 5 different color-sets
const COLOR_SETS: usize = 0b11111 + 1;

const fn add_order(a: &mut [[u8; 5]; COLOR_SETS], color: Color, offsets: &[usize]) {
    let mut set = ColorSet::new();
    let mut i: usize = 0;
    while i < offsets.len() {
        let c = color.next(offsets[i]);
        set.set_color(c);
        i += 1;
    }
    let mut i: usize = 1;
    while i < offsets.len() {
        let c = color.next(offsets[i]);
        a[set.bitset as usize][c as usize] = i as u8;
        i += 1;
    }
}

// We precompute the order of each color combination
const ORDER_ARRAY: [[u8; 5]; COLOR_SETS] = {
    let mut array = [[0; 5]; COLOR_SETS];
    let mut color_i = 0;
    while color_i != 5 {
        let color = ALL_COLORS[color_i];

        // when we have zero and one active bits we just return zeros
        // so we start at two values

        // Adjacent colors
        add_order(&mut array, color, &[0, 1]);

        // Two steps away
        add_order(&mut array, color, &[0, 2]);

        // Three adjacent colors
        add_order(&mut array, color, &[0, 1, 2]);

        // Two adjacent and one opposite. We do not minimize total distance between
        // adjacent mana values here.
        add_order(&mut array, color, &[1, 3, 0]);

        // Four colors
        add_order(&mut array, color, &[0, 1, 2, 3]);
        color_i += 1;
    }

    // Five colors
    array[0b11111] = [0, 1, 2, 3, 4];

    array
};

impl ColorSet {
    pub const fn new() -> Self {
        Self { bitset: 0 }
    }

    pub const fn set_color(&mut self, color: Color) {
        // colors should be numbered [0-4]
        debug_assert!((color as u8) < 5);

        // set the bit corresponding to the color
        self.bitset |= 1 << color as u8;
    }

    pub const fn order_values(&self) -> &[u8] {
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
    fn urw() {
        let mut unsorted = [Color::White, Color::Red, Color::Blue];
        let sorted = [Color::Blue, Color::Red, Color::White];
        sort_colors(&mut unsorted, &sorted);
    }
}
