use app_ascii::{glyph_for, unit_glyph};
use lib_terrain::TerrainType;
use lib_unit::UnitType;

#[test]
fn should_show_a_free_colonist_as_an_at_sign_unlike_any_terrain() {
    let glyph = unit_glyph(UnitType::FreeColonist);

    assert_eq!('@', glyph.symbol);
    TerrainType::ALL.into_iter().for_each(|terrain| {
        assert_ne!(glyph_for(terrain).symbol, glyph.symbol, "{terrain:?}");
        assert_ne!(glyph_for(terrain).color, glyph.color, "{terrain:?}");
    });
}
