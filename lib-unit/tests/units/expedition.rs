use lib_unit::{Direction, Expedition, TilePosition, UnitType};

use crate::support::TerrainGrid;

#[test]
fn should_embark_the_colonist_when_it_moves_onto_the_caravel() {
    let shore = TerrainGrid(&[".~"]);
    let mut expedition = Expedition::land(&shore.tiles());

    expedition.move_colonist(Direction::East, &shore);

    let on_map: Vec<(UnitType, usize)> = expedition
        .units_on_map()
        .map(|unit| (unit.unit_type(), unit.units_aboard().count()))
        .collect();
    assert_eq!(vec![(UnitType::Caravel, 1)], on_map);
    assert_eq!(
        Some(TilePosition { x: 1, y: 0 }),
        expedition.colonist_position()
    );
}

#[test]
fn should_walk_the_colonist_when_the_caravel_is_elsewhere() {
    let shore = TerrainGrid(&["..~"]);
    let mut expedition = Expedition::land(&shore.tiles());

    expedition.move_colonist(Direction::East, &shore);

    let on_map: Vec<(UnitType, TilePosition)> = expedition
        .units_on_map()
        .map(|unit| (unit.unit_type(), unit.position()))
        .collect();
    assert_eq!(
        vec![
            (UnitType::Colonist, TilePosition { x: 1, y: 0 }),
            (UnitType::Caravel, TilePosition { x: 2, y: 0 }),
        ],
        on_map
    );
}

#[test]
fn should_land_the_colonist_aboard_when_it_moves() {
    let shore = TerrainGrid(&[".~", ".."]);
    let mut expedition = Expedition::land(&shore.tiles());
    expedition.move_colonist(Direction::East, &shore);

    expedition.move_colonist(Direction::South, &shore);

    let on_map: Vec<(UnitType, TilePosition, usize)> = expedition
        .units_on_map()
        .map(|unit| {
            (
                unit.unit_type(),
                unit.position(),
                unit.units_aboard().count(),
            )
        })
        .collect();
    assert_eq!(
        vec![
            (UnitType::Colonist, TilePosition { x: 1, y: 1 }, 0),
            (UnitType::Caravel, TilePosition { x: 1, y: 0 }, 0),
        ],
        on_map
    );
}

#[test]
fn should_sail_the_caravel_with_the_colonist_aboard() {
    let shore = TerrainGrid(&[".~~"]);
    let mut expedition = Expedition::land(&shore.tiles());
    expedition.move_colonist(Direction::East, &shore);

    expedition.move_caravel(Direction::East, &shore);

    assert_eq!(
        Some(TilePosition { x: 2, y: 0 }),
        expedition.caravel_position()
    );
    assert_eq!(
        Some(TilePosition { x: 2, y: 0 }),
        expedition.colonist_position()
    );
}

#[test]
fn should_set_out_without_a_caravel_when_there_is_no_sea() {
    let island = TerrainGrid(&[".."]);
    let mut expedition = Expedition::land(&island.tiles());

    expedition.move_caravel(Direction::East, &island);
    expedition.move_colonist(Direction::East, &island);

    let on_map: Vec<(UnitType, TilePosition)> = expedition
        .units_on_map()
        .map(|unit| (unit.unit_type(), unit.position()))
        .collect();
    assert_eq!(
        vec![(UnitType::Colonist, TilePosition { x: 1, y: 0 })],
        on_map
    );
    assert_eq!(None, expedition.caravel_position());
}

#[test]
fn should_settle_the_colonist_ashore_off_the_map() {
    let shore = TerrainGrid(&["..~"]);
    let mut expedition = Expedition::land(&shore.tiles());

    let settler = expedition.settle_colonist().unwrap();

    assert_eq!(UnitType::Colonist, settler.unit_type());
    assert_eq!(TilePosition { x: 0, y: 0 }, settler.position());
    let on_map: Vec<UnitType> = expedition
        .units_on_map()
        .map(|unit| unit.unit_type())
        .collect();
    assert_eq!(vec![UnitType::Caravel], on_map);
}

#[test]
fn should_not_settle_the_colonist_while_it_is_aboard() {
    let shore = TerrainGrid(&[".~"]);
    let mut expedition = Expedition::land(&shore.tiles());
    expedition.move_colonist(Direction::East, &shore);

    let settler = expedition.settle_colonist();

    assert_eq!(None, settler);
    assert_eq!(
        Some(TilePosition { x: 1, y: 0 }),
        expedition.colonist_position()
    );
}

#[test]
fn should_tell_the_colonist_ashore_only_while_it_is_not_aboard() {
    let shore = TerrainGrid(&[".~"]);
    let mut expedition = Expedition::land(&shore.tiles());
    let ashore = expedition.colonist_ashore().map(|unit| unit.position());

    expedition.move_colonist(Direction::East, &shore);

    assert_eq!(Some(TilePosition { x: 0, y: 0 }), ashore);
    assert_eq!(None, expedition.colonist_ashore());
}

#[test]
fn should_leave_no_colonist_to_order_once_it_settled() {
    let shore = TerrainGrid(&["..~"]);
    let mut expedition = Expedition::land(&shore.tiles());
    expedition.settle_colonist();

    expedition.move_colonist(Direction::East, &shore);

    assert_eq!(None, expedition.colonist_position());
    assert_eq!(None, expedition.settle_colonist());
}
