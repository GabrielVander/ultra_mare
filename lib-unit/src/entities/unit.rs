use lib_terrain::TerrainType;

use crate::entities::{tile_position::TilePosition, unit_type::UnitType};

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    unit_type: UnitType,
    position: TilePosition,
}

impl Unit {
    /// Places the unit on the first of `tiles` it can enter, or nowhere if it can enter none.
    pub fn land(
        unit_type: UnitType,
        tiles: impl IntoIterator<Item = (TilePosition, TerrainType)>,
    ) -> Option<Self> {
        let (position, _) = tiles
            .into_iter()
            .find(|(_, terrain)| unit_type.can_enter(*terrain))?;
        Some(Self {
            unit_type,
            position,
        })
    }

    pub fn unit_type(&self) -> UnitType {
        self.unit_type
    }

    pub fn position(&self) -> TilePosition {
        self.position
    }
}
