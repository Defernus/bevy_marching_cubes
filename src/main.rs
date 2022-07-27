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
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::new(1., 0., 0.5), 1.7)),
        ..default()
    });
}
