use lib_world_generation::{MapPosition, MapSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanDirection {
    Left,
    Right,
    Up,
    Down,
}

/// The part of the map on screen: `columns` × `rows` tiles from `origin` onwards.
/// It never shows more beyond the map's edges than it must.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    map_size: MapSize,
    columns: u32,
    rows: u32,
    origin: MapPosition,
}

impl Viewport {
    pub fn new(map_size: MapSize, columns: u32, rows: u32) -> Self {
        Self {
            map_size,
            columns,
            rows,
            origin: MapPosition { x: 0, y: 0 },
        }
    }

    pub fn origin(&self) -> MapPosition {
        self.origin
    }

    pub fn pan(&mut self, direction: PanDirection) {
        let MapPosition { x, y } = self.origin;
        let farthest = self.farthest_origin();
        self.origin = match direction {
            PanDirection::Left => MapPosition {
                x: x.saturating_sub(1),
                y,
            },
            PanDirection::Right => MapPosition {
                x: (x + 1).min(farthest.x),
                y,
            },
            PanDirection::Up => MapPosition {
                x,
                y: y.saturating_sub(1),
            },
            PanDirection::Down => MapPosition {
                x,
                y: (y + 1).min(farthest.y),
            },
        };
    }

    fn farthest_origin(&self) -> MapPosition {
        MapPosition {
            x: self.map_size.width().saturating_sub(self.columns),
            y: self.map_size.height().saturating_sub(self.rows),
        }
    }
}
