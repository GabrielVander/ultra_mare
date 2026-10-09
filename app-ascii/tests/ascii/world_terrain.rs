use app_ascii::{WorldTerrain, map_position};
use lib_terrain::TerrainType;
use lib_unit::{MapTerrain, TilePosition};
use lib_world_generation::{
    MapPosition, MapSize, NoiseSource, NoiseValue, TerrainGenerationWeights, WorldMap,
};

#[test]
fn should_tell_units_the_terrain_of_each_world_tile() {
    let map: WorldMap = land_on_top_row(3, 2);
    let terrain = WorldTerrain::new(&map);

    assert_eq!(
        Some(TerrainType::Plains),
        terrain.terrain_at(TilePosition { x: 2, y: 0 })
    );
    assert_eq!(
        Some(TerrainType::Sea),
        terrain.terrain_at(TilePosition { x: 1, y: 1 })
    );
}

#[test]
fn should_tell_units_there_is_no_terrain_off_the_world_map() {
    let map: WorldMap = land_on_top_row(3, 2);
    let terrain = WorldTerrain::new(&map);

    assert_eq!(None, terrain.terrain_at(TilePosition { x: 3, y: 0 }));
    assert_eq!(None, terrain.terrain_at(TilePosition { x: 0, y: 2 }));
}

#[test]
fn should_list_every_tile_with_its_terrain_row_by_row() {
    let map: WorldMap = land_on_top_row(2, 2);

    let tiles: Vec<(TilePosition, TerrainType)> = WorldTerrain::new(&map).tiles().collect();

    assert_eq!(
        vec![
            (TilePosition { x: 0, y: 0 }, TerrainType::Plains),
            (TilePosition { x: 1, y: 0 }, TerrainType::Plains),
            (TilePosition { x: 0, y: 1 }, TerrainType::Sea),
            (TilePosition { x: 1, y: 1 }, TerrainType::Sea),
        ],
        tiles
    );
}

#[test]
fn should_place_a_unit_tile_at_the_same_map_position() {
    assert_eq!(
        MapPosition { x: 7, y: 4 },
        map_position(TilePosition { x: 7, y: 4 })
    );
}

fn land_on_top_row(width: u32, height: u32) -> WorldMap {
    let weights =
        TerrainGenerationWeights::new(&[(TerrainType::Sea, 0.5), (TerrainType::Plains, 0.5)])
            .unwrap();
    WorldMap::generate(
        MapSize::new(width, height).unwrap(),
        &weights,
        &LandOnTopRow,
    )
}

struct LandOnTopRow;

impl NoiseSource for LandOnTopRow {
    fn noise_at(&self, position: MapPosition) -> NoiseValue {
        NoiseValue::new(if position.y == 0 { 0.9 } else { 0.1 }).unwrap()
    }
}
