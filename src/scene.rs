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
    // Refactored to realistic physical scales to prevent multi-spectral over-exposure bloom.
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.6, 0.75, 1.0), 
        brightness: 120.0, // Standard physical baseline for soft sky-fill reflection
    });

    // 3. SOLAR IRRADIANCE (Primary Directional Light)
    // Calibrated for sharp specular glints and realistic shadow depth filtering.
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 12000.0, // Calibrated sun intensity matching ambient registers
            shadows_enabled: true, 
            color: Color::rgb(1.0, 0.98, 0.92), 
            shadow_depth_bias: 0.05,
            shadow_normal_bias: 0.1,
            ..default()
        },
        transform: Transform::from_xyz(150.0, 300.0, 150.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 4. MARITIME ATMOSPHERICS (Volumetric Fog)
    commands.spawn(FogSettings {
        color: Color::rgb(0.55, 0.7, 0.8), 
        falloff: FogFalloff::Exponential { density: 0.0008 },
        ..default()
    });
}

/// Dynamic Reactive System linking atmospheric fog density to ocean turbidity settings.
/// Leverages explicit `Changed<OceanSettings>` filters to fully eliminate redundant VRAM updates.
pub fn update_scene_system(
    settings: Res<OceanSettings>,
    mut query_fog: Query<&mut FogSettings>,
) {
    // REACTIVE GUARD: Only execute fog recalculation if environmental metrics drift
    if !settings.is_changed() {
        return;
    }

    let visibility_factor = (1.0 - settings.turbidity).max(0.1);
    let next_density = 0.0015 / visibility_factor;

    for mut fog in query_fog.iter_mut() {
        // Only trigger mutation if a mathematical difference exists
        if let FogFalloff::Exponential { density } = fog.falloff {
            if (density - next_density).abs() > 0.00001 {
                fog.falloff = FogFalloff::Exponential { density: next_density };
            }
        }
    }
}