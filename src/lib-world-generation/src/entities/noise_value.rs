#[derive(Debug, PartialEq)]
pub(crate) enum NoiseValueError {
    OutOfRange,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct NoiseValue(f32);

impl NoiseValue {
    pub fn new(value: f32) -> Result<Self, NoiseValueError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(NoiseValueError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::{NoiseValue, NoiseValueError};

    #[test]
    fn should_accept_values_from_zero_to_one() {
        let values: Vec<f32> = vec![0.0, 0.5, 1.0];

        values.into_iter().for_each(|value| {
            assert_eq!(Ok(value), NoiseValue::new(value).map(|noise| noise.value()))
        });
    }

    #[test]
    fn should_reject_values_outside_zero_to_one() {
        let values: Vec<f32> = vec![-0.01, 1.01, f32::NAN, f32::INFINITY, f32::NEG_INFINITY];

        values.into_iter().for_each(|value| {
            assert_eq!(
                Err(NoiseValueError::OutOfRange),
                NoiseValue::new(value),
                "{:?}",
                value
            )
        });
    }
}
