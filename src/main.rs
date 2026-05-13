use bevy::prelude::*;
use bevy_egui::EguiPlugin; 

mod constants;
mod environment;
mod vehicle;
mod scene;
mod models;
mod optics;
mod ui; 

use environment::{setup_ocean_environment, sync_ocean_material, OceanMaterial};
use vehicle::{spawn_vehicle, move_vehicle, float_vehicle_system};
use scene::{setup_scene, update_scene_system}; 
use constants::OceanSettings; 
use ui::update_ui_system;    

fn main() {
    println!("--- USV Digital Twin: High-Fidelity Gerstner Surface Simulation ---");

    App::new()
        // 1. PRIMARY WINDOW CONFIGURATION
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "USV Strategic Digital Twin | Gerstner Photonics Engine".into(),
                resolution: (1440.0, 900.0).into(), 
                present_mode: bevy::window::PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        
        // 2. PLUGINS & ASSET PIPELINE
        .add_plugins(EguiPlugin) 
        .add_plugins(MaterialPlugin::<OceanMaterial>::default())
        
        // 3. GLOBAL RESOURCES
        .init_resource::<OceanSettings>() 
        
        // 4. STARTUP SYSTEMS (Sequential Initialization)
        .add_systems(Startup, (
            setup_camera, 
            setup_scene, 
            setup_ocean_environment,
            spawn_vehicle
        ))
        
        // 5. UPDATE SYSTEMS (The Simulation Core Loop)
        .add_systems(Update, (
            // UI must always be processed first to capture user intent
            update_ui_system,
            
            // Atmospheric and Material sync happens immediately after UI updates
            (update_scene_system, sync_ocean_material).after(update_ui_system),
            
            // Physics and Navigation follow the environmental parameters
            (move_vehicle, float_vehicle_system)
                .chain() // Navigation affects position, then float system corrects Y height
                .after(sync_ocean_material),
        ))
        
        .run();
}

/// Initializes the 3D perspective camera at a high-angle strategic position.
/// Focal point is set to the USV's operation origin.
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        // Positioned for an optimal view of Gerstner crests and USV deck stability
        transform: Transform::from_xyz(-30.0, 22.0, 30.0) 
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}