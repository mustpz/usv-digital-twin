use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use crate::constants::OceanSettings;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OceanMaterial {
   
    #[uniform(0)]
    pub turbidity: f32,
    #[uniform(0)]
    pub wave_amplitude: f32,
    #[uniform(0)]
    pub wave_frequency: f32,
    #[uniform(0)]
    pub time: f32,

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
    /* CRITICAL UPDATE: Subdivisions.
       Gerstner waves require high vertex density to render sharp crests.
       Increasing subdivisions from default to 400 for structural fidelity.
    */
    let ocean_mesh = meshes.add(
   Mesh::from(bevy::prelude::shape::Plane {
    size: 2000.0,
    subdivisions: 400,
})
);
   
    let water_normal_handle = asset_server.load("textures/water_normal.png");

    let ocean_material = materials.add(OceanMaterial {
        turbidity: 0.1,
        wave_amplitude: 1.0,
        wave_frequency: 0.2,
        time: 0.0,
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

/// Dynamic synchronization system.
pub fn sync_ocean_material(
    settings: Res<OceanSettings>,
    time: Res<Time>,
    mut materials: ResMut<Assets<OceanMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        material.turbidity = settings.turbidity;
        material.wave_amplitude = settings.wave_amplitude;
        material.wave_frequency = settings.wave_frequency;
        material.time = time.elapsed_seconds();
        
        let base_color = match settings.ocean_type {
            crate::constants::OceanType::Aegean => Color::rgb(0.0, 0.67, 0.63),
            crate::constants::OceanType::Caribbean => Color::rgb(0.0, 0.55, 0.67),
            crate::constants::OceanType::Baltic => Color::rgb(0.08, 0.18, 0.15),
        };
        material.deep_water_color = base_color;
    }
}