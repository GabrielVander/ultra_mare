use app_ascii::{ColonyMarker, colony_glyph, present_colonies};
use lib_colony::Colonies;
use lib_unit::{Direction, Expedition};
use lib_world_generation::MapPosition;

use crate::support::Shore;

#[test]
fn should_mark_each_colony_on_its_tile() {
    let mut expedition = Expedition::land(&Shore.tiles());
    expedition.move_colonist(Direction::East, &Shore);
    let mut colonies = Colonies::default();
    colonies.found(&mut expedition, &Shore).unwrap();

    let markers = present_colonies(&colonies);

    assert_eq!(
        vec![ColonyMarker {
            position: MapPosition { x: 1, y: 0 },
            glyph: colony_glyph(),
        }],
        markers
    );
}
