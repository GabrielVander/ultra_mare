use std::time::Duration;

use app_ascii::{
    ControlHint, Glyph, KeyRepeat, LegendEntry, PanDirection, Rgb, ScreenLayout, Viewport,
    generate_world, present_legend, present_map,
};
use bevy::{prelude::*, window::PrimaryWindow};
use lib_world_generation::{MapPosition, WorldMap};

const LAYOUT: ScreenLayout = ScreenLayout {
    tile_width: 14.0,
    tile_height: 18.0,
    legend_width: 220.0,
};
const GLYPH_FONT_SIZE: f32 = 16.0;
const LEGEND_FONT_SIZE: f32 = 16.0;
const LEGEND_BACKGROUND: Color = Color::srgb(0.08, 0.08, 0.1);
const LEGEND_TEXT: Color = Color::srgb(0.85, 0.85, 0.85);
const KEY_REPEAT_DELAY: Duration = Duration::from_millis(250);
const KEY_REPEAT_INTERVAL: Duration = Duration::from_millis(40);

const PAN_KEYS: [(PanDirection, [KeyCode; 2]); 4] = [
    (PanDirection::Left, [KeyCode::ArrowLeft, KeyCode::KeyA]),
    (PanDirection::Right, [KeyCode::ArrowRight, KeyCode::KeyD]),
    (PanDirection::Up, [KeyCode::ArrowUp, KeyCode::KeyW]),
    (PanDirection::Down, [KeyCode::ArrowDown, KeyCode::KeyS]),
];
const REGENERATE_KEY: KeyCode = KeyCode::KeyR;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ultra Mare".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_camera, spawn_legend, spawn_world))
        .add_systems(
            Update,
            (
                regenerate_world,
                fit_viewport_to_window,
                pan_viewport,
                aim_camera,
            )
                .chain(),
        )
        .run();
}

#[derive(Resource)]
struct Navigation {
    viewport: Viewport,
    pan_repeats: Vec<(PanDirection, KeyRepeat)>,
}

#[derive(Component)]
struct TileGlyph;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_legend(mut commands: Commands) {
    let legend = present_legend();

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(LAYOUT.legend_width),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(LEGEND_BACKGROUND),
        ))
        .with_children(|panel| {
            panel.spawn(legend_text("Terrain", LEGEND_TEXT));
            legend.terrains.iter().for_each(|entry| {
                spawn_terrain_entry(panel, entry);
            });
            panel.spawn(legend_text("\nControls", LEGEND_TEXT));
            legend.controls.iter().for_each(|control| {
                spawn_control_hint(panel, control);
            });
        });
}

fn spawn_terrain_entry(panel: &mut ChildSpawnerCommands, entry: &LegendEntry) {
    panel
        .spawn(legend_text(
            &format!("{}  ", entry.glyph.symbol),
            color_of(entry.glyph.color),
        ))
        .with_child(legend_span(&entry.label));
}

fn spawn_control_hint(panel: &mut ChildSpawnerCommands, control: &ControlHint) {
    panel
        .spawn(legend_text(&format!("{}  ", control.keys), LEGEND_TEXT))
        .with_child(legend_span(&control.action));
}

fn legend_text(text: &str, color: Color) -> impl Bundle {
    (
        Text::new(text),
        TextFont::from_font_size(LEGEND_FONT_SIZE),
        TextColor(color),
    )
}

fn legend_span(text: &str) -> impl Bundle {
    (
        TextSpan::new(text),
        TextFont::from_font_size(LEGEND_FONT_SIZE),
        TextColor(LEGEND_TEXT),
    )
}

fn spawn_world(mut commands: Commands) {
    let world = generate_world(rand::random());
    spawn_tiles(&mut commands, &world);
    commands.insert_resource(Navigation {
        viewport: Viewport::new(world.size(), 0, 0),
        pan_repeats: PAN_KEYS
            .iter()
            .map(|(direction, _)| (*direction, new_key_repeat()))
            .collect(),
    });
}

fn new_key_repeat() -> KeyRepeat {
    KeyRepeat::new(KEY_REPEAT_DELAY, KEY_REPEAT_INTERVAL)
        .expect("the key repeat interval is not zero")
}

fn spawn_tiles(commands: &mut Commands, world: &WorldMap) {
    let view_model = present_map(world);
    for (y, row) in (0..).zip(&view_model.rows) {
        for (x, glyph) in (0..).zip(row) {
            commands.spawn(tile_glyph(MapPosition { x, y }, *glyph));
        }
    }
}

fn tile_glyph(position: MapPosition, glyph: Glyph) -> impl Bundle {
    let (x, y) = LAYOUT.tile_translation(position);
    (
        TileGlyph,
        Text2d::new(glyph.symbol.to_string()),
        TextFont::from_font_size(GLYPH_FONT_SIZE),
        TextColor(color_of(glyph.color)),
        Transform::from_xyz(x, y, 0.0),
    )
}

fn color_of(rgb: Rgb) -> Color {
    Color::srgb_u8(rgb.red, rgb.green, rgb.blue)
}

fn regenerate_world(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    tiles: Query<Entity, With<TileGlyph>>,
) {
    if !keys.just_pressed(REGENERATE_KEY) {
        return;
    }
    tiles
        .iter()
        .for_each(|tile| commands.entity(tile).despawn());
    spawn_tiles(&mut commands, &generate_world(rand::random()));
}

fn fit_viewport_to_window(
    window: Single<&Window, With<PrimaryWindow>>,
    mut navigation: ResMut<Navigation>,
) {
    let (columns, rows) = LAYOUT.viewport_size(window.width(), window.height());
    navigation.viewport.resize(columns, rows);
}

fn pan_viewport(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut navigation: ResMut<Navigation>,
) {
    let Navigation {
        viewport,
        pan_repeats,
    } = navigation.as_mut();

    for ((direction, repeat), (_, bound_keys)) in pan_repeats.iter_mut().zip(PAN_KEYS) {
        // A tap released within the same frame is no longer pressed, yet must still move.
        let held = keys.any_pressed(bound_keys) || keys.any_just_pressed(bound_keys);
        repeat.tick(held, time.delta());
        (0..repeat.steps_due()).for_each(|_| viewport.pan(*direction));
    }
}

fn aim_camera(
    window: Single<&Window, With<PrimaryWindow>>,
    navigation: Res<Navigation>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    let (x, y) = LAYOUT.camera_translation(
        navigation.viewport.origin(),
        window.width(),
        window.height(),
    );
    camera.translation.x = x;
    camera.translation.y = y;
}
