use lib_terrain::TerrainType;
use lib_unit::{MapTerrain, TilePosition, Unit, UnitType};

/// A map drawn as rows of text: `~` is sea, any other character is plains.
pub struct TerrainGrid(pub &'static [&'static str]);

impl MapTerrain for TerrainGrid {
    fn terrain_at(&self, position: TilePosition) -> Option<TerrainType> {
        let row = self.0.get(position.y as usize)?;
        let symbol = row.chars().nth(position.x as usize)?;
        Some(match symbol {
            '~' => TerrainType::Sea,
            _ => TerrainType::Plains,
        })
    }
}

pub fn colonist_at(position: TilePosition) -> Unit {
    Unit::land(UnitType::Colonist, [(position, TerrainType::Plains)]).unwrap()
}

pub fn caravel_at(position: TilePosition) -> Unit {
    Unit::land(UnitType::Caravel, [(position, TerrainType::Sea)]).unwrap()
}

/// Land everywhere, so only the limits of a tile position end the map.
pub struct EndlessLand;

impl MapTerrain for EndlessLand {
    fn terrain_at(&self, _position: TilePosition) -> Option<TerrainType> {
        Some(TerrainType::Plains)
    }
}
