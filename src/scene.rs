use bevy::prelude::*;
// Essential for seamless texture wrapping and physical rendering consistency
use bevy::render::render_resource::{AddressMode, SamplerDescriptor}; 
use crate::optics::{calculate_light_attenuation, calculate_seabed_uv_offset};

#[derive(Component)]
pub struct SeabedComponent;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 1. MATERIAL & OPTICAL PROPERTIES
    // Reflectance and roughness are tuned for realistic light interaction on seawater.
    let water_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/seawater_texture.png")),
        base_color: Color::rgba(0.0, 0.12, 0.28, 0.75),
        normal_map_texture: Some(asset_server.load("textures/water_normal_map_seamless.png")),
        
        // FRESNEL EFFECT: 0.5 reflectance provides a balance between refraction and surface reflection.
        reflectance: 0.5,
        // SPECULAR SOFTENING: Adjusted roughness to simulate natural water surface highlights.
        perceptual_roughness: 0.1,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // High-resolution mesh for the sea surface/seabed to minimize tiling artifacts.
    let mesh_handle = meshes.add(Plane3d::default().mesh().size(500.0, 500.0));

    // 2. SEAMLESS TANDEM (WAGON) SYSTEM
    // Spawning two planes to create an infinite ocean loop. 
    // This setup prevents any visual gaps during high-speed transitions.
    for i in 0..2 {
        commands.spawn((
            PbrBundle {
                mesh: mesh_handle.clone(),
                material: water_material.clone(),
                transform: Transform::from_xyz(0.0, 0.0, i as f32 * 500.0),
                ..default()
            },
            SeabedComponent,
        ));
    }

    // 3. UNDERWATER ATMOSPHERICS (Volumetric Approximation)
    commands.spawn(FogSettings {
        color: Color::rgb(0.0, 0.08, 0.15),
        falloff: FogFalloff::Exponential { density: 0.02 },
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 25000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(25.0, 50.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.1, 0.4, 0.6),
        brightness: 0.4,
    });
}

pub fn update_underwater_scene(
    time: Res<Time>,
    mut query_fog: Query<&mut FogSettings>,
    mut query_seabed: Query<(&mut Transform, Entity), With<SeabedComponent>>,
) {
    let elapsed = time.elapsed_seconds();
    let vessel_speed = 4.5;
    let plane_size = 500.0;
    
    // PHYSICAL WAVE PARAMETERS
    // wave_frequency and amplitude define the sea state intensity.
    let wave_frequency = 1.25; 
    let wave_amplitude = 0.4;

    // FOG DYNAMICS: Utilizing Beer-Lambert Law for depth-based light attenuation.
    for mut fog in query_fog.iter_mut() {
        let current_depth = 12.0; 
        let water_turbidity = 0.04;
        let visibility = calculate_light_attenuation(current_depth, water_turbidity);
        
        fog.color = Color::rgb(0.0, 0.1 * visibility, 0.2 * visibility);
        let safe_visibility = visibility.max(0.001);
        fog.falloff = FogFalloff::Exponential { density: 0.03 / safe_visibility };
    }

    // INFINITE LOOP & ORBITAL MOTION INTEGRATION
    let global_z_offset = (elapsed * vessel_speed) % (plane_size * 2.0);

    // ORBITAL MOTION: Combining Transverse (Y) and Longitudinal (Z) components.
    // This mimics the circular path of water particles in a surface wave, 
    // breaking the "robotic" linear movement.
    let orbital_y = (elapsed * wave_frequency).sin() * wave_amplitude;
    let orbital_z = (elapsed * wave_frequency).cos() * (wave_amplitude * 0.6);

    let mut index = 0;
    for (mut transform, _) in query_seabed.iter_mut() {
        let mut target_z = (index as f32 * plane_size) - global_z_offset;

        // Reset point for the infinite plane loop
        if target_z < -plane_size {
            target_z += plane_size * 2.0;
        }

        // Apply linear translation combined with physical orbital oscillation
        transform.translation.z = target_z + orbital_z;
        transform.translation.y = orbital_y; 

        index += 1;
    }
}