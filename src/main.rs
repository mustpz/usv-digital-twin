use bevy::prelude::*;
use bevy_egui::EguiPlugin; 


mod constants;
mod environment;
mod vehicle;
mod scene;
mod models;
mod optics;
mod ui; 


use environment::Environment;
use vehicle::{spawn_vehicle, move_vehicle}; 
use scene::{setup_scene, update_underwater_scene}; 
use constants::OceanSettings; 
use ui::update_ui_system;    

fn main() {
    println!("--- USV Digital Twin Simulation Starting ---");

    App::new()
        // 1. WINDOW CONFIGURATION
        // Setting up the primary window with multispectral title and HD resolution.
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "USV Multispectral Digital Twin v0.1.0".into(),
                resolution: (1280.0, 720.0).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        
        // 2. PLUGINS
        // Adding Egui for real-time parameter manipulation.
        .add_plugins(EguiPlugin)
        
        // 3. RESOURCES
        // Initializing the simulation state. OceanSettings is the core source of truth.
        .insert_resource(Environment::default())
        .insert_resource(OceanSettings::default()) 
        
        // 4. STARTUP SYSTEMS
        // Executed once during the initialization phase.
        .add_systems(Startup, (
            setup_camera, 
            setup_scene, 
            spawn_vehicle
        ))
        
        // 5. UPDATE SYSTEMS
        // Logic executed every frame. Order is constrained to ensure consistency.
        .add_systems(Update, (
            update_ui_system, // First, collect user input
            move_vehicle,     // Second, calculate vehicle physics
            
            // Finally, update optics and environment after inputs and physics are settled.
            // This ensures optical shifts (Beer-Lambert) align perfectly with current settings.
            update_underwater_scene
                .after(update_ui_system)
                .after(move_vehicle),
        ))
        
        .run();
}

/// Initializes the 3D perspective camera for the scene.
/// Positioned to provide a clear view of the USV and water surface.
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    println!("Status: 3D Perspective Camera initialized.");
}