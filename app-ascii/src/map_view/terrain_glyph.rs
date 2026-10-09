use lib_terrain::TerrainType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Glyph {
    pub symbol: char,
    pub color: Rgb,
}

pub fn glyph_for(terrain: TerrainType) -> Glyph {
    match terrain {
        TerrainType::Mountain => glyph('^', 170, 170, 170),
        TerrainType::Hills => glyph('n', 176, 136, 80),
        TerrainType::Forest => glyph('T', 46, 160, 67),
        TerrainType::Jungle => glyph('%', 20, 100, 40),
        TerrainType::Plains => glyph('"', 190, 210, 90),
        TerrainType::Sea => glyph('~', 50, 110, 210),
    }
}

fn glyph(symbol: char, red: u8, green: u8, blue: u8) -> Glyph {
    Glyph {
        symbol,
        color: Rgb { red, green, blue },
    }
}
