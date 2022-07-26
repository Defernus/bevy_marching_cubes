use bevy::prelude::*;
use bevy_flycam::{MovementSettings, PlayerPlugin};
use marching_cubes::plugins::chunks::ChunksPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            speed: 100.,
            ..default()
        })
        .add_plugin(ChunksPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // add light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
