use lib_world_generation::{MapPosition, MapSize, WorldMap};

use crate::map_view::terrain_glyph::{Glyph, glyph_for};

/// The world map as rows of glyphs, top row first.
#[derive(Debug, PartialEq)]
pub struct MapViewModel {
    pub rows: Vec<Vec<Glyph>>,
}

#[must_use]
pub fn present_map(map: &WorldMap) -> MapViewModel {
    let size: MapSize = map.size();
    let rows: Vec<Vec<Glyph>> = (0..size.height())
        .map(|y| {
            (0..size.width())
                .filter_map(|x| map.tile_at(MapPosition { x, y }))
                .map(|tile| glyph_for(tile.terrain_type()))
                .collect()
        })
        .collect();

    MapViewModel { rows }
}
