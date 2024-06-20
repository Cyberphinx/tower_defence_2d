use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use tower_defence::{
    bullet::BulletPlugin,
    target::TargetPlugin,
    tower::{Tower, TowerPlugin},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tower Defence Game".into(),
                    ..default()
                }),
                ..default()
            }),
            WorldInspectorPlugin::new(),
            TowerPlugin,
            TargetPlugin,
            BulletPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("tower.png");

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture,
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            bullet_offset: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
        })
        .insert(Name::new("Tower"));
}
