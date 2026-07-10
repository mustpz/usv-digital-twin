use bevy::prelude::*;
use bevy_egui::EguiPlugin; 

pub mod optics;

mod constants;
mod environment;
mod vehicle;
mod scene;
mod models;
mod ui; 
mod telemetry; 
mod biomimicry;
mod bridge; 

use telemetry::{stream_biomimetic_telemetry_system, TelemetryStreamConfig};
use environment::{setup_ocean_environment, sync_ocean_material, OceanMaterial};
use vehicle::{
    spawn_vehicle, 
    move_vehicle, 
    float_vehicle_system, 
    sensor_sampling_system, 
    apply_camouflage_system 
};
use scene::{setup_scene, update_scene_system}; 
use constants::OceanSettings; 
use ui::update_ui_system;     

// INGRESS PIPELINE IMPORTS
use bridge::{telemetry_ingress_bridge_system, hardware_polling_bridge_system, TelemetryBridgeConfig, HardwareSensorEvent};

fn main() {
    println!("--- USV Digital Twin: High-Fidelity Gerstner Surface Simulation ---");
    println!("--- Initializing Optical Signature Management & Adaptive Camouflage ---");

    App::new()
        // 1. PRIMARY WINDOW & DEFAULT PLUGINS CONFIGURATION
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
        
        // 2. THIRD-PARTY PLUGINS & ASSET PIPELINE
        .add_plugins(EguiPlugin) 
        .add_plugins(MaterialPlugin::<OceanMaterial>::default())
        .add_plugins(optics::OpticsPlugin)
        
        // 3. FINITE STATE MACHINE (FSM) & INTER-PROCESS EVENTS REGISTRATION
        .init_state::<crate::biomimicry::EvasionMode>()
        .add_event::<HardwareSensorEvent>() // Registers the thread-safe hardware event channel
        
        // 4. GLOBAL RESOURCES & TELEMETRY CONFIGURATIONS
        .init_resource::<OceanSettings>() 
        .init_resource::<TelemetryBridgeConfig>() // Initializes the 10Hz hardware ingress timer
        .insert_resource(TelemetryStreamConfig {
            client: reqwest::Client::new(),
            api_url: "https://api.yourcontrolstation.com/v1/telemetry".to_string(),
            rate_limiter: Timer::from_seconds(0.2, TimerMode::Repeating),
        })
        
        // 5. STARTUP SYSTEMS
        .add_systems(Startup, (
            setup_camera, 
            setup_scene, 
            setup_ocean_environment,
            spawn_vehicle 
        ))
        
        // 6. UPDATE SYSTEMS (The Deterministic Simulation Loop)
        .add_systems(Update, (
            // A. INPUT PHASE
            update_ui_system,
            
            // B. HARDWARE INGRESS PHASE (New): Poll physical buses and pipe data into the ECS event channel
            hardware_polling_bridge_system.after(update_ui_system),
            telemetry_ingress_bridge_system.after(hardware_polling_bridge_system),

            // C. PERCEPTION PHASE: USV samples the water color based on UI/Environment settings
            sensor_sampling_system.after(update_ui_system),
            
            // D. SYNC PHASE: Update ocean materials and atmospheric conditions
            (update_scene_system, sync_ocean_material).after(update_ui_system),

            // E. VISUALIZATION PHASE: Apply the sampled color to the USV's hull
            apply_camouflage_system.after(sensor_sampling_system),
            
            // F. KINEMATICS PHASE: Calculate movement and buoyancy
            (move_vehicle, float_vehicle_system)
                .chain() 
                .after(sync_ocean_material),
                
            // G. DETERMINISTIC PROTECTION & TELEMETRY NETWORK LAYER
            // Integrated seamlessly after the ingress bridge pipe evaluates safety boundaries
            crate::biomimicry::calculate_biomimetic_evasion_system.after(telemetry_ingress_bridge_system),
            stream_biomimetic_telemetry_system.after(crate::biomimicry::calculate_biomimetic_evasion_system)
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