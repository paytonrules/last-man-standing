use bevy::prelude::*;
use rand::{thread_rng, Rng};

mod player;

const PLAYER_INDEX: usize = 84;
const ENEMY_INDICES: [usize; 14] = [85, 86, 87, 88, 96, 97, 98, 99, 100, 108, 109, 110, 111, 112];
const MIN_DISTANCE_TO_ENEMY: f32 = 140.0;
const MAX_DISTANCE_TO_ENEMY: f32 = 550.0;
const ENEMY_SPEED: f32 = 2.9;

#[derive(Component, Default)]
struct Enemy {
    destination: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (player::move_player, move_enemies, check_collisions),
        )
        .add_systems(
            Update,
            (player::update_player_direction, bevy::window::close_on_esc),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

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
    let window = window_query.single();
    spawn_enemies(
        sprite_template.clone(),
        &player_transform,
        &window,
        &mut commands,
    );
}

// This is doing a bunch of calculations every frame. You could make this on an event and just use
// components you update on change
fn check_collisions(
    mut commands: Commands,
    mut player: Query<(Entity, &player::Player, &mut Transform, &Handle<Image>)>,
    enemies: Query<(Entity, &Transform, &Handle<Image>), Without<player::Player>>,
) {
    let Ok((player_entity, _player, mut transform, _image)) = player.get_single_mut() else {
        return;
    };

    let scaled = 16.0 * transform.scale.truncate();
    let player_rect = Rect::from_center_size(transform.translation.truncate(), scaled);

    for (entity, enemy_transform, _image) in enemies.iter() {
        let scaled = 16.0 * enemy_transform.scale.truncate();
        let enemy_rect = Rect::from_center_size(enemy_transform.translation.truncate(), scaled);

        if !player_rect.intersect(enemy_rect).is_empty() {
            if transform.scale.length_squared() > enemy_transform.scale.length_squared() {
                commands.entity(entity).despawn();
                transform.scale += 0.2;
            } else {
                commands.entity(player_entity).despawn();
            }
        }
    }
}

fn spawn_player(mut bundle: SpriteSheetBundle, commands: &mut Commands) {
    bundle.atlas.index = PLAYER_INDEX;
    commands.spawn((player::Player::default(), bundle));
}

fn spawn_enemies(
    bundle: SpriteSheetBundle,
    player_transform: &Transform,
    window: &Window,
    commands: &mut Commands,
) {
    let mut rng = thread_rng();
    for _ in 0..10 {
        let mut bundle = bundle.clone();
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

        bundle.atlas.index = ENEMY_INDICES[rng.gen_range(0..ENEMY_INDICES.len())];
        bundle.transform = player_transform
            .with_scale(player_transform.scale * rng.gen_range(0.8..1.2))
            .with_translation(player_transform.translation + direction);

        commands.spawn((
            Enemy {
                destination: random_destination(window),
            },
            bundle,
        ));
    }
}

fn move_enemies(mut enemies: Query<(&mut Enemy, &mut Transform)>, window_query: Query<&Window>) {
    let window = window_query.single();
    enemies.iter_mut().for_each(|(mut enemy, mut transform)| {
        // Give it a fudge factor
        if transform
            .translation
            .xy()
            .distance_squared(enemy.destination)
            < 4.0
        {
            enemy.destination = random_destination(window);
        } else {
            let direction =
                (enemy.destination - transform.translation.xy()).normalize() * ENEMY_SPEED;
            transform.translation.x += direction.x;
            transform.translation.y += direction.y;
        }
    });
}

fn random_destination(window: &Window) -> Vec2 {
    let mut rng = thread_rng();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;
    Vec2 {
        x: rng.gen_range(-half_width..half_width),
        y: rng.gen_range(-half_height..half_height),
    }
}
