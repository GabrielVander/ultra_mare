use lib_terrain::TerrainType;
use lib_unit::UnitType;

use crate::{
    map_view::terrain_glyph::{Glyph, glyph_for},
    units::unit_glyph::unit_glyph,
};

#[derive(Debug, PartialEq)]
pub struct LegendViewModel {
    pub terrains: Vec<LegendEntry>,
    pub units: Vec<LegendEntry>,
    pub controls: Vec<ControlHint>,
}

#[derive(Debug, PartialEq)]
pub struct LegendEntry {
    pub glyph: Glyph,
    pub label: String,
}

#[derive(Debug, PartialEq)]
pub struct ControlHint {
    pub keys: String,
    pub action: String,
}

pub fn present_legend() -> LegendViewModel {
    LegendViewModel {
        terrains: TerrainType::ALL.into_iter().map(terrain_entry).collect(),
        units: vec![
            unit_entry(UnitType::Colonist),
            unit_entry(UnitType::Caravel),
        ],
        controls: vec![
            control_hint("WASD", "Move unit"),
            control_hint("Tab", "Switch unit"),
            control_hint("B", "Build colony"),
            control_hint("Arrows", "Pan view"),
            control_hint("R", "New map"),
        ],
    }
}

fn terrain_entry(terrain: TerrainType) -> LegendEntry {
    LegendEntry {
        glyph: glyph_for(terrain),
        label: terrain_name(terrain).to_string(),
    }
}

fn terrain_name(terrain: TerrainType) -> &'static str {
    match terrain {
        TerrainType::Mountain => "Mountain",
        TerrainType::Hills => "Hills",
        TerrainType::Forest => "Forest",
        TerrainType::Jungle => "Jungle",
        TerrainType::Plains => "Plains",
        TerrainType::Sea => "Sea",
    }
}

fn unit_entry(unit_type: UnitType) -> LegendEntry {
    LegendEntry {
        glyph: unit_glyph(unit_type),
        label: unit_name(unit_type).to_string(),
    }
}

fn unit_name(unit_type: UnitType) -> &'static str {
    match unit_type {
        UnitType::Colonist => "Colonist",
        UnitType::Caravel => "Caravel",
    }
}

fn control_hint(keys: &str, action: &str) -> ControlHint {
    ControlHint {
        keys: keys.to_string(),
        action: action.to_string(),
    }
}
