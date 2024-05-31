use bevy::prelude::*;
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

#[derive(Resource)]
struct SpriteSheetTemplate(SpriteSheetBundle);

#[derive(Event, Default)]
struct Spawn;

#[derive(Event)]
struct ZoomOut;

fn main() {
    App::new()
        .init_state::<GameStates>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
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
                bevy::window::close_on_esc,
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

    let sprite_template = SpriteSheetBundle {
        texture: texture.clone(),
        atlas: TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        },
        ..default()
    };
    commands.insert_resource(SpriteSheetTemplate(sprite_template.clone()));

    spawn_event.send_default();
}

// This is doing a bunch of calculations every frame. You could make this on an event and just use
// components you update on change
fn check_collisions(
    mut commands: Commands,
    mut player: Query<(Entity, &player::Player, &mut Transform)>,
    enemies: Query<(Entity, &Transform, &enemies::Enemy), Without<player::Player>>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut zoom_out_event: EventWriter<ZoomOut>,
) {
    if enemies.iter().len() <= 0 {
        return;
    }

    let mut all_enemies_deleted = true;
    for (player_entity, _player, transform) in player.iter_mut() {
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
    template: Res<SpriteSheetTemplate>,
    mut spawn_event: EventReader<Spawn>,
    mut commands: Commands,
) {
    for _event in spawn_event.read() {
        let mut bundle = template.0.clone();
        bundle.atlas.index = PLAYER_INDEX;
        bundle.transform = PLAYER_STARTING_TRANSFORM;
        commands.spawn((player::Player::default(), bundle));
    }
}

fn spawn_enemies(mut restart_event: EventReader<Spawn>, mut commands: Commands) {
    for _event in restart_event.read() {
        commands.spawn(enemies::SpawnEnemiesTimer::default());
    }
}

fn spawn_enemies_timer(
    template: Res<SpriteSheetTemplate>,
    window_query: Query<&Window>,
    time: Res<Time>,
    mut spawn_enemies_timer: Query<(Entity, &mut enemies::SpawnEnemiesTimer)>,
    mut commands: Commands,
) {
    let window = window_query.single();

    for (entity, mut spawner) in spawn_enemies_timer.iter_mut() {
        spawner.timer.tick(time.delta());

        if spawner.timer.finished() {
            let bundle = &template.0;
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
                let direction = direction.unwrap()
                    * rng.gen_range(MIN_DISTANCE_TO_ENEMY..MAX_DISTANCE_TO_ENEMY);

                bundle.atlas.index = ENEMY_INDICES[rng.gen_range(0..ENEMY_INDICES.len())];
                bundle.transform = PLAYER_STARTING_TRANSFORM
                    .with_scale(PLAYER_STARTING_TRANSFORM.scale * rng.gen_range(0.8..1.2))
                    .with_translation(PLAYER_STARTING_TRANSFORM.translation + direction);

                commands.spawn((
                    enemies::Enemy {
                        destination: enemies::random_destination(window),
                    },
                    bundle,
                ));
            }
            commands.entity(entity).despawn();
        }
    }
}

fn restart_game(
    entities: Query<Entity, Or<(With<player::Player>, With<enemies::Enemy>)>>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut restart_event: EventReader<player::Restart>,
    mut spawn_event: EventWriter<Spawn>,
    mut commands: Commands,
) {
    for _event in restart_event.read() {
        entities.iter().for_each(|entity| {
            commands.entity(entity).despawn();
        });

        spawn_event.send_default();

        next_state.set(GameStates::Running);
    }
}

fn expand_camera(
    mut zoom_out_event: EventReader<ZoomOut>,
    mut camera_query: Query<(&Camera, &mut OrthographicProjection)>,
) {
    for _event in zoom_out_event.read() {
        let (_camera, mut projection) = camera_query.single_mut();
        projection.scale *= 1.3;
    }
}
