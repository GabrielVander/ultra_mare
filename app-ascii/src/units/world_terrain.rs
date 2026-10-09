use lib_terrain::TerrainType;
use lib_unit::{MapTerrain, TilePosition};
use lib_world_generation::{MapPosition, MapTile, WorldMap};

/// The generated world map, seen through the unit context's eyes.
pub struct WorldTerrain<'a> {
    map: &'a WorldMap,
}

impl<'a> WorldTerrain<'a> {
    #[must_use]
    pub fn new(map: &'a WorldMap) -> Self {
        Self { map }
    }

    /// Every tile with its terrain, row by row from the top-left corner.
    pub fn tiles(&self) -> impl Iterator<Item = (TilePosition, TerrainType)> {
        let size = self.map.size();
        (0..size.height())
            .flat_map(move |y| (0..size.width()).map(move |x| TilePosition { x, y }))
            .filter_map(|position| self.terrain_at(position).map(|terrain| (position, terrain)))
    }
}

impl MapTerrain for WorldTerrain<'_> {
    fn terrain_at(&self, position: TilePosition) -> Option<TerrainType> {
        self.map
            .tile_at(map_position(position))
            .map(MapTile::terrain_type)
    }
}

#[must_use]
pub fn map_position(position: TilePosition) -> MapPosition {
    MapPosition {
        x: position.x,
        y: position.y,
    }
}
