use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    iter,
};

use lib_terrain::entities::terrain_type::TerrainType;
use ordered_float::OrderedFloat;

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
    weights_min_heap: BinaryHeap<Reverse<(TerrainType, OrderedFloat<f32>)>>,
}

impl TerrainGenerationWeights {
    pub fn new(weights: &[(TerrainType, f32)]) -> Result<Self, TerrainGenerationWeightsError> {
        TerrainGenerationWeights::ensure_no_duplicate_terrain(weights)?;
        TerrainGenerationWeights::ensure_every_weight_is_valid(weights)?;

        let min_heap: BinaryHeap<Reverse<(TerrainType, OrderedFloat<f32>)>> =
            TerrainGenerationWeights::build_min_heap_with_given_weights(weights);

        let sum: f32 = TerrainGenerationWeights::get_min_heap_sum(&min_heap);
        if (sum - 1.0).abs() > WEIGHT_SUM_TOLERANCE {
            return Err(TerrainGenerationWeightsError::WeightsDoNotSumToOne);
        }

        Ok(Self {
            weights_min_heap: min_heap,
        })
    }

    pub fn get_terrain_from_given_noise_value(&self, value: &f32) -> TerrainType {
        self.weights_min_heap
            .iter()
            .find(|weight| weight.0.1 == *value)
            .unwrap()
            .0
            .0
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

    fn build_min_heap_with_given_weights(
        weights: &[(TerrainType, f32)],
    ) -> BinaryHeap<Reverse<(TerrainType, OrderedFloat<f32>)>> {
        let min_heap: BinaryHeap<_> = weights
            .iter()
            .map(|(k, v)| Reverse((*k, OrderedFloat(*v))))
            .collect();
        min_heap
    }

    fn get_min_heap_sum(min_heap: &BinaryHeap<Reverse<(TerrainType, OrderedFloat<f32>)>>) -> f32 {
        min_heap.iter().map(|e| e.0.1.0).sum()
    }
}

#[cfg(test)]
mod test {
    use lib_terrain::entities::terrain_type::TerrainType;

    use super::{TerrainGenerationWeights, TerrainGenerationWeightsError};

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
    fn should_return_terrain_when_given_only_one() {
        let raw_weights = Vec::from([
            (TerrainType::Mountain, 0.05),
            (TerrainType::Hills, 0.1),
            (TerrainType::Forest, 0.15),
            (TerrainType::Jungle, 0.3),
            (TerrainType::Plains, 0.4),
        ]);

        let weights = TerrainGenerationWeights::new(&raw_weights).unwrap();

        raw_weights.iter().for_each(|(expected_terrain, weight)| {
            assert_eq!(
                expected_terrain,
                &weights.get_terrain_from_given_noise_value(weight)
            )
        });
    }
}
