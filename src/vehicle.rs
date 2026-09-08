use bevy::prelude::*;
use crate::constants::{
    OceanSettings, OceanType, GRAVITY, CRITICAL_FROUDE_NUMBER, DRAG_COEFFICIENT, SEAWATER_DENSITY
};
use crate::models::UnmannedSurfaceVehicle; 
use crate::biomimicry::{
    EvasionMode, ThreatVector, OctopodEvasionMatrix, HullDynamics, Velocity
};

#[derive(Component)]
pub struct Vehicle;

/// 6-Degrees-of-Freedom (6-DOF) kinematic state descriptor.
#[derive(Component, Debug, Clone)]
pub struct RigidBody6DOF {
    pub linear_velocity: Vec3,      // World-space translational velocity [u, v, w]
    pub angular_velocity: Vec3,     // Body-frame rotational velocity [p (roll), q (pitch), r (yaw)]
    pub linear_acceleration: Vec3,  // World-space translational acceleration
    pub angular_acceleration: Vec3, // Body-frame rotational acceleration
}

impl Default for RigidBody6DOF {
    fn default() -> Self {
        Self {
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            linear_acceleration: Vec3::ZERO,
            angular_acceleration: Vec3::ZERO,
        }
    }
}

/// Structural and mass-distribution parameters for seakeeping analysis.
#[derive(Component, Debug, Clone)]
pub struct HydrostaticProperties {
    pub mass: f32,                          // Vessel dry displacement mass (kg)
    pub inertia_tensor: Vec3,               // Principal moments of inertia [Ixx, Iyy, Izz] (kg*m^2)
    pub center_of_gravity: Vec3,            // Center of gravity (CG) relative to model datum
    pub metacentric_height: f32,            // Transverse metacentric height (GM_T) in meters
    pub buoyancy_cells: [BuoyancyCell; 8],   // Discretized volumetric hull cells
}

/// Volumetric cell definition for localized hydrostatic and wave-interaction sampling.
#[derive(Debug, Clone, Copy)]
pub struct BuoyancyCell {
    pub local_offset: Vec3, // Local position vector relative to vehicle datum
    pub volume: f32,        // Cell volume displacement capacity (m^3)
}

impl Default for HydrostaticProperties {
    fn default() -> Self {
        let total_mass = 120.0;
        // Equivalent block inertia approximations (w: 1.0, h: 0.5, l: 2.0)
        let i_xx = (1.0 / 12.0) * total_mass * (0.5 * 0.5 + 2.0 * 2.0); // Roll inertia
        let i_yy = (1.0 / 12.0) * total_mass * (1.0 * 1.0 + 2.0 * 2.0); // Yaw inertia
        let i_zz = (1.0 / 12.0) * total_mass * (1.0 * 1.0 + 0.5 * 0.5); // Pitch inertia

        // 8-point spatial discretization across hull boundaries
        let buoyancy_cells = [
            // Bow section
            BuoyancyCell { local_offset: Vec3::new( 0.35, -0.15,  0.75), volume: 0.035 },
            BuoyancyCell { local_offset: Vec3::new(-0.35, -0.15,  0.75), volume: 0.035 },
            BuoyancyCell { local_offset: Vec3::new( 0.35,  0.10,  0.75), volume: 0.020 },
            BuoyancyCell { local_offset: Vec3::new(-0.35,  0.10,  0.75), volume: 0.020 },
            // Stern section
            BuoyancyCell { local_offset: Vec3::new( 0.35, -0.15, -0.75), volume: 0.035 },
            BuoyancyCell { local_offset: Vec3::new(-0.35, -0.15, -0.75), volume: 0.035 },
            BuoyancyCell { local_offset: Vec3::new( 0.35,  0.10, -0.75), volume: 0.020 },
            BuoyancyCell { local_offset: Vec3::new(-0.35,  0.10, -0.75), volume: 0.020 },
        ];

        Self {
            mass: total_mass,
            inertia_tensor: Vec3::new(i_xx, i_yy, i_zz),
            center_of_gravity: Vec3::new(0.0, -0.05, 0.0),
            metacentric_height: 0.28,
            buoyancy_cells,
        }
    }
}

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
        // 6-DOF RIGID BODY DYNAMICS STATE
        RigidBody6DOF::default(),
        HydrostaticProperties::default(),
        // BIOMIMICRY & TACTICAL EVASION COMPONENTS
        EvasionMode::default(),
        ThreatVector {
            source_id: 0,
            direction: Vec3::ZERO,
            distance: 999.0,
            approach_velocity: 0.0,
            severity: 0.0,
            target_heading: Vec3::Z,
        },
        OctopodEvasionMatrix {
            jet_propulsion_force: 45.0,
            ink_decoy_cooldown: 0.0,
            body_morph_drag_coeff: 1.0,
        },
        HullDynamics {
            mass: 120.0,
            baseline_drag_area: 0.85,
        },
        Velocity::default(),
    ));
}

/// Change-Detection Reactive Environmental Sensor System.
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
            usv.target_camouflage_color = Color::rgb(adaptive_color.x, adaptive_color.y, adaptive_color.z);
            usv.stealth_alpha = calculated_stealth; 
        }
    }
}

/// Adaptive Multispectral Camouflage Application System.
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

/// Tactical Propulsion and Thruster Control Interface.
/// Injects commanded forces and moments directly into the 6-DOF dynamic state.
pub fn move_vehicle(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&Transform, &mut RigidBody6DOF, &mut UnmannedSurfaceVehicle), With<Vehicle>>, 
    time: Res<Time>, 
) {
    let base_propulsion_force = 22.0; 
    let yaw_control_torque = 6.0; 
    let hull_characteristic_length: f32 = 2.0; 
    let delta_sec = time.delta_seconds();
    
    let froude_denominator = (GRAVITY * hull_characteristic_length).sqrt();
    let is_w_pressed = keyboard_input.pressed(KeyCode::KeyW);
    let is_s_pressed = keyboard_input.pressed(KeyCode::KeyS);

    for (transform, mut rb, mut usv) in query.iter_mut() {
        let current_velocity_magnitude = rb.linear_velocity.length();
        let froude_number = current_velocity_magnitude / froude_denominator;

        let dynamic_drag_modifier = if froude_number >= CRITICAL_FROUDE_NUMBER { 0.55 } else { 1.00 };

        if is_w_pressed {
            let exponential_velocity_drag = 0.5 * SEAWATER_DENSITY * current_velocity_magnitude.powi(2) * DRAG_COEFFICIENT;
            usv.hydrodynamics.current_drag = ((usv.hydrodynamics.current_drag + (exponential_velocity_drag * 0.01)) * dynamic_drag_modifier).min(3.5);
            usv.hydrodynamics.is_flow_steady = true; 
        } else {
            usv.hydrodynamics.current_drag = (usv.hydrodynamics.current_drag - 0.1).max(0.0);
            usv.hydrodynamics.is_flow_steady = false;
        }

        let effective_thrust = (base_propulsion_force - usv.hydrodynamics.current_drag).max(0.0);
        usv.vessel_speed = current_velocity_magnitude;

        // Apply propulsion vector along heading axis
        let forward = transform.forward();
        if is_w_pressed {
            rb.linear_velocity += forward * (effective_thrust * delta_sec);
        } else if is_s_pressed {
            rb.linear_velocity -= forward * (effective_thrust * 0.5 * delta_sec);
        }
        
        // Rudder/thruster moment applied directly around yaw axis
        if keyboard_input.pressed(KeyCode::KeyA) {
            rb.angular_velocity.y += yaw_control_torque * delta_sec;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rb.angular_velocity.y -= yaw_control_torque * delta_sec;
        }
    }
}
