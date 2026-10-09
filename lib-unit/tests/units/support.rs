use lib_terrain::TerrainType;
use lib_unit::{MapTerrain, TilePosition, Unit, UnitType};

pub struct TerrainGrid(pub &'static [&'static str]);

impl TerrainGrid {
    pub fn tiles(&self) -> Vec<(TilePosition, TerrainType)> {
        (0..self.0.len())
            .flat_map(|y| (0..self.0[y].len()).map(move |x| (x, y)))
            .map(|(x, y)| TilePosition {
                x: u32::try_from(x).unwrap(),
                y: u32::try_from(y).unwrap(),
            })
            .filter_map(|position| self.terrain_at(position).map(|terrain| (position, terrain)))
            .collect()
    }
}

impl MapTerrain for TerrainGrid {
    fn terrain_at(&self, position: TilePosition) -> Option<TerrainType> {
        let row: &str = self.0.get(position.y as usize)?;
        let symbol: char = row.chars().nth(position.x as usize)?;
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

pub struct EndlessLand;

impl MapTerrain for EndlessLand {
    fn terrain_at(&self, _position: TilePosition) -> Option<TerrainType> {
        Some(TerrainType::Plains)
    }
}
