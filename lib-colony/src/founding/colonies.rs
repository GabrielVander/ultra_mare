use lib_unit::{Expedition, TilePosition, Unit};

use crate::{entities::colony::Colony, founding::found_colony_error::FoundColonyError};

/// Every colony founded so far.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Colonies {
    colonies: Vec<Colony>,
}

impl Colonies {
    /// Founds a colony on the tile of the expedition's colonist, who joins it and leaves the map.
    pub fn found(&mut self, expedition: &mut Expedition) -> Result<TilePosition, FoundColonyError> {
        let site = expedition
            .colonist_ashore()
            .map(Unit::position)
            .ok_or(FoundColonyError::NoColonistAshore)?;
        if self.is_crowded(site) {
            return Err(FoundColonyError::TooCloseToColony);
        }
        self.colonies
            .extend(expedition.settle_colonist().map(Colony::founded_by));
        Ok(site)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Colony> {
        self.colonies.iter()
    }

    /// Whether a colony stands on `site` or on any of the eight tiles around it.
    fn is_crowded(&self, site: TilePosition) -> bool {
        self.colonies.iter().any(|colony| {
            let position = colony.position();
            position.x.abs_diff(site.x) <= 1 && position.y.abs_diff(site.y) <= 1
        })
    }
}
