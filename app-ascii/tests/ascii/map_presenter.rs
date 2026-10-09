use app_ascii::{Glyph, MapViewModel, glyph_for, present_map};
use lib_terrain::TerrainType;
use lib_world_generation::{
    MapPosition, MapSize, NoiseSource, NoiseValue, TerrainGenerationWeights, WorldMap,
};

#[test]
fn should_show_each_tile_as_the_glyph_of_its_terrain_at_its_position() {
    let weights =
        TerrainGenerationWeights::new(&[(TerrainType::Sea, 0.5), (TerrainType::Plains, 0.5)])
            .unwrap();
    let map = WorldMap::generate(MapSize::new(3, 2).unwrap(), &weights, &LandOnTopRow);

    let view_model: MapViewModel = present_map(&map);

    let land: Glyph = glyph_for(TerrainType::Plains);
    let sea: Glyph = glyph_for(TerrainType::Sea);
    assert_eq!(
        vec![vec![land, land, land], vec![sea, sea, sea]],
        view_model.rows
    );
}

struct LandOnTopRow;

impl NoiseSource for LandOnTopRow {
    fn noise_at(&self, position: MapPosition) -> NoiseValue {
        NoiseValue::new(if position.y == 0 { 0.9 } else { 0.1 }).unwrap()
    }
}
