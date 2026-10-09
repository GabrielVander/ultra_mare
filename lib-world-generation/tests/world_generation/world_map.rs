use lib_terrain::TerrainType;
use lib_world_generation::{
    MapPosition, MapSize, MapTile, NoiseSource, NoiseValue, TerrainGenerationWeights, WorldMap,
};

use crate::support::{ConstantNoise, noise};

#[test]
fn should_have_the_requested_size() {
    let size = MapSize::new(3, 2).unwrap();
    let weights = TerrainGenerationWeights::new(&[(TerrainType::Sea, 1.0)]).unwrap();

    let map = WorldMap::generate(size, &weights, &ConstantNoise(0.5));

    assert_eq!(size, map.size());
}

#[test]
fn should_give_each_tile_the_terrain_of_the_noise_at_its_position() {
    let size = MapSize::new(4, 3).unwrap();
    let weights =
        TerrainGenerationWeights::new(&[(TerrainType::Sea, 0.5), (TerrainType::Plains, 0.5)])
            .unwrap();

    let map = WorldMap::generate(size, &weights, &LandOnOddColumns);

    (0..size.height()).for_each(|y| {
        (0..size.width()).for_each(|x| {
            let expected_terrain: TerrainType = if x % 2 == 1 {
                TerrainType::Plains
            } else {
                TerrainType::Sea
            };
            assert_eq!(
                Some(expected_terrain),
                map.tile_at(MapPosition { x, y }).map(MapTile::terrain_type),
                "({x}, {y})"
            );
        });
    });
}

struct LandOnOddColumns;

impl NoiseSource for LandOnOddColumns {
    fn noise_at(&self, position: MapPosition) -> NoiseValue {
        noise(if position.x % 2 == 1 { 0.9 } else { 0.1 })
    }
}

#[test]
fn should_have_no_tile_outside_the_map() {
    let size = MapSize::new(4, 3).unwrap();
    let weights = TerrainGenerationWeights::new(&[(TerrainType::Sea, 1.0)]).unwrap();

    let map = WorldMap::generate(size, &weights, &ConstantNoise(0.5));

    [(4, 0), (0, 3), (4, 3), (u32::MAX, u32::MAX)]
        .into_iter()
        .for_each(|(x, y)| assert!(map.tile_at(MapPosition { x, y }).is_none(), "({x}, {y})"));
}
