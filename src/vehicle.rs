use bevy::prelude::*;
use crate::constants::{
    OceanSettings, OceanType, GRAVITY, CRITICAL_FROUDE_NUMBER, DRAG_COEFFICIENT, SEAWATER_DENSITY
};
use crate::models::UnmannedSurfaceVehicle; 

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
                base_color: Color::rgb(0.9, 0.9, 1.0), // Industrial Base Gray
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

/// The Sensor System. Samples environment color and updates the USV's 'Target Color'.
/// High turbidity levels automatically increase the 'Stealth Alpha' for adaptive masking.
pub fn sensor_sampling_system(
    ocean_settings: Res<OceanSettings>,
    mut usv_query: Query<&mut UnmannedSurfaceVehicle>,
) {
    for mut usv in usv_query.iter_mut() {
        if usv.multispectral_sensor_active {
            
            let base_color = match ocean_settings.ocean_type {
                OceanType::Aegean => Vec3::new(0.02, 0.05, 0.1),
                OceanType::Caribbean => Vec3::new(0.0, 0.3, 0.4),
                OceanType::Baltic => Vec3::new(0.01, 0.04, 0.05),
            };
            let turbidity_factor = ocean_settings.turbidity * 2.0;
            let green_shift = Vec3::new(0.1, 0.2, 0.1) * turbidity_factor;
            let adaptive_color = (base_color + green_shift).clamp(Vec3::ZERO, Vec3::ONE);
            usv.target_camouflage_color = Color::rgb(adaptive_color.x, adaptive_color.y, adaptive_color.z);

            let auto_stealth = (ocean_settings.turbidity / 0.3).clamp(0.0, 1.0);
            usv.stealth_alpha = auto_stealth; 
        }
    }
}

/// Applies the calculated camouflage by interpolating the hull color in real-time.
/// Physical material properties (metallic/roughness) shift to minimize optical signature.
pub fn apply_camouflage_system(
    usv_query: Query<(&UnmannedSurfaceVehicle, &Handle<StandardMaterial>)>,
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

// --- GERSTNER WAVE ENGINE & VEHICLE DYNAMICS ---

fn calculate_gerstner_component(
    pos: Vec2, 
    dir: Vec2, 
    stepness: f32, 
    freq: f32, 
    time: f32, 
    amplitude: f32
) -> Vec3 {
    let d = dir.normalize();
    let f = freq * d.dot(pos) + time;
    let a = stepness * amplitude / freq;

    Vec3::new(
        d.x * (a * f.cos()), 
        a * f.sin(),         
        d.y * (a * f.cos()) 
    )
}

pub fn get_total_wave_displacement(x: f32, z: f32, time: f32, amplitude: f32, frequency: f32) -> Vec3 {
    let pos = Vec2::new(x, z);
    let freq = frequency * 0.4;

    let w1 = calculate_gerstner_component(pos, Vec2::new(1.0, 0.2), 0.3, freq, time, amplitude);
    let w2 = calculate_gerstner_component(pos, Vec2::new(-0.7, 0.9), 0.2, freq * 1.5, time * 1.2, amplitude);
    let w3 = calculate_gerstner_component(pos, Vec2::new(0.2, -0.8), 0.1, freq * 2.5, time * 1.8, amplitude);

    w1 + w2 + w3
}

pub fn float_vehicle_system(
    time: Res<Time>,
    settings: Res<OceanSettings>,
    mut query: Query<&mut Transform, With<Vehicle>>, 
) {
    let elapsed = time.elapsed_seconds();
    
    for mut transform in query.iter_mut() {
        let x = transform.translation.x;
        let z = transform.translation.z;
        
        let displacement = get_total_wave_displacement(
            x, z, elapsed, settings.wave_amplitude, settings.wave_frequency
        );
        
        let delta = 0.8; 
        let d_forward = get_total_wave_displacement(x, z + delta, elapsed, settings.wave_amplitude, settings.wave_frequency);
            let d_right = get_total_wave_displacement(x + delta, z, elapsed, settings.wave_amplitude, settings.wave_frequency);

        transform.translation.y = displacement.y + 0.8; 

        let target_normal = Vec3::new(
            displacement.y - d_right.y, 
            delta, 
            displacement.y - d_forward.y
        ).normalize();

        let target_rotation = Quat::from_rotation_arc(Vec3::Y, target_normal);
        
        let current_yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
        let yaw_rotation = Quat::from_rotation_y(current_yaw);
        
        transform.rotation = yaw_rotation * target_rotation;
    }
}

/// Core propulsion system. Integrates hydrodynamic drag force and dynamic Froude Number state changes.
/// When the calculated Froude Number passes CRITICAL_FROUDE_NUMBER, the hull breaks its wave-making
/// resistance barrier and transitions into high-velocity planing mode.
pub fn move_vehicle(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&mut Transform, &mut UnmannedSurfaceVehicle), With<Vehicle>>, 
    time: Res<Time>, 
) {
    let base_propulsion_force = 12.0; 
    let rotation_speed = 2.5; 
    
    // USV Characteristic Wetted Length along the keel profile (meters)
    let hull_characteristic_length: f32 = 2.0; 

    for (mut transform, mut usv) in query.iter_mut() {
        
        // Retrieve current forward-axis velocity magnitude from the kinematic state
        let current_velocity_magnitude = usv.vessel_speed;

        // --- NON-LINEAR FROUDE NUMBER & PLANING PHASE DETERMINATION ---
        // Formula: Fr = v / sqrt(g * L)
        let froude_number = current_velocity_magnitude / (GRAVITY * hull_characteristic_length).sqrt();

        // Establish structural fluid damping modifiers based on hull hydro-regime
        let dynamic_drag_modifier = if froude_number >= CRITICAL_FROUDE_NUMBER {
            // Planing Phase: Hull dynamically breaks displacement limits, decoupling wave-making resistance
            // Simulates severe drag drop (e.g., 45% dampening bypass) as the hull skims the surface interface
            0.55
        } else {
            // Displacement Phase: Standard fluid-boundary profile active; wave resistance operates at nominal index
            1.00
        };

        // --- REAL-TIME HYDRODYNAMIC CALCULATIONS ---
        // Standard Fluid Drag Force Equation modulated by the calculated hull phase boundary:
        // F_drag = (1/2 * ρ * v^2 * C_d) * dynamic_drag_modifier
        if keyboard_input.pressed(KeyCode::KeyW) {
            let exponential_velocity_drag = 0.5 * SEAWATER_DENSITY * current_velocity_magnitude.powi(2) * DRAG_COEFFICIENT;
            
            // Incrementally buffer active system drag while embedding the planing optimization matrix
            usv.hydrodynamics.current_drag = ((usv.hydrodynamics.current_drag + (exponential_velocity_drag * 0.01)) * dynamic_drag_modifier).min(3.5);
            usv.hydrodynamics.is_flow_steady = true; 
        } else {
            usv.hydrodynamics.current_drag = (usv.hydrodynamics.current_drag - 0.1).max(0.0);
            usv.hydrodynamics.is_flow_steady = false;
        }

        // Resolve kinematics by integrating propulsion magnitude minus boundary resistance factors
        let effective_speed = (base_propulsion_force - usv.hydrodynamics.current_drag).max(0.0);
        
        // Cache calculated effective speed vector directly back into the state resource
        usv.vessel_speed = if keyboard_input.pressed(KeyCode::KeyW) { effective_speed } else { 0.0 };

        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward = transform.forward();
            transform.translation += forward * effective_speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let back = transform.back();
            transform.translation += back * (effective_speed * 0.5) * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.rotate_y(rotation_speed * time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.rotate_y(-rotation_speed * time.delta_seconds());
        }
    }
}