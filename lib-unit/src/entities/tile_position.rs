use crate::movement::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TilePosition {
    pub x: u32,
    pub y: u32,
}

impl TilePosition {
    pub(crate) fn neighbour(self, direction: Direction) -> Option<TilePosition> {
        let TilePosition { x, y } = self;
        Some(match direction {
            Direction::North => TilePosition {
                x,
                y: y.checked_sub(1)?,
            },
            Direction::South => TilePosition {
                x,
                y: y.checked_add(1)?,
            },
            Direction::East => TilePosition {
                x: x.checked_add(1)?,
                y,
            },
            Direction::West => TilePosition {
                x: x.checked_sub(1)?,
                y,
            },
        })
    }
}
