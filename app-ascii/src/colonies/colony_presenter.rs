use lib_colony::{Colonies, Colony};
use lib_world_generation::MapPosition;

use crate::{
    colonies::colony_glyph::colony_glyph, map_view::terrain_glyph::Glyph,
    units::world_terrain::map_position,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColonyMarker {
    pub position: MapPosition,
    pub glyph: Glyph,
}

pub fn present_colonies(colonies: &Colonies) -> Vec<ColonyMarker> {
    colonies.iter().map(colony_marker).collect()
}

fn colony_marker(colony: &Colony) -> ColonyMarker {
    ColonyMarker {
        position: map_position(colony.position()),
        glyph: colony_glyph(),
    }
}
