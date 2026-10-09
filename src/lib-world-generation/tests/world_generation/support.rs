use lib_world_generation::NoiseValue;

pub fn noise(value: f32) -> NoiseValue {
    NoiseValue::new(value).unwrap()
}
