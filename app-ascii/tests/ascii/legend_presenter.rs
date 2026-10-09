use app_ascii::{Glyph, LegendViewModel, colony_glyph, glyph_for, present_legend, unit_glyph};
use lib_terrain::TerrainType;
use lib_unit::UnitType;

#[test]
fn should_name_every_terrain_next_to_its_map_glyph() {
    let legend: LegendViewModel = present_legend();

    let entries: Vec<(Glyph, &str)> = legend
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

#[test]
fn should_name_every_unit_next_to_its_map_glyph() {
    let legend: LegendViewModel = present_legend();

    let entries: Vec<(Glyph, &str)> = legend
        .units
        .iter()
        .map(|entry| (entry.glyph, entry.label.as_str()))
        .collect();
    assert_eq!(
        vec![
            (unit_glyph(UnitType::Colonist), "Colonist"),
            (unit_glyph(UnitType::Caravel), "Caravel"),
        ],
        entries
    );
}

#[test]
fn should_name_the_colony_next_to_its_map_glyph() {
    let legend: LegendViewModel = present_legend();

    let entries: Vec<(Glyph, &str)> = legend
        .colonies
        .iter()
        .map(|entry| (entry.glyph, entry.label.as_str()))
        .collect();
    assert_eq!(vec![(colony_glyph(), "Colony")], entries);
}

#[test]
fn should_list_the_controls() {
    let legend: LegendViewModel = present_legend();

    let controls: Vec<(&str, &str)> = legend
        .controls
        .iter()
        .map(|control| (control.keys.as_str(), control.action.as_str()))
        .collect();
    assert_eq!(
        vec![
            ("WASD", "Move unit"),
            ("Tab", "Switch unit"),
            ("B", "Build colony"),
            ("Arrows", "Pan view"),
            ("R", "New map"),
        ],
        controls
    );
}
