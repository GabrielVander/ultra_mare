use lib_colony::{Colonies, FoundColonyError};
use lib_unit::{Direction, Expedition, TilePosition, UnitType};

use crate::support::TerrainGrid;

const SHORE: TerrainGrid = TerrainGrid(&["..~"]);

#[test]
fn should_found_a_colony_where_the_colonist_stands() {
    let mut expedition = Expedition::land(&SHORE.tiles());
    let mut colonies = Colonies::default();

    let founded = colonies.found(&mut expedition);

    assert_eq!(Ok(TilePosition { x: 0, y: 0 }), founded);
    let sites: Vec<TilePosition> = colonies.iter().map(|colony| colony.position()).collect();
    assert_eq!(vec![TilePosition { x: 0, y: 0 }], sites);
    let on_map: Vec<UnitType> = expedition
        .units_on_map()
        .map(|unit| unit.unit_type())
        .collect();
    assert_eq!(vec![UnitType::Caravel], on_map);
}

#[test]
fn should_take_the_founding_colonist_into_the_colony() {
    let mut expedition = Expedition::land(&SHORE.tiles());
    let mut colonies = Colonies::default();

    colonies.found(&mut expedition).unwrap();

    let colonists: Vec<(UnitType, TilePosition)> = colonies
        .iter()
        .flat_map(|colony| colony.colonists())
        .map(|unit| (unit.unit_type(), unit.position()))
        .collect();
    assert_eq!(
        vec![(UnitType::Colonist, TilePosition { x: 0, y: 0 })],
        colonists
    );
}

#[test]
fn should_refuse_to_found_a_colony_while_the_colonist_is_aboard() {
    let shore = TerrainGrid(&[".~"]);
    let mut expedition = Expedition::land(&shore.tiles());
    expedition.move_colonist(Direction::East, &shore);
    let expedition_before = expedition.clone();
    let mut colonies = Colonies::default();

    let founded = colonies.found(&mut expedition);

    assert_eq!(Err(FoundColonyError::NoColonistAshore), founded);
    assert_eq!(0, colonies.iter().count());
    assert_eq!(expedition_before, expedition);
}

#[test]
fn should_refuse_to_found_a_second_colony_once_the_colonist_settled() {
    let mut expedition = Expedition::land(&SHORE.tiles());
    let mut colonies = Colonies::default();
    colonies.found(&mut expedition).unwrap();

    let founded = colonies.found(&mut expedition);

    assert_eq!(Err(FoundColonyError::NoColonistAshore), founded);
    assert_eq!(1, colonies.iter().count());
}
