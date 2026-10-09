#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum TerrainType {
    Mountain,
    Hills,
    Forest,
    Jungle,
    Plains,
    Sea,
}

impl TerrainType {
    pub const ALL: [TerrainType; 6] = [
        TerrainType::Mountain,
        TerrainType::Hills,
        TerrainType::Forest,
        TerrainType::Jungle,
        TerrainType::Plains,
        TerrainType::Sea,
    ];

    pub fn is_water(&self) -> bool {
        matches!(self, TerrainType::Sea)
    }
}
