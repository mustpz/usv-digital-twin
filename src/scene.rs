use bevy::prelude::*;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.2, 0.4), 
            reflectance: 0.8, 
            ..default()
        }),
        ..default()
    });

    
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15000.0, 
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2, 
    });

   
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}