use lib_unit::{Direction, EmbarkError, TilePosition, Unit, UnitType};

use crate::support::{TerrainGrid, caravel_at, colonist_at};

#[test]
fn should_take_aboard_a_colonist_embarking_from_the_next_tile() {
    let mut caravel: Unit = caravel_at(TilePosition { x: 1, y: 0 });
    let colonist: Unit = colonist_at(TilePosition { x: 0, y: 0 });

    let embarked: Result<(), (EmbarkError, Unit)> = colonist.embark(&mut caravel);

    assert!(embarked.is_ok());
    let aboard: Vec<UnitType> = caravel.units_aboard().map(Unit::unit_type).collect();
    assert_eq!(vec![UnitType::Colonist], aboard);
}

#[test]
fn should_refuse_a_colonist_that_is_not_next_to_the_caravel() {
    [
        TilePosition { x: 3, y: 1 },
        TilePosition { x: 2, y: 2 },
        TilePosition { x: 1, y: 1 },
    ]
    .into_iter()
    .for_each(|position| {
        let mut caravel: Unit = caravel_at(TilePosition { x: 1, y: 1 });
        let colonist: Unit = colonist_at(position);

        let embarked: Result<(), EmbarkError> =
            colonist.embark(&mut caravel).map_err(|(reason, _)| reason);

        assert_eq!(Err(EmbarkError::NotAdjacent), embarked, "{position:?}");
        assert_eq!(0, caravel.units_aboard().count(), "{position:?}");
    });
}

#[test]
fn should_take_aboard_a_colonist_from_any_neighbouring_tile() {
    [
        TilePosition { x: 1, y: 0 },
        TilePosition { x: 1, y: 2 },
        TilePosition { x: 0, y: 1 },
        TilePosition { x: 2, y: 1 },
    ]
    .into_iter()
    .for_each(|position| {
        let mut caravel: Unit = caravel_at(TilePosition { x: 1, y: 1 });
        let colonist: Unit = colonist_at(position);

        let embarked: Result<(), (EmbarkError, Unit)> = colonist.embark(&mut caravel);

        assert!(embarked.is_ok(), "{position:?}");
        assert_eq!(1, caravel.units_aboard().count(), "{position:?}");
    });
}

#[test]
fn should_carry_the_colonists_aboard_wherever_the_caravel_sails() {
    let coast = TerrainGrid(&["~~~", ".~~"]);
    let mut caravel: Unit = caravel_at(TilePosition { x: 1, y: 1 });
    colonist_at(TilePosition { x: 0, y: 1 })
        .embark(&mut caravel)
        .unwrap();

    caravel.move_toward(Direction::North, &coast).unwrap();
    caravel.move_toward(Direction::East, &coast).unwrap();

    let positions: Vec<TilePosition> = caravel.units_aboard().map(Unit::position).collect();
    assert_eq!(vec![TilePosition { x: 2, y: 0 }], positions);
}

#[test]
fn should_put_an_embarked_colonist_on_the_caravel_tile() {
    let mut caravel: Unit = caravel_at(TilePosition { x: 1, y: 0 });

    colonist_at(TilePosition { x: 0, y: 0 })
        .embark(&mut caravel)
        .unwrap();

    let positions: Vec<TilePosition> = caravel.units_aboard().map(Unit::position).collect();
    assert_eq!(vec![TilePosition { x: 1, y: 0 }], positions);
}

#[test]
fn should_refuse_a_third_colonist_once_two_are_aboard() {
    let mut caravel: Unit = caravel_at(TilePosition { x: 1, y: 1 });
    colonist_at(TilePosition { x: 0, y: 1 })
        .embark(&mut caravel)
        .unwrap();
    colonist_at(TilePosition { x: 1, y: 0 })
        .embark(&mut caravel)
        .unwrap();

    let embarked: Result<(), EmbarkError> = colonist_at(TilePosition { x: 2, y: 1 })
        .embark(&mut caravel)
        .map_err(|(reason, _)| reason);

    assert_eq!(Err(EmbarkError::NoSpace), embarked);
    assert_eq!(2, caravel.units_aboard().count());
}

#[test]
fn should_refuse_to_board_a_colonist() {
    let mut colonist: Unit = colonist_at(TilePosition { x: 1, y: 0 });

    let embarked: Result<(), EmbarkError> = colonist_at(TilePosition { x: 0, y: 0 })
        .embark(&mut colonist)
        .map_err(|(reason, _)| reason);

    assert_eq!(Err(EmbarkError::NotACarrier), embarked);
    assert_eq!(0, colonist.units_aboard().count());
}

#[test]
fn should_refuse_to_carry_a_caravel() {
    let mut caravel: Unit = caravel_at(TilePosition { x: 1, y: 0 });

    let embarked: Result<(), EmbarkError> = caravel_at(TilePosition { x: 0, y: 0 })
        .embark(&mut caravel)
        .map_err(|(reason, _)| reason);

    assert_eq!(Err(EmbarkError::CannotBeCarried), embarked);
    assert_eq!(0, caravel.units_aboard().count());
}

#[test]
fn should_hand_back_a_refused_unit_and_leave_the_caravel_as_it_was() {
    let mut caravel: Unit = caravel_at(TilePosition { x: 2, y: 0 });
    let caravel_before: Unit = caravel.clone();
    let colonist: Unit = colonist_at(TilePosition { x: 0, y: 0 });

    let embarked: Result<(), (EmbarkError, Unit)> = colonist.clone().embark(&mut caravel);

    assert_eq!(Err((EmbarkError::NotAdjacent, colonist)), embarked);
    assert_eq!(caravel_before, caravel);
}

#[test]
fn should_place_a_caravel_with_nobody_aboard() {
    let caravel: Unit = caravel_at(TilePosition { x: 0, y: 0 });

    assert_eq!(0, caravel.units_aboard().count());
}
