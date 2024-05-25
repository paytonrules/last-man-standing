use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

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
        ..default()
    });
}
