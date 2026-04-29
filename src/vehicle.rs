#[derive(Component)]
pub struct Vehicle;

use bevy::prelude::*;

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
            base_color: Color::rgb(0.1, 0.3, 0.5), // Deep blue color
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },
    Vehicle,
 ));

}


/// Handles the movement logic for the vehicle using keyboard inputs.
/// This system updates the vehicle's transform based on WASD keys.
pub fn move_vehicle(
    // Monitors keyboard events
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    // Queries all entities that have the 'Vehicle' component to update their position/rotation
    mut query: Query<&mut Transform, With<Vehicle>>, 
    // Global resource to ensure frame-rate independent movement
    time: Res<Time>, 
) {
    let speed = 5.0; // Linear velocity of the vehicle
    let rotation_speed = 2.0; // Angular velocity (radians per second)

    for mut transform in query.iter_mut() {
        // Forward - Backward Movement (W - S)
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward = transform.forward();
            // Move along the forward vector scaled by speed and delta time
            transform.translation += forward * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let back = transform.back();
            // Move along the backward vector
            transform.translation += back * speed * time.delta_seconds();
        }

        // Steering / Rotation (A - D)
        if keyboard_input.pressed(KeyCode::KeyA) {
            // Rotate around the Y-axis (Up axis) to turn left
            transform.rotate_y(rotation_speed * time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            // Rotate around the Y-axis to turn right
            transform.rotate_y(-rotation_speed * time.delta_seconds());
        }
    }
}