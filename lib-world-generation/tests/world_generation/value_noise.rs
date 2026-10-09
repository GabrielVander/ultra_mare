use std::num::NonZeroU32;

use lib_world_generation::{MapPosition, NoiseSource, ValueNoise};

const FEATURE_SIZE: NonZeroU32 = NonZeroU32::new(8).unwrap();

#[test]
fn should_give_the_same_noise_for_the_same_seed() {
    let first = ValueNoise::new(42, FEATURE_SIZE);
    let second = ValueNoise::new(42, FEATURE_SIZE);

    assert_eq!(sample(&first), sample(&second));
}

#[test]
fn should_give_different_noise_for_different_seeds() {
    let first = ValueNoise::new(42, FEATURE_SIZE);
    let second = ValueNoise::new(43, FEATURE_SIZE);

    assert_ne!(sample(&first), sample(&second));
}

#[test]
fn should_vary_across_the_map() {
    let values = sample(&ValueNoise::new(42, FEATURE_SIZE));

    let lowest = values.iter().copied().fold(f32::INFINITY, f32::min);
    let highest = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    assert!(highest - lowest > 0.5, "{lowest}..{highest}");
}

#[test]
fn should_change_gradually_between_neighbouring_tiles() {
    let noise = ValueNoise::new(42, FEATURE_SIZE);
    let value_at = |x, y| noise.noise_at(MapPosition { x, y }).value();

    (0..32).for_each(|y| {
        (0..32).for_each(|x| {
            let here = value_at(x, y);
            [value_at(x + 1, y), value_at(x, y + 1)]
                .into_iter()
                .for_each(|neighbour| {
                    assert!(
                        (here - neighbour).abs() <= 0.2,
                        "({x}, {y}): {here} next to {neighbour}"
                    )
                });
        })
    });
}

#[test]
fn should_give_noise_at_the_farthest_position() {
    let noise = ValueNoise::new(42, NonZeroU32::MIN);
    let farthest = MapPosition {
        x: u32::MAX,
        y: u32::MAX,
    };

    assert_eq!(noise.noise_at(farthest), noise.noise_at(farthest));
}

fn sample(noise: &impl NoiseSource) -> Vec<f32> {
    (0..32)
        .flat_map(|y| (0..32).map(move |x| MapPosition { x, y }))
        .map(|position| noise.noise_at(position).value())
        .collect()
}
