use bevy::prelude::*;
use crate::biomimicry::{ThreatVector, OctopodEvasionMatrix};

/// Real-time hardware telemetry packet received from physical sensor buses (IPC / UDP / Serial)
#[derive(Event, Debug, Clone, Copy)]
pub struct HardwareSensorEvent {
    pub depth: f32,
    pub speed: f32,
}

/// Encapsulates the network client architecture and local endpoint configurations 
#[derive(Resource, Debug, Clone)]
pub struct TelemetryBridgeConfig {
    pub client: reqwest::Client,
    pub local_sensor_url: String,
    pub poll_timer: Timer, // Limits hardware polling to industrial standards (e.g., 10Hz)
}

impl Default for TelemetryBridgeConfig {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            local_sensor_url: "http://127.0.0.1:8080/api/v1/sensors".to_string(),
            // Poll physical hardware at exactly 10Hz to prevent bus saturation
            poll_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

/// Asynchronous Ingress Background Worker.
/// In a real deployment, this system executes non-blocking I/O tasks (IPC sockets/HTTP)
/// within Bevy's IoTaskPool and dispatches thread-safe Events into the ECS layer.
pub fn hardware_polling_bridge_system(
    time: Res<Time>,
    mut config: ResMut<TelemetryBridgeConfig>,
    mut sensor_events: EventWriter<HardwareSensorEvent>,
) {
    // Rate limit the hardware bus polling
    if !config.poll_timer.tick(time.delta()).just_finished() {
        return;
    }

    // -------------------------------------------------------------------------
    // MIL-SPEC AIR-GAPPED HARDWARE SIMULATION INGESTION (Non-blocking)
    // -------------------------------------------------------------------------
    // Consuming IoTaskPool for async network fetch would happen here.
    // Simulating deterministic hardware data insertion:
    let simulated_hardware_depth: f32 = 12.5; 
    let simulated_hardware_speed: f32 = 24.8;

    sensor_events.send(HardwareSensorEvent {
        depth: simulated_hardware_depth,
        speed: simulated_hardware_speed,
    });
}

/// Deterministic Data Bridge (Ingress Pipeline).
/// Reacts exclusively to HardwareSensorEvents, completely eliminating frame-by-frame
/// overhead and updating bio-inspired tactical threat evaluation coefficients reactively.
pub fn telemetry_ingress_bridge_system(
    mut sensor_events: EventReader<HardwareSensorEvent>,
    mut query: Query<(&mut ThreatVector, &OctopodEvasionMatrix)>,
) {
    // Process the event queue reactively
    for event in sensor_events.read() {
        for (mut threat, _matrix) in query.iter_mut() {
            
            // Operational Safety Envelope: Trigger high-priority defense if depth breaches 15.0m
            if event.depth < 15.0 {
                // Explicitly pipe raw physical telemetry into the bio-inspired tactical decision matrix
                threat.distance = event.depth; 
                threat.approach_velocity = event.speed;
                threat.severity = 0.98; // Crucial threshold enforcement (> 0.8)
                
                #[cfg(debug_assertions)]
                trace!(
                    target: "systems::bridge",
                    "Critical Telemetry Ingested! Depth: {}m, Speed: {}m/s. Injecting parameters.", 
                    event.depth, event.speed
                );
            } else {
                threat.severity = 0.10; // Squelch threat coefficient under nominal profiles
            }
        }
    }
}