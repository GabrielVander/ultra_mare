use crate::entities::{map_position::MapPosition, noise_value::NoiseValue};

/// Supplies the noise that decides the terrain of each tile, so the source of
/// randomness stays outside the generation rules.
pub trait NoiseSource {
    fn noise_at(&self, position: MapPosition) -> NoiseValue;
}
