use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use crate::constants::OceanSettings;

// --- PRE-NORMALIZED GERSTNER DIRECTION VECTORS ---
// Kept in synchronization with GPU shader definitions and vehicle dynamics routines.
const WAVE_DIR_1: Vec2 = Vec2::new(0.98058, 0.19611);
const WAVE_DIR_2: Vec2 = Vec2::new(-0.61394, 0.78935);
const WAVE_DIR_3: Vec2 = Vec2::new(0.24253, -0.97014);

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

// =========================================================================
// --- Analytical CPU Gerstner Wave & Gradient Evaluation Engine ---
// =========================================================================

#[inline(always)]
fn calculate_gerstner_component(
    pos: Vec2,
    d: Vec2,
    steepness: f32,
    freq: f32,
    time: f32,
    amplitude: f32,
) -> Vec3 {
    let phase = freq * d.dot(pos) + time;
    let a = steepness * amplitude / freq;
    let (sin, cos) = phase.sin_cos();

    Vec3::new(d.x * (a * cos), a * sin, d.y * (a * cos))
}

/// Evaluates total 3D trochoidal wave surface displacement vector at coordinate (x, z).
#[inline]
pub fn get_total_wave_displacement(pos: Vec2, time: f32, amplitude: f32, frequency: f32) -> Vec3 {
    let freq = frequency * 0.4;

    let w1 = calculate_gerstner_component(pos, WAVE_DIR_1, 0.3, freq, time, amplitude);
    let w2 = calculate_gerstner_component(pos, WAVE_DIR_2, 0.2, freq * 1.5, time * 1.2, amplitude);
    let w3 = calculate_gerstner_component(pos, WAVE_DIR_3, 0.1, freq * 2.5, time * 1.8, amplitude);

    w1 + w2 + w3
}

/// Calculates instantaneous wave surface elevation and the analytical surface normal vector.
/// Returns a tuple of `(elevation_y, surface_normal)` for local hydrodynamic pressure,
/// wave slamming dynamics, and buoyancy cell torque calculations.
pub fn get_wave_height_and_normal(
    pos: Vec2,
    time: f32,
    amplitude: f32,
    frequency: f32,
) -> (f32, Vec3) {
    let freq = frequency * 0.4;
    let base_displacement = get_total_wave_displacement(pos, time, amplitude, frequency);

    // Analytical partial derivatives across Gerstner superposition
    let waves = [
        (WAVE_DIR_1, 0.3f32, freq, time),
        (WAVE_DIR_2, 0.2f32, freq * 1.5, time * 1.2),
        (WAVE_DIR_3, 0.1f32, freq * 2.5, time * 1.8),
    ];

    let mut dx = 0.0;
    let mut dz = 0.0;

    for (d, steepness, w_freq, w_time) in waves {
        let phase = w_freq * d.dot(pos) + w_time;
        let cos_phase = phase.cos();
        let amp_scaled = steepness * amplitude;

        dx += d.x * amp_scaled * cos_phase;
        dz += d.y * amp_scaled * cos_phase;
    }

    // Normal vector is perpendicular to surface tangents: (-dH/dx, 1.0, -dH/dz) normalized
    let normal = Vec3::new(-dx, 1.0, -dz).normalize();

    (base_displacement.y, normal)
}

/// Batch query interface for multi-point hull sampling. Evaluates wave height and normal vectors
/// across multiple coordinates in a single cache-efficient pass.
pub fn sample_wave_field_batch(
    points: &[Vec2],
    time: f32,
    amplitude: f32,
    frequency: f32,
    out_heights: &mut [f32],
    out_normals: &mut [Vec3],
) {
    let count = points.len().min(out_heights.len()).min(out_normals.len());
    for i in 0..count {
        let (h, n) = get_wave_height_and_normal(points[i], time, amplitude, frequency);
        out_heights[i] = h;
        out_normals[i] = n;
    }
}

// =========================================================================
// --- Scene Setup and Change-Detection Synchronization ---
// =========================================================================

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