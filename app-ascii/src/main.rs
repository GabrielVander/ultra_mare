use std::time::Duration;

use app_ascii::{
    ControlHint, Glyph, KeyRepeat, LegendEntry, PanDirection, Rgb, ScreenLayout, Viewport,
    WorldTerrain, generate_world, map_position, present_legend, present_map, unit_glyph,
};
use bevy::{prelude::*, window::PrimaryWindow};
use lib_unit::{Direction, Unit, UnitType};
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
const UNIT_BACKGROUND: Color = Color::srgb(0.55, 0.12, 0.12);
const KEY_REPEAT_DELAY: Duration = Duration::from_millis(250);
const KEY_REPEAT_INTERVAL: Duration = Duration::from_millis(40);

const PAN_KEYS: [(PanDirection, KeyCode); 4] = [
    (PanDirection::Left, KeyCode::ArrowLeft),
    (PanDirection::Right, KeyCode::ArrowRight),
    (PanDirection::Up, KeyCode::ArrowUp),
    (PanDirection::Down, KeyCode::ArrowDown),
];
const MOVE_KEYS: [(Direction, KeyCode); 4] = [
    (Direction::North, KeyCode::KeyW),
    (Direction::West, KeyCode::KeyA),
    (Direction::South, KeyCode::KeyS),
    (Direction::East, KeyCode::KeyD),
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
                move_colonist,
                follow_colonist,
                pan_viewport,
                place_colonist,
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

#[derive(Resource)]
struct World(WorldMap);

/// `None` when the map has no land to put the colonist on.
#[derive(Resource)]
struct Colonist(Option<Unit>);

#[derive(Resource)]
struct ColonistOrders {
    move_repeats: Vec<(Direction, KeyRepeat)>,
}

#[derive(Component)]
struct MapGlyph;

#[derive(Component)]
struct ColonistGlyph;

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
                spawn_legend_entry(panel, entry);
            });
            panel.spawn(legend_text("\nUnits", LEGEND_TEXT));
            legend.units.iter().for_each(|entry| {
                spawn_legend_entry(panel, entry);
            });
            panel.spawn(legend_text("\nControls", LEGEND_TEXT));
            legend.controls.iter().for_each(|control| {
                spawn_control_hint(panel, control);
            });
        });
}

fn spawn_legend_entry(panel: &mut ChildSpawnerCommands, entry: &LegendEntry) {
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
    commands.insert_resource(Navigation {
        viewport: Viewport::new(world.size(), 0, 0),
        pan_repeats: PAN_KEYS
            .iter()
            .map(|(direction, _)| (*direction, new_key_repeat()))
            .collect(),
    });
    commands.insert_resource(ColonistOrders {
        move_repeats: MOVE_KEYS
            .iter()
            .map(|(direction, _)| (*direction, new_key_repeat()))
            .collect(),
    });
    populate_world(&mut commands, world);
}

fn new_key_repeat() -> KeyRepeat {
    KeyRepeat::new(KEY_REPEAT_DELAY, KEY_REPEAT_INTERVAL)
        .expect("the key repeat interval is not zero")
}

fn populate_world(commands: &mut Commands, world: WorldMap) {
    spawn_tiles(commands, &world);
    let colonist = Unit::land(UnitType::FreeColonist, WorldTerrain::new(&world).tiles());
    if let Some(unit) = &colonist {
        commands.spawn(colonist_glyph(unit));
    }
    commands.insert_resource(Colonist(colonist));
    commands.insert_resource(World(world));
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
        MapGlyph,
        Text2d::new(glyph.symbol.to_string()),
        TextFont::from_font_size(GLYPH_FONT_SIZE),
        TextColor(color_of(glyph.color)),
        Transform::from_xyz(x, y, 0.0),
    )
}

/// A coloured square hiding the terrain glyph beneath, with the unit's glyph on top.
fn colonist_glyph(unit: &Unit) -> impl Bundle {
    let glyph = unit_glyph(unit.unit_type());
    let (x, y) = LAYOUT.tile_translation(map_position(unit.position()));
    (
        MapGlyph,
        ColonistGlyph,
        Sprite::from_color(
            UNIT_BACKGROUND,
            Vec2::new(LAYOUT.tile_width, LAYOUT.tile_height),
        ),
        Transform::from_xyz(x, y, 1.0),
        children![(
            Text2d::new(glyph.symbol.to_string()),
            TextFont::from_font_size(GLYPH_FONT_SIZE),
            TextColor(color_of(glyph.color)),
            Transform::from_xyz(0.0, 0.0, 1.0),
        )],
    )
}

fn color_of(rgb: Rgb) -> Color {
    Color::srgb_u8(rgb.red, rgb.green, rgb.blue)
}

fn regenerate_world(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    glyphs: Query<Entity, With<MapGlyph>>,
) {
    if !keys.just_pressed(REGENERATE_KEY) {
        return;
    }
    glyphs
        .iter()
        .for_each(|glyph| commands.entity(glyph).despawn());
    populate_world(&mut commands, generate_world(rand::random()));
}

fn fit_viewport_to_window(
    window: Single<&Window, With<PrimaryWindow>>,
    mut navigation: ResMut<Navigation>,
) {
    let (columns, rows) = LAYOUT.viewport_size(window.width(), window.height());
    navigation.viewport.resize(columns, rows);
}

fn move_colonist(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    world: Res<World>,
    mut orders: ResMut<ColonistOrders>,
    mut colonist: ResMut<Colonist>,
) {
    let terrain = WorldTerrain::new(&world.0);
    for ((direction, repeat), (_, key)) in orders.move_repeats.iter_mut().zip(MOVE_KEYS) {
        repeat.tick(is_held(&keys, key), time.delta());
        for _ in 0..repeat.steps_due() {
            // Touching the colonist only when a step is due keeps change detection meaningful
            // for following it. A blocked step just leaves it where it stands.
            if let Some(unit) = colonist.0.as_mut() {
                let _ = unit.move_toward(*direction, &terrain);
            }
        }
    }
}

fn follow_colonist(colonist: Res<Colonist>, mut navigation: ResMut<Navigation>) {
    if !colonist.is_changed() {
        return;
    }
    if let Some(unit) = &colonist.0 {
        navigation
            .viewport
            .keep_in_view(map_position(unit.position()));
    }
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

    for ((direction, repeat), (_, key)) in pan_repeats.iter_mut().zip(PAN_KEYS) {
        repeat.tick(is_held(&keys, key), time.delta());
        (0..repeat.steps_due()).for_each(|_| viewport.pan(*direction));
    }
}

fn is_held(keys: &ButtonInput<KeyCode>, key: KeyCode) -> bool {
    // A tap released within the same frame is no longer pressed, yet must still move.
    keys.pressed(key) || keys.just_pressed(key)
}

fn place_colonist(colonist: Res<Colonist>, mut glyph: Query<&mut Transform, With<ColonistGlyph>>) {
    let (Some(unit), Ok(mut transform)) = (&colonist.0, glyph.single_mut()) else {
        return;
    };
    let (x, y) = LAYOUT.tile_translation(map_position(unit.position()));
    transform.translation.x = x;
    transform.translation.y = y;
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
