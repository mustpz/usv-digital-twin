use crate::vehicle::move_vehicle;

use bevy::prelude::*;

// Importing project modules
mod constants;
mod environment;
mod vehicle;
mod scene;
mod models;

use environment::Environment;
use vehicle::spawn_vehicle;
use scene::setup_scene;

fn main() {
    println!("--- USV Digital Twin Simulation Starting ---");

    App::new()
    .add_systems(Update, move_vehicle)

        // DefaultPlugins handles window creation, input, and rendering
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "USV Multispectral Digital Twin v0.1.0".into(),
                ..default()
            }),
            ..default()
        }))
        // Initialize global simulation resources
        .insert_resource(Environment::default())
        // Setup systems run once at startup
        .add_systems(Startup, (setup_camera, setup_scene, spawn_vehicle))
        .run();
}

/// Spawns a 3D camera so we can actually see the simulation
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    println!("Status: 3D Camera initialized.");
}