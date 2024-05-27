use bevy::prelude::*;

#[derive(Resource)]
struct Player {
    direction: Vec2, // Not sure these are the structs should use (and should this store the
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc, update_keys))
        .add_systems(FixedUpdate, move_player)
        .run();
}

fn update_keys(keys: Res<ButtonInput<KeyCode>>, mut player: ResMut<Player>) {
    player.direction.x = 0.0;
    player.direction.y = 0.0;
    if keys.pressed(KeyCode::KeyW) {
        player.direction.y += 1.0;
    }

    if keys.pressed(KeyCode::KeyS) {
        player.direction.y += -1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        player.direction.x += -1.0;
    }

    if keys.pressed(KeyCode::KeyD) {
        player.direction.x += 1.0;
    }
}

// TODO: Player Sprite query is going to get every transform eventually. You should probably tag
// the player sprite sheet
fn move_player(player: ResMut<Player>, mut player_sprite: Query<(&mut Transform, &Handle<Image>)>) {
    let (mut transform, _image) = player_sprite.single_mut();
    transform.translation.x += player.direction.x * 3.0;
    transform.translation.y += player.direction.y * 3.0;
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Player {
        direction: Vec2::default(),
    });

    let texture = asset_server.load("textures/tilemap.png");

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(16.0, 16.0),
        12,
        11,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: 84,
        },
        transform: Transform::from_scale(Vec3::splat(3.0)),
        ..default()
    });
}
