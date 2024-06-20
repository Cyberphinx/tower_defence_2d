use bevy::{prelude::*, utils::FloatOrd};
use bevy_xpbd_2d::plugins::collision::Collider;

use crate::{
    bullet::{Bullet, Lifetime},
    target::Target,
};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_systems(Update, tower_shooting);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

fn tower_shooting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
) {
    for (tower_entity, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| {
                    closest_target.translation() - bullet_spawn - Vec3::new(-25., 0., 0.)
                });

            if let Some(direction) = direction {
                let texture = asset_server.load("bullet.png");
                commands.entity(tower_entity).with_children(|commands| {
                    commands
                        .spawn(SpriteBundle {
                            texture,
                            transform: Transform::from_translation(bullet_spawn),
                            ..default()
                        })
                        .insert(Lifetime {
                            timer: Timer::from_seconds(1000.5, TimerMode::Once),
                        })
                        .insert(Bullet {
                            direction,
                            speed: 200.,
                            collider: Collider::circle(16.),
                        })
                        .insert(Name::new("Bullet"));
                });
            }
        }
    }
}
