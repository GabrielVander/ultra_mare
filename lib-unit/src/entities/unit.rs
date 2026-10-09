use lib_terrain::TerrainType;

use crate::{
    entities::{tile_position::TilePosition, unit_type::UnitType},
    movement::{direction::Direction, map_terrain::MapTerrain, move_error::MoveError},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    unit_type: UnitType,
    position: TilePosition,
    pub(crate) aboard: Vec<Unit>,
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
            aboard: Vec::new(),
        })
    }

    /// Steps onto the neighbouring tile, as often as asked: units have no movement limit yet.
    pub fn move_toward(
        &mut self,
        direction: Direction,
        map: &impl MapTerrain,
    ) -> Result<(), MoveError> {
        let destination = self
            .position
            .neighbour(direction)
            .ok_or(MoveError::OffMap)?;
        let terrain = map.terrain_at(destination).ok_or(MoveError::OffMap)?;
        if !self.unit_type.can_enter(terrain) {
            return Err(MoveError::Impassable(terrain));
        }
        self.relocate(destination);
        Ok(())
    }

    pub fn unit_type(&self) -> UnitType {
        self.unit_type
    }

    pub fn position(&self) -> TilePosition {
        self.position
    }

    pub(crate) fn relocate(&mut self, position: TilePosition) {
        self.position = position;
        self.aboard
            .iter_mut()
            .for_each(|unit| unit.relocate(position));
    }
}
