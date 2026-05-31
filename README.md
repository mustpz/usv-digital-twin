# Real-Time Digital Twin Prototype for USV Multispectral Camouflage Systems

## Project Vision
A modular real-time digital twin framework for simulating environmental interaction 
and visual signature behavior of Unmanned Surface Vehicles (USVs). This project focuses on building a real-time simulation environment to model multispectral camouflage responses under dynamic environmental conditions. Multispectral modeling is planned for future implementation and is not yet included.

## Current Demo 
![Current Simulation State](./demo.gif) 

 ### The Current Prototype: Core Implementation

* **Adaptive Signature Management:** A Closed-Loop system that samples turbidity and spectral data to adjust the USV's optical signature in real-time—bridging theoretical photonics with practical naval stealth applications.
* **Gerstner Wave Synthesis:** Multi-layered displacement model producing realistic "sharp" crests and horizontal vertex displacement for a naturally turbulent sea state.
* **Non-Repetitive Foam Dynamics:** Adaptive foam generation triggered by wave height, utilizing a triple-overlay normal map technique with asymmetrical panning to eliminate visual repetition.
* **1:1 Physics Synchronization:** Vessel buoyancy is fully coupled with GPU displacement, enabling realistic Pitch and Roll responses based on dynamic wave slopes.
* **Physics-Based Rendering:** Ocean color is dynamically computed via the Beer-Lambert Law, using turbidity as a physical extinction coefficient for light absorption and scattering.


### Architectural Choice: Why Gerstner Waves Over FFT?

* **Hardware Scalability:** Avoids heavy FFT compute overhead, maintaining **60+ FPS on mid-range hardware (e.g., RTX 3050)**.
* **Zero-Lag Buoyancy:** Allows instant CPU-side physics sampling, ensuring perfect synchronization between visual wave geometry and vessel kinematics.
* **Deterministic Parameter Control:** Provides total control over environmental vectors (Salinity, Turbidity, Sea State) essential for sensor-testing Digital Twins.

> 🎯 **Core Objective:** To prove that autonomous maritime platforms can interpret environmental physics to execute tactical survival decisions independently.


### 🚧 Next Phase
- [x] **Asynchronous Bidirectional Telemetry Pipeline & Closed-Loop API Integration** (Completed)
- [X] **Bio-Inspired Adaptive Escape Dynamics** (Core State Machine & ECS Framework Compiled)
- [ ] **Real-Time Multi-Spectral Camouflage Response Subsystems**
 

## ## Technical Framework & Core Stack

| Component | Technology | Architectural Role & Implementation |
| :--- | :--- | :--- |
| **Core Engine** | Rust | Memory safety, zero-cost abstractions, data-driven simulation logic. |
| **Framework** | Bevy 0.13 | Entity Component System (ECS) driving massive entity parallelization. |
| **Networking** | Reqwest & Tokio | Non-blocking Async I/O runtime for continuous telemetry ingress/egress. |
| **Serialization** | Serde & JSON | Low-overhead data serialization for edge-computing telemetry packets. |
| **Shading & Physics**| WGSL | Custom WebGPU shaders for procedural waves and Beer-Lambert attenuation. |
| **User Interface** | bevy_egui | Immediate-mode GUI for real-time optical and hydrodynamic manipulation. |

### 🛠️ Architecture Highlights
* **Closed-Loop Isolation:** Designed entirely for local edge-computing. Zero dependency on non-deterministic external cloud APIs, ensuring maximum tactical data security.
* **Zero-Blocking Architecture:** Network operations run fully asynchronous via dedicated connection pools, guarantees 0% frame drops across the core Bevy simulation loop.

## Architecture 
src

main.rs * Application Orchestrator: Initializes the Bevy engine, registers global resources, and schedules system execution orders.

ui.rs * Interaction Layer: Implements the bevy_egui control panel. It acts as the primary interface for real-time parameter manipulation of the OceanSettings resource.

constants.rs * Physical Core: Defines the OceanSettings struct and stores hardcoded environmental constants (e.g., Sea level pressure, refractive index base).

optics.rs * Optical Physics Engine: Contains the mathematical implementations for Snell’s Law, Fresnel Reflectance, and the Beer-Lambert Law for light attenuation.

scene.rs * Environment & Rendering: Manages the infinite ocean tiling system, volumetric fog, and orbital wave oscillations. It bridges the physical data from optics.rs to the visual mesh.

vehicle.rs * Kinematic Controller: Defines the USV (Unmanned Surface Vehicle) entity, its spawn parameters, and the real-time movement logic responsive to sea state dynamics.

environment.rs * Atmospheric Modeling: Handles broader environmental states and global simulation parameters.

models.rs * Hardware Abstraction: Future module reserved for multispectral sensor models and advanced camera optics.

telemetry.rs * Design and initialization of an asynchronous data pipeline to handle real-time vehicle diagnostics and optical sensor telemetry. Encapsulates critical kinematic and environmental data streams, including depth parameters and velocity matrices. Prepares the data layer for asynchronous transmission to remote telemetry dashboards or control hubs.

biomimicry.rs * Implements a deterministic, low-latency state machine inspired by cephalopod mechanics to execute tactical autonomous evasion maneuvers. This layer consumes real-time async telemetry data to dynamically adapt hull kinematics and multispectral signatures against incoming hostile threat vectors.

bridge.rs * Converts raw hardware telemetry data into actionable threat coefficients for the biomimetic escape matrix. It serves as the deterministic, non-blocking bridge connecting real-world sensors to the autonomous decision loop.


## Theoretical Foundation & References
The core algorithms and optical models within this digital twin are grounded in rigorous electro-optical engineering principles. Key references used for system analysis, sensor modeling, and testing include:

* **Michael C. Dudzik** – *Electro-Optical Systems Design, Analysis, and Testing*
* **Cornelius J. Willers** – *Electro-Optical System Analysis and Design*
* **Sherman Karp** – *Fundamentals of Electro-Optics Systems Design*
* **William D. Rogatto** – *Electro-Optical Components*
* **George W. Masters** – *Electro-Optical Systems Test and Evaluation*
* **Roger T. Hanlon and John B. Messenger** - *Cephalopod Behaviour*

These references guide the future of implementation of sensor and optical response models.

## ## Project Status
**Stage:** Active Technical Prototype / Real-Time Simulation Framework

### 🏆 Completed 

- [x] **Asynchronous Telemetry Pipeline & Network Architecture Integration**
  - Implemented a non-blocking I/O network framework using reqwest and serde. Core data structures (UsvTelemetryData) and async ingress/egress functions are fully compiled, setting up the foundation for upcoming real-time data streaming and bio-mimicry processing.

- [x] **Autonomous Environmental Perception Layer**
  - Prototyped an autonomous perception matrix that dynamically synchronizes the USV’s optical signature with real-time ocean turbidity and spectral data, proving the feasibility of adaptive stealth logic in maritime digital twins.

- [x] **Gerstner Wave Synthesis & Physics Sync**
  - Implemented a multi-layered Gerstner displacement model with 1:1 CPU-GPU synchronization, enabling the USV to realistically align its pitch and roll with procedural wave slopes.

- [x] **Photonic Ocean Rendering (Beer-Lambert Law)**
  - Integrated a physics-based Beer-Lambert light attenuation model where turbidity acts as an extinction coefficient, dynamically calculating spectral shifts and visibility ranges.

- [x] **Adaptive Surface Detail & Foam Dynamics**
  - Developed a procedural foam system and triple-layered normal map noise to simulate high-frequency sea surface turbulence and crest-dependent foam generation.

- [x] **Coupled Atmospherics**
  - Synchronized volumetric fog density with maritime turbidity levels to create a cohesive and strategically consistent environmental simulation.

- [x] **Hydrodynamics Module & Laminar Flow Analysis**
  - Integrated a hydrodynamics layer to track laminar flow stability. The system distinguishes between laminar and turbulent flow regimes, laying the analytical foundation for how surface disturbances affect the autonomous stealth signature of the USV.

- [x] **Bio-Inspired Adaptive Escape Dynamics** 
  - *Implementation:* Developed and fully compiled a deterministic, low-latency tactical state machine (`EvasionMode`) and component matrix (`ThreatVector`, `OctopodEvasionMatrix`) within the Bevy ECS architecture. The framework is engineered to consume real-time asynchronous telemetry data, enabling independent, localized micro-maneuvers and reactive hydroelastic evasion profiles against simulated hostile assets.  

### 🚧 In Progress

- [ ] **Full Multispectral Camouflage & Perception Engine**
  - *Current Focus:* Refining a granular wavelength-dependent absorption and reflection model. This will simulate active Non-Line-of-Sight (NLOS) and Near-Infrared (NIR/SWIR) signatures for advanced USV stealth testing against multi-band radar/optical sensors.

- [ ] **Adaptive Wake & Splash Simulation (Bevy Particle System)**
  - *Current Focus:* Developing a high-performance particle-based systemwithin Bevy ECS to render visual water displacement effects, hull friction trails, and spray diagnostics behind the USV as it traverses procedural high-amplitude waves.

- [ ] **Sensor Fusion Layer (Ray-Casting LiDAR/Radar)**
  - *Current Focus:* Implementing parallelized ray-casting and wave-scattering algorithms within the Bevy ECS architecture to simulate physical autonomous navigation sensors, enabling true distance detection and spatial awareness against dynamic wave surfaces.

- [ ] **Dynamic Day/Night Solar Tracking**
  - *Current Focus:* Integrating an automated solar-tracking matrix to calculate time-of-day dependent atmospheric light scattering and its direct degradation vectors on the USV’s optical sensor suite.

### 🎯 Long-Term Planned Objectives

- [ ] **Infrared (IR) & Active Thermal Signature Simulation**
  - *Objective:* Simulating thermodynamic dissipation profiles across the USV's hull. Integrating high-fidelity Long-Wave Infrared (LWIR) and Mid-Wave Infrared (MWIR) sensor feedback loops to test advanced multi-spectral camouflage efficacy against airborne thermal surveillance assets.

- [ ] **Deterministic Autonomous Decision-Making & Rules-of-the-Road (COLREGs) Layer**
  - *Objective:* Integrating a fully deterministic, low-latency classical logic framework combined with International Regulations for Preventing Collisions at Sea (COLREGs). This layer will govern ethical maritime navigation constraints and autonomous threat-avoidance priorites without relying on non-deterministic cloud APIs.



## Credits & Acknowledgments

Water Normal Map: Derived from the Three.js core examples (water shader). This high-frequency normal map is used to enhance surface micro-turbulence and light reflection.



## Get in Touch

I am actively developing this framework and open to technical discussions, feedback, or potential collaboration opportunities. Feel free to reach out if you have questions about the implementation:

📧 Email:muserreftpz@gmail.com