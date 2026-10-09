use lib_terrain::TerrainType;

use crate::map_view::terrain_glyph::{Glyph, glyph_for};

#[derive(Debug, PartialEq)]
pub struct LegendViewModel {
    pub terrains: Vec<LegendEntry>,
}

#[derive(Debug, PartialEq)]
pub struct LegendEntry {
    pub glyph: Glyph,
    pub label: String,
}

pub fn present_legend() -> LegendViewModel {
    LegendViewModel {
        terrains: TerrainType::ALL.into_iter().map(legend_entry).collect(),
    }
}

fn legend_entry(terrain: TerrainType) -> LegendEntry {
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
