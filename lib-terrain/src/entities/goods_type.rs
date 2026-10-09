#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum GoodsType {
    Food,
    Wood,
    Ore,
    Cotton,
    Furs,
    Sugar,
    Tobacco,
    Silver,
}

impl GoodsType {
    pub const ALL: [GoodsType; 8] = [
        GoodsType::Food,
        GoodsType::Wood,
        GoodsType::Ore,
        GoodsType::Cotton,
        GoodsType::Furs,
        GoodsType::Sugar,
        GoodsType::Tobacco,
        GoodsType::Silver,
    ];
}
