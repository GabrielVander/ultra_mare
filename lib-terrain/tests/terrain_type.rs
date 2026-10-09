use std::collections::HashSet;

use lib_terrain::TerrainType;

#[test]
fn should_list_every_terrain_type_exactly_once() {
    let expected: HashSet<TerrainType> = HashSet::from([
        TerrainType::Mountain,
        TerrainType::Hills,
        TerrainType::Forest,
        TerrainType::Jungle,
        TerrainType::Plains,
        TerrainType::Sea,
    ]);

    assert_eq!(expected.len(), TerrainType::ALL.len());
    assert_eq!(expected, HashSet::from(TerrainType::ALL));
}
