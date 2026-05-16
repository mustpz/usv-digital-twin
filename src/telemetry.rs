pub struct VehicleTelemetry {
    pub timestamp: u64,
    pub velocity_knots: f32,
    pub depth_meters: f32,
    pub light_transmission_ratio: f32, 
}

impl VehicleTelemetry {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            velocity_knots: 0.0,
            depth_meters: 0.0,
            light_transmission_ratio: 1.0,
        }
    }
}