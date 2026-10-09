use app_ascii::{Glyph, glyph_for, unit_glyph};
use lib_terrain::TerrainType;
use lib_unit::UnitType;

#[test]
fn should_show_a_colonist_as_an_at_sign_unlike_any_terrain() {
    let glyph: Glyph = unit_glyph(UnitType::Colonist);

    assert_eq!('@', glyph.symbol);
    TerrainType::ALL.into_iter().for_each(|terrain| {
        assert_ne!(glyph_for(terrain).symbol, glyph.symbol, "{terrain:?}");
        assert_ne!(glyph_for(terrain).color, glyph.color, "{terrain:?}");
    });
}

#[test]
fn should_show_a_caravel_as_a_v_unlike_any_terrain_or_colonist() {
    let glyph: Glyph = unit_glyph(UnitType::Caravel);
    let colonist: Glyph = unit_glyph(UnitType::Colonist);

    assert_eq!('V', glyph.symbol);
    assert_ne!(colonist.symbol, glyph.symbol);
    assert_ne!(colonist.color, glyph.color);
    TerrainType::ALL.into_iter().for_each(|terrain| {
        assert_ne!(glyph_for(terrain).symbol, glyph.symbol, "{terrain:?}");
        assert_ne!(glyph_for(terrain).color, glyph.color, "{terrain:?}");
    });
}
