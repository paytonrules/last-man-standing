use bevy::prelude::*;

const PLAYER_SPEED: f32 = 3.0;

#[derive(Component, Default)]
pub struct Player {
    direction: Vec2,
}

#[derive(Event)]
pub struct Restart;

// TODO Should you just make these loops?
// TODO: Player Sprite query is going to get every transform eventually. You should probably tag
// the player sprite sheet
pub fn move_player(mut player_query: Query<(&Player, &mut Transform)>) {
    let Ok((player, mut transform)) = player_query.get_single_mut() else {
        return;
    };
    transform.translation.x += player.direction.x * PLAYER_SPEED;
    transform.translation.y += player.direction.y * PLAYER_SPEED;
}

pub fn update_player_direction(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };
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

pub fn listen_for_restart_button(
    keys: Res<ButtonInput<KeyCode>>,
    mut restart: EventWriter<Restart>,
) {
    if keys.pressed(KeyCode::KeyR) {
        restart.send(Restart);
    }
}
