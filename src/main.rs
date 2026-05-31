use bevy::prelude::*;
use bevy_egui::EguiPlugin; 

mod constants;
mod environment;
mod vehicle;
mod scene;
mod models;
mod optics;
mod ui; 
mod telemetry; 
mod biomimicry;
mod bridge; 

use environment::{setup_ocean_environment, sync_ocean_material, OceanMaterial};
use vehicle::{
    spawn_vehicle, 
    move_vehicle, 
    float_vehicle_system, 
    sensor_sampling_system, // Bridge: Samples environment color
    apply_camouflage_system // Visualization: Updates hull material
};
use scene::{setup_scene, update_scene_system}; 
use constants::OceanSettings; 
use ui::update_ui_system;    

fn main() {
    println!("--- USV Digital Twin: High-Fidelity Gerstner Surface Simulation ---");
    println!("--- Initializing Optical Signature Management & Adaptive Camouflage ---");

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
        
        // 4. STARTUP SYSTEMS
        .add_systems(Startup, (
            setup_camera, 
            setup_scene, 
            setup_ocean_environment,
            spawn_vehicle 
        ))
        
        // 5. UPDATE SYSTEMS (The Simulation Loop)
        .add_systems(Update, (
            // A. INPUT PHASE: Capture UI commands first
            update_ui_system,
            
            // B. PERCEPTION PHASE: USV samples the water color based on UI/Environment settings
            sensor_sampling_system.after(update_ui_system),
            
            // C. SYNC PHASE: Update ocean materials and atmospheric conditions
            (update_scene_system, sync_ocean_material).after(update_ui_system),

            // D. VISUALIZATION PHASE: Apply the sampled color to the USV's hull
            apply_camouflage_system.after(sensor_sampling_system),
            
            // E. KINEMATICS PHASE: Calculate movement and buoyancy
            (move_vehicle, float_vehicle_system)
                .chain() 
                .after(sync_ocean_material),
        ))
        
        .run();
}

/// Initializes the 3D perspective camera at a strategic high-angle.
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-30.0, 22.0, 30.0) 
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}