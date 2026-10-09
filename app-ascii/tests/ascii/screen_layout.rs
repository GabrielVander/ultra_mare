use app_ascii::ScreenLayout;
use lib_world_generation::MapPosition;

const LAYOUT: ScreenLayout = ScreenLayout {
    tile_width: 16.0,
    tile_height: 20.0,
    legend_width: 200.0,
};

#[test]
fn should_fit_whole_tiles_left_of_the_legend() {
    [
        ((1000.0, 600.0), (50, 30)),
        ((1015.0, 619.0), (50, 30)),
        ((1016.0, 620.0), (51, 31)),
    ]
    .into_iter()
    .for_each(|((window_width, window_height), expected)| {
        assert_eq!(
            expected,
            LAYOUT.viewport_size(window_width, window_height),
            "{window_width}x{window_height}"
        )
    });
}

#[test]
fn should_fit_no_tiles_when_the_legend_fills_the_window() {
    assert_eq!((0, 30), LAYOUT.viewport_size(150.0, 600.0));
}

#[test]
fn should_lay_tiles_out_left_to_right_and_top_to_bottom() {
    let (x, y) = LAYOUT.tile_translation(MapPosition { x: 0, y: 0 });
    let (right_x, right_y) = LAYOUT.tile_translation(MapPosition { x: 1, y: 0 });
    let (below_x, below_y) = LAYOUT.tile_translation(MapPosition { x: 0, y: 1 });

    assert_eq!((x + 16.0, y), (right_x, right_y));
    assert_eq!((x, y - 20.0), (below_x, below_y));
}

#[test]
fn should_aim_the_camera_so_the_viewport_origin_fills_the_top_left_corner() {
    let (window_width, window_height) = (1000.0, 600.0);
    [MapPosition { x: 0, y: 0 }, MapPosition { x: 7, y: 3 }]
        .into_iter()
        .for_each(|origin| {
            let (tile_x, tile_y) = LAYOUT.tile_translation(origin);
            let (camera_x, camera_y) =
                LAYOUT.camera_translation(origin, window_width, window_height);

            let top_left_corner = (
                camera_x - window_width / 2.0,
                camera_y + window_height / 2.0,
            );
            assert_eq!((tile_x - 8.0, tile_y + 10.0), top_left_corner, "{origin:?}");
        });
}
