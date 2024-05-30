use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::time::Duration;

const ENEMY_SPEED: f32 = 2.9;

#[derive(Component, Default)]
pub struct Enemy {
    pub destination: Vec2,
}

#[derive(Component)]
pub struct SpawnEnemiesTimer {
    pub timer: Timer,
}

impl Default for SpawnEnemiesTimer {
    fn default() -> Self {
        SpawnEnemiesTimer {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
        }
    }
}

pub fn move_enemies(
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
    window_query: Query<&Window>,
) {
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

pub fn random_destination(window: &Window) -> Vec2 {
    let mut rng = thread_rng();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;
    Vec2 {
        x: rng.gen_range(-half_width..half_width),
        y: rng.gen_range(-half_height..half_height),
    }
}
