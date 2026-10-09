use lib_colony::{Colonies, FoundColonyError};
use lib_unit::{Direction, Expedition, TilePosition, UnitType};

use crate::support::{TerrainGrid, colonist_at};

const SHORE: TerrainGrid = TerrainGrid(&[".~"]);

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

#[test]
fn should_refuse_to_found_a_colony_on_or_next_to_another() {
    (0..3)
        .flat_map(|y| (0..3).map(move |x| TilePosition { x, y }))
        .for_each(|site| {
            let mut colonies = Colonies::default();
            colonies
                .found(&mut colonist_at(TilePosition { x: 1, y: 1 }))
                .unwrap();
            let mut expedition = colonist_at(site);

            let founded = colonies.found(&mut expedition);

            assert_eq!(Err(FoundColonyError::TooCloseToColony), founded, "{site:?}");
            assert_eq!(1, colonies.iter().count(), "{site:?}");
            assert_eq!(1, expedition.units_on_map().count(), "{site:?}");
        });
}

#[test]
fn should_found_a_colony_two_tiles_away_from_another() {
    [
        TilePosition { x: 4, y: 2 },
        TilePosition { x: 0, y: 2 },
        TilePosition { x: 2, y: 4 },
        TilePosition { x: 2, y: 0 },
        TilePosition { x: 4, y: 4 },
    ]
    .into_iter()
    .for_each(|site| {
        let mut colonies = Colonies::default();
        colonies
            .found(&mut colonist_at(TilePosition { x: 2, y: 2 }))
            .unwrap();

        let founded = colonies.found(&mut colonist_at(site));

        assert_eq!(Ok(site), founded, "{site:?}");
        assert_eq!(2, colonies.iter().count(), "{site:?}");
    });
}
