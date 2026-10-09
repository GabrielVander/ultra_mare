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
