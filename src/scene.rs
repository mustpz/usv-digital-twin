use bevy::prelude::*;
use crate::constants::OceanSettings;
use crate::environment::OceanMaterial;

/// Refactored for Procedural Fidelity.
/// Removed legacy 'Vagon Tiling' logic as procedural noise eliminates texture repetition artifacts.
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OceanMaterial>>,
    settings: Res<OceanSettings>,
) {
    // Global Resources Setup
    commands.insert_resource(ClearColor(Color::rgb(0.4, 0.6, 0.8)));
    
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 5.0, 
    });

    // 1. PROCEDURAL OCEAN SETUP
    // Using a single large mesh. The WGSL shader handles infinite coordinate mapping.
    let ocean_mesh = meshes.add(Mesh::from(Plane3d::default().mesh().size(2000.0, 2000.0)));

    commands.spawn((
        MaterialMeshBundle {
            mesh: ocean_mesh,
            material: materials.add(OceanMaterial {
                turbidity: settings.turbidity,
                wave_amplitude: settings.wave_amplitude,
                wave_frequency: settings.wave_frequency,
                time: 0.0,
                deep_water_color: Color::rgb(0.01, 0.05, 0.1),
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Name::new("Strategic_Ocean_Surface"),
    ));

    // 2. ATMOSPHERICS (Volumetric Fog)
    // Blends the horizon line with the sky color to hide mesh edges.
    commands.spawn(FogSettings {
        color: Color::rgb(0.4, 0.6, 0.8), 
        falloff: FogFalloff::Exponential { density: 0.005 },
        ..default()
    });

    // 3. SOLAR IRRADIANCE (Global Illumination)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 150000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Dynamic Scene Update
pub fn update_scene_system(
    settings: Res<OceanSettings>,
    mut query_fog: Query<&mut FogSettings>,
) {
    // Synchronizing Fog density with real-time turbidity for visibility analysis
    for mut fog in query_fog.iter_mut() {
        let visibility_factor = (1.0 - settings.turbidity).max(0.1);
        fog.falloff = FogFalloff::Exponential { 
            density: 0.01 / visibility_factor 
        };
    }
}