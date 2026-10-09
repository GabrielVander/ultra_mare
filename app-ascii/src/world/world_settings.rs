use std::num::NonZeroU32;

use lib_terrain::TerrainType;
use lib_world_generation::{MapSize, TerrainGenerationWeights, ValueNoise, WorldMap};

const MAP_WIDTH: u32 = 120;
const MAP_HEIGHT: u32 = 80;
const LANDMASS_SIZE: NonZeroU32 = NonZeroU32::new(12).unwrap();

// Ordered from the lowest noise to the highest, so the noise reads as elevation:
// sea fills the valleys and mountains crown the peaks.
const TERRAIN_WEIGHTS: [(TerrainType, f32); 6] = [
    (TerrainType::Sea, 0.55),
    (TerrainType::Plains, 0.12),
    (TerrainType::Jungle, 0.08),
    (TerrainType::Forest, 0.12),
    (TerrainType::Hills, 0.08),
    (TerrainType::Mountain, 0.05),
];

#[must_use]
#[expect(
    clippy::missing_panics_doc,
    reason = "the constant settings are valid, so it never panics"
)]
pub fn generate_world(seed: u64) -> WorldMap {
    let size = MapSize::new(MAP_WIDTH, MAP_HEIGHT).expect("the map size is not empty");
    let weights =
        TerrainGenerationWeights::new(&TERRAIN_WEIGHTS).expect("the terrain weights are valid");

    WorldMap::generate(size, &weights, &ValueNoise::new(seed, LANDMASS_SIZE))
}
