use lib_terrain::TerrainType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    OffMap,
    Impassable(TerrainType),
}
