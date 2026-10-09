use lib_terrain::TerrainType;
use lib_unit::{MapTerrain, TilePosition};

pub struct Shore;

impl Shore {
    pub fn tiles(&self) -> Vec<(TilePosition, TerrainType)> {
        (0..4)
            .map(|x| TilePosition { x, y: 0 })
            .filter_map(|position| self.terrain_at(position).map(|terrain| (position, terrain)))
            .collect()
    }
}

impl MapTerrain for Shore {
    fn terrain_at(&self, position: TilePosition) -> Option<TerrainType> {
        match (position.x, position.y) {
            (0..=1, 0) => Some(TerrainType::Plains),
            (2..=3, 0) => Some(TerrainType::Sea),
            _ => None,
        }
    }
}

/// A map of a single tile.
pub struct Island(pub TerrainType);

impl Island {
    pub fn tiles(&self) -> Vec<(TilePosition, TerrainType)> {
        vec![(TilePosition { x: 0, y: 0 }, self.0)]
    }
}

impl MapTerrain for Island {
    fn terrain_at(&self, position: TilePosition) -> Option<TerrainType> {
        (position == TilePosition { x: 0, y: 0 }).then_some(self.0)
    }
}
