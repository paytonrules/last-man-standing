use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Player {
    direction: Vec2, // Not sure these are the structs should use (and should this store the
}

// TODO: Player Sprite query is going to get every transform eventually. You should probably tag
// the player sprite sheet
pub fn move_player(
    player: ResMut<Player>,
    mut player_sprite: Query<(&mut Transform, &Handle<Image>)>,
) {
    let (mut transform, _image) = player_sprite.single_mut();
    transform.translation.x += player.direction.x * 3.0;
    transform.translation.y += player.direction.y * 3.0;
}

pub fn update_player_direction(keys: Res<ButtonInput<KeyCode>>, mut player: ResMut<Player>) {
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
