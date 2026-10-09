use lib_terrain::TerrainType;
use lib_unit::UnitType;

#[test]
fn should_let_a_colonist_enter_land_but_not_water() {
    let enterable: Vec<TerrainType> = TerrainType::ALL
        .into_iter()
        .filter(|terrain| UnitType::Colonist.can_enter(*terrain))
        .collect();

    assert_eq!(
        vec![
            TerrainType::Mountain,
            TerrainType::Hills,
            TerrainType::Forest,
            TerrainType::Jungle,
            TerrainType::Plains,
        ],
        enterable
    );
}

#[test]
fn should_let_a_caravel_enter_water_but_not_land() {
    let enterable: Vec<TerrainType> = TerrainType::ALL
        .into_iter()
        .filter(|terrain| UnitType::Caravel.can_enter(*terrain))
        .collect();

    assert_eq!(vec![TerrainType::Sea], enterable);
}
