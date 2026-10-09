use lib_terrain::{GoodsType, Production, TerrainType};
use lib_unit::{Expedition, MapTerrain, TilePosition};

use crate::{
    legend::legend_presenter::LegendEntry,
    map_view::{terrain_glyph::glyph_for, terrain_name::terrain_name},
    units::active_unit::ActiveUnit,
};

/// The tile the active unit stands on.
#[derive(Debug, PartialEq)]
pub struct TileInfoViewModel {
    pub terrain: LegendEntry,
    pub production: Vec<ProductionLine>,
}

#[derive(Debug, PartialEq)]
pub struct ProductionLine {
    pub goods: String,
    pub amount: u8,
}

#[must_use]
pub fn present_tile_info(
    expedition: &Expedition,
    active: ActiveUnit,
    map: &dyn MapTerrain,
) -> Option<TileInfoViewModel> {
    let position: TilePosition = active.position(expedition)?;
    let terrain: TerrainType = map.terrain_at(position)?;

    Some(TileInfoViewModel {
        terrain: LegendEntry {
            glyph: glyph_for(terrain),
            label: terrain_name(terrain).to_string(),
        },
        production: terrain
            .production()
            .iter()
            .copied()
            .map(production_line)
            .collect(),
    })
}

fn production_line(production: Production) -> ProductionLine {
    ProductionLine {
        goods: goods_name(production.goods).to_string(),
        amount: production.amount,
    }
}

fn goods_name(goods: GoodsType) -> &'static str {
    match goods {
        GoodsType::Food => "Food",
        GoodsType::Wood => "Wood",
        GoodsType::Ore => "Ore",
        GoodsType::Cotton => "Cotton",
        GoodsType::Furs => "Furs",
        GoodsType::Sugar => "Sugar",
        GoodsType::Tobacco => "Tobacco",
        GoodsType::Silver => "Silver",
    }
}
