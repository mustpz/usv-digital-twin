use bevy::prelude::*;
use crate::constants::OceanSettings;

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
        Name::new("Strategic_USV_Unit"),
    ));
}

fn calculate_gerstner_component(
    pos: Vec2, 
    dir: Vec2, 
    steepness: f32, 
    freq: f32, 
    time: f32, 
    amplitude: f32
) -> Vec3 {
    let d = dir.normalize();
    let f = freq * d.dot(pos) + time;
    let a = steepness * amplitude / freq;

    Vec3::new(
        d.x * (a * f.cos()), 
        a * f.sin(),         
        d.y * (a * f.cos()) 
    )
}

pub fn get_total_wave_displacement(x: f32, z: f32, time: f32, amplitude: f32, frequency: f32) -> Vec3 {
    let pos = Vec2::new(x, z);
    let freq = frequency * 0.4;

    // Layer 1: Main swell
    let w1 = calculate_gerstner_component(pos, Vec2::new(1.0, 0.2), 0.3, freq, time, amplitude);
    // Layer 2: Crossing wave
    let w2 = calculate_gerstner_component(pos, Vec2::new(-0.7, 0.9), 0.2, freq * 1.5, time * 1.2, amplitude);
    // Layer 3: High-frequency chop
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

pub fn move_vehicle(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<&mut Transform, With<Vehicle>>, 
    time: Res<Time>, 
) {
    let speed = 10.0; 
    let rotation_speed = 2.5; 

    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward = transform.forward();
            transform.translation += forward * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let back = transform.back();
            transform.translation += back * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.rotate_y(rotation_speed * time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.rotate_y(-rotation_speed * time.delta_seconds());
        }
    }
}