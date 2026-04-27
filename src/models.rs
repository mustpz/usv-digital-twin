pub struct UnmannedSurfaceVehicle {
    pub name: String,
    pub speed: f64,
    pub multispectral_sensor_active: bool,
}

impl UnmannedSurfaceVehicle {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            speed: 0.0,
            multispectral_sensor_active: true,
        }
    }
}