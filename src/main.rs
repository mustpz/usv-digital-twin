use bevy::prelude::*;
use bevy_egui::EguiPlugin; 

// Module declarations
mod constants;
mod environment;
mod vehicle;
mod scene;
mod models;
mod optics;
mod ui; 

// Type imports
use environment::Environment;
use vehicle::{spawn_vehicle, move_vehicle}; 
use scene::{setup_scene, update_underwater_scene}; 
use constants::OceanSettings; 
use ui::update_ui_system;    

fn main() {
    println!("--- USV Digital Twin Simulation Starting ---");

    App::new()
        // Configure the primary window settings
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "USV Multispectral Digital Twin v0.1.0".into(),
                resolution: (1280.0, 720.0).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        
        // Add Egui plugin for the user interface
        .add_plugins(EguiPlugin)
        
        // Initialize global resources
        .insert_resource(Environment::default())
        .insert_resource(OceanSettings::default()) // Core simulation settings resource
        
        // Register Startup systems (Run once at launch)
        .add_systems(Startup, (setup_camera, setup_scene, spawn_vehicle))
        
        // Register Update systems (Run every frame)
        .add_systems(Update, (
            update_ui_system, // Renders the UI overlay
            move_vehicle,     // Handles USV physics and movement         
            update_underwater_scene.after(move_vehicle), // Updates optics and environment    
        ))
        
        .run();
}

/// Initializes the 3D perspective camera for the scene
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    println!("Status: 3D Camera initialized.");
}