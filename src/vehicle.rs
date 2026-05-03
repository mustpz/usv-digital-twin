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
            base_color: Color::rgb(0.1, 0.3, 0.5), 
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
  
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    
    mut query: Query<&mut Transform, With<Vehicle>>, 
    
    time: Res<Time>, 
) {
    let speed = 5.0; 
    let rotation_speed = 2.0; 

    for mut transform in query.iter_mut() {
        // Forward - Backward Movement (W - S)
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward = transform.forward();
            
            transform.translation += forward * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let back = transform.back();
            /
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