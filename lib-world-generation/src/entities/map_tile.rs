use lib_terrain::TerrainType;

use crate::entities::{
    noise_value::NoiseValue, terrain_generation_weights::TerrainGenerationWeights,
};

#[derive(Debug)]
pub struct MapTile {
    terrain_type: TerrainType,
}

impl MapTile {
    #[must_use]
    pub fn generate_from_noise_value(
        weights: &TerrainGenerationWeights,
        noise: NoiseValue,
    ) -> Self {
        Self {
            terrain_type: weights.terrain_for(noise),
        }
    }

    #[must_use]
    pub fn terrain_type(&self) -> TerrainType {
        self.terrain_type
    }
}
