use lib_terrain::TerrainType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    Colonist,
    Caravel,
}

impl UnitType {
    pub fn can_enter(&self, terrain: TerrainType) -> bool {
        match self {
            UnitType::Colonist => !terrain.is_water(),
            UnitType::Caravel => terrain.is_water(),
        }
    }

    pub(crate) fn is_carrier(&self) -> bool {
        self.capacity() > 0
    }

    pub(crate) fn capacity(&self) -> usize {
        match self {
            UnitType::Colonist => 0,
            UnitType::Caravel => 2,
        }
    }

    pub(crate) fn can_be_carried(&self) -> bool {
        match self {
            UnitType::Colonist => true,
            UnitType::Caravel => false,
        }
    }
}
