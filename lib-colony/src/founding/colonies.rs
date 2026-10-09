use lib_unit::{Expedition, MapTerrain, TilePosition, Unit};

use crate::{entities::colony::Colony, founding::found_colony_error::FoundColonyError};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Colonies {
    colonies: Vec<Colony>,
}

impl Colonies {
    /// Founds a colony on the tile of the expedition's colonist, who joins it and leaves the map.
    /// For now colonies stand only on the coast.
    ///
    /// # Errors
    ///
    /// Fails when no colonist is ashore, when the site borders another colony or when it
    /// lies inland.
    pub fn found(
        &mut self,
        expedition: &mut Expedition,
        map: &impl MapTerrain,
    ) -> Result<TilePosition, FoundColonyError> {
        let site = expedition
            .colonist_ashore()
            .map(Unit::position)
            .ok_or(FoundColonyError::NoColonistAshore)?;
        if self.is_crowded(site) {
            return Err(FoundColonyError::TooCloseToColony);
        }
        if !is_coastal(site, map) {
            return Err(FoundColonyError::Landlocked);
        }
        self.colonies
            .extend(expedition.settle_colonist().map(Colony::founded_by));
        Ok(site)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Colony> {
        self.colonies.iter()
    }

    fn is_crowded(&self, site: TilePosition) -> bool {
        self.colonies.iter().any(|colony| {
            let position = colony.position();
            position.x.abs_diff(site.x) <= 1 && position.y.abs_diff(site.y) <= 1
        })
    }
}

fn is_coastal(site: TilePosition, map: &impl MapTerrain) -> bool {
    surroundings(site)
        .filter_map(|position| map.terrain_at(position))
        .any(|terrain| terrain.is_water())
}

fn surroundings(site: TilePosition) -> impl Iterator<Item = TilePosition> {
    (-1..=1).flat_map(move |dy| {
        (-1..=1).filter_map(move |dx| {
            Some(TilePosition {
                x: site.x.checked_add_signed(dx)?,
                y: site.y.checked_add_signed(dy)?,
            })
        })
    })
}
