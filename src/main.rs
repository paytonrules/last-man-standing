use bevy::prelude::*;
use rand::{thread_rng, Rng};

mod player;

const PLAYER_INDEX: usize = 84;
const ENEMY_INDICES: [usize; 14] = [85, 86, 87, 88, 96, 97, 98, 99, 100, 108, 109, 110, 111, 112];
const MIN_DISTANCE_TO_ENEMY: f32 = 140.0;
const MAX_DISTANCE_TO_ENEMY: f32 = 550.0;
const ENEMY_SPEED: f32 = 2.9;
const PLAYER_STARTING_TRANSFORM: Transform = Transform::from_scale(Vec3::splat(3.0));

#[derive(Component, Default)]
struct Enemy {
    destination: Vec2,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameStates {
    #[default]
    Running,
    Dead,
}

#[derive(Resource)]
struct SpriteSheetTemplate(SpriteSheetBundle);

#[derive(Event)]
struct SpawnEnemies;

fn main() {
    App::new()
        .init_state::<GameStates>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_event::<player::Restart>()
        .add_event::<player::Spawn>()
        .add_event::<SpawnEnemies>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (player::move_player, move_enemies, check_collisions)
                .run_if(in_state(GameStates::Running)),
        )
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                spawn_player,
                spawn_enemies,
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
    mut spawn_player_event: EventWriter<player::Spawn>,
    mut spawn_enemies_event: EventWriter<SpawnEnemies>,
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

    spawn_characters(&mut spawn_player_event, &mut spawn_enemies_event);
}

// This is doing a bunch of calculations every frame. You could make this on an event and just use
// components you update on change
fn check_collisions(
    mut commands: Commands,
    mut player: Query<(Entity, &player::Player, &mut Transform)>,
    enemies: Query<(Entity, &Transform, &Enemy), Without<player::Player>>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    let Ok((player_entity, _player, mut transform)) = player.get_single_mut() else {
        return;
    };

    let scaled = 16.0 * transform.scale.truncate();
    let player_rect = Rect::from_center_size(transform.translation.truncate(), scaled);

    enemies
        .iter()
        .for_each(|(entity, enemy_transform, _enemy_component)| {
            let scaled = 16.0 * enemy_transform.scale.truncate();
            let enemy_rect = Rect::from_center_size(enemy_transform.translation.truncate(), scaled);

            if !player_rect.intersect(enemy_rect).is_empty() {
                if transform.scale.length_squared() > enemy_transform.scale.length_squared() {
                    commands.entity(entity).despawn();
                    transform.scale += 0.2;
                } else {
                    commands.entity(player_entity).despawn();
                    next_state.set(GameStates::Dead);
                }
            }
        });
}

fn spawn_player(
    template: Res<SpriteSheetTemplate>,
    mut spawn_event: EventReader<player::Spawn>,
    mut commands: Commands,
) {
    for _event in spawn_event.read() {
        let mut bundle = template.0.clone();
        bundle.atlas.index = PLAYER_INDEX;
        bundle.transform = PLAYER_STARTING_TRANSFORM;
        commands.spawn((player::Player::default(), bundle));
    }
}

fn spawn_enemies(
    template: Res<SpriteSheetTemplate>,
    window_query: Query<&Window>,
    mut restart_event: EventReader<SpawnEnemies>,
    mut commands: Commands,
) {
    let window = window_query.single();

    for _event in restart_event.read() {
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
            let direction =
                direction.unwrap() * rng.gen_range(MIN_DISTANCE_TO_ENEMY..MAX_DISTANCE_TO_ENEMY);

            bundle.atlas.index = ENEMY_INDICES[rng.gen_range(0..ENEMY_INDICES.len())];
            bundle.transform = PLAYER_STARTING_TRANSFORM
                .with_scale(PLAYER_STARTING_TRANSFORM.scale * rng.gen_range(0.8..1.2))
                .with_translation(PLAYER_STARTING_TRANSFORM.translation + direction);

            commands.spawn((
                Enemy {
                    destination: random_destination(window),
                },
                bundle,
            ));
        }
    }
}

fn restart_game(
    entities: Query<Entity, Or<(With<player::Player>, With<Enemy>)>>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut restart_event: EventReader<player::Restart>,
    mut spawn_player_event: EventWriter<player::Spawn>,
    mut spawn_enemies_event: EventWriter<SpawnEnemies>,
    mut commands: Commands,
) {
    for _event in restart_event.read() {
        entities.iter().for_each(|entity| {
            commands.entity(entity).despawn();
        });

        // Extract method here
        spawn_characters(&mut spawn_player_event, &mut spawn_enemies_event);

        next_state.set(GameStates::Running);
    }
}

// TODO: See if you can put these on timers
fn spawn_characters(
    spawn_player_event: &mut EventWriter<player::Spawn>,
    spawn_enemies_event: &mut EventWriter<SpawnEnemies>,
) {
    spawn_player_event.send(player::Spawn);
    spawn_enemies_event.send(SpawnEnemies);
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
