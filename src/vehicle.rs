use bevy::prelude::*;
use crate::constants::OceanSettings;

#[derive(Component)]
pub struct Vehicle;

/// Spawns the USV (Unmanned Surface Vehicle) into the simulation.
pub fn spawn_vehicle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // This spawns a simple box to represent our USV
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(1.0, 0.5, 2.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.9, 1.0), 
                metallic: 0.8,
                perceptual_roughness: 0.2,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Vehicle,
    ));
}

/// Core wave mathematics shared with the WGSL shader for physical synchronization.
pub fn get_wave_height(x: f32, z: f32, time: f32, amplitude: f32, frequency: f32) -> f32 {
    let freq = frequency * 0.2;
    
    // Exact replica of the shader's 3-wave interference pattern
    let d1 = (x * 1.0 + z * 0.2) * freq + time;
    let d2 = (x * -0.5 + z * 0.8) * (freq * 1.5) + (time * 1.2);
    let d3 = (x * 0.2 + z * -0.9) * (freq * 2.0) + (time * 0.8);
    
    (d1.sin() * amplitude) + (d2.sin() * (amplitude * 0.4)) + (d3.cos() * (amplitude * 0.2))
}

/// System that calculates the vertical position (buoyancy) of the vehicle.
pub fn float_vehicle_system(
    time: Res<Time>,
    settings: Res<OceanSettings>,
    mut query: Query<&mut Transform, With<Vehicle>>, 
) {
    let elapsed = time.elapsed_seconds();
    
    for mut transform in query.iter_mut() {
        let x = transform.translation.x;
        let z = transform.translation.z;
        
        let wave_height = get_wave_height(
            x, 
            z, 
            elapsed, 
            settings.wave_amplitude, 
            settings.wave_frequency
        );
        
        // Updates y position to stay on top of the wave with a small offset (0.2)
        transform.translation.y = wave_height + 0.25; 
    }
}

/// Handles the movement logic for the vehicle using keyboard inputs.
pub fn move_vehicle(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<&mut Transform, With<Vehicle>>, 
    time: Res<Time>, 
) {
    let speed = 7.0; 
    let rotation_speed = 2.5; 

    for mut transform in query.iter_mut() {
        // Forward - Backward Movement (W - S)
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward = transform.forward();
            transform.translation += forward * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let back = transform.back();
            transform.translation += back * speed * time.delta_seconds();
        }

        // Steering / Rotation (A - D)
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.rotate_y(rotation_speed * time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.rotate_y(-rotation_speed * time.delta_seconds());
        }
    }
}