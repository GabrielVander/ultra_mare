use crate::entities::{goods_type::GoodsType, production::Production, terrain_type::TerrainType};

impl TerrainType {
    /// What a tile of this terrain yields each turn, food first.
    #[must_use]
    pub fn production(self) -> &'static [Production] {
        match self {
            TerrainType::Mountain => &MOUNTAIN,
            TerrainType::Hills => &HILLS,
            TerrainType::Forest => &FOREST,
            TerrainType::Jungle => &JUNGLE,
            TerrainType::Plains => &PLAINS,
            TerrainType::Sea => &SEA,
        }
    }
}

const MOUNTAIN: [Production; 2] = [
    Production {
        goods: GoodsType::Ore,
        amount: 4,
    },
    Production {
        goods: GoodsType::Silver,
        amount: 1,
    },
];

const HILLS: [Production; 2] = [
    Production {
        goods: GoodsType::Food,
        amount: 4,
    },
    Production {
        goods: GoodsType::Ore,
        amount: 4,
    },
];

const FOREST: [Production; 3] = [
    Production {
        goods: GoodsType::Food,
        amount: 3,
    },
    Production {
        goods: GoodsType::Wood,
        amount: 6,
    },
    Production {
        goods: GoodsType::Furs,
        amount: 3,
    },
];

const JUNGLE: [Production; 4] = [
    Production {
        goods: GoodsType::Food,
        amount: 2,
    },
    Production {
        goods: GoodsType::Wood,
        amount: 4,
    },
    Production {
        goods: GoodsType::Sugar,
        amount: 3,
    },
    Production {
        goods: GoodsType::Furs,
        amount: 1,
    },
];

const PLAINS: [Production; 3] = [
    Production {
        goods: GoodsType::Food,
        amount: 5,
    },
    Production {
        goods: GoodsType::Cotton,
        amount: 3,
    },
    Production {
        goods: GoodsType::Tobacco,
        amount: 2,
    },
];

const SEA: [Production; 1] = [Production {
    goods: GoodsType::Food,
    amount: 4,
}];
