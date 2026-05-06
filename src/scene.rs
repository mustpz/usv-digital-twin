use bevy::prelude::*;
// These imports are essential for handling texture wrapping and seamless tiling
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
    // 1. MATERIAL OPTIMIZATION
    let water_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/seawater_texture.png")),
        base_color: Color::rgba(0.0, 0.12, 0.28, 0.75),
        normal_map_texture: Some(asset_server.load("textures/water_normal_map_seamless.png")),
        
        // FRESNEL & REFLECTION: reflectance 0.5 simulates realistic water-light interaction.
        reflectance: 0.5,
        // ROUGHNESS: Adjusted to 0.1 to soften specular highlights and hide texture seams.
        perceptual_roughness: 0.1,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // We use a large 500x500 mesh to keep the "reset point" far from the camera.
    let mesh_handle = meshes.add(Plane3d::default().mesh().size(500.0, 500.0));

    // 2. DUAL-PLANE "VAGON" SYSTEM
    // We spawn two identical planes. While one is leaving the view, 
    // the other is already entering, preventing any "gray gaps" or empty spaces.
    commands.spawn((
        PbrBundle {
            mesh: mesh_handle.clone(),
            material: water_material.clone(),
            ..default()
        },
        SeabedComponent,
    ));

    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: water_material,
            // Offset by exactly one plane size (500 units) for perfect alignment.
            transform: Transform::from_xyz(0.0, 0.0, 500.0),
            ..default()
        },
        SeabedComponent,
    ));

    // 3. ATMOSPHERIC SCATTERING (Fog) & LIGHTS
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
    let current_depth = 12.0;
    let water_turbidity = 0.04;

    // FOG DYNAMICS: Adjusts underwater visibility based on depth (Beer-Lambert Law).
    for mut fog in query_fog.iter_mut() {
        let visibility = calculate_light_attenuation(current_depth, water_turbidity);
        fog.color = Color::rgb(0.0, 0.1 * visibility, 0.2 * visibility);
        let safe_visibility = visibility.max(0.001);
        fog.falloff = FogFalloff::Exponential { density: 0.03 / safe_visibility };
    }

    // INFINITE LOOP LOGIC: 
    // Moves both planes simultaneously. The modulo (%) operator resets the position 
    // seamlessly, creating the illusion of an endless ocean.
    let mut i = 0.0;
    for (mut transform, _) in query_seabed.iter_mut() {
        let offset = (calculate_seabed_uv_offset(vessel_speed, elapsed) * 500.0) % 500.0;
        transform.translation.z = offset - 250.0 + (i * 500.0);
        i -= 1.0; 
    }
}