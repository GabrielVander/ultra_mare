use lib_world_generation::{MapSize, MapSizeError};

#[test]
fn should_accept_a_positive_width_and_height() {
    let size = MapSize::new(40, 100).unwrap();

    assert_eq!((40, 100), (size.width(), size.height()));
}

#[test]
fn should_reject_a_map_without_width_or_height() {
    [(0, 100), (40, 0), (0, 0)]
        .into_iter()
        .for_each(|(width, height)| {
            assert_eq!(
                Err(MapSizeError::Empty),
                MapSize::new(width, height),
                "{width}x{height}"
            );
        });
}
