use app_ascii::{glyph_for, present_legend};
use lib_terrain::TerrainType;

#[test]
fn should_name_every_terrain_next_to_its_map_glyph() {
    let legend = present_legend();

    let entries: Vec<_> = legend
        .terrains
        .iter()
        .map(|entry| (entry.glyph, entry.label.as_str()))
        .collect();
    assert_eq!(
        vec![
            (glyph_for(TerrainType::Mountain), "Mountain"),
            (glyph_for(TerrainType::Hills), "Hills"),
            (glyph_for(TerrainType::Forest), "Forest"),
            (glyph_for(TerrainType::Jungle), "Jungle"),
            (glyph_for(TerrainType::Plains), "Plains"),
            (glyph_for(TerrainType::Sea), "Sea"),
        ],
        entries
    );
}
