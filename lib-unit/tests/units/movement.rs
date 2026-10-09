use lib_terrain::TerrainType;
use lib_unit::{Direction, MoveError, TilePosition};

use crate::support::{EndlessLand, TerrainGrid, caravel_at, colonist_at};

const OPEN_LAND: TerrainGrid = TerrainGrid(&["...", "...", "..."]);

#[test]
fn should_move_one_tile_in_the_given_direction() {
    [
        (Direction::North, TilePosition { x: 1, y: 0 }),
        (Direction::South, TilePosition { x: 1, y: 2 }),
        (Direction::East, TilePosition { x: 2, y: 1 }),
        (Direction::West, TilePosition { x: 0, y: 1 }),
    ]
    .into_iter()
    .for_each(|(direction, expected)| {
        let mut unit = colonist_at(TilePosition { x: 1, y: 1 });

        let moved = unit.move_toward(direction, &OPEN_LAND);

        assert_eq!(Ok(()), moved, "{direction:?}");
        assert_eq!(expected, unit.position(), "{direction:?}");
    });
}

#[test]
fn should_keep_moving_without_running_out_of_moves() {
    let long_coast = TerrainGrid(&[".........."]);
    let mut unit = colonist_at(TilePosition { x: 0, y: 0 });

    (0..9).for_each(|_| unit.move_toward(Direction::East, &long_coast).unwrap());

    assert_eq!(TilePosition { x: 9, y: 0 }, unit.position());
}

#[test]
fn should_not_walk_into_the_sea() {
    let shore = TerrainGrid(&[".~"]);
    let mut unit = colonist_at(TilePosition { x: 0, y: 0 });

    let moved = unit.move_toward(Direction::East, &shore);

    assert_eq!(Err(MoveError::Impassable(TerrainType::Sea)), moved);
    assert_eq!(TilePosition { x: 0, y: 0 }, unit.position());
}

#[test]
fn should_not_walk_off_the_far_edges_of_the_map() {
    let bottom_right = TilePosition { x: 2, y: 2 };
    [Direction::East, Direction::South]
        .into_iter()
        .for_each(|direction| {
            let mut unit = colonist_at(bottom_right);

            let moved = unit.move_toward(direction, &OPEN_LAND);

            assert_eq!(Err(MoveError::OffMap), moved, "{direction:?}");
            assert_eq!(bottom_right, unit.position(), "{direction:?}");
        });
}

#[test]
fn should_not_walk_off_the_near_edges_of_the_map() {
    let top_left = TilePosition { x: 0, y: 0 };
    [Direction::North, Direction::West]
        .into_iter()
        .for_each(|direction| {
            let mut unit = colonist_at(top_left);

            let moved = unit.move_toward(direction, &OPEN_LAND);

            assert_eq!(Err(MoveError::OffMap), moved, "{direction:?}");
            assert_eq!(top_left, unit.position(), "{direction:?}");
        });
}

#[test]
fn should_not_walk_past_the_last_representable_tile() {
    let last = TilePosition {
        x: u32::MAX,
        y: u32::MAX,
    };
    [Direction::East, Direction::South]
        .into_iter()
        .for_each(|direction| {
            let mut unit = colonist_at(last);

            let moved = unit.move_toward(direction, &EndlessLand);

            assert_eq!(Err(MoveError::OffMap), moved, "{direction:?}");
            assert_eq!(last, unit.position(), "{direction:?}");
        });
}

#[test]
fn should_sail_a_caravel_across_the_sea() {
    let open_sea = TerrainGrid(&["~~"]);
    let mut unit = caravel_at(TilePosition { x: 0, y: 0 });

    let moved = unit.move_toward(Direction::East, &open_sea);

    assert_eq!(Ok(()), moved);
    assert_eq!(TilePosition { x: 1, y: 0 }, unit.position());
}

#[test]
fn should_not_sail_a_caravel_onto_land() {
    let shore = TerrainGrid(&["~."]);
    let mut unit = caravel_at(TilePosition { x: 0, y: 0 });

    let moved = unit.move_toward(Direction::East, &shore);

    assert_eq!(Err(MoveError::Impassable(TerrainType::Plains)), moved);
    assert_eq!(TilePosition { x: 0, y: 0 }, unit.position());
}
