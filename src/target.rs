use bevy::prelude::*;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .add_systems(Startup, spawn_basic_targets)
            .add_systems(Update, move_targets);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    speed: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    value: i32,
}

fn spawn_basic_targets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("enemy_m.png");

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(-100., -200., 0.),
            texture: texture.clone(),
            ..default()
        })
        .insert(Target { speed: 30. })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(-200., -200., 0.),
            texture: texture.clone(),
            ..default()
        })
        .insert(Target { speed: 30. })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(-300., -200., 0.),
            texture,
            ..default()
        })
        .insert(Target { speed: 30. })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));
}

fn move_targets(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}
