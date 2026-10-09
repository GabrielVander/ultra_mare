use lib_unit::UnitType;

use crate::map_view::terrain_glyph::{Glyph, Rgb};

pub fn unit_glyph(unit_type: UnitType) -> Glyph {
    match unit_type {
        UnitType::Colonist => Glyph {
            symbol: '@',
            color: Rgb {
                red: 255,
                green: 255,
                blue: 255,
            },
        },
        UnitType::Caravel => Glyph {
            symbol: 'V',
            color: Rgb {
                red: 205,
                green: 133,
                blue: 63,
            },
        },
    }
}
