use bevy::prelude::*;
use rand::{random, thread_rng, Rng};

mod player;

const PLAYER_INDEX: usize = 84;
const ENEMY_INDICES: [usize; 14] = [85, 86, 87, 88, 96, 97, 98, 99, 100, 108, 109, 110, 111, 112];
const MIN_DISTANCE_TO_ENEMY: f32 = 140.0;
const MAX_DISTANCE_TO_ENEMY: f32 = 550.0;

#[derive(Component)]
struct EnemyTag;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (bevy::window::close_on_esc, player::update_player_direction),
        )
        .add_systems(FixedUpdate, player::move_player)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(player::Player::default());

    let texture: Handle<Image> = asset_server.load("textures/tilemap.png");

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(16.0, 16.0),
        12,
        11,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_transform = Transform::from_scale(Vec3::splat(3.0));
    let sprite_template = SpriteSheetBundle {
        texture: texture.clone(),
        atlas: TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        },
        transform: player_transform,
        ..default()
    };

    spawn_player(sprite_template.clone(), &mut commands);
    spawn_enemies(sprite_template.clone(), &player_transform, &mut commands);
}

fn spawn_player(mut bundle: SpriteSheetBundle, commands: &mut Commands) {
    bundle.atlas.index = PLAYER_INDEX;
    commands.spawn((player::PlayerTag, bundle));
}

fn spawn_enemies(
    mut bundle: SpriteSheetBundle,
    player_transform: &Transform,
    commands: &mut Commands,
) {
    let mut rng = thread_rng();
    let length = Vec3 {
        x: rng.gen_range(MIN_DISTANCE_TO_ENEMY..MAX_DISTANCE_TO_ENEMY),
        y: 0.0,
        z: 0.0,
    };
    let mut direction = None;
    while direction.is_none() {
        direction = Vec3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: 0.0,
        }
        .try_normalize();
    }
    let direction =
        direction.unwrap() * rng.gen_range(MIN_DISTANCE_TO_ENEMY..MAX_DISTANCE_TO_ENEMY);

    bundle.atlas.index = 85;
    bundle.transform = player_transform
        .with_scale(Vec3::splat(2.0))
        .with_translation(direction);

    commands.spawn((EnemyTag, bundle));
}
