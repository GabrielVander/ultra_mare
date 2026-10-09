use lib_terrain::TerrainType;

pub(crate) fn terrain_name(terrain: TerrainType) -> &'static str {
    match terrain {
        TerrainType::Mountain => "Mountain",
        TerrainType::Hills => "Hills",
        TerrainType::Forest => "Forest",
        TerrainType::Jungle => "Jungle",
        TerrainType::Plains => "Plains",
        TerrainType::Sea => "Sea",
    }
}
