use lib_unit::{Direction, Expedition, MapTerrain, TilePosition};

/// The unit the player's orders go to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveUnit {
    Colonist,
    Caravel,
}

impl ActiveUnit {
    pub fn switched(self) -> Self {
        match self {
            ActiveUnit::Colonist => ActiveUnit::Caravel,
            ActiveUnit::Caravel => ActiveUnit::Colonist,
        }
    }

    pub fn order_move(
        self,
        expedition: &mut Expedition,
        direction: Direction,
        map: &impl MapTerrain,
    ) {
        match self {
            ActiveUnit::Colonist => expedition.move_colonist(direction, map),
            ActiveUnit::Caravel => expedition.move_caravel(direction, map),
        }
    }

    pub fn position(self, expedition: &Expedition) -> Option<TilePosition> {
        match self {
            ActiveUnit::Colonist => expedition.colonist_position(),
            ActiveUnit::Caravel => expedition.caravel_position(),
        }
    }
}
