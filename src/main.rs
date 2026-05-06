use bevy::prelude::*;

mod constants;
mod environment;
mod vehicle;
mod scene;
mod models;
mod optics;

use environment::Environment;
use vehicle::{spawn_vehicle, move_vehicle}; 
use scene::{setup_scene, update_underwater_scene}; 

fn main() {
    println!("--- USV Digital Twin Simulation Starting ---");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "USV Multispectral Digital Twin v0.1.0".into(),
                resolution: (1280.0, 720.0).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        
        .insert_resource(Environment::default())
        
        .add_systems(Startup, (setup_camera, setup_scene, spawn_vehicle))
        
        .add_systems(Update, (
            move_vehicle,              
            update_underwater_scene.after(move_vehicle),     
        ))
        
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    println!("Status: 3D Camera initialized.");
}