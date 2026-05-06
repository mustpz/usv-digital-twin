use bevy::prelude::*;
use crate::optics::{calculate_light_attenuation, calculate_seabed_uv_offset};

/// Component tag to identify the seabed entity within the simulation.
/// This allows the update system to efficiently query and move the seabed.
#[derive(Component)]
pub struct SeabedComponent;

/// Initializes the 3D environment, including the seabed, lighting, and underwater fog.
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 1. SEABED INITIALIZATION
    // We create a large plane to represent the ocean floor.
    // The material uses a dark, deep-water base color to enhance realism.
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(500.0, 500.0)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/seawater_texture.png")),
                base_color: Color::rgba(0.0, 0.12, 0.28, 0.75),
                normal_map_texture: Some(asset_server.load("textures/water_normal_map_seamless.png")),

                // reflectance at 1.0 simulates the strong Fresnel reflections seen on oceans.
                reflectance: 1.0,           
                // Very low roughness (0.02) makes the water look "wet" and glassy.
                perceptual_roughness: 0.02,
                // Enables transparency so we can see the vessel (USV) through the surface.
                alpha_mode: AlphaMode::Blend,

               ..default()
            }),
            ..default()
        },
        SeabedComponent, 
    ));

    // 2. UNDERWATER FOG (Light Attenuation Simulation)
    // Fog simulates the scattering and absorption of light in water.
    commands.spawn(FogSettings {
        color: Color::rgb(0.0, 0.08, 0.15), // Initial water tint
        falloff: FogFalloff::Exponential { density: 0.02 },
        ..default()
    });

    // 3. DIRECTIONAL LIGHT (Primary Sunlight)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 22000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(25.0, 45.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 2. AMBIENT LIGHTING (Scattered Environment Light)
    // Softens the shadows to mimic underwater light scattering.
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.1, 0.35, 0.55),
        brightness: 0.35,
    });
}

/// System that runs every frame to update the visual environmental effects.
/// It syncs the seabed movement with the vessel's speed and adjusts fog based on depth.
pub fn update_underwater_scene(
    time: Res<Time>,
    mut query_fog: Query<&mut FogSettings>,
    mut query_seabed: Query<&mut Transform, With<SeabedComponent>>,
) {
    let elapsed = time.elapsed_seconds();
    
    let vessel_speed = 4.5; // Estimated speed in m/s (approx. 8.7 knots)
    let current_depth = 12.0; // Current depth of the USV in meters
    let water_turbidity = 0.04; // Clarity factor (0.01=Clear, 0.1=Murky)

    // A. LIGHT ATTENUATION UPDATE
    // Adjusting the fog density and color according to the Beer-Lambert Law calculated in optics.rs.
    for mut fog in query_fog.iter_mut() {
        let visibility = calculate_light_attenuation(current_depth, water_turbidity);
        
        // Darken the water color as we go deeper
        fog.color = Color::rgb(0.0, 0.1 * visibility, 0.2 * visibility);
        
        // Increase fog density to simulate less visibility at depth
        let safe_visibility = visibility.max(0.001);
        fog.falloff = FogFalloff::Exponential { 
            density: 0.03 / safe_visibility 
        };
    }

    // B. SEABED MOTION (Optical Flow Simulation)
    // Offsetting the seabed mesh position to create a sense of forward motion.
    for mut transform in query_seabed.iter_mut() {
        let offset_factor = calculate_seabed_uv_offset(vessel_speed, elapsed);
        
        // Repeating the texture/mesh translation to maintain the illusion of an infinite ocean
        // 500.0 is the seabed size; -250.0 offsets it by half to achieve centering.
        transform.translation.z = (offset_factor * 500.0) % 500.0 - 250.0;
    }
}