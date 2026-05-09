# Real-Time Digital Twin Prototype for USV Multispectral Camouflage Systems

## Project Vision
A modular real-time digital twin framework for simulating environmental interaction 
and visual signature behavior of Unmanned Surface Vehicles (USVs). This project focuses on building a real-time simulation environment to model multispectral camouflage responses under dynamic environmental conditions. Multispectral modeling is planned for future implementation and is not yet included.

## Current Demo 
![Current Simulation State](./demo.gif)

  The Current Prototype Demonstrates:

Interactive Physics: Real-time manipulation of sea states and vessel dynamics via an integrated GUI.

Optical Fidelity: Physical modeling of light behavior underwater using the Beer-Lambert Law and Snell’s Law.

Infinite Simulation: Seamless environmental looping for long-endurance USV mission testing.

ECS Architecture: High-performance, data-driven framework built with Rust for maximum memory safety and parallel execution.

Atmospheric Realism: Volumetric fog and PBR-based water surfaces that simulate realistic maritime conditions.

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

[x] Core Architecture: Modular Rust/Bevy-based simulation skeleton leveraging ECS (Entity Component System) for high-performance, memory-safe execution.

[x] Real-Time Control Interface: Integrated a dynamic Graphical User Interface (GUI) using bevy_egui, allowing live manipulation of physical, optical, and navigational parameters without restarting the simulation.

[x] Infinite Ocean Environment: Implemented a seamless dual-plane tiling system ensuring continuous optical flow and environment looping.

[x] Physical Wave Dynamics: Integrated Orbital Motion by synthesizing Transverse and Longitudinal wave components (Sine/Cosine phase shifting). This creates a non-linear, high-fidelity surface oscillation that directly affects entity behavior.

[x] Dynamic Optical Modeling: Implemented the Beer-Lambert Law for light attenuation. Underwater visibility and fog density are dynamically calculated based on real-time Turbidity (Absorption) and Depth variables.

[x] Environmental Physics Resource: Centralized simulation state management using Rust Resources, enabling synchronized updates across the physics, optics, and rendering layers.

[x] Advanced Material Surface: PBR (Physically Based Rendering) water surface with seamless normal mapping, Fresnel-based reflectance, and optimized specular roughness for realistic sea-surface glint.

[x] Kinematic Vessel Controller: Modular maneuvering layer for USV movement with real-time velocity adjustments and physical oscillation response.


In progress:

[ ] Environment-Driven Response: Developing reactive logic based on simulated sea states.

[ ] Sensor Simulation Layer: Initial work on ray-casting based LiDAR/Sonar placeholders.


Planned:

[ ] Multispectral Modeling: Modeling surface materials for different wavelength responses.

[ ] Infrared & Thermal Response: Simulating thermal signatures and IR sensor feedback.

[ ] Autonomous Decision Layer: Integrating classical logic and ethical decision-making frameworks for maritime navigation.