use lib_terrain::TerrainType;
use lib_world_generation::{MapTile, TerrainGenerationWeights};

use crate::support::noise;

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
            let map_tile: MapTile = MapTile::generate_from_noise_value(&weights, noise(value));

            assert_eq!(*terrain, map_tile.terrain_type(), "{value}");
        });
    });
}
