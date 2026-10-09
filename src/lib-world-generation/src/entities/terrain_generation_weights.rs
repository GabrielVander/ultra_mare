use std::collections::HashSet;

use lib_terrain::TerrainType;

use crate::entities::noise_value::NoiseValue;

// Weights like 0.35 + 0.45 + 0.2 add up to 1.0 on paper but not in `f32`.
const WEIGHT_SUM_TOLERANCE: f32 = 1e-6;

#[derive(Debug, PartialEq)]
pub enum TerrainGenerationWeightsError {
    WeightsDoNotSumToOne,
    DuplicateTerrain(TerrainType),
    InvalidWeight(TerrainType),
}

#[derive(Debug)]
pub struct TerrainGenerationWeights {
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
