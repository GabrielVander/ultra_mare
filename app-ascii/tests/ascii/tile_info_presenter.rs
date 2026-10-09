use app_ascii::{ActiveUnit, Glyph, TileInfoViewModel, glyph_for, present_tile_info};
use lib_terrain::TerrainType;
use lib_unit::{Direction, Expedition};

use crate::support::{Island, Shore};

#[test]
fn should_name_the_terrain_under_the_active_colonist_next_to_its_map_glyph() {
    let expedition = Expedition::land(&Shore.tiles());

    let tile_info: Option<TileInfoViewModel> =
        present_tile_info(&expedition, ActiveUnit::Colonist, &Shore);

    let terrain: Option<(Glyph, &str)> = tile_info
        .as_ref()
        .map(|info| (info.terrain.glyph, info.terrain.label.as_str()));
    assert_eq!(Some((glyph_for(TerrainType::Plains), "Plains")), terrain);
}

#[test]
fn should_list_what_the_tile_produces() {
    let expedition = Expedition::land(&Shore.tiles());

    let tile_info: Option<TileInfoViewModel> =
        present_tile_info(&expedition, ActiveUnit::Colonist, &Shore);

    let production: Option<Vec<(&str, u8)>> = tile_info.as_ref().map(production_of);
    assert_eq!(
        Some(vec![("Food", 5), ("Cotton", 3), ("Tobacco", 2)]),
        production
    );
}

fn production_of(tile_info: &TileInfoViewModel) -> Vec<(&str, u8)> {
    tile_info
        .production
        .iter()
        .map(|line| (line.goods.as_str(), line.amount))
        .collect()
}

#[test]
fn should_name_every_goods_type() {
    let land: [TerrainType; 3] = [
        TerrainType::Forest,
        TerrainType::Jungle,
        TerrainType::Mountain,
    ];

    let goods: Vec<String> = land
        .into_iter()
        .filter_map(|terrain| {
            let island = Island(terrain);

            present_tile_info(
                &Expedition::land(&island.tiles()),
                ActiveUnit::Colonist,
                &island,
            )
        })
        .flat_map(|tile_info| tile_info.production)
        .map(|line| line.goods)
        .collect();

    assert_eq!(
        vec![
            "Food", "Wood", "Furs", "Food", "Wood", "Sugar", "Furs", "Ore", "Silver"
        ],
        goods
    );
}

#[test]
fn should_follow_the_active_unit() {
    let expedition = Expedition::land(&Shore.tiles());

    let tile_info: Option<TileInfoViewModel> =
        present_tile_info(&expedition, ActiveUnit::Caravel, &Shore);

    let label: Option<&str> = tile_info.as_ref().map(|info| info.terrain.label.as_str());
    assert_eq!(Some("Sea"), label);

    let production: Option<Vec<(&str, u8)>> = tile_info.as_ref().map(production_of);
    assert_eq!(Some(vec![("Food", 4)]), production);
}

#[test]
fn should_show_the_tile_of_the_caravel_carrying_the_active_colonist() {
    let mut expedition = Expedition::land(&Shore.tiles());
    expedition.move_colonist(Direction::East, &Shore);
    expedition.move_colonist(Direction::East, &Shore);

    let tile_info: Option<TileInfoViewModel> =
        present_tile_info(&expedition, ActiveUnit::Colonist, &Shore);

    let label: Option<&str> = tile_info.as_ref().map(|info| info.terrain.label.as_str());
    assert_eq!(Some("Sea"), label);
}

#[test]
fn should_show_no_tile_when_the_active_unit_is_nowhere_on_the_map() {
    let expedition = Expedition::land(&[]);

    let tile_info: Option<TileInfoViewModel> =
        present_tile_info(&expedition, ActiveUnit::Colonist, &Shore);

    assert_eq!(None, tile_info);
}

#[test]
fn should_show_no_tile_when_the_map_has_no_terrain_under_the_active_unit() {
    let expedition = Expedition::land(&Shore.tiles());

    let tile_info: Option<TileInfoViewModel> = present_tile_info(
        &expedition,
        ActiveUnit::Caravel,
        &Island(TerrainType::Plains),
    );

    assert_eq!(None, tile_info);
}
