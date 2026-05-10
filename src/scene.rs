use bevy::prelude::*;
use crate::constants::OceanSettings;
use crate::optics::{calculate_beer_lambert_attenuation};

#[derive(Component)]
pub struct SeabedComponent;

#[derive(Component)]
pub struct WaterMaterialMarker; 

/// Initializes the scene with overlapping infinite planes and high-intensity global lighting.
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 1. INITIAL MATERIAL SETUP
    let base_water_color = Color::rgba(0.0, 0.4, 0.8, 0.75);

    let water_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/seawater_texture.png")),
        base_color: base_water_color,
        normal_map_texture: Some(asset_server.load("textures/water_normal_map_seamless.png")),
        reflectance: 0.5,
        perceptual_roughness: 0.15,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Mesh size slightly larger than step size to prevent technical gaps
    let mesh_handle = meshes.add(Plane3d::default().mesh().size(501.0, 501.0));

    // 2. INFINITE OCEAN PLANES
    for i in 0..2 {
        commands.spawn((
            PbrBundle {
                mesh: mesh_handle.clone(),
                material: water_material.clone(),
                transform: Transform::from_xyz(0.0, 0.0, i as f32 * 500.0),
                ..default()
            },
            SeabedComponent,
            WaterMaterialMarker,
        ));
    }

    // 3. ATMOSPHERICS & LIGHTING
    commands.spawn(FogSettings {
        color: Color::rgb(0.0, 0.05, 0.1),
        falloff: FogFalloff::Exponential { density: 0.02 },
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(25.0, 50.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.2, 0.5, 0.8), 
        brightness: 0.8,
    });
}

/// Updates environment and optics with linear interpolation (LERP) for smooth transitions.
pub fn update_underwater_scene(
    time: Res<Time>,
    settings: Res<OceanSettings>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query_fog: Query<&mut FogSettings>,
    mut query_seabed: Query<(&mut Transform, &Handle<StandardMaterial>), With<SeabedComponent>>,
) {
    let elapsed = time.elapsed_seconds();
    let plane_size = 500.0; 
    
    let vessel_speed = settings.vessel_speed;
    let wave_frequency = settings.wave_frequency; 
    let wave_amplitude = settings.wave_amplitude;
    let water_turbidity = settings.turbidity;

    // MULTISPECTRAL OPTICS UPDATE
    for (transform, mat_handle) in query_seabed.iter_mut() {
        if let Some(material) = materials.get_mut(mat_handle) {
            let scaling_factor = 25.0; 
            let reference_distance = (transform.translation.y.abs() + 1.0) * scaling_factor; 
            let initial_color = Color::rgb(0.0, 0.6, 1.0); 

           // 1. Calculate the target color based on Beer-Lambert physics
            let target_color = calculate_beer_lambert_attenuation(
                initial_color, 
                reference_distance, 
                &settings
            );

            // 2. SMOOTH TRANSITION (Manual LERP):
            // Since Bevy's Color doesn't have a native lerp, we blend each channel.
            // factor 0.1 means 10% change per frame towards the target.
            let lerp_factor = 0.1;
            let current_r = material.base_color.r();
            let current_g = material.base_color.g();
            let current_b = material.base_color.b();

            let new_r = current_r + (target_color.r() - current_r) * lerp_factor;
            let new_g = current_g + (target_color.g() - current_g) * lerp_factor;
            let new_b = current_b + (target_color.b() - current_b) * lerp_factor;

            material.base_color = Color::rgb(new_r, new_g, new_b);
            
            // 3. Sync everything else
            material.emissive = material.base_color * 0.2; 
            material.base_color.set_a(0.7 + (settings.turbidity * 1.5).min(0.25));
        }
    }

    // FOG DYNAMICS
    for mut fog in query_fog.iter_mut() {
        let reference_depth = 15.0;
        let atten_factor = (-water_turbidity * reference_depth).exp();
        fog.color = Color::rgb(0.0, 0.06 * atten_factor, 0.12 * atten_factor);
        let safe_visibility = atten_factor.max(0.001);
        fog.falloff = FogFalloff::Exponential { density: 0.04 / safe_visibility };
    }

    // INFINITE LOOP MOTION
    let global_z_offset = (elapsed * vessel_speed) % (plane_size * 2.0);
    let orbital_y = (elapsed * wave_frequency).sin() * wave_amplitude;
    let orbital_z = (elapsed * wave_frequency).cos() * (wave_amplitude * 0.6);

    let mut index = 0;
    for (mut transform, _) in query_seabed.iter_mut() {
        let mut target_z = (index as f32 * plane_size) - global_z_offset;

        if target_z < -plane_size {
            target_z += plane_size * 2.0;
        }

        transform.translation.z = target_z + orbital_z;
        transform.translation.y = orbital_y; 

        index += 1;
    }
}