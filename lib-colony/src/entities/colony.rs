use lib_unit::{TilePosition, Unit};

/// A settlement on the map, worked by the colonists who joined it.
#[derive(Debug, Clone, PartialEq)]
pub struct Colony {
    position: TilePosition,
    colonists: Vec<Unit>,
}

impl Colony {
    pub(crate) fn founded_by(founder: Unit) -> Self {
        Self {
            position: founder.position(),
            colonists: vec![founder],
        }
    }

    pub fn position(&self) -> TilePosition {
        self.position
    }

    /// The colonists working the colony, in the order they joined.
    pub fn colonists(&self) -> impl Iterator<Item = &Unit> {
        self.colonists.iter()
    }
}
