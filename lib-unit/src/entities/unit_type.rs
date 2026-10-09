use lib_terrain::TerrainType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    FreeColonist,
}

impl UnitType {
    pub fn can_enter(&self, terrain: TerrainType) -> bool {
        match self {
            UnitType::FreeColonist => !terrain.is_water(),
        }
    }
}
