# Real-Time Digital Twin Prototype for USV Multispectral Camouflage Systems

## Project Vision
A modular real-time digital twin framework for simulating environmental interaction 
and visual signature behavior of Unmanned Surface Vehicles (USVs). This project focuses on building a real-time simulation environment to model multispectral camouflage responses under dynamic environmental conditions. Multispectral modeling is planned for future implementation and is not yet included.

## Current Demo 
![Current Simulation State](./demo.gif)

  The Current Prototype Demonstrates:

- Real-time 3D simulation environment
- USV placeholder entity (vehicle model)
- Environmental surface representation (sea plane)
- Camera and lighting system
- Modular simulation architecture
- Interactive Maneuvering: Real-time movement control using WASD keys.
- Modular Controller: A dedicated vehicle control system integrated with the Bevy engine.

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
[x] Core Architecture: Modular Rust/Bevy-based simulation skeleton.

[x] Infinite Ocean Environment: Implemented a seamless dual-plane tiling system for continuous optical flow.

[x] Dynamic Optical Fog: Integrated Beer-Lambert Law-based light attenuation for realistic underwater depth perception.

[x] Physical Material Surface: Integrated normal mapping with fine-tuned specular reflectance and roughness.

[x] Vessel Motion: Core kinematic layer for USV movement simulation.

[x] Global Lighting: Directional and ambient light synchronization for maritime environments.

In progress:
[ ] Procedural Water Surfaces: Implementing noise-based (Perlin/Gerstner) textures to eliminate tiling artifacts.

[ ] Environment-Driven Response: Developing reactive logic based on simulated sea states.

[ ] Sensor Simulation Layer: Initial work on ray-casting based LiDAR/Sonar placeholders.

Planned:
[ ] Multispectral Modeling: Modeling surface materials for different wavelength responses.

[ ] Infrared & Thermal Response: Simulating thermal signatures and IR sensor feedback.

[ ] Autonomous Decision Layer: Integrating classical logic and ethical decision-making frameworks for maritime navigation.