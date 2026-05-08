# Real-Time Digital Twin Prototype for USV Multispectral Camouflage Systems

## Project Vision
A modular real-time digital twin framework for simulating environmental interaction 
and visual signature behavior of Unmanned Surface Vehicles (USVs). This project focuses on building a real-time simulation environment to model multispectral camouflage responses under dynamic environmental conditions. Multispectral modeling is planned for future implementation and is not yet included.

## Current Demo 
![Current Simulation State](./demo.gif)

  The Current Prototype Demonstrates:

Physically-Based Orbital Motion: Integrated wave dynamics using transverse and longitudinal wave components (Sine/Cosine synthesis) to simulate realistic elliptical particle movement for surface entities.

Optics-Driven Environment (Beer-Lambert Law): Real-time light attenuation and underwater visibility calculations based on water turbidity and depth, ensuring optical fidelity in the simulation.

High-Performance Infinite Ocean Loop: A dual-plane "Vagon" tiling system implemented in Rust/Bevy, utilizing modulo-based coordinate wrapping for seamless, endless environmental rendering.

Atmospheric & Volumetric Fogging: Dynamic exponential fog settings that respond to underwater lighting conditions, simulating complex aquatic scattering effects.

Advanced PBR Water Surface: Custom StandardMaterial configuration featuring Fresnel-based reflectance, normal mapping for micro-surface detail, and alpha-blending for realistic transparency.

Interactive USV Control System: A modular vehicle controller with real-time maneuvering (WASD) integrated into the Bevy ECS (Entity Component System) architecture.

## Technical Framework & Implementation
To ensure maximum reliability and real-time performance, the project is architected with the following technologies:

* **Rust:** Chosen for its **memory safety** and high-performance computational efficiency, ensuring the simulation is robust enough for critical defense applications.
* **Bevy Engine:** Utilized as the primary **3D simulation environment**, providing a high-fidelity workspace to model physical interactions.
* **Rerun:** Employed for **real-time data visualization and logging**, allowing for the monitoring of live sensor streams and performance metrics.

## Architecture 
src/
* main.rs          # Application entry point
* constants.rs     # Physical and simulation constants
* environment.rs   # Environmental state modeling
* vehicle.rs       # USV entity creation & Real-time movement logic
* scene.rs         # Scene setup (camera, light, sea)
* models.rs        # Future sensor and optical models
* optics.rs        # Mechanism


## Theoretical Foundation & References
The core algorithms and optical models within this digital twin are grounded in rigorous electro-optical engineering principles. Key references used for system analysis, sensor modeling, and testing include:

* **Michael C. Dudzik** – *Electro-Optical Systems Design, Analysis, and Testing*
* **Cornelius J. Willers** – *Electro-Optical System Analysis and Design*
* **Sherman Karp** – *Fundamentals of Electro-Optics Systems Design*
* **William D. Rogatto** – *Electro-Optical Components*
* **George W. Masters** – *Electro-Optical Systems Test and Evaluation*

These references guide the future of implementation of sensor and optical response models.

## Project Status
Stage: Active Technical Prototype / Simulation Framework

Completed:

[x] Core Architecture: Modular Rust/Bevy-based simulation skeleton leveraging ECS (Entity Component System) for high-performance execution.

[x] Infinite Ocean Environment: Implemented a seamless dual-plane "Vagon" tiling system ensuring continuous optical flow without visual artifacts.

[x] Physical Wave Dynamics: Integrated Orbital Motion by synthesizing Transverse and Longitudinal wave components (Sine/Cosine phase shifting) for realistic entity buoyancy.

[x] Dynamic Optical Fog: Integrated Beer-Lambert Law-based light attenuation, dynamically adjusting underwater visibility based on depth and water turbidity.

[x] Advanced Material Surface: PBR (Physically Based Rendering) water surface with seamless normal mapping, Fresnel-based reflectance, and optimized roughness.

[x] Volumetric Atmospheric Effects: Synchronized directional and ambient lighting with exponential fog falloff to simulate complex maritime scattering.

[x] Adaptive Vessel Controller: Modular kinematic layer for USV movement with real-time WASD maneuvering and physical oscillation response.


In progress:

[ ] Environment-Driven Response: Developing reactive logic based on simulated sea states.

[ ] Sensor Simulation Layer: Initial work on ray-casting based LiDAR/Sonar placeholders.


Planned:

[ ] Multispectral Modeling: Modeling surface materials for different wavelength responses.

[ ] Infrared & Thermal Response: Simulating thermal signatures and IR sensor feedback.

[ ] Autonomous Decision Layer: Integrating classical logic and ethical decision-making frameworks for maritime navigation.