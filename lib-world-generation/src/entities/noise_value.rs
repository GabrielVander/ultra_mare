#[derive(Debug, PartialEq)]
pub enum NoiseValueError {
    OutOfRange,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseValue(f32);

impl NoiseValue {
    /// # Errors
    ///
    /// Fails when `value` lies outside `0.0..=1.0`.
    pub fn new(value: f32) -> Result<Self, NoiseValueError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(NoiseValueError::OutOfRange);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn value(&self) -> f32 {
        self.0
    }
}
