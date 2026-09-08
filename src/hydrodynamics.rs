use bevy::prelude::*;
use crate::constants::{
    GRAVITY, SEAWATER_DENSITY,
    HYDRO_ADDED_MASS_X_UDOT, HYDRO_ADDED_MASS_Y_VDOT, HYDRO_ADDED_MASS_Z_WDOT,
    HYDRO_ADDED_MASS_K_PDOT, HYDRO_ADDED_MASS_M_QDOT, HYDRO_ADDED_MASS_N_RDOT,
    HYDRO_DAMPING_LINEAR_SURGE, HYDRO_DAMPING_LINEAR_SWAY, HYDRO_DAMPING_LINEAR_HEAVE,
    HYDRO_DAMPING_LINEAR_ROLL, HYDRO_DAMPING_LINEAR_PITCH, HYDRO_DAMPING_LINEAR_YAW,
    HYDRO_DAMPING_QUAD_SURGE, HYDRO_DAMPING_QUAD_SWAY, HYDRO_DAMPING_QUAD_YAW,
    OceanSettings,
};
use crate::environment::get_wave_height_and_normal;
use crate::vehicle::{RigidBody6DOF, HydrostaticProperties, Vehicle};

/// Solves 6-DOF Cummins-style equations of motion for maritime vessels, incorporating:
/// 1. Discretized Archimedean hydrostatic buoyancy and righting arm moments.
/// 2. Metacentric transverse restoration torque (GM_T).
/// 3. SNAME Added Mass tensor corrections across translational and rotational degrees of freedom.
/// 4. Coupled linear (viscous) and non-linear quadratic (cross-flow drag) hydrodynamic damping.
pub fn solve_hydrodynamic_6dof_system(
    time: Res<Time>,
    settings: Res<OceanSettings>,
    mut query: Query<(&mut Transform, &mut RigidBody6DOF, &HydrostaticProperties), With<Vehicle>>,
) {
    let dt = time.delta_seconds();
    if dt <= 0.0 || dt > 0.1 {
        return; // Guard against numerical divergence during system hitching or window operations
    }

    let elapsed = time.elapsed_seconds();
    let amp = settings.wave_amplitude;
    let freq = settings.wave_frequency;
    let gravity_vec = Vec3::new(0.0, -GRAVITY, 0.0);

    // Added mass diagonal tensors [X_udot, Y_vdot, Z_wdot] and [K_pdot, M_qdot, N_rdot]
    let added_mass_linear = Vec3::new(
        HYDRO_ADDED_MASS_X_UDOT,
        HYDRO_ADDED_MASS_Z_WDOT, // Bevy Y corresponds to vertical Heave (w)
        HYDRO_ADDED_MASS_Y_VDOT, // Bevy Z corresponds to Sway/Heading axis
    );
    let added_mass_rotational = Vec3::new(
        HYDRO_ADDED_MASS_K_PDOT, // Roll around X
        HYDRO_ADDED_MASS_N_RDOT, // Yaw around Y
        HYDRO_ADDED_MASS_M_QDOT, // Pitch around Z
    );

    for (mut transform, mut rb, hydro) in query.iter_mut() {
        let mut total_buoyancy_force = Vec3::ZERO;
        let mut total_torque_world = Vec3::ZERO;

        let rotation = transform.rotation;
        let world_cg = transform.translation + rotation * hydro.center_of_gravity;

        // Discretized volumetric hull integration
        for cell in &hydro.buoyancy_cells {
            let world_cell_pos = transform.translation + rotation * cell.local_offset;
            let (water_surface_y, surface_normal) = get_wave_height_and_normal(
                Vec2::new(world_cell_pos.x, world_cell_pos.z),
                elapsed,
                amp,
                freq,
            );

            let cell_submerged_depth = water_surface_y - world_cell_pos.y;

            if cell_submerged_depth > 0.0 {
                // Submersion fraction based on characteristic cell draught (0.2m)
                let submersion_ratio = (cell_submerged_depth / 0.2).clamp(0.0, 1.0);
                let submerged_volume = cell.volume * submersion_ratio;

                // Archimedean vertical buoyant force aligned with surface dynamic pressure gradient
                let buoyancy_magnitude = SEAWATER_DENSITY * GRAVITY * submerged_volume;
                let cell_buoyancy_force = Vec3::Y * buoyancy_magnitude + (surface_normal * (buoyancy_magnitude * 0.05));

                total_buoyancy_force += cell_buoyancy_force;

                // Restoring arm torque about Center of Gravity (CG)
                let moment_arm = world_cell_pos - world_cg;
                total_torque_world += moment_arm.cross(cell_buoyancy_force);
            }
        }

        // --- 1. TRANSLATIONAL MOTION INTEGRATION (Surge, Sway, Heave) ---
        let gravity_force = gravity_vec * hydro.mass;

        // Coupled linear and quadratic hydrodynamic damping
        let vel = rb.linear_velocity;
        let linear_damping_force = -Vec3::new(
            vel.x * HYDRO_DAMPING_LINEAR_SURGE,
            vel.y * HYDRO_DAMPING_LINEAR_HEAVE,
            vel.z * HYDRO_DAMPING_LINEAR_SWAY,
        );
        let quad_damping_force = -Vec3::new(
            vel.x * vel.x.abs() * HYDRO_DAMPING_QUAD_SURGE,
            0.0,
            vel.z * vel.z.abs() * HYDRO_DAMPING_QUAD_SWAY,
        );

        let net_force = total_buoyancy_force + gravity_force + linear_damping_force + quad_damping_force;

        // Virtual mass matrix: M_virtual = M_dry + M_added
        let virtual_mass = Vec3::splat(hydro.mass) + added_mass_linear;

        let linear_acc = net_force / virtual_mass;
        rb.linear_acceleration = linear_acc;
        rb.linear_velocity += linear_acc * dt;
        transform.translation += rb.linear_velocity * dt;

        // --- 2. ROTATIONAL MOTION INTEGRATION (Roll, Pitch, Yaw) ---
        // Transform torque into vessel body coordinates
        let inv_rotation = rotation.inverse();
        let body_torque = inv_rotation * total_torque_world;

        let ang_vel = rb.angular_velocity;
        let body_linear_damping = -Vec3::new(
            ang_vel.x * HYDRO_DAMPING_LINEAR_ROLL,
            ang_vel.y * HYDRO_DAMPING_LINEAR_YAW,
            ang_vel.z * HYDRO_DAMPING_LINEAR_PITCH,
        );
        let body_quad_damping = -Vec3::new(
            0.0,
            ang_vel.y * ang_vel.y.abs() * HYDRO_DAMPING_QUAD_YAW,
            0.0,
        );

        // Hydrostatic restoring righting moment: tau_roll = -m * g * GM_T * sin(phi)
        let (_, _, roll) = rotation.to_euler(EulerRot::YXZ);
        let metacentric_restoring_roll = -hydro.mass * GRAVITY * hydro.metacentric_height * roll.sin();
        let restoration_vector = Vec3::new(metacentric_restoring_roll, 0.0, 0.0);

        let net_body_torque = body_torque + body_linear_damping + body_quad_damping + restoration_vector;

        // Euler's rotational equations with hydrodynamic virtual inertia:
        // I_virtual * domega/dt + omega x (I_virtual * omega) = tau_net
        let virtual_inertia = hydro.inertia_tensor + added_mass_rotational;
        let i_omega = virtual_inertia * rb.angular_velocity;
        let gyroscopic_torque = rb.angular_velocity.cross(i_omega);

        let angular_acc = (net_body_torque - gyroscopic_torque) / virtual_inertia;
        rb.angular_acceleration = angular_acc;
        rb.angular_velocity += angular_acc * dt;

        // Quaternion angular velocity integration
        let angular_velocity_world = rotation * rb.angular_velocity;
        let delta_quat = Quat::from_scaled_axis(angular_velocity_world * dt);
        transform.rotation = (delta_quat * transform.rotation).normalize();
    }
}