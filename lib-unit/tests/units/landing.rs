use lib_terrain::TerrainType;
use lib_unit::{TilePosition, Unit, UnitType};

#[test]
fn should_land_on_the_first_tile_when_it_is_land() {
    let tiles: [(TilePosition, TerrainType); 2] = [
        (TilePosition { x: 0, y: 0 }, TerrainType::Plains),
        (TilePosition { x: 1, y: 0 }, TerrainType::Forest),
    ];

    let unit = Unit::land(UnitType::Colonist, tiles).unwrap();

    assert_eq!(UnitType::Colonist, unit.unit_type());
    assert_eq!(TilePosition { x: 0, y: 0 }, unit.position());
}

#[test]
fn should_skip_water_and_land_on_the_first_land_tile() {
    let tiles: [(TilePosition, TerrainType); 4] = [
        (TilePosition { x: 0, y: 0 }, TerrainType::Sea),
        (TilePosition { x: 1, y: 0 }, TerrainType::Sea),
        (TilePosition { x: 2, y: 0 }, TerrainType::Hills),
        (TilePosition { x: 3, y: 0 }, TerrainType::Plains),
    ];

    let unit = Unit::land(UnitType::Colonist, tiles).unwrap();

    assert_eq!(TilePosition { x: 2, y: 0 }, unit.position());
}

#[test]
fn should_not_land_when_there_is_no_land() {
    let all_water: [(TilePosition, TerrainType); 2] = [
        (TilePosition { x: 0, y: 0 }, TerrainType::Sea),
        (TilePosition { x: 1, y: 0 }, TerrainType::Sea),
    ];

    assert_eq!(None, Unit::land(UnitType::Colonist, all_water));
    assert_eq!(None, Unit::land(UnitType::Colonist, []));
}

#[test]
fn should_place_a_caravel_on_the_first_water_tile() {
    let tiles: [(TilePosition, TerrainType); 4] = [
        (TilePosition { x: 0, y: 0 }, TerrainType::Plains),
        (TilePosition { x: 1, y: 0 }, TerrainType::Forest),
        (TilePosition { x: 2, y: 0 }, TerrainType::Sea),
        (TilePosition { x: 3, y: 0 }, TerrainType::Sea),
    ];

    let unit = Unit::land(UnitType::Caravel, tiles).unwrap();

    assert_eq!(UnitType::Caravel, unit.unit_type());
    assert_eq!(TilePosition { x: 2, y: 0 }, unit.position());
}

#[test]
fn should_not_place_a_caravel_when_there_is_no_water() {
    let all_land: [(TilePosition, TerrainType); 2] = [
        (TilePosition { x: 0, y: 0 }, TerrainType::Plains),
        (TilePosition { x: 1, y: 0 }, TerrainType::Mountain),
    ];

    assert_eq!(None, Unit::land(UnitType::Caravel, all_land));
}
