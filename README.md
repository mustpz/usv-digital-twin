# Real-Time Digital Twin Prototype for USV Multispectral Camouflage Systems

## Project Vision
A modular real-time digital twin framework for simulating environmental interaction 
and visual signature behavior of Unmanned Surface Vehicles (USVs). This project focuses on building a real-time simulation environment to model multispectral camouflage responses under dynamic environmental conditions. Multispectral modeling is planned for future implementation and is not yet included.

## Current Demo 
![Current Simulation State](./demo.gif)

  The Current Prototype Demonstrates:

Interactive Physics: Real-time manipulation of sea states and vessel dynamics via an integrated GUI.

Optical Fidelity: Physical modeling of light behavior underwater using the Beer-Lambert Law and Snell’s Law.

ECS Architecture: High-performance, data-driven framework built with Rust for maximum memory safety and parallel execution.

Atmospheric Realism: Volumetric fog and PBR-based water surfaces that simulate realistic maritime conditions.

This is an early multispectral digital twin prototype. While the core physics (Beer-Lambert, Snell's Law) are fully functional, you might encounter minor flickering and tiling seams. These are priority items on the roadmap, and I am actively working on moving the optical calculations entirely to WGSL shaders for flawless performance and visual stability.


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

[x] Multispectral Optical Modeling: Replaced basic color shifting with a physics-based Beer-Lambert Law implementation. Light attenuation is now calculated per spectral channel (R, G, B), accurately simulating the rapid absorption of red wavelengths compared to blue, resulting in realistic depth-dependent cyan/blue color shifts.

[x] Dynamic Visibility Metrics: Integrated real-time calculation of Secchi Depth (transparency limit) based on the turbidity coefficient. The UI provides live scientific feedback on underwater visibility distances.

[x] Temporal Smoothing (LERP): Implemented Linear Interpolation for optical transitions. Sudden changes in turbidity or light intensity are smoothed over time to prevent visual flickering and ensure a stable, high-fidelity simulation experience.

[x] Adaptive Alpha-Modulation: Underwater opacity and fog density are dynamically coupled with absorption variables, creating a volumetric "murkiness" effect that reflects environmental changes.


In progress:

[ ] Currently optimizing the optical engine to eliminate sudden brightness spikes during real-time turbidity shifts.

[ ] Implementing sub-pixel precision for plane tiling to remove visual seams during high-speed USV navigation.

[ ] Environment-Driven Response: Developing reactive logic based on simulated sea states.

[ ] Sensor Simulation Layer: Initial work on ray-casting based LiDAR/Sonar placeholders.


Planned:

[ ] Multispectral Modeling: Modeling surface materials for different wavelength responses.

[ ] Infrared & Thermal Response: Simulating thermal signatures and IR sensor feedback.

[ ] Autonomous Decision Layer: Integrating classical logic and ethical decision-making frameworks for maritime navigation.