mod legend;
mod map_view;

pub use legend::legend_presenter::{ControlHint, LegendEntry, LegendViewModel, present_legend};
pub use map_view::{
    map_presenter::{MapViewModel, present_map},
    terrain_glyph::{Glyph, Rgb, glyph_for},
};
