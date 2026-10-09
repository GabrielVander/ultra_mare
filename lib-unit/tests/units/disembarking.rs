use lib_terrain::TerrainType;
use lib_unit::{Direction, DisembarkError, MoveError, TilePosition, UnitType};

use crate::support::{TerrainGrid, caravel_at, colonist_at};

const SHORE: TerrainGrid = TerrainGrid(&["..~"]);

#[test]
fn should_land_the_colonist_aboard_on_the_neighbouring_tile() {
    let mut caravel = caravel_at(TilePosition { x: 2, y: 0 });
    colonist_at(TilePosition { x: 1, y: 0 })
        .embark(&mut caravel)
        .unwrap();

    let colonist = caravel.disembark(Direction::West, &SHORE).unwrap();

    assert_eq!(UnitType::Colonist, colonist.unit_type());
    assert_eq!(TilePosition { x: 1, y: 0 }, colonist.position());
    assert_eq!(0, caravel.units_aboard().count());
}

#[test]
fn should_land_every_colonist_aboard_one_by_one() {
    let lagoon = TerrainGrid(&[".~.", "..."]);
    let mut caravel = caravel_at(TilePosition { x: 1, y: 0 });
    colonist_at(TilePosition { x: 0, y: 0 })
        .embark(&mut caravel)
        .unwrap();
    colonist_at(TilePosition { x: 2, y: 0 })
        .embark(&mut caravel)
        .unwrap();

    let first = caravel.disembark(Direction::South, &lagoon).unwrap();
    let second = caravel.disembark(Direction::West, &lagoon).unwrap();

    assert_eq!(TilePosition { x: 1, y: 1 }, first.position());
    assert_eq!(TilePosition { x: 0, y: 0 }, second.position());
    assert_eq!(0, caravel.units_aboard().count());
}

#[test]
fn should_keep_the_colonist_aboard_rather_than_land_it_in_the_sea() {
    let mut caravel = caravel_at(TilePosition { x: 2, y: 0 });
    colonist_at(TilePosition { x: 1, y: 0 })
        .embark(&mut caravel)
        .unwrap();
    let open_sea = TerrainGrid(&["~~~~"]);

    let landed = caravel.disembark(Direction::East, &open_sea);

    assert_eq!(
        Err(DisembarkError::Blocked(MoveError::Impassable(
            TerrainType::Sea
        ))),
        landed
    );
    assert_eq!(1, caravel.units_aboard().count());
}

#[test]
fn should_keep_the_colonist_aboard_rather_than_land_it_off_the_map() {
    let mut caravel = caravel_at(TilePosition { x: 2, y: 0 });
    colonist_at(TilePosition { x: 1, y: 0 })
        .embark(&mut caravel)
        .unwrap();

    let landed = caravel.disembark(Direction::North, &SHORE);

    assert_eq!(Err(DisembarkError::Blocked(MoveError::OffMap)), landed);
    assert_eq!(1, caravel.units_aboard().count());
}

#[test]
fn should_land_nobody_when_nobody_is_aboard() {
    let mut caravel = caravel_at(TilePosition { x: 2, y: 0 });

    let landed = caravel.disembark(Direction::West, &SHORE);

    assert_eq!(Err(DisembarkError::NobodyAboard), landed);
}
