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
    #[uniform(0)]
    pub deep_water_color: Color,
}

impl Material for OceanMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ocean_shader.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/ocean_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

pub fn setup_ocean_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OceanMaterial>>,
) {
    
   let ocean_mesh = meshes.add(Mesh::from(Plane3d::default().mesh().size(2000.0, 2000.0)));
   
    let ocean_material = materials.add(OceanMaterial {
        turbidity: 0.0,
        wave_amplitude: 0.0,
        wave_frequency: 0.0,
        time: 0.0,
        deep_water_color: Color::RED,
    });

commands.spawn((
    MaterialMeshBundle {
        mesh: ocean_mesh,
        material: ocean_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },
    Name::new("Strategic_Ocean_Surface"),
));


    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2.0,
    });
}

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
            crate::constants::OceanType::Aegean => Color::rgb(0.02, 0.1, 0.3),
            crate::constants::OceanType::Caribbean => Color::rgb(0.0, 0.6, 0.8),
        };
        material.deep_water_color = base_color;
    }
}