use bevy::prelude::*;

pub fn spawn_vehicle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // This spawns a simple box to represent our USV
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3::new(1.0, 0.5, 2.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.1, 0.3, 0.5), // Deep blue color
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}