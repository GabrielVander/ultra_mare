use app_ascii::{PanDirection, Viewport};
use lib_world_generation::{MapPosition, MapSize};

#[test]
fn should_start_at_the_top_left_of_the_map() {
    let viewport = Viewport::new(map_size(20, 10), 5, 4);

    assert_eq!(MapPosition { x: 0, y: 0 }, viewport.origin());
}

#[test]
fn should_pan_one_tile_in_the_given_direction() {
    [
        (PanDirection::Left, MapPosition { x: 2, y: 3 }),
        (PanDirection::Right, MapPosition { x: 4, y: 3 }),
        (PanDirection::Up, MapPosition { x: 3, y: 2 }),
        (PanDirection::Down, MapPosition { x: 3, y: 4 }),
    ]
    .into_iter()
    .for_each(|(direction, expected_origin)| {
        let mut viewport = viewport_at(MapPosition { x: 3, y: 3 });

        viewport.pan(direction);

        assert_eq!(expected_origin, viewport.origin(), "{direction:?}");
    });
}

#[test]
fn should_not_pan_past_the_edges_of_the_map() {
    let top_left = MapPosition { x: 0, y: 0 };
    let bottom_right = MapPosition { x: 15, y: 6 };
    [
        (top_left, PanDirection::Left),
        (top_left, PanDirection::Up),
        (bottom_right, PanDirection::Right),
        (bottom_right, PanDirection::Down),
    ]
    .into_iter()
    .for_each(|(origin, direction)| {
        let mut viewport = viewport_at(origin);

        viewport.pan(direction);

        assert_eq!(origin, viewport.origin(), "{direction:?} from {origin:?}");
    });
}

#[test]
fn should_stay_at_the_top_left_when_the_whole_map_fits() {
    let mut viewport = Viewport::new(map_size(4, 3), 10, 10);

    viewport.pan(PanDirection::Right);
    viewport.pan(PanDirection::Down);

    assert_eq!(MapPosition { x: 0, y: 0 }, viewport.origin());
}

#[test]
fn should_pull_back_inside_the_map_when_resized() {
    let mut viewport = viewport_at(MapPosition { x: 15, y: 6 });

    viewport.resize(8, 6);

    assert_eq!(MapPosition { x: 12, y: 4 }, viewport.origin());
}

#[test]
fn should_pan_within_its_new_size_after_a_resize() {
    let mut viewport = Viewport::new(map_size(20, 10), 5, 4);
    viewport.resize(18, 9);

    (0..5).for_each(|_| viewport.pan(PanDirection::Right));
    (0..5).for_each(|_| viewport.pan(PanDirection::Down));

    assert_eq!(MapPosition { x: 2, y: 1 }, viewport.origin());
}

#[test]
fn should_stay_put_when_keeping_an_already_visible_position_in_view() {
    let origin = MapPosition { x: 3, y: 2 };
    [
        MapPosition { x: 3, y: 2 },
        MapPosition { x: 7, y: 5 },
        MapPosition { x: 5, y: 3 },
    ]
    .into_iter()
    .for_each(|position| {
        let mut viewport = viewport_at(origin);

        viewport.keep_in_view(position);

        assert_eq!(origin, viewport.origin(), "{position:?}");
    });
}

#[test]
fn should_pan_just_enough_to_show_a_position_right_of_or_below_it() {
    [
        (MapPosition { x: 8, y: 3 }, MapPosition { x: 4, y: 2 }),
        (MapPosition { x: 5, y: 7 }, MapPosition { x: 3, y: 4 }),
        (MapPosition { x: 12, y: 9 }, MapPosition { x: 8, y: 6 }),
    ]
    .into_iter()
    .for_each(|(position, expected_origin)| {
        let mut viewport = viewport_at(MapPosition { x: 3, y: 2 });

        viewport.keep_in_view(position);

        assert_eq!(expected_origin, viewport.origin(), "{position:?}");
    });
}

#[test]
fn should_pan_just_enough_to_show_a_position_left_of_or_above_it() {
    [
        (MapPosition { x: 1, y: 6 }, MapPosition { x: 1, y: 6 }),
        (MapPosition { x: 9, y: 2 }, MapPosition { x: 7, y: 2 }),
        (MapPosition { x: 0, y: 0 }, MapPosition { x: 0, y: 0 }),
    ]
    .into_iter()
    .for_each(|(position, expected_origin)| {
        let mut viewport = viewport_at(MapPosition { x: 7, y: 6 });

        viewport.keep_in_view(position);

        assert_eq!(expected_origin, viewport.origin(), "{position:?}");
    });
}

#[test]
fn should_not_pan_past_the_edges_of_the_map_to_keep_a_position_in_view() {
    let mut viewport = viewport_at(MapPosition { x: 3, y: 2 });

    viewport.keep_in_view(MapPosition { x: 25, y: 30 });

    assert_eq!(MapPosition { x: 15, y: 6 }, viewport.origin());
}

fn viewport_at(origin: MapPosition) -> Viewport {
    let mut viewport = Viewport::new(map_size(20, 10), 5, 4);
    (0..origin.x).for_each(|_| viewport.pan(PanDirection::Right));
    (0..origin.y).for_each(|_| viewport.pan(PanDirection::Down));
    viewport
}

fn map_size(width: u32, height: u32) -> MapSize {
    MapSize::new(width, height).unwrap()
}
