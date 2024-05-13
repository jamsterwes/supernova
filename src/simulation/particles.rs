extern crate gl;

use glfw::ffi::glfwGetTime;

const G: f32 = 0.001;

// Enum for particle type
#[derive(Clone, Copy, PartialEq)]
pub enum ParticleType {
    Neutron,
    Fissile,
    Reflector,
    StarterCap
}

// Particle data struct
#[derive(Clone, Copy)]
pub struct Particle {
    pub position: (f32, f32),
    last_position: (f32, f32),
    acceleration: (f32, f32),
    pub mass: f32,  // For now, mass = radius
    pub particle_type: ParticleType,
}

pub struct Simulation {
    pub particles: Vec<Particle>,
    grid_res: usize,
    last_t: f64,
}

pub trait Simulatable {
    // Simulation steps
    fn simulate(&mut self, cull: bool, detonate: bool);
    fn integrate(&mut self, dt: f32);
    fn construct_grid(&self) -> Vec<Vec<usize>>;
    fn resolve_collisions(&mut self, grid: &Vec<Vec<usize>>);

    // Add particle
    fn defer_particle_with_momentum(&mut self, position: (f32, f32), mass: f32, momentum: (f32, f32)) -> Particle;
    fn add_particle_with_momentum(&mut self, position: (f32, f32), mass: f32, momentum: (f32, f32));
    fn add_fissile(&mut self, position: (f32, f32), mass: f32);
    fn add_reflector(&mut self, position: (f32, f32), mass: f32);
    fn add_starter_cap(&mut self, position: (f32, f32), mass: f32);
}

// Create a simulation object
pub fn create_simulation(grid_res: usize) -> Simulation {
    return Simulation {
        particles: vec![],
        grid_res: grid_res,
        last_t: unsafe { glfwGetTime() as f64 },
    }
}

// Implement simulation
impl Simulatable for Simulation {
    fn simulate(&mut self, cull: bool, detonate: bool) {
        // Step 1: Get dt (in sec)
        let t = unsafe { glfwGetTime() as f64 };
        let dt: f32 = self.last_t as f32 - t as f32;

        // Step 1b: Detonate starter caps?
        if detonate {
            let mut to_add: Vec<Particle> = vec![];
            for i in 0..self.particles.len() {
                if self.particles[i].particle_type == ParticleType::StarterCap && self.particles[i].mass > 0.0 {

                    // Random momentum
                    for _ in 0..10 {
                        let mx = (rand::random::<f32>() - 0.5) * crate::N_MOMENTUM;
                        let my = (rand::random::<f32>() - 0.5) * crate::N_MOMENTUM;
                        to_add.push(self.defer_particle_with_momentum(self.particles[i].position, crate::N_RADIUS, (mx, my)));
                    }

                    self.particles[i].mass = 0.0;
                    self.particles[i].position.0 = -100000.0;
                }
            }
            self.particles.append(&mut to_add);
        }

        // DO IN SUBSTEPS
        let mut last_grid = vec![];
        for _ in 0..8 {
            // Step 2: Resolve collisions with floor
            for i in 0..self.particles.len() {
                // Get y-component
                let y = self.particles[i].position.1;

                // Get radius
                let radius = self.particles[i].mass;

                // If y-component is < radius, resolve
                if y < radius {
                    // Collision:
                    // Move by +/- 1/2 * normal * collision depth
                    let depth = radius - y;
                    self.particles[i].position.1 += 0.5 * depth;
                }
            }

            // Step 3: Resolve collisions
            let grid = self.construct_grid();
            self.resolve_collisions(&grid);

            // Step 4: Verlet integrate particles
            self.integrate(dt / 2.0);

            last_grid = grid;
        }

        // Step 5: Cull
        if cull {
            // Get all in-bounds particles
            let mut new_particles: Vec<Particle> = vec![];
            for cell in &last_grid {
                for i in cell {
                    new_particles.push(self.particles[*i]);
                }
            }

            // Update
            self.particles = new_particles;
        }
    }

    fn integrate(&mut self, dt: f32) {
        for i in 0..self.particles.len() {
            // Get x_n, x_(n-1), a
            let pres = self.particles[i].position;
            let past = self.particles[i].last_position;
            let accel = self.particles[i].acceleration;

            // Get next x, next y
            let xnext = 2.0 * pres.0 - past.0 + accel.0 * dt*dt;
            let ynext = 2.0 * pres.1 - past.1 + accel.1 * dt*dt;

            // Update position/last_position
            self.particles[i].last_position = self.particles[i].position;
            self.particles[i].position = (xnext, ynext);
        }
    }

    // Construct a fixed-grid index of all particles
    fn construct_grid(&self) -> Vec<Vec<usize>> {
        // Step 1: Allocate grid
        let mut grid: Vec<Vec<usize>> = vec![vec![]; self.grid_res * self.grid_res];

        // Step 2: Assign particles to their cells
        for i in 0..self.particles.len() {
            let pos = self.particles[i].position;

            // Step 1: Is particle even in bounds?
            if pos.0 < 0.0 || pos.0 >= crate::SCR_WIDTH as f32 || pos.1 < 0.0 || pos.1 >= crate::SCR_HEIGHT as f32 {
                continue;
            }

            // Step 2: Calculate grid coordinates
            let cx = (pos.0 / (self.grid_res * crate::SCR_WIDTH as usize) as f32) as usize;
            let cy = (pos.1 / (self.grid_res * crate::SCR_HEIGHT as usize) as f32) as usize;

            // Step 3: Insert into proper cell
            grid[cx+cy*self.grid_res].push(i);
        }

        return grid;
    }

    fn resolve_collisions(&mut self, grid: &Vec<Vec<usize>>) {
        let mut to_add: Vec<Particle> = vec![];

        for cx in 0..self.grid_res {
            for cy in 0..self.grid_res {
                // Get the particles in this cell
                let cell_particles = &grid[cx+cy*self.grid_res];

                // Loop O(n^2) over this cell
                for i in 0..cell_particles.len() {
                    for j in (i+1)..cell_particles.len() {

                        // Get i&j cell
                        let ci = cell_particles[i];
                        let cj = cell_particles[j];

                        // Ignore neutron-less collisions
                        if self.particles[ci].particle_type != ParticleType::Neutron && self.particles[cj].particle_type != ParticleType::Neutron {
                            continue;
                        }

                        // Contains non-neutron?
                        let contains_non_neutron = self.particles[ci].particle_type != ParticleType::Neutron || self.particles[cj].particle_type != ParticleType::Neutron;

                        // Compare radii
                        let ri = self.particles[ci].mass;
                        let rj = self.particles[cj].mass;
        
                        // Compare positions
                        let pi = self.particles[ci].position;
                        let pj = self.particles[cj].position;
                        let dx = pi.0 - pj.0;
                        let dy = pi.1 - pj.1;
                        let distance = (dx*dx + dy*dy).sqrt();
        
                        // If collision
                        if distance < ri + rj {
                            // Collision:
                            // Move by +/- 1/2 * normal * collision depth
                            let depth = (ri + rj) - distance;
                            let nx = dx / distance;
                            let ny = dy / distance;
                            
                            if self.particles[ci].particle_type == ParticleType::Neutron {
                                self.particles[ci].position.0 += (if contains_non_neutron { 1.0 } else { 0.5 }) * depth * nx;
                                self.particles[ci].position.1 += (if contains_non_neutron { 1.0 } else { 0.5 }) * depth * ny;
                            } else if self.particles[ci].particle_type == ParticleType::Fissile {
                                // Diminish mass
                                self.particles[ci].mass -= 1.0;

                                // Spawn a neutron (25% chance)
                                if rand::random::<f32>() > 0.75 {
                                    let ipx = (self.particles[cj].position.0 - self.particles[cj].last_position.0) * crate::N_RADIUS;
                                    let ipy = (self.particles[cj].position.1 - self.particles[cj].last_position.1) * crate::N_RADIUS;
                                    to_add.push(self.defer_particle_with_momentum(
                                        self.particles[cj].position, 
                                        crate::N_RADIUS, 
                                        (ipx, ipy)
                                    ));
                                }

                                // If mass is < 0, teleport OOB
                                if self.particles[ci].mass < 0.0 {
                                    self.particles[ci].position.0 -= 100000.0;
                                }
                            } else {
                                // Diminish mass
                                self.particles[ci].mass -= 0.1;

                                // If mass is < 0, teleport OOB
                                if self.particles[ci].mass < 0.0 {
                                    self.particles[ci].position.0 -= 100000.0;
                                }
                            }

                            if self.particles[cj].particle_type == ParticleType::Neutron {
                                self.particles[cj].position.0 -= (if contains_non_neutron { 1.0 } else { 0.5 }) * depth * nx;
                                self.particles[cj].position.1 -= (if contains_non_neutron { 1.0 } else { 0.5 }) * depth * ny;
                            } else if self.particles[ci].particle_type == ParticleType::Fissile {
                                // Diminish mass
                                self.particles[cj].mass -= 1.0;

                                // Spawn a neutron (25% chance)
                                if rand::random::<f32>() > 0.75 {
                                    let ipx = (self.particles[ci].position.0 - self.particles[ci].last_position.0) * crate::N_RADIUS;
                                    let ipy = (self.particles[ci].position.1 - self.particles[ci].last_position.1) * crate::N_RADIUS;
                                    to_add.push(self.defer_particle_with_momentum(
                                        self.particles[ci].position, 
                                        crate::N_RADIUS, 
                                        (ipx, ipy)
                                    ));
                                }

                                // If mass is < 0, teleport OOB
                                if self.particles[cj].mass < 0.0 {
                                    self.particles[cj].position.0 -= 100000.0;
                                }
                            } else {
                                // Diminish mass
                                self.particles[cj].mass -= 0.1;

                                // If mass is < 0, teleport OOB
                                if self.particles[cj].mass < 0.0 {
                                    self.particles[cj].position.0 -= 100000.0;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Spawn the to_add particles
        self.particles.append(&mut to_add);
    }

    fn defer_particle_with_momentum(&mut self, position: (f32, f32), mass: f32, momentum: (f32, f32)) -> Particle {
        // p = mv -> v = p/m
        let vx = momentum.0 / mass;
        let vy = momentum.1 / mass;

        // Create particle
        Particle {
            position: position,
            last_position: (position.0 - vx, position.1 - vy),
            acceleration: (0.0, 0.0),
            mass: mass,
            particle_type: ParticleType::Neutron,
        }
    }

    fn add_particle_with_momentum(&mut self, position: (f32, f32), mass: f32, momentum: (f32, f32)) {
        // p = mv -> v = p/m
        let vx = momentum.0 / mass;
        let vy = momentum.1 / mass;

        // Create particle
        let particle = Particle {
            position: position,
            last_position: (position.0 - vx, position.1 - vy),
            acceleration: (0.0, 0.0),
            mass: mass,
            particle_type: ParticleType::Neutron,
        };

        // Insert into particles list
        self.particles.push(particle);
    }

    fn add_fissile(&mut self, position: (f32, f32), mass: f32) {
        // Create particle
        let particle = Particle {
            position: position,
            last_position: position,
            acceleration: (0.0, 0.0),
            mass: mass,
            particle_type: ParticleType::Fissile,
        };

        // Insert into particles list
        self.particles.push(particle);
    }

    fn add_reflector(&mut self, position: (f32, f32), mass: f32) {
        // Create particle
        let particle = Particle {
            position: position,
            last_position: position,
            acceleration: (0.0, 0.0),
            mass: mass,
            particle_type: ParticleType::Reflector,
        };

        // Insert into particles list
        self.particles.push(particle);
    }

    fn add_starter_cap(&mut self, position: (f32, f32), mass: f32) {
        // Create particle
        let particle = Particle {
            position: position,
            last_position: position,
            acceleration: (0.0, 0.0),
            mass: mass,
            particle_type: ParticleType::StarterCap,
        };

        // Insert into particles list
        self.particles.push(particle);
    }
}