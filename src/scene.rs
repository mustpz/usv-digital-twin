use bevy::prelude::*;
use crate::constants::OceanSettings;

/// Responsible for Global Illumination, Solar Irradiance, and Maritime Atmospherics.
pub fn setup_scene(
    mut commands: Commands,
) {
    // 1. GLOBAL BACKGROUND (Horizon Alignment)
    // Matches the skybox color with the fog to create an infinite horizon effect.
    commands.insert_resource(ClearColor(Color::rgb(0.45, 0.65, 0.85)));
    
    // 2. AMBIENT LIGHTING (Global Illumination)
    // Soft blue-tinted fill light to simulate sky reflection in shadowed wave areas.
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.6, 0.75, 1.0), 
        brightness: 1000.0, 
    });

    // 3. SOLAR IRRADIANCE (Primary Directional Light)
    // For generating the "specular glints" and foam highlights on wave peaks.
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 25000.0, 
            shadows_enabled: true, 
            color: Color::rgb(1.0, 0.98, 0.92), 
            shadow_depth_bias: 0.05,
            shadow_normal_bias: 0.1,
            ..default()
        },
        transform: Transform::from_xyz(150.0, 300.0, 150.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 4. MARITIME ATMOSPHERICS (Volumetric Fog).
    commands.spawn(FogSettings {
        color: Color::rgb(0.55, 0.7, 0.8), 
        falloff: FogFalloff::Exponential { density: 0.0008 },
        ..default()
    });
}

/// Dynamic system that links atmospheric fog density to ocean turbidity settings.
/// Simulates how murky water often accompanies hazy meteorological conditions.
pub fn update_scene_system(
    settings: Res<OceanSettings>,
    mut query_fog: Query<&mut FogSettings>,
) {
    for mut fog in query_fog.iter_mut() {
        // As turbidity (murkiness) increases, atmospheric visibility decreases.
        let visibility_factor = (1.0 - settings.turbidity).max(0.1);
        fog.falloff = FogFalloff::Exponential { 
            density: 0.0015 / visibility_factor 
        };
    }
}