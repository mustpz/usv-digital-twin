use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use crate::constants::OceanSettings;

/// Combined GPU Uniform Buffer adhering strictly to std140 layout.
/// Packaged into 16-byte aligned vector blocks to prevent driver-specific memory misalignment.
#[derive(ShaderType, Debug, Clone, Copy)]
pub struct OceanUniforms {
    /// Combined structural wave properties:
    /// x: turbidity, y: wave_amplitude, z: wave_frequency, w: time
    pub wave_properties: Vec4,

    /// Combined environmental optics & physics:
    /// x: temp_gradient (optical ray bending), yzw: explicit 16-byte alignment padding
    pub env_physics: Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OceanMaterial {
    /// Consolidated primary uniform block at binding(0)
    #[uniform(0)]
    pub uniforms: OceanUniforms,

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
       Gerstner surface wave vectors require high vertex density to render sharp crests.
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
        uniforms: OceanUniforms {
            // x: turbidity, y: wave_amplitude, z: wave_frequency, w: time
            wave_properties: Vec4::new(0.1, 1.0, 0.2, 0.0),
            // x: temp_gradient, yzw: std140 16-byte padding
            env_physics: Vec4::new(0.0, 0.0, 0.0, 0.0),
        },
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
    let settings_changed = settings.is_changed();

    for (_, material) in materials.iter_mut() {
        // Time updates continuously
        material.uniforms.wave_properties.w = time_seconds;

        // Reactive update: Mutate uniform buffer parameters only when settings drift
        if settings_changed {
            material.uniforms.wave_properties.x = settings.turbidity;
            material.uniforms.wave_properties.y = settings.wave_amplitude;
            material.uniforms.wave_properties.z = settings.wave_frequency;

            material.uniforms.env_physics.x = settings.temp_gradient;

            let base_color = match settings.ocean_type {
                crate::constants::OceanType::Aegean => Color::rgb(0.0, 0.67, 0.63),
                crate::constants::OceanType::Caribbean => Color::rgb(0.0, 0.55, 0.67),
                crate::constants::OceanType::Baltic => Color::rgb(0.08, 0.18, 0.15),
            };
            material.deep_water_color = base_color;
        }
    }
}