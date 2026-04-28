use bevy::prelude::*;

#[derive(Resource)]
pub struct Environment {
    pub temperature: f32,
    pub background_intensity: f32,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            temperature: 20.0,
            background_intensity: 0.5,
        }
    }
}