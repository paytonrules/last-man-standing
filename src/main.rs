use bevy::prelude::*;

mod player;

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
