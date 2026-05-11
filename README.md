# Real-Time Digital Twin Prototype for USV Multispectral Camouflage Systems

## Project Vision
A modular real-time digital twin framework for simulating environmental interaction 
and visual signature behavior of Unmanned Surface Vehicles (USVs). This project focuses on building a real-time simulation environment to model multispectral camouflage responses under dynamic environmental conditions. Multispectral modeling is planned for future implementation and is not yet included.

## Current Demo 
![Current Simulation State](./demo.gif)

  The Current Prototype Demonstrates:

Procedural Foundation: Moved from static textures to a WGSL-based procedural wave model. The core mathematics for wave interference is now functional on the GPU, though visual refinement (Gerstner tuning) is ongoing.

Physics-UI Integration: Real-time synchronization between GUI parameters (Amplitude/Frequency) and the ocean surface is active.

Optical Modeling: Light attenuation is calculated using the Beer-Lambert Law, providing a physical basis for water color transitions.

Environmental Blending: Atmospheric fog has been integrated to manage horizon rendering and visibility range.

My current focus is on fine-tuning the vertex displacement for more natural wave shapes and perfecting the vessel's buoyancy response (staying flush with the wave surface).


## Technical Framework & Implementation
To ensure maximum reliability and real-time performance, the project is architected with the following technologies:

Rust: Chosen for its memory safety and high-performance computational efficiency, ensuring the simulation is robust and ready for safety-critical applications.

Bevy Engine: Utilized as the primary Data-Driven 3D environment, providing a high-fidelity workspace to model complex physical interactions via ECS.

bevy_egui: Employed for Real-time Parameter Orchestration, providing an integrated graphical interface to monitor and manipulate simulation variables on the fly.

## Architecture 
src/
main.rs * Application Orchestrator: Initializes the Bevy engine, registers global resources, and schedules system execution orders.

ui.rs * Interaction Layer: Implements the bevy_egui control panel. It acts as the primary interface for real-time parameter manipulation of the OceanSettings resource.

constants.rs * Physical Core: Defines the OceanSettings struct and stores hardcoded environmental constants (e.g., Sea level pressure, refractive index base).

optics.rs * Optical Physics Engine: Contains the mathematical implementations for Snell’s Law, Fresnel Reflectance, and the Beer-Lambert Law for light attenuation.

scene.rs * Environment & Rendering: Manages the infinite ocean tiling system, volumetric fog, and orbital wave oscillations. It bridges the physical data from optics.rs to the visual mesh.

vehicle.rs * Kinematic Controller: Defines the USV (Unmanned Surface Vehicle) entity, its spawn parameters, and the real-time movement logic responsive to sea state dynamics.

environment.rs * Atmospheric Modeling: Handles broader environmental states and global simulation parameters.

models.rs * Hardware Abstraction: Future module reserved for multispectral sensor models and advanced camera optics.


## Theoretical Foundation & References
The core algorithms and optical models within this digital twin are grounded in rigorous electro-optical engineering principles. Key references used for system analysis, sensor modeling, and testing include:

* **Michael C. Dudzik** – *Electro-Optical Systems Design, Analysis, and Testing*
* **Cornelius J. Willers** – *Electro-Optical System Analysis and Design*
* **Sherman Karp** – *Fundamentals of Electro-Optics Systems Design*
* **William D. Rogatto** – *Electro-Optical Components*
* **George W. Masters** – *Electro-Optical Systems Test and Evaluation*

These references guide the future of implementation of sensor and optical response models.

## Project Status
Stage: Active Technical Prototype / Real-Time Simulation Framework

Completed:

[x] Procedural Wave Synthesis (WGSL): Migrated from static tiling textures to a dynamic, GPU-calculated wave interference model. Using WGSL shaders, the ocean surface now generates non-repeating, procedural ripples based on multi-layered sine/cosine interference, eliminating visual repetition artifacts.

[x] Multispectral Optical Modeling: Replaced basic color shifting with a physics-based Beer-Lambert Law implementation. Light attenuation is calculated per spectral channel (R, G, B), accurately simulating the rapid absorption of red wavelengths to produce realistic depth-dependent cyan/blue shifts.

[x] Dynamic Visibility & Secchi Depth: Integrated real-time calculation of underwater transparency limits based on the turbidity coefficient. The UI provides live scientific feedback on visual range, synchronized with atmospheric fog density.


In progress:

[ ] Temporal Smoothing (LERP): Implementing Linear Interpolation for all UI-driven transitions (Turbidity, Amplitude, Frequency). The goal is to eliminate sudden visual jumps and ensure smooth, cinematic environmental shifts.

[ ] 6-DOF Buoyancy Physics: Developing the real-time link between the procedural wave height and the USV's Transform. This will allow the vessel to physically pitch, roll, and heave according to the simulated sea state.

[ ] Optical Engine Optimization: Fine-tuning the WGSL fragment shader to normalize brightness spikes during high-turbidity shifts, ensuring consistent exposure across different sea conditions.

[ ] Sensor Simulation Layer (LiDAR/Sonar): Initial R&D on ray-casting logic within the Bevy ECS to simulate basic distance sensors for autonomous navigation testing.


Planned:

[ ] Infrared & Thermal Response: Simulating thermal signatures and IR sensor feedback.

[ ] Autonomous Decision Layer: Integrating classical logic and ethical decision-making frameworks for maritime navigation.