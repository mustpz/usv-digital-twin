use bevy::prelude::*;
// Importing the decoupled telemetry layers and the core biomimetic state registers
use crate::telemetry::{UsvTelemetryData}; 
use crate::biomimicry::{ThreatVector, OctopodEvasionMatrix};

/// Encapsulates the network client architecture and local endpoint configurations 
/// within a global Bevy resource for centralized dependency injection.
#[derive(Resource, Debug, Clone)]
pub struct TelemetryBridgeConfig {
    /// Non-blocking HTTP client thread pool manager for isolated asynchronous I/O execution
    pub client: reqwest::Client,
    /// Fully qualified local or air-gapped URI pointing to the hardware sensor distribution board
    pub local_sensor_url: String,
}

/// Bevy ECS system acting as a deterministic data bridge (Ingress Pipeline).
/// It ingests raw telemetry metrics from internal hardware streams and adaptively converts 
/// them into bio-inspired tactical threat evaluation coefficients.
pub fn telemetry_ingress_bridge_system(
    config: Res<TelemetryBridgeConfig>,
    mut query: Query<(&mut ThreatVector, &OctopodEvasionMatrix)>,
) {
    // -------------------------------------------------------------------------
    // MIL-SPEC AIR-GAPPED HARDWARE SIMULATION INGESTION
    // -------------------------------------------------------------------------
    // In a deployed Unmanned Surface Vehicle (USV), this segment processes the local
    // asynchronous task network or inter-process communication (IPC) sockets.
    // Here, we simulate a sudden, highly critical shallow water or obstacle trajectory profile.
    let simulated_hardware_depth: f32 = 12.5;  // Sudden critical shallow obstacle detected by hardware sensors (meters)
    let simulated_hardware_speed: f32 = 24.8;  // Current telemetry velocity register read from the hull (m/s)
    
    // -------------------------------------------------------------------------
    // THE CYBER-PHYSICAL BIOMIMETIC FEEDBACK LOOP
    // -------------------------------------------------------------------------
    for (mut threat, _matrix) in query.iter_mut() {
        
        // Operational Safety Envelope: Trigger a high-priority threat profile 
        // if the hardware telemetry feeds breach the 15.0-meter safety boundary.
        if simulated_hardware_depth < 15.0 {
            
            // Explicitly pipe the raw physical telemetry data into the bio-inspired tactical decision matrix
            threat.distance = simulated_hardware_depth; 
            threat.approach_velocity = simulated_hardware_speed;
            
            // Elevate the severity index to trigger the immediate cephalopod-inspired emergency state machine
            threat.severity = 0.98; // Crucial threshold enforcement (> 0.8)
            
            #[cfg(debug_assertions)]
            println!(
                "🚨 [SYSTEM INTEGRATION BRIDGE] Critical Telemetry Triggered! Sensor Depth: {}m. Injecting parameters into OctopodEvasionMatrix.", 
                simulated_hardware_depth
            );
        } else {
            // Squelch and dampen the threat evaluation coefficient under nominal cruising telemetry profiles
            threat.severity = 0.10;
        }
    }
}