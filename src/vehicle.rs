use bevy::prelude::*;
use crate::constants::{
    OceanSettings, OceanType, GRAVITY, CRITICAL_FROUDE_NUMBER, DRAG_COEFFICIENT, SEAWATER_DENSITY
};
// CENTRALIZED DATA INJECTION: Pipes the newly optimized structures directly from models.rs
use crate::models::UnmannedSurfaceVehicle; 

// --- PRE-NORMALIZED GERSTNER DIRECTION VECTORS (Zero Runtime Cast) ---
const WAVE_DIR_1: Vec2 = Vec2::new(0.98058, 0.19611);  
const WAVE_DIR_2: Vec2 = Vec2::new(-0.61394, 0.78935); 
const WAVE_DIR_3: Vec2 = Vec2::new(0.24253, -0.97014); 

#[derive(Component)]
pub struct Vehicle;

pub fn spawn_vehicle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(1.0, 0.5, 2.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.9, 0.9, 1.0), 
                metallic: 0.9,          
                perceptual_roughness: 0.1, 
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.2, 0.0),
            ..default()
        },
        Vehicle, 
        UnmannedSurfaceVehicle::new("Strategic_USV_Unit"),
        Name::new("Strategic_USV_Unit"),
    ));
}

/// Change-Detection Reactive Sensor System.
pub fn sensor_sampling_system(
    ocean_settings: Res<OceanSettings>,
    mut usv_query: Query<&mut UnmannedSurfaceVehicle>,
) {
    if !ocean_settings.is_changed() {
        return;
    }

    let base_color = match ocean_settings.ocean_type {
        OceanType::Aegean => Vec3::new(0.02, 0.05, 0.1),
        OceanType::Caribbean => Vec3::new(0.0, 0.3, 0.4),
        OceanType::Baltic => Vec3::new(0.01, 0.04, 0.05),
    };
    
    let turbidity_factor = ocean_settings.turbidity * 2.0;
    let green_shift = Vec3::new(0.1, 0.2, 0.1) * turbidity_factor;
    let adaptive_color = (base_color + green_shift).clamp(Vec3::ZERO, Vec3::ONE);
    let calculated_stealth = (ocean_settings.turbidity / 0.3).clamp(0.0, 1.0);

    for mut usv in usv_query.iter_mut() {
        if usv.multispectral_sensor_active {
            // Using the updated slice method from models.rs if multi-point sampling is triggered
            usv.target_camouflage_color = Color::rgb(adaptive_color.x, adaptive_color.y, adaptive_color.z);
            usv.stealth_alpha = calculated_stealth; 
        }
    }
}

/// Adaptive Optical Signature Minimization System.
pub fn apply_camouflage_system(
    usv_query: Query<(&UnmannedSurfaceVehicle, &Handle<StandardMaterial>), Changed<UnmannedSurfaceVehicle>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (usv, material_handle) in usv_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            let base_color = Vec3::new(0.9, 0.9, 1.0); 
            let target_color = Vec3::new(
                usv.target_camouflage_color.r(),
                usv.target_camouflage_color.g(),
                usv.target_camouflage_color.b(),
            );

            let final_rgb = base_color.lerp(target_color, usv.stealth_alpha);
            
            material.base_color = Color::rgb(final_rgb.x, final_rgb.y, final_rgb.z);
            material.perceptual_roughness = 0.1 + (usv.stealth_alpha * 0.4); 
            material.metallic = 0.9 * (1.0 - usv.stealth_alpha * 0.5);
        }
    }
}

// --- MATHEMATICALLY OPTIMIZED GERSTNER WAVE ENGINE ---

#[inline(always)]
fn calculate_gerstner_component(
    pos: Vec2, 
    d: Vec2, 
    stepness: f32, 
    freq: f32, 
    time: f32, 
    amplitude: f32
) -> Vec3 {
    let f = freq * d.dot(pos) + time;
    let a = stepness * amplitude / freq;
    let (sin, cos) = f.sin_cos(); 

    Vec3::new(d.x * (a * cos), a * sin, d.y * (a * cos))
}

pub fn get_total_wave_displacement(pos: Vec2, time: f32, amplitude: f32, frequency: f32) -> Vec3 {
    let freq = frequency * 0.4;

    let w1 = calculate_gerstner_component(pos, WAVE_DIR_1, 0.3, freq, time, amplitude);
    let w2 = calculate_gerstner_component(pos, WAVE_DIR_2, 0.2, freq * 1.5, time * 1.2, amplitude);
    let w3 = calculate_gerstner_component(pos, WAVE_DIR_3, 0.1, freq * 2.5, time * 1.8, amplitude);

    w1 + w2 + w3
}

pub fn float_vehicle_system(
    time: Res<Time>,
    settings: Res<OceanSettings>,
    mut query: Query<&mut Transform, With<Vehicle>>, 
) {
    let elapsed = time.elapsed_seconds();
    let amp = settings.wave_amplitude;
    let freq = settings.wave_frequency;
    let delta = 0.8; 
    
    for mut transform in query.iter_mut() {
        let x = transform.translation.x;
        let z = transform.translation.z;
        let current_pos = Vec2::new(x, z);
        
        let displacement = get_total_wave_displacement(current_pos, elapsed, amp, freq);
        let d_forward = get_total_wave_displacement(current_pos + Vec2::new(0.0, delta), elapsed, amp, freq);
        let d_right = get_total_wave_displacement(current_pos + Vec2::new(delta, 0.0), elapsed, amp, freq);

        transform.translation.y = displacement.y + 0.8; 

        let target_normal = Vec3::new(
            displacement.y - d_right.y, 
            delta, 
            displacement.y - d_forward.y
        ).normalize();

        let target_rotation = Quat::from_rotation_arc(Vec3::Y, target_normal);
        let current_yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
        
        transform.rotation = Quat::from_rotation_y(current_yaw) * target_rotation;
    }
}

pub fn move_vehicle(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&mut Transform, &mut UnmannedSurfaceVehicle), With<Vehicle>>, 
    time: Res<Time>, 
) {
    let base_propulsion_force = 12.0; 
    let rotation_speed = 2.5; 
    let hull_characteristic_length: f32 = 2.0; 
    let delta_sec = time.delta_seconds();
    
    let froude_denominator = (GRAVITY * hull_characteristic_length).sqrt();
    let is_w_pressed = keyboard_input.pressed(KeyCode::KeyW);
    let is_s_pressed = keyboard_input.pressed(KeyCode::KeyS);

    for (mut transform, mut usv) in query.iter_mut() {
        let current_velocity_magnitude = usv.vessel_speed;
        let froude_number = current_velocity_magnitude / froude_denominator;

        // Comply with core FVM modules linked directly to the hydrodynamics schema
        let dynamic_drag_modifier = if froude_number >= CRITICAL_FROUDE_NUMBER { 0.55 } else { 1.00 };

        if is_w_pressed {
            let exponential_velocity_drag = 0.5 * SEAWATER_DENSITY * current_velocity_magnitude.powi(2) * DRAG_COEFFICIENT;
            usv.hydrodynamics.current_drag = ((usv.hydrodynamics.current_drag + (exponential_velocity_drag * 0.01)) * dynamic_drag_modifier).min(3.5);
            usv.hydrodynamics.is_flow_steady = true; 
        } else {
            usv.hydrodynamics.current_drag = (usv.hydrodynamics.current_drag - 0.1).max(0.0);
            usv.hydrodynamics.is_flow_steady = false;
        }

        let effective_speed = (base_propulsion_force - usv.hydrodynamics.current_drag).max(0.0);
        usv.vessel_speed = if is_w_pressed { effective_speed } else { 0.0 };

        if is_w_pressed {
            let forward = transform.forward();
            transform.translation += forward * effective_speed * delta_sec;
        } else if is_s_pressed {
            let back = transform.back();
            transform.translation += back * (effective_speed * 0.5) * delta_sec;
        }
        
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.rotate_y(rotation_speed * delta_sec);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.rotate_y(-rotation_speed * delta_sec);
        }
    }
}