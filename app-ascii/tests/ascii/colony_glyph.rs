use app_ascii::{colony_glyph, glyph_for, unit_glyph};
use lib_terrain::TerrainType;
use lib_unit::UnitType;

#[test]
fn should_show_a_colony_as_a_hash_unlike_any_terrain_or_unit() {
    let glyph = colony_glyph();

    assert_eq!('#', glyph.symbol);
    TerrainType::ALL.into_iter().for_each(|terrain| {
        assert_ne!(glyph_for(terrain).symbol, glyph.symbol, "{terrain:?}");
        assert_ne!(glyph_for(terrain).color, glyph.color, "{terrain:?}");
    });
    [UnitType::Colonist, UnitType::Caravel]
        .into_iter()
        .for_each(|unit_type| {
            assert_ne!(unit_glyph(unit_type).symbol, glyph.symbol, "{unit_type:?}");
            assert_ne!(unit_glyph(unit_type).color, glyph.color, "{unit_type:?}");
        });
}
