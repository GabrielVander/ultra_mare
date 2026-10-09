use lib_world_generation::MapPosition;

/// Where tiles and the legend sit on screen, in Bevy world units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenLayout {
    pub tile_width: f32,
    pub tile_height: f32,
    pub legend_width: f32,
}

impl ScreenLayout {
    /// How many whole tiles fit in the window beside the legend, as `(columns, rows)`.
    pub fn viewport_size(&self, window_width: f32, window_height: f32) -> (u32, u32) {
        let map_area_width = window_width - self.legend_width;
        (
            (map_area_width / self.tile_width) as u32,
            (window_height / self.tile_height) as u32,
        )
    }

    /// The centre of a tile. Rows run downwards while Bevy's y axis points up.
    pub fn tile_translation(&self, position: MapPosition) -> (f32, f32) {
        let (left, top) = self.tile_top_left(position);
        (left + self.tile_width / 2.0, top - self.tile_height / 2.0)
    }

    /// Where the camera must look for the viewport's origin tile to sit in the
    /// window's top-left corner.
    pub fn camera_translation(
        &self,
        origin: MapPosition,
        window_width: f32,
        window_height: f32,
    ) -> (f32, f32) {
        let (left, top) = self.tile_top_left(origin);
        (left + window_width / 2.0, top - window_height / 2.0)
    }

    fn tile_top_left(&self, position: MapPosition) -> (f32, f32) {
        (
            position.x as f32 * self.tile_width,
            -(position.y as f32) * self.tile_height,
        )
    }
}
