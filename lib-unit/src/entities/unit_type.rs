use lib_terrain::TerrainType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    Colonist,
}

impl UnitType {
    pub fn can_enter(&self, terrain: TerrainType) -> bool {
        match self {
            UnitType::Colonist => !terrain.is_water(),
        }
    }
}
