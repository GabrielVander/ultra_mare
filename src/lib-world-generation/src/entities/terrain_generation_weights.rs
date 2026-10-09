use std::collections::HashSet;

use lib_terrain::TerrainType;

use crate::entities::noise_value::NoiseValue;

// Weights like 0.35 + 0.45 + 0.2 add up to 1.0 on paper but not in `f32`.
const WEIGHT_SUM_TOLERANCE: f32 = 1e-6;

#[derive(Debug, PartialEq)]
pub(crate) enum TerrainGenerationWeightsError {
    WeightsDoNotSumToOne,
    DuplicateTerrain(TerrainType),
    InvalidWeight(TerrainType),
}

#[derive(Debug)]
pub(crate) struct TerrainGenerationWeights {
    bands: Vec<TerrainBand>,
}

#[derive(Debug)]
struct TerrainBand {
    terrain: TerrainType,
    upper_bound: f32,
}

impl TerrainGenerationWeights {
    pub fn new(weights: &[(TerrainType, f32)]) -> Result<Self, TerrainGenerationWeightsError> {
        TerrainGenerationWeights::ensure_no_duplicate_terrain(weights)?;
        TerrainGenerationWeights::ensure_every_weight_is_valid(weights)?;
        TerrainGenerationWeights::ensure_weights_sum_to_one(weights)?;

        Ok(Self {
            bands: TerrainGenerationWeights::build_bands(weights),
        })
    }

    pub fn terrain_for(&self, noise: NoiseValue) -> TerrainType {
        self.bands
            .iter()
            .find(|band| noise.value() < band.upper_bound)
            .or(self.bands.last())
            .expect("weights summing to one always leave at least one band")
            .terrain
    }

    fn ensure_no_duplicate_terrain(
        weights: &[(TerrainType, f32)],
    ) -> Result<(), TerrainGenerationWeightsError> {
        let mut seen: HashSet<TerrainType> = HashSet::new();
        match weights.iter().find(|(terrain, _)| !seen.insert(*terrain)) {
            Some((terrain, _)) => Err(TerrainGenerationWeightsError::DuplicateTerrain(*terrain)),
            None => Ok(()),
        }
    }

    fn ensure_every_weight_is_valid(
        weights: &[(TerrainType, f32)],
    ) -> Result<(), TerrainGenerationWeightsError> {
        match weights
            .iter()
            .find(|(_, weight)| !TerrainGenerationWeights::is_valid_weight(*weight))
        {
            Some((terrain, _)) => Err(TerrainGenerationWeightsError::InvalidWeight(*terrain)),
            None => Ok(()),
        }
    }

    fn is_valid_weight(weight: f32) -> bool {
        weight.is_finite() && weight >= 0.0
    }

    fn ensure_weights_sum_to_one(
        weights: &[(TerrainType, f32)],
    ) -> Result<(), TerrainGenerationWeightsError> {
        let sum: f32 = weights.iter().map(|(_, weight)| weight).sum();
        if (sum - 1.0).abs() > WEIGHT_SUM_TOLERANCE {
            return Err(TerrainGenerationWeightsError::WeightsDoNotSumToOne);
        }
        Ok(())
    }

    // A terrain without weight would get an empty band; leaving it out keeps the
    // fallback to the last band for noise at the very top of the range correct.
    fn build_bands(weights: &[(TerrainType, f32)]) -> Vec<TerrainBand> {
        weights
            .iter()
            .filter(|(_, weight)| *weight > 0.0)
            .scan(0.0, |upper_bound, (terrain, weight)| {
                *upper_bound += weight;
                Some(TerrainBand {
                    terrain: *terrain,
                    upper_bound: *upper_bound,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use lib_terrain::TerrainType;

    use super::{TerrainGenerationWeights, TerrainGenerationWeightsError};
    use crate::entities::noise_value::NoiseValue;

    fn noise(value: f32) -> NoiseValue {
        NoiseValue::new(value).unwrap()
    }

    #[test]
    fn should_fail_when_given_weights_that_dont_add_up_to_100_percent() {
        let weight_inputs: Vec<Vec<(TerrainType, f32)>> = vec![
            Vec::new(),
            Vec::from([(TerrainType::Mountain, 0.0)]),
            Vec::from([(TerrainType::Mountain, 0.9999)]),
            Vec::from([
                (TerrainType::Mountain, 0.0),
                (TerrainType::Hills, 0.0),
                (TerrainType::Forest, 0.0),
            ]),
            Vec::from([
                (TerrainType::Mountain, 0.3),
                (TerrainType::Hills, 0.3),
                (TerrainType::Forest, 0.3),
            ]),
            Vec::from([
                (TerrainType::Mountain, 1.0),
                (TerrainType::Hills, 0.0),
                (TerrainType::Forest, 0.1),
            ]),
            Vec::from([
                (TerrainType::Mountain, 0.21),
                (TerrainType::Hills, 0.21),
                (TerrainType::Forest, 0.21),
                (TerrainType::Jungle, 0.21),
                (TerrainType::Plains, 0.21),
            ]),
        ];

        weight_inputs.into_iter().for_each(|weights| {
            assert_eq!(
                Err(TerrainGenerationWeightsError::WeightsDoNotSumToOne),
                TerrainGenerationWeights::new(&weights).map(|_| ()),
                "{:?}",
                weights
            )
        });
    }

    #[test]
    fn should_succeed_when_given_weights_that_add_up_to_100_percent() {
        let weight_inputs: Vec<Vec<(TerrainType, f32)>> = vec![
            Vec::from([(TerrainType::Mountain, 1.0)]),
            Vec::from([
                (TerrainType::Mountain, 1.0),
                (TerrainType::Hills, 0.0),
                (TerrainType::Forest, 0.0),
            ]),
            Vec::from([
                (TerrainType::Mountain, 0.3),
                (TerrainType::Hills, 0.3),
                (TerrainType::Forest, 0.3),
                (TerrainType::Jungle, 0.1),
            ]),
            Vec::from([
                (TerrainType::Mountain, 0.2),
                (TerrainType::Hills, 0.2),
                (TerrainType::Forest, 0.2),
                (TerrainType::Jungle, 0.2),
                (TerrainType::Plains, 0.2),
            ]),
        ];

        weight_inputs.into_iter().for_each(|weights| {
            assert_eq!(Ok(()), TerrainGenerationWeights::new(&weights).map(|_| ()))
        });
    }

    #[test]
    fn should_succeed_when_weights_add_up_to_100_percent_except_for_float_rounding() {
        let weights = Vec::from([
            (TerrainType::Mountain, 0.35),
            (TerrainType::Hills, 0.45),
            (TerrainType::Forest, 0.2),
        ]);
        let rounded_sum: f32 = weights.iter().map(|(_, weight)| weight).sum();
        assert_ne!(
            1.0, rounded_sum,
            "precondition: the sum must be off by rounding"
        );

        assert_eq!(Ok(()), TerrainGenerationWeights::new(&weights).map(|_| ()))
    }

    #[test]
    fn should_fail_when_given_the_same_terrain_more_than_once() {
        let weight_inputs: Vec<(Vec<(TerrainType, f32)>, TerrainType)> = vec![
            (
                Vec::from([(TerrainType::Mountain, 0.5), (TerrainType::Mountain, 0.5)]),
                TerrainType::Mountain,
            ),
            (
                Vec::from([
                    (TerrainType::Mountain, 0.3),
                    (TerrainType::Hills, 0.3),
                    (TerrainType::Hills, 0.4),
                ]),
                TerrainType::Hills,
            ),
            (
                Vec::from([
                    (TerrainType::Mountain, 0.3),
                    (TerrainType::Hills, 0.3),
                    (TerrainType::Hills, 0.6),
                    (TerrainType::Forest, 0.3),
                    (TerrainType::Jungle, 0.1),
                ]),
                TerrainType::Hills,
            ),
            (
                Vec::from([
                    (TerrainType::Mountain, 1.0),
                    (TerrainType::Mountain, 0.9),
                    (TerrainType::Hills, 0.0),
                    (TerrainType::Forest, 0.1),
                ]),
                TerrainType::Mountain,
            ),
        ];

        weight_inputs
            .into_iter()
            .for_each(|(weights, duplicated_terrain)| {
                assert_eq!(
                    Err(TerrainGenerationWeightsError::DuplicateTerrain(
                        duplicated_terrain
                    )),
                    TerrainGenerationWeights::new(&weights).map(|_| ()),
                    "{:?}",
                    weights
                )
            });
    }

    #[test]
    fn should_fail_when_given_an_invalid_weight() {
        let weight_inputs: Vec<(Vec<(TerrainType, f32)>, TerrainType)> = vec![
            (
                Vec::from([(TerrainType::Mountain, 1.5), (TerrainType::Hills, -0.5)]),
                TerrainType::Hills,
            ),
            (
                Vec::from([(TerrainType::Sea, -0.1), (TerrainType::Plains, 0.5)]),
                TerrainType::Sea,
            ),
            (
                Vec::from([(TerrainType::Forest, f32::NAN), (TerrainType::Jungle, 1.0)]),
                TerrainType::Forest,
            ),
            (
                Vec::from([
                    (TerrainType::Forest, f32::INFINITY),
                    (TerrainType::Jungle, f32::NEG_INFINITY),
                ]),
                TerrainType::Forest,
            ),
        ];

        weight_inputs
            .into_iter()
            .for_each(|(weights, invalid_terrain)| {
                assert_eq!(
                    Err(TerrainGenerationWeightsError::InvalidWeight(
                        invalid_terrain
                    )),
                    TerrainGenerationWeights::new(&weights).map(|_| ()),
                    "{:?}",
                    weights
                )
            });
    }

    #[test]
    fn should_pick_the_only_terrain_for_any_noise_value() {
        let weights = TerrainGenerationWeights::new(&[(TerrainType::Sea, 1.0)]).unwrap();

        [0.0, 0.25, 0.5, 0.75, 1.0].into_iter().for_each(|value| {
            assert_eq!(
                TerrainType::Sea,
                weights.terrain_for(noise(value)),
                "{}",
                value
            )
        });
    }

    #[test]
    fn should_pick_the_band_containing_the_noise_value_with_boundaries_in_the_upper_band() {
        let weights =
            TerrainGenerationWeights::new(&[(TerrainType::Plains, 0.5), (TerrainType::Sea, 0.5)])
                .unwrap();

        [
            (0.0, TerrainType::Plains),
            (0.49, TerrainType::Plains),
            (0.5, TerrainType::Sea),
            (0.75, TerrainType::Sea),
        ]
        .into_iter()
        .for_each(|(value, expected_terrain)| {
            assert_eq!(
                expected_terrain,
                weights.terrain_for(noise(value)),
                "{}",
                value
            )
        });
    }

    #[test]
    fn should_pick_the_last_band_for_the_maximum_noise_value() {
        let weight_inputs: Vec<Vec<(TerrainType, f32)>> = vec![
            Vec::from([(TerrainType::Plains, 0.5), (TerrainType::Sea, 0.5)]),
            Vec::from([
                (TerrainType::Mountain, 0.35),
                (TerrainType::Hills, 0.45),
                (TerrainType::Sea, 0.2),
            ]),
        ];

        weight_inputs.into_iter().for_each(|raw_weights| {
            let weights = TerrainGenerationWeights::new(&raw_weights).unwrap();

            assert_eq!(
                TerrainType::Sea,
                weights.terrain_for(noise(1.0)),
                "{:?}",
                raw_weights
            )
        });
    }

    #[test]
    fn should_never_pick_a_terrain_without_weight() {
        let weight_inputs: Vec<Vec<(TerrainType, f32)>> = vec![
            Vec::from([(TerrainType::Mountain, 0.0), (TerrainType::Sea, 1.0)]),
            Vec::from([
                (TerrainType::Plains, 0.5),
                (TerrainType::Mountain, 0.0),
                (TerrainType::Sea, 0.5),
            ]),
            Vec::from([(TerrainType::Sea, 1.0), (TerrainType::Mountain, 0.0)]),
        ];

        weight_inputs.into_iter().for_each(|raw_weights| {
            let weights = TerrainGenerationWeights::new(&raw_weights).unwrap();

            [0.0, 0.5, 1.0].into_iter().for_each(|value| {
                assert_ne!(
                    TerrainType::Mountain,
                    weights.terrain_for(noise(value)),
                    "{:?} at {}",
                    raw_weights,
                    value
                )
            });
        });
    }

    #[test]
    fn should_pick_terrains_in_proportion_to_their_weights() {
        let weights = TerrainGenerationWeights::new(&[
            (TerrainType::Mountain, 0.05),
            (TerrainType::Hills, 0.1),
            (TerrainType::Forest, 0.15),
            (TerrainType::Jungle, 0.3),
            (TerrainType::Plains, 0.4),
        ])
        .unwrap();

        [
            (0.02, TerrainType::Mountain),
            (0.1, TerrainType::Hills),
            (0.2, TerrainType::Forest),
            (0.45, TerrainType::Jungle),
            (0.8, TerrainType::Plains),
        ]
        .into_iter()
        .for_each(|(value, expected_terrain)| {
            assert_eq!(
                expected_terrain,
                weights.terrain_for(noise(value)),
                "{}",
                value
            )
        });
    }
}
