use crate::{
    boarding::embark_error::EmbarkError, entities::unit::Unit, movement::direction::Direction,
};

impl Unit {
    /// Boards `carrier`, which from then on holds this unit. A refused unit is handed back.
    ///
    /// # Errors
    ///
    /// Fails when `carrier` carries no units, when this unit cannot be carried, when the two
    /// are not adjacent or when `carrier` is full.
    pub fn embark(mut self, carrier: &mut Unit) -> Result<(), (EmbarkError, Unit)> {
        if let Some(reason) = self.embark_refusal(carrier) {
            return Err((reason, self));
        }
        self.relocate(carrier.position());
        carrier.aboard.push(self);
        Ok(())
    }

    /// The units this one carries, in the order they boarded.
    pub fn units_aboard(&self) -> impl Iterator<Item = &Unit> {
        self.aboard.iter()
    }

    fn embark_refusal(&self, carrier: &Unit) -> Option<EmbarkError> {
        if !carrier.unit_type().is_carrier() {
            Some(EmbarkError::NotACarrier)
        } else if !self.unit_type().can_be_carried() {
            Some(EmbarkError::CannotBeCarried)
        } else if !self.is_next_to(carrier) {
            Some(EmbarkError::NotAdjacent)
        } else if !carrier.has_space_left() {
            Some(EmbarkError::NoSpace)
        } else {
            None
        }
    }

    fn is_next_to(&self, other: &Unit) -> bool {
        Direction::ALL
            .into_iter()
            .any(|direction| self.position().neighbour(direction) == Some(other.position()))
    }

    fn has_space_left(&self) -> bool {
        self.aboard.len() < self.unit_type().capacity()
    }
}
