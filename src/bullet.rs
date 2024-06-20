use bevy::prelude::*;
use bevy_xpbd_2d::plugins::collision::Collider;

use crate::target::{Health, Target};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (bullet_despawn, move_bullets, bullet_collision));
    }
}

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
    pub collider: Collider,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
) {
    for (bullet, bullet_transform) in &bullets {
        for (mut health, target_transform) in &mut targets {
            if Vec2::distance(
                bullet_transform.translation().xy(),
                target_transform.translation.xy(),
            ) < 25.
            {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
                info!("Target got hit 1 point");
                break;
            }
        }
    }
}
