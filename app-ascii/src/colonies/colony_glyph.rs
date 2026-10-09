use crate::map_view::terrain_glyph::{Glyph, Rgb};

pub fn colony_glyph() -> Glyph {
    Glyph {
        symbol: '#',
        color: Rgb {
            red: 235,
            green: 70,
            blue: 60,
        },
    }
}
