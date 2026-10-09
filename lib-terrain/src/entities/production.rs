use crate::entities::goods_type::GoodsType;

/// How much of one goods type a tile yields each turn.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Production {
    pub goods: GoodsType,
    pub amount: u8,
}
