use std::collections::HashSet;

use lib_terrain::GoodsType;

#[test]
fn should_list_every_goods_type_exactly_once() {
    let expected: HashSet<GoodsType> = HashSet::from([
        GoodsType::Food,
        GoodsType::Wood,
        GoodsType::Ore,
        GoodsType::Cotton,
        GoodsType::Furs,
        GoodsType::Sugar,
        GoodsType::Tobacco,
        GoodsType::Silver,
    ]);

    assert_eq!(expected.len(), GoodsType::ALL.len());
    assert_eq!(expected, HashSet::from(GoodsType::ALL));
}
