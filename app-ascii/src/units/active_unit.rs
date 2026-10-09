use lib_colony::Colonies;
use lib_unit::{Direction, Expedition, MapTerrain, TilePosition};

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

    /// Has the colonist found a colony where it stands. It joins the colony, so the caravel
    /// takes the orders from then on.
    pub fn order_found_colony(
        &mut self,
        expedition: &mut Expedition,
        colonies: &mut Colonies,
        map: &impl MapTerrain,
    ) {
        if *self == ActiveUnit::Colonist && colonies.found(expedition, map).is_ok() {
            *self = ActiveUnit::Caravel;
        }
    }

    pub fn position(self, expedition: &Expedition) -> Option<TilePosition> {
        match self {
            ActiveUnit::Colonist => expedition.colonist_position(),
            ActiveUnit::Caravel => expedition.caravel_position(),
        }
    }
}
