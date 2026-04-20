use std::collections::HashMap;

use lib_terrain::entities::terrain_type::TerrainType;

#[derive(Debug, PartialEq)]
pub(crate) struct TerrainGenerationWeights(HashMap<TerrainType, f32>);

impl TerrainGenerationWeights {
    pub fn new(weights: &HashMap<TerrainType, f32>) -> Result<Self, String> {
        let sum: f32 = weights.values().fold(0.0, |acc, e| acc + e);
        if sum != 1.0 {
            return Err("Given weights should add up to 1.0".to_owned());
        }

        Ok(Self(weights.clone()))
    }

    pub fn get_weight_for(&self, target: &TerrainType) -> &f32 {
        self.0.get(target).unwrap_or(&0.0)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use lib_terrain::entities::terrain_type::TerrainType;

    use super::TerrainGenerationWeights;

    #[test]
    fn should_fail_when_given_weights_that_dont_add_up_to_100_percent() {
        let weight_inputs: Vec<HashMap<TerrainType, f32>> = vec![
            HashMap::new(),
            HashMap::from([(TerrainType::Mountain, 0.0)]),
            HashMap::from([(TerrainType::Mountain, 0.9999)]),
            HashMap::from([
                (TerrainType::Mountain, 0.0),
                (TerrainType::Hills, 0.0),
                (TerrainType::Forest, 0.0),
            ]),
            HashMap::from([
                (TerrainType::Mountain, 0.3),
                (TerrainType::Hills, 0.3),
                (TerrainType::Forest, 0.3),
            ]),
            HashMap::from([
                (TerrainType::Mountain, 1.0),
                (TerrainType::Hills, 0.0),
                (TerrainType::Forest, 0.1),
            ]),
            HashMap::from([
                (TerrainType::Mountain, 0.21),
                (TerrainType::Hills, 0.21),
                (TerrainType::Forest, 0.21),
                (TerrainType::Jungle, 0.21),
                (TerrainType::Plains, 0.21),
            ]),
        ];

        weight_inputs.into_iter().for_each(|weights| {
            assert_eq!(
                Err("Given weights should add up to 1.0".to_owned()),
                TerrainGenerationWeights::new(&weights)
            )
        });
    }

    #[test]
    fn should_succeed_when_given_weights_that_add_up_to_100_percent() {
        let weight_inputs: Vec<HashMap<TerrainType, f32>> = vec![
            HashMap::from([(TerrainType::Mountain, 1.0)]),
            HashMap::from([
                (TerrainType::Mountain, 1.0),
                (TerrainType::Hills, 0.0),
                (TerrainType::Forest, 0.0),
            ]),
            HashMap::from([
                (TerrainType::Mountain, 0.3),
                (TerrainType::Hills, 0.3),
                (TerrainType::Forest, 0.3),
                (TerrainType::Jungle, 0.1),
            ]),
            HashMap::from([
                (TerrainType::Mountain, 0.2),
                (TerrainType::Hills, 0.2),
                (TerrainType::Forest, 0.2),
                (TerrainType::Jungle, 0.2),
                (TerrainType::Plains, 0.2),
            ]),
        ];

        weight_inputs.into_iter().for_each(|weights| {
            assert_eq!(
                Ok(TerrainGenerationWeights(weights.clone())),
                TerrainGenerationWeights::new(&weights)
            )
        });
    }

    #[test]
    fn should_return_0_when_given_unset_terrain() {
        let raw_weights = HashMap::from([
            (TerrainType::Mountain, 0.05),
            (TerrainType::Hills, 0.1),
            (TerrainType::Forest, 0.15),
            (TerrainType::Jungle, 0.3),
            (TerrainType::Plains, 0.4),
        ]);

        let weights = TerrainGenerationWeights::new(&raw_weights).unwrap();

        raw_weights.iter().for_each(|(terrain, expected_weight)| {
            assert_eq!(expected_weight, weights.get_weight_for(terrain))
        });
    }

    #[test]
    fn should_return_expected_weight_given_set_terrain() {
        let weights =
            TerrainGenerationWeights::new(&HashMap::from([(TerrainType::Mountain, 1.0)])).unwrap();

        let unset_terrains: Vec<TerrainType> = vec![
            TerrainType::Hills,
            TerrainType::Forest,
            TerrainType::Jungle,
            TerrainType::Plains,
        ];

        unset_terrains
            .iter()
            .for_each(|unset_terrain| assert_eq!(&0.0, weights.get_weight_for(unset_terrain)));
    }
}
