use std::collections::HashSet;

use app_ascii::{Glyph, Rgb, glyph_for};
use lib_terrain::TerrainType;

#[test]
fn should_give_every_terrain_its_own_symbol_and_color() {
    let glyphs: [Glyph; 6] = TerrainType::ALL.map(glyph_for);

    let symbols: HashSet<char> = glyphs.iter().map(|glyph| glyph.symbol).collect();
    let colors: HashSet<Rgb> = glyphs.iter().map(|glyph| glyph.color).collect();
    assert_eq!(TerrainType::ALL.len(), symbols.len());
    assert_eq!(TerrainType::ALL.len(), colors.len());
}
