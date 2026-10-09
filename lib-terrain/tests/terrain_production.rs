use std::collections::HashSet;

use lib_terrain::{GoodsType, Production, TerrainType};

#[test]
fn should_produce_food_cotton_and_tobacco_on_plains() {
    let production: &[Production] = TerrainType::Plains.production();

    assert_eq!(
        [
            produce(GoodsType::Food, 5),
            produce(GoodsType::Cotton, 3),
            produce(GoodsType::Tobacco, 2),
        ],
        production
    );
}

#[test]
fn should_produce_food_wood_and_furs_in_forest() {
    let production: &[Production] = TerrainType::Forest.production();

    assert_eq!(
        [
            produce(GoodsType::Food, 3),
            produce(GoodsType::Wood, 6),
            produce(GoodsType::Furs, 3),
        ],
        production
    );
}

#[test]
fn should_produce_food_wood_sugar_and_furs_in_jungle() {
    let production: &[Production] = TerrainType::Jungle.production();

    assert_eq!(
        [
            produce(GoodsType::Food, 2),
            produce(GoodsType::Wood, 4),
            produce(GoodsType::Sugar, 3),
            produce(GoodsType::Furs, 1),
        ],
        production
    );
}

#[test]
fn should_produce_food_and_ore_in_hills() {
    let production: &[Production] = TerrainType::Hills.production();

    assert_eq!(
        [produce(GoodsType::Food, 4), produce(GoodsType::Ore, 4)],
        production
    );
}

#[test]
fn should_produce_ore_and_silver_in_mountains() {
    let production: &[Production] = TerrainType::Mountain.production();

    assert_eq!(
        [produce(GoodsType::Ore, 4), produce(GoodsType::Silver, 1)],
        production
    );
}

#[test]
fn should_produce_only_food_at_sea() {
    let production: &[Production] = TerrainType::Sea.production();

    assert_eq!([produce(GoodsType::Food, 4)], production);
}

#[test]
fn should_produce_every_goods_type_somewhere() {
    let produced: HashSet<GoodsType> = TerrainType::ALL
        .into_iter()
        .flat_map(TerrainType::production)
        .map(|production| production.goods)
        .collect();

    assert_eq!(HashSet::from(GoodsType::ALL), produced);
}

#[test]
fn should_list_each_goods_type_once_per_terrain_with_a_positive_amount() {
    TerrainType::ALL.into_iter().for_each(|terrain| {
        let production: &[Production] = terrain.production();

        let goods: HashSet<GoodsType> = production.iter().map(|each| each.goods).collect();
        assert_eq!(production.len(), goods.len(), "{terrain:?}");
        assert!(production.iter().all(|each| each.amount > 0), "{terrain:?}");
    });
}

fn produce(goods: GoodsType, amount: u8) -> Production {
    Production { goods, amount }
}
