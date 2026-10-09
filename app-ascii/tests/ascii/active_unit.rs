use app_ascii::ActiveUnit;
use lib_colony::Colonies;
use lib_unit::{Direction, Expedition, TilePosition};

use crate::support::Shore;

#[test]
fn should_switch_between_the_colonist_and_the_caravel() {
    assert_eq!(ActiveUnit::Caravel, ActiveUnit::Colonist.switched());
    assert_eq!(ActiveUnit::Colonist, ActiveUnit::Caravel.switched());
}

#[test]
fn should_send_a_move_order_to_the_active_unit_only() {
    let mut expedition = Expedition::land(&Shore.tiles());

    ActiveUnit::Colonist.order_move(&mut expedition, Direction::East, &Shore);
    ActiveUnit::Caravel.order_move(&mut expedition, Direction::East, &Shore);

    assert_eq!(
        Some(TilePosition { x: 1, y: 0 }),
        expedition.colonist_position()
    );
    assert_eq!(
        Some(TilePosition { x: 3, y: 0 }),
        expedition.caravel_position()
    );
}

#[test]
fn should_locate_the_active_unit() {
    let mut expedition = Expedition::land(&Shore.tiles());
    expedition.move_caravel(Direction::East, &Shore);

    assert_eq!(
        Some(TilePosition { x: 0, y: 0 }),
        ActiveUnit::Colonist.position(&expedition)
    );
    assert_eq!(
        Some(TilePosition { x: 3, y: 0 }),
        ActiveUnit::Caravel.position(&expedition)
    );
}

#[test]
fn should_found_a_colony_with_the_active_colonist_and_hand_over_to_the_caravel() {
    let mut expedition = Expedition::land(&Shore.tiles());
    let mut colonies = Colonies::default();
    let mut active = ActiveUnit::Colonist;

    active.order_found_colony(&mut expedition, &mut colonies);

    let sites: Vec<TilePosition> = colonies.iter().map(|colony| colony.position()).collect();
    assert_eq!(vec![TilePosition { x: 0, y: 0 }], sites);
    assert_eq!(ActiveUnit::Caravel, active);
}

#[test]
fn should_found_no_colony_when_the_caravel_is_active() {
    let mut expedition = Expedition::land(&Shore.tiles());
    let mut colonies = Colonies::default();
    let mut active = ActiveUnit::Caravel;

    active.order_found_colony(&mut expedition, &mut colonies);

    assert_eq!(0, colonies.iter().count());
    assert_eq!(
        Some(TilePosition { x: 0, y: 0 }),
        expedition.colonist_position()
    );
    assert_eq!(ActiveUnit::Caravel, active);
}

#[test]
fn should_keep_the_colonist_active_when_it_cannot_found_a_colony() {
    let mut expedition = Expedition::land(&Shore.tiles());
    expedition.move_colonist(Direction::East, &Shore);
    expedition.move_colonist(Direction::East, &Shore);
    let mut colonies = Colonies::default();
    let mut active = ActiveUnit::Colonist;

    active.order_found_colony(&mut expedition, &mut colonies);

    assert_eq!(0, colonies.iter().count());
    assert_eq!(ActiveUnit::Colonist, active);
}
