use bevy::prelude::*;

const MAP_WIDTH: i32 = 100;
const MAP_HEIGHT: i32 = 80;
const TILE_SIZE: f32 = 16.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_world)
        .add_systems(Update, move_camera)
        .run();
}

#[derive(Component)]
struct MainCamera {
    panning_speed: f32,
}

fn setup_world(mut commands: Commands) {
    let right_edge_x = (MAP_WIDTH / 2) as f32 * TILE_SIZE;
    let start_x = rand::random_range((right_edge_x * 0.8)..right_edge_x);
    let start_y = rand::random_range(
        -(MAP_HEIGHT as f32 * TILE_SIZE / 4.0)..(MAP_HEIGHT as f32 * TILE_SIZE / 4.0),
    );

    commands.spawn((
        Camera2d,
        MainCamera {
            panning_speed: 25.0,
        },
        Transform::from_xyz(start_x, start_y, 10.0),
    ));

    for y in -MAP_HEIGHT / 2..MAP_HEIGHT / 2 {
        for x in -MAP_WIDTH / 2..MAP_WIDTH / 2 {
            let normalized_x = (x as f32 + (MAP_WIDTH / 2) as f32) / MAP_WIDTH as f32;

            let land_probability = (1.2 - (normalized_x * 1.5)).clamp(0.0, 1.0);
            let is_land = rand::random_bool(land_probability as f64);

            let (character, color) = if is_land {
                let terrain_type: f32 = rand::random::<f32>();
                if terrain_type < 0.1 {
                    ("#", Color::srgb(0.6, 0.6, 0.6)) // Mountain
                } else if terrain_type < 0.3 {
                    ("^", Color::srgb(0.6, 0.5, 0.3)) // Hills
                } else if terrain_type < 0.6 {
                    ("T", Color::srgb(0.1, 0.6, 0.2)) // Forest
                } else if terrain_type < 0.8 {
                    ("@", Color::srgb(0.0, 0.4, 0.1)) // Jungle
                } else {
                    (",", Color::srgb(0.5, 0.8, 0.3)) // Plains
                }
            } else {
                (".", Color::srgb(0.2, 0.4, 0.8)) // Sea
            };

            commands.spawn((
                Text2d::new(character),
                TextFont {
                    font_size: TILE_SIZE,
                    ..default()
                },
                TextColor(color),
                Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
            ));
        }
    }
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut main_camera_query: Query<(&mut Transform, &MainCamera)>,
    time: Res<Time>,
) {
    let (mut camera_transform, main_camera) = main_camera_query.single_mut().unwrap();
    let speed = main_camera.panning_speed * time.delta_secs() * TILE_SIZE;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        camera_transform.translation.x -= speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        camera_transform.translation.x += speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        camera_transform.translation.y += speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        camera_transform.translation.y -= speed;
    }
}
