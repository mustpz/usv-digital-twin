# Real-Time Digital Twin Prototype for USV Multispectral Camouflage Systems

## Project Vision
A modular real-time digital twin framework for simulating environmental interaction 
and visual signature behavior of Unmanned Surface Vehicles (USVs). This project focuses on building a real-time simulation environment to model multispectral camouflage responses under dynamic environmental conditions. Multispectral modeling is planned for future implementation and is not yet included.

## Current Demo 
![Current Simulation State](./demo.gif) 

  The Current Prototype:

Implemented a Closed-Loop Signature Management System that dynamically samples environmental optical data (Turbidity, Ocean Type) to adjust the USV's visual signature in real-time. This prototype serves as a Proof of Concept (PoC) to demonstrate the feasibility of Adaptive Camouflage in autonomous maritime platforms, bridging the gap between theoretical photonics and practical naval stealth applications.

 Transitioned from simple sine waves to a multi-layered Gerstner Wave model. This provides realistic "sharp" crests and horizontal vertex displacement, creating a naturally turbulent sea state.

 Implemented an adaptive foam system triggered by wave height. To eliminate visual repetition, I used a triple-overlay normal map technique with asymmetrical panning.

 The USV (vehicle) buoyancy is 1:1 synchronized with the GPU displacement. The vessel now realistically responds with Pitch and Roll alignment based on the wave slopes.

 Ocean color is no longer static. It is dynamically calculated using the Beer-Lambert Law, where Turbidity acts as a physical extinction coefficient, affecting light absorption and scattering.

 Why I Chose This Over FFT (For Now)?

 Hardware Scalability: Optimized for 60+ FPS on mid-range hardware (like RTX 3050) by avoiding heavy FFT compute overhead.

 Gerstner waves allow instant CPU-side physics calculations, ensuring zero lag between the visual ocean and the vehicle's movement.

 This hybrid model offers total control over environmental parameters (Salinity, Turbidity, Sea State), which is essential for a Digital Twin focused on sensor testing.

 The goal of this module is not just visual aesthetics, but to show that autonomous systems can interpret environmental physics to make tactical survival decisions without human intervention.

## Technical Framework & Implementation
To ensure maximum reliability and real-time performance, the project is architected with the following technologies:

Core Engine: Rust  – Leveraging memory safety and zero-cost abstractions for high-performance simulation logic.

Framework: Bevy 0.13 – Utilizing a data-driven Entity Component System (ECS) for massive parallelization of maritime entities.

Shading & Physics: WGSL  – Custom WebGPU Shading Language implementations for procedural wave generation and real-time spectral light attenuation.

User Interface: bevy_egui – Integrated immediate-mode GUI for real-time manipulation of optical and hydrodynamic parameters.

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

[x] Prototyped an autonomous environmental perception layer that dynamically synchronizes the USV’s optical signature with real-time ocean turbidity and spectral data, proving the feasibility of adaptive stealth logic in maritime digital twins.

[x] Gerstner Wave Synthesis & Physics Sync: Implemented a multi-layered Gerstner displacement model with 1:1 CPU-GPU synchronization, enabling the USV to realistically align its pitch and roll with procedural wave slopes.

[x] Photonic Ocean Rendering: Integrated a physics-based Beer-Lambert light attenuation model where turbidity acts as an extinction coefficient, dynamically calculating spectral shifts and visibility ranges.

[x] Adaptive Surface Detail: Developed a procedural foam system and triple-layered normal map noise to simulate high-frequency sea surface turbulence and crest-dependent foam generation.

[x] Coupled Atmospherics: Synchronized volumetric fog density with maritime turbidity levels to create a cohesive and strategically consistent environmental simulation.


In progress:

[ ] Adaptive Wake & Splash Simulation: Developing a particle-based system to simulate water displacement and spray behind the USV's hull as it navigates through high-amplitude waves.

[ ] Sensor Fusion Layer (LiDAR/Radar): Researching ray-casting logic within the Bevy ECS to simulate autonomous navigation sensors, enabling distance detection against the procedural ocean surface.

[ ] Dynamic Day/Night Cycle: Integrating a solar-tracking system to calculate time-of-day dependent light scattering and its impact on the USV's optical sensor simulation.

[ ] Full Multispectral Engine: Developing a more granular wavelength-dependent absorption model (400nm to 900nm) to simulate non-visible spectrum sensors (NIR/SWIR) for advanced USV perception testing.


Planned:

[ ] Infrared & Thermal Response: Simulating thermal signatures and IR sensor feedback.

[ ] Autonomous Decision Layer: Integrating classical logic and ethical decision-making frameworks for maritime navigation.



## Credits & Acknowledgments

Water Normal Map: Derived from the Three.js core examples (water shader). This high-frequency normal map is used to enhance surface micro-turbulence and light reflection.



## Get in Touch

I am actively developing this framework and open to technical discussions, feedback, or potential collaboration opportunities. Feel free to reach out if you have questions about the implementation:

📧 Email:muserreftpz@gmail.com