use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use crate::constants::OceanSettings;

/// GPU uniform padding requirements (std140 layout compliance).
/// Combines independent f32 metrics into unified vector blocks 
/// to eliminate memory misalignment and minimize buffer upload overhead.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OceanMaterial {
    /// Combined structural properties:
    /// x: turbidity, y: wave_amplitude, z: wave_frequency, w: time
    #[uniform(0)]
    pub wave_properties: Vec4,
    
    /// Combined environmental physics properties:
    /// x: temp_gradient (optical ray bending), yzw: internal padding for 16-byte boundary
    #[uniform(0)]
    pub env_physics: Vec4,

    #[uniform(1)] 
    pub deep_water_color: Color,

    #[texture(2)] 
    #[sampler(3)]
    pub water_normal: Handle<Image>,
}

impl Material for OceanMaterial {
    fn fragment_shader() -> ShaderRef { "shaders/ocean_shader.wgsl".into() }
    fn vertex_shader() -> ShaderRef { "shaders/ocean_shader.wgsl".into() }
    fn alpha_mode(&self) -> AlphaMode { AlphaMode::Blend }
}

pub fn setup_ocean_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OceanMaterial>>,
    asset_server: Res<AssetServer>, 
) {
    /* CRITICAL MESH OPTIMIZATION:
       Gerstner surface wave vectors require a massive vertex density matrix to render sharp crests.
       Maintaining plane subdivisions at 400 to prevent edge distortion during dynamic macro-oscillations.
    */
    let ocean_mesh = meshes.add(
        Mesh::from(bevy::prelude::shape::Plane {
            size: 2000.0,
            subdivisions: 400,
        })
    );
   
    let water_normal_handle = asset_server.load("textures/water_normal.png");

    let ocean_material = materials.add(OceanMaterial {
        // x: turbidity, y: wave_amplitude, z: wave_frequency, w: time
        wave_properties: Vec4::new(0.1, 1.0, 0.2, 0.0),
        // x: temp_gradient, yzw: padding
        env_physics: Vec4::new(0.0, 0.0, 0.0, 0.0), 
        deep_water_color: Color::rgb(0.01, 0.05, 0.1),
        water_normal: water_normal_handle,
    });

    commands.spawn((
        MaterialMeshBundle {
            mesh: ocean_mesh,
            material: ocean_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Name::new("Strategic_Ocean_Surface_HighRes"),
    ));
}

/// Change-Detection Optimized Sync System.
/// Leverages Bevy's change detection (`Changed<OceanSettings>`) to completely bypass 
/// unnecessary VRAM buffer uploads unless explicit environment state modification occurs.
pub fn sync_ocean_material(
    settings: Res<OceanSettings>,
    time: Res<Time>,
    mut materials: ResMut<Assets<OceanMaterial>>,
) {
    let time_seconds = time.elapsed_seconds();
    
    // Check if global settings actually triggered a change event
    let settings_changed = settings.is_changed();

    for (_, material) in materials.iter_mut() {
        // Time must update every frame dynamically
        material.wave_properties.w = time_seconds;

        // Reactive update: Only touch heavy uniform structures if state layer changed
        if settings_changed {
            material.wave_properties.x = settings.turbidity;
            material.wave_properties.y = settings.wave_amplitude;
            material.wave_properties.z = settings.wave_frequency;
            
            material.env_physics.x = settings.temp_gradient; 

            let base_color = match settings.ocean_type {
                crate::constants::OceanType::Aegean => Color::rgb(0.0, 0.67, 0.63),
                crate::constants::OceanType::Caribbean => Color::rgb(0.0, 0.55, 0.67),
                crate::constants::OceanType::Baltic => Color::rgb(0.08, 0.18, 0.15),
            };
            material.deep_water_color = base_color;
        }
    }
}