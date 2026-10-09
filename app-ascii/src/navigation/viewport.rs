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
        let panned = match direction {
            PanDirection::Left => MapPosition {
                x: x.saturating_sub(1),
                y,
            },
            PanDirection::Right => MapPosition {
                x: x.saturating_add(1),
                y,
            },
            PanDirection::Up => MapPosition {
                x,
                y: y.saturating_sub(1),
            },
            PanDirection::Down => MapPosition {
                x,
                y: y.saturating_add(1),
            },
        };
        self.origin = self.clamped(panned);
    }

    /// Pans as little as possible to show `position`, never beyond the map's edges.
    pub fn keep_in_view(&mut self, position: MapPosition) {
        let showing = MapPosition {
            x: Self::nearest_showing(self.origin.x, position.x, self.columns),
            y: Self::nearest_showing(self.origin.y, position.y, self.rows),
        };
        self.origin = self.clamped(showing);
    }

    pub fn resize(&mut self, columns: u32, rows: u32) {
        self.columns = columns;
        self.rows = rows;
        self.origin = self.clamped(self.origin);
    }

    fn clamped(&self, origin: MapPosition) -> MapPosition {
        let farthest = self.farthest_origin();
        MapPosition {
            x: origin.x.min(farthest.x),
            y: origin.y.min(farthest.y),
        }
    }

    fn farthest_origin(&self) -> MapPosition {
        MapPosition {
            x: self.map_size.width().saturating_sub(self.columns),
            y: self.map_size.height().saturating_sub(self.rows),
        }
    }

    /// The origin coordinate closest to `origin` whose `span` still shows `coordinate`.
    fn nearest_showing(origin: u32, coordinate: u32, span: u32) -> u32 {
        let first_showing = coordinate.saturating_sub(span.saturating_sub(1));
        origin.clamp(first_showing, coordinate)
    }
}
