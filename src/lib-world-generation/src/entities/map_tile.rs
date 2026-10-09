use lib_terrain::entities::terrain_type::TerrainType;

use crate::entities::{
    noise_value::NoiseValue, terrain_generation_weights::TerrainGenerationWeights,
};

#[derive(Debug, PartialEq)]
struct MapTile {
    terrain_type: TerrainType,
}

impl MapTile {
    fn generate_from_noise_value(weights: &TerrainGenerationWeights, noise: NoiseValue) -> Self {
        Self {
            terrain_type: weights.terrain_for(noise),
        }
    }
}

#[cfg(test)]
mod test {
    use lib_terrain::entities::terrain_type::TerrainType;

    use crate::entities::{
        map_tile::MapTile, noise_value::NoiseValue,
        terrain_generation_weights::TerrainGenerationWeights,
    };

    #[test]
    fn should_generate_only_one_terrain_given_type_and_any_noise_value() {
        let terrains: Vec<TerrainType> = vec![
            TerrainType::Mountain,
            TerrainType::Hills,
            TerrainType::Forest,
            TerrainType::Jungle,
            TerrainType::Plains,
            TerrainType::Sea,
        ];

        terrains.iter().for_each(|terrain| {
            let weights: TerrainGenerationWeights =
                TerrainGenerationWeights::new(&Vec::from([(*terrain, 1.0)])).unwrap();

            let noise_values: Vec<f32> = vec![0.0, 0.1, 0.4, 0.5, 0.6, 0.9, 1.0];

            noise_values.into_iter().for_each(|value| {
                let noise = NoiseValue::new(value).unwrap();
                let map_tile: MapTile = MapTile::generate_from_noise_value(&weights, noise);

                assert_eq!(
                    MapTile {
                        terrain_type: *terrain
                    },
                    map_tile
                )
            });
        });
    }
}
