use lib_world_generation::{MapPosition, NoiseSource, NoiseValue};

pub fn noise(value: f32) -> NoiseValue {
    NoiseValue::new(value).unwrap()
}

pub struct ConstantNoise(pub f32);

impl NoiseSource for ConstantNoise {
    fn noise_at(&self, _position: MapPosition) -> NoiseValue {
        noise(self.0)
    }
}
