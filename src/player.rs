use bevy::prelude::*;

const INITIAL_PLAYER_SPEED: f32 = 3.0;

#[derive(Component)]
pub struct Player {
    direction: Vec2,
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            direction: Vec2::default(),
            speed: INITIAL_PLAYER_SPEED,
        }
    }
}

#[derive(Component, Default)]
pub struct Tween {
    pub step_value: Vec3,
    pub destination_scale: Vec3,
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
    transform.translation.x += player.direction.x * player.speed;
    transform.translation.y += player.direction.y * player.speed;
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

// TODO: For now this just does growing, but really this could handle any tween
// This doesn't do any kind of easing
pub fn animate_growth(mut tweens: Query<(Entity, &Tween, &mut Transform)>, mut commands: Commands) {
    for (entity, tween, mut transform) in tweens.iter_mut() {
        // NOTE: If the step distance isn't in the right direction (that is towards the
        // destination) then this will go forever.
        if tween.destination_scale.distance_squared(transform.scale)
            < tween.step_value.length_squared()
        {
            transform.scale = tween.destination_scale;
            commands.entity(entity).remove::<Tween>();
        } else {
            transform.scale += tween.step_value;
        }
    }
}
