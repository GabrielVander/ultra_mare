use std::num::NonZeroU32;

use crate::{
    entities::{map_position::MapPosition, noise_value::NoiseValue},
    noise::noise_source::NoiseSource,
};

/// Smooth, seeded noise: random values sit on a lattice spaced `feature_size`
/// tiles apart and the tiles in between blend them, which yields landmasses
/// roughly `feature_size` tiles across.
#[derive(Debug, Clone, Copy)]
pub struct ValueNoise {
    seed: u64,
    feature_size: NonZeroU32,
}

impl ValueNoise {
    #[must_use]
    pub fn new(seed: u64, feature_size: NonZeroU32) -> Self {
        Self { seed, feature_size }
    }
}

impl NoiseSource for ValueNoise {
    fn noise_at(&self, position: MapPosition) -> NoiseValue {
        // Blending stays between the blended values, but float rounding may step a hair past them.
        let value = self.blended_value(position).clamp(0.0, 1.0);
        NoiseValue::new(value).expect("a clamped value is within the noise range")
    }
}

impl ValueNoise {
    #[expect(
        clippy::cast_precision_loss,
        reason = "map coordinates stay far below the 2^24 that f32 holds exactly"
    )]
    fn blended_value(&self, position: MapPosition) -> f32 {
        let feature_size = self.feature_size.get();
        let (cell_x, cell_y) = (position.x / feature_size, position.y / feature_size);
        let along_x = smoothstep((position.x % feature_size) as f32 / feature_size as f32);
        let along_y = smoothstep((position.y % feature_size) as f32 / feature_size as f32);

        let top = lerp(
            self.lattice_value(cell_x, cell_y),
            self.lattice_value(cell_x.wrapping_add(1), cell_y),
            along_x,
        );
        let bottom = lerp(
            self.lattice_value(cell_x, cell_y.wrapping_add(1)),
            self.lattice_value(cell_x.wrapping_add(1), cell_y.wrapping_add(1)),
            along_x,
        );
        lerp(top, bottom, along_y)
    }

    fn lattice_value(&self, x: u32, y: u32) -> f32 {
        unit_interval(mix(mix(self.seed ^ u64::from(x)) ^ (u64::from(y) << 32)))
    }
}

// Easing the blend hides the lattice's grid lines.
fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t
}

// SplitMix64's finalizer: cheap, and good enough to scatter neighbouring inputs.
fn mix(value: u64) -> u64 {
    let mut z = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

// The top 24 bits fill an `f32` mantissa exactly, so the result never rounds up to 1.
#[expect(clippy::cast_precision_loss, reason = "24 bits convert to f32 exactly")]
fn unit_interval(hash: u64) -> f32 {
    (hash >> 40) as f32 / (1u64 << 24) as f32
}
