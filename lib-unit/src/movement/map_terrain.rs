use lib_terrain::TerrainType;

use crate::entities::tile_position::TilePosition;

/// The map as units see it: what terrain lies where.
pub trait MapTerrain {
    /// The terrain at `position`, or `None` when it is off the map.
    fn terrain_at(&self, position: TilePosition) -> Option<TerrainType>;
}
