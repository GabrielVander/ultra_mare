use lib_terrain::TerrainType;
use lib_unit::{Expedition, MapTerrain, TilePosition};

/// A map drawn as rows of text: `~` is sea, any other character is plains.
pub struct TerrainGrid(pub &'static [&'static str]);

impl TerrainGrid {
    /// Every tile with its terrain, row by row from the top-left corner.
    pub fn tiles(&self) -> Vec<(TilePosition, TerrainType)> {
        (0..self.0.len() as u32)
            .flat_map(|y| (0..self.0[y as usize].len() as u32).map(move |x| TilePosition { x, y }))
            .filter_map(|position| self.terrain_at(position).map(|terrain| (position, terrain)))
            .collect()
    }
}

impl MapTerrain for TerrainGrid {
    fn terrain_at(&self, position: TilePosition) -> Option<TerrainType> {
        let row = self.0.get(position.y as usize)?;
        let symbol = row.chars().nth(position.x as usize)?;
        Some(match symbol {
            '~' => TerrainType::Sea,
            _ => TerrainType::Plains,
        })
    }
}

/// An expedition whose colonist stands on `position`, with no caravel.
pub fn colonist_at(position: TilePosition) -> Expedition {
    Expedition::land(&[(position, TerrainType::Plains)])
}
