use crate::{Color, color::ALL_COLORS};

pub(crate) struct ColorSet {
    bitset: u8,
}

const VALUES: usize = 0b11111 + 1;

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
