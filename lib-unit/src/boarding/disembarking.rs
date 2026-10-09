use crate::{
    boarding::disembark_error::DisembarkError,
    entities::unit::Unit,
    movement::{direction::Direction, map_terrain::MapTerrain},
};

impl Unit {
    /// Lands one of the units aboard on the neighbouring tile in `direction`.
    ///
    /// # Errors
    ///
    /// Fails when nobody is aboard or when the unit cannot step onto that tile.
    pub fn disembark(
        &mut self,
        direction: Direction,
        map: &impl MapTerrain,
    ) -> Result<Unit, DisembarkError> {
        let mut unit = self
            .aboard
            .first()
            .cloned()
            .ok_or(DisembarkError::NobodyAboard)?;
        unit.move_toward(direction, map)
            .map_err(DisembarkError::Blocked)?;
        self.aboard.remove(0);
        Ok(unit)
    }
}
