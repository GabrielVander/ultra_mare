use crate::{
    entities::{
        map_position::MapPosition, map_size::MapSize, map_tile::MapTile,
        terrain_generation_weights::TerrainGenerationWeights,
    },
    noise::noise_source::NoiseSource,
};

#[derive(Debug)]
pub struct WorldMap {
    size: MapSize,
    tiles: Vec<MapTile>,
}

impl WorldMap {
    pub fn generate(
        size: MapSize,
        weights: &TerrainGenerationWeights,
        noise: &impl NoiseSource,
    ) -> Self {
        let tiles: Vec<MapTile> = WorldMap::positions(size)
            .map(|position| MapTile::generate_from_noise_value(weights, noise.noise_at(position)))
            .collect();

        Self { size, tiles }
    }

    #[must_use]
    pub fn size(&self) -> MapSize {
        self.size
    }

    #[must_use]
    pub fn tile_at(&self, position: MapPosition) -> Option<&MapTile> {
        if !self.contains(position) {
            return None;
        }

        self.tiles
            .get(position.y as usize * self.size.width() as usize + position.x as usize)
    }

    fn contains(&self, position: MapPosition) -> bool {
        position.x < self.size.width() && position.y < self.size.height()
    }

    fn positions(size: MapSize) -> impl Iterator<Item = MapPosition> {
        (0..size.height()).flat_map(move |y| (0..size.width()).map(move |x| MapPosition { x, y }))
    }
}
