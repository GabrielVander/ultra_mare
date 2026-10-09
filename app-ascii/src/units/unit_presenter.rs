use lib_unit::{Expedition, TilePosition, Unit};
use lib_world_generation::MapPosition;

use crate::{
    map_view::terrain_glyph::Glyph,
    units::{active_unit::ActiveUnit, unit_glyph::unit_glyph, world_terrain::map_position},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitMarker {
    pub position: MapPosition,
    pub glyph: Glyph,
    pub is_active: bool,
}

#[must_use]
pub fn present_units(expedition: &Expedition, active: ActiveUnit) -> Vec<UnitMarker> {
    let active_position: Option<TilePosition> = active.position(expedition);
    expedition
        .units_on_map()
        .map(|unit| unit_marker(unit, active_position == Some(unit.position())))
        .collect()
}

fn unit_marker(unit: &Unit, is_active: bool) -> UnitMarker {
    UnitMarker {
        position: map_position(unit.position()),
        glyph: unit_glyph(unit.unit_type()),
        is_active,
    }
}
