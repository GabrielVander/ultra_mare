use lib_terrain::TerrainType;

use crate::{
    entities::{tile_position::TilePosition, unit::Unit, unit_type::UnitType},
    movement::{direction::Direction, map_terrain::MapTerrain},
};

/// The colonist and the caravel the player sets out with.
#[derive(Debug, Clone, PartialEq)]
pub struct Expedition {
    /// `None` while aboard the caravel, or when there was no land to put it on.
    colonist: Option<Unit>,
    caravel: Option<Unit>,
}

impl Expedition {
    /// Puts the colonist on the first land tile and the caravel on the first sea tile.
    #[must_use]
    pub fn land(tiles: &[(TilePosition, TerrainType)]) -> Self {
        Self {
            colonist: Unit::land(UnitType::Colonist, tiles.iter().copied()),
            caravel: Unit::land(UnitType::Caravel, tiles.iter().copied()),
        }
    }

    /// Steps the colonist toward `direction`. Stepping onto the caravel boards it, and stepping
    /// while aboard lands it. A blocked step leaves it where it is.
    pub fn move_colonist(&mut self, direction: Direction, map: &impl MapTerrain) {
        self.colonist = match self.colonist.take() {
            Some(colonist) => self.walk_or_embark(colonist, direction, map),
            None => self.disembark(direction, map),
        };
    }

    /// Sails the caravel, and whoever is aboard, toward `direction`. A blocked step leaves it
    /// where it is.
    pub fn move_caravel(&mut self, direction: Direction, map: &impl MapTerrain) {
        if let Some(caravel) = self.caravel.as_mut() {
            let _ = caravel.move_toward(direction, map);
        }
    }

    /// The colonist standing on land, `None` while it is aboard the caravel.
    #[must_use]
    pub fn colonist_ashore(&self) -> Option<&Unit> {
        self.colonist.as_ref()
    }

    /// Takes the colonist off the map, for it to settle where it stands.
    pub fn settle_colonist(&mut self) -> Option<Unit> {
        self.colonist.take()
    }

    /// The units standing on a tile of their own, not carried by another.
    pub fn units_on_map(&self) -> impl Iterator<Item = &Unit> {
        self.colonist.iter().chain(self.caravel.iter())
    }

    pub fn colonist_position(&self) -> Option<TilePosition> {
        self.colonist.as_ref().map(Unit::position).or_else(|| {
            self.caravel
                .iter()
                .flat_map(Unit::units_aboard)
                .next()
                .map(Unit::position)
        })
    }

    pub fn caravel_position(&self) -> Option<TilePosition> {
        self.caravel.as_ref().map(Unit::position)
    }

    fn walk_or_embark(
        &mut self,
        colonist: Unit,
        direction: Direction,
        map: &impl MapTerrain,
    ) -> Option<Unit> {
        match self.caravel.as_mut() {
            Some(caravel) if lies_toward(direction, &colonist, caravel) => {
                colonist.embark(caravel).err().map(|(_, colonist)| colonist)
            }
            _ => Some(stepped(colonist, direction, map)),
        }
    }

    fn disembark(&mut self, direction: Direction, map: &impl MapTerrain) -> Option<Unit> {
        self.caravel
            .as_mut()
            .and_then(|caravel| caravel.disembark(direction, map).ok())
    }
}

fn lies_toward(direction: Direction, from: &Unit, other: &Unit) -> bool {
    from.position().neighbour(direction) == Some(other.position())
}

fn stepped(mut unit: Unit, direction: Direction, map: &impl MapTerrain) -> Unit {
    let _ = unit.move_toward(direction, map);
    unit
}
