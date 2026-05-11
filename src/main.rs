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
    println!("--- USV Digital Twin Simulation: Procedural Physics Edition ---");

    App::new()
        // 1. WINDOW CONFIGURATION
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "USV Strategic Digital Twin | Procedural Photonics".into(),
                resolution: (1440.0, 900.0).into(), 
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        
        // 2. PLUGINS
        .add_plugins(EguiPlugin)
        // Essential for linking our WGSL shader to the Bevy pipeline.
        .add_plugins(MaterialPlugin::<OceanMaterial>::default())
        
        // 3. RESOURCES
        .insert_resource(OceanSettings::default()) 
        
        // 4. STARTUP SYSTEMS (Spawns entities at the beginning)
        .add_systems(Startup, (
            setup_camera, 
            setup_scene, 
            spawn_vehicle
        ))
        
        // 5. UPDATE SYSTEMS (Runs every frame)
        .add_systems(Update, (
            update_ui_system,      
            move_vehicle,          
            float_vehicle_system, 
            update_scene_system,   
            
            // Updates the GPU shader parameters every frame.
            sync_ocean_material
                .after(update_ui_system)
                .after(move_vehicle),
        ))
        
        .run();
}

/// Initializes the 3D perspective camera for strategic overview.
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-70.0, 50.0, 70.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}