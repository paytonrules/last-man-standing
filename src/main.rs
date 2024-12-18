use bevy::{asset::AssetMetaCheck, prelude::*};
use player::Player;
use rand::{thread_rng, Rng};

mod enemies;
mod player;

const PLAYER_INDEX: usize = 84;
const ENEMY_INDICES: [usize; 14] = [85, 86, 87, 88, 96, 97, 98, 99, 100, 108, 109, 110, 111, 112];
const MIN_DISTANCE_TO_ENEMY: f32 = 140.0;
const MAX_DISTANCE_TO_ENEMY: f32 = 550.0;
const PLAYER_STARTING_TRANSFORM: Transform = Transform::from_scale(Vec3::splat(3.0));

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameStates {
    #[default]
    Running,
    Dead,
}

#[derive(Event, Default)]
struct Spawn;

#[derive(Event)]
struct ZoomOut;

#[derive(Resource)]
struct GlobalLayoutResource(Handle<TextureAtlasLayout>);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#last-giant-standing".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .init_state::<GameStates>()
        .add_event::<player::Restart>()
        .add_event::<Spawn>()
        .add_event::<ZoomOut>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                player::move_player,
                enemies::move_enemies,
                check_collisions,
                player::animate_growth,
                expand_camera,
            )
                .run_if(in_state(GameStates::Running)),
        )
        .add_systems(
            Update,
            (
                spawn_player,
                spawn_enemies,
                spawn_enemies_timer,
                (player::update_player_direction).run_if(in_state(GameStates::Running)),
                (player::listen_for_restart_button, restart_game)
                    .run_if(in_state(GameStates::Dead)),
            ),
        )
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawn_event: EventWriter<Spawn>,
    mut commands: Commands,
) {
    commands.spawn(Camera2d::default());

    // Setup TextureAtlasLayout once and store it in a resource
    let layout =
        TextureAtlasLayout::from_grid(UVec2::new(16, 16), 12, 11, Some(UVec2::new(1, 1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.insert_resource(GlobalLayoutResource(texture_atlas_layout));

    let starbase: Handle<Image> = asset_server.load("textures/Starbasesnow.png");

    commands.spawn((
        Sprite::from_image(starbase),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
    spawn_event.send_default();
}

// This is doing a bunch of calculations every frame. You could make this on an event and just use
// components you update on change
fn check_collisions(
    mut commands: Commands,
    player: Query<(Entity, &player::Player, &mut Transform)>,
    enemies: Query<(Entity, &Transform, &enemies::Enemy), Without<player::Player>>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut zoom_out_event: EventWriter<ZoomOut>,
) {
    if enemies.iter().len() <= 0 {
        return;
    }

    let mut all_enemies_deleted = true;
    for (player_entity, _player, transform) in player.iter() {
        let scaled = 16.0 * transform.scale.truncate();
        let player_rect = Rect::from_center_size(transform.translation.truncate(), scaled);

        for (entity, enemy_transform, _enemy_component) in enemies.iter() {
            let scaled = 16.0 * enemy_transform.scale.truncate();
            let enemy_rect = Rect::from_center_size(enemy_transform.translation.truncate(), scaled);

            if !player_rect.intersect(enemy_rect).is_empty() {
                if transform.scale.length_squared() > enemy_transform.scale.length_squared() {
                    commands.entity(entity).despawn();

                    commands.entity(player_entity).insert(player::Tween {
                        destination_scale: transform.scale + 0.4,
                        step_value: Vec3::splat(0.08),
                    });
                } else {
                    commands.entity(player_entity).despawn();
                    next_state.set(GameStates::Dead);
                }
            } else {
                all_enemies_deleted = false;
            }
        }
    }

    if all_enemies_deleted {
        zoom_out_event.send(ZoomOut);
        // Pretty hacky but should work
        commands.spawn(enemies::SpawnEnemiesTimer::default());
    }
}

fn spawn_player(
    texture_layout: Res<GlobalLayoutResource>,
    asset_server: Res<AssetServer>,
    mut spawn_event: EventReader<Spawn>,
    mut commands: Commands,
) {
    let texture: Handle<Image> = asset_server.load("textures/tilemap.png");
    let texture_layout = texture_layout.0.clone();

    for _event in spawn_event.read() {
        commands.spawn((
            player::Player::default(),
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    index: PLAYER_INDEX,
                    layout: texture_layout.clone(),
                },
            ),
            PLAYER_STARTING_TRANSFORM.clone(),
        ));
    }
}

fn spawn_enemies(mut restart_event: EventReader<Spawn>, mut commands: Commands) {
    for _event in restart_event.read() {
        commands.spawn(enemies::SpawnEnemiesTimer::default());
    }
}

fn spawn_enemies_timer(
    texture_layout: Res<GlobalLayoutResource>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    window_query: Query<&Window>,
    players: Query<&Transform, With<player::Player>>,
    mut spawn_enemies_timer: Query<(Entity, &mut enemies::SpawnEnemiesTimer)>,
    mut commands: Commands,
) {
    let texture: Handle<Image> = asset_server.load("textures/tilemap.png");

    for (entity, mut spawner) in spawn_enemies_timer.iter_mut() {
        let window = window_query.single();
        let player_transform = players.single(); // NOTE: This can crash when the player dies at
                                                 // the right time.
        spawner.timer.tick(time.delta());

        if spawner.timer.finished() {
            let mut rng = thread_rng();
            for _ in 0..10 {
                let mut direction = None;
                while direction.is_none() {
                    direction = Vec3 {
                        x: rng.gen_range(-1.0..1.0),
                        y: rng.gen_range(-1.0..1.0),
                        z: 0.0,
                    }
                    .try_normalize();
                }
                let direction = direction.unwrap()
                    * rng.gen_range(MIN_DISTANCE_TO_ENEMY..MAX_DISTANCE_TO_ENEMY);

                let enemy_index = ENEMY_INDICES[rng.gen_range(0..ENEMY_INDICES.len())];
                commands.spawn((
                    enemies::Enemy {
                        destination: enemies::random_destination(window),
                    },
                    Sprite::from_atlas_image(
                        texture.clone(),
                        TextureAtlas {
                            index: enemy_index,
                            layout: texture_layout.0.clone(),
                        },
                    ),
                    player_transform
                        .with_scale(player_transform.scale * rng.gen_range(0.6..1.4))
                        .with_translation(player_transform.translation + direction),
                ));
            }
            commands.entity(entity).despawn();
        }
    }
}

fn restart_game(
    entities: Query<Entity, Or<(With<player::Player>, With<enemies::Enemy>)>>,
    mut camera_query: Query<(&Camera, &mut OrthographicProjection)>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut restart_event: EventReader<player::Restart>,
    mut spawn_event: EventWriter<Spawn>,
    mut commands: Commands,
) {
    for _event in restart_event.read() {
        entities.iter().for_each(|entity| {
            commands.entity(entity).despawn();
        });

        let (_camera, mut projection) = camera_query.single_mut();
        projection.scale = 1.0;

        spawn_event.send_default();

        next_state.set(GameStates::Running);
    }
}

fn expand_camera(
    mut zoom_out_event: EventReader<ZoomOut>,
    mut camera_query: Query<(&Camera, &mut OrthographicProjection)>,
    mut players: Query<&mut Player>,
) {
    for _event in zoom_out_event.read() {
        let (_camera, mut projection) = camera_query.single_mut();
        projection.scale *= 1.3;
        for mut player in players.iter_mut() {
            player.speed *= 1.2;
        }
    }
}
