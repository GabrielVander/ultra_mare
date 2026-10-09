use app_ascii::{ActiveUnit, UnitMarker, present_units, unit_glyph};
use lib_unit::{Direction, Expedition, UnitType};
use lib_world_generation::MapPosition;

use crate::support::Shore;

#[test]
fn should_mark_each_unit_on_its_tile_and_highlight_the_active_one() {
    let expedition = Expedition::land(&Shore.tiles());

    let markers: Vec<UnitMarker> = present_units(&expedition, ActiveUnit::Caravel);

    assert_eq!(
        vec![
            UnitMarker {
                position: MapPosition { x: 0, y: 0 },
                glyph: unit_glyph(UnitType::Colonist),
                is_active: false,
            },
            UnitMarker {
                position: MapPosition { x: 2, y: 0 },
                glyph: unit_glyph(UnitType::Caravel),
                is_active: true,
            },
        ],
        markers
    );
}

#[test]
fn should_highlight_the_caravel_carrying_the_active_colonist() {
    let mut expedition = Expedition::land(&Shore.tiles());
    expedition.move_colonist(Direction::East, &Shore);
    expedition.move_colonist(Direction::East, &Shore);

    let markers: Vec<UnitMarker> = present_units(&expedition, ActiveUnit::Colonist);

    assert_eq!(
        vec![UnitMarker {
            position: MapPosition { x: 2, y: 0 },
            glyph: unit_glyph(UnitType::Caravel),
            is_active: true,
        }],
        markers
    );
}
