extern crate gl;

use gl::types::GLuint;

use glfw::ffi::glfwGetTime;
use rendering::shaders;
use rendering::textures;

// Particle data struct
pub struct Particle {
    pub position: (f32, f32),
    last_position: (f32, f32),
    acceleration: (f32, f32),
    mass: f32,
}

pub struct Simulation {
    pub particles: Vec<Particle>,
    last_t: f64,
}

pub trait Simulatable {
    fn simulate(&mut self);
    fn add_particle_with_momentum(&mut self, position: (f32, f32), mass: f32, momentum: (f32, f32));
}

// Create a simulation object
pub fn create_simulation() -> Simulation {
    return Simulation {
        particles: vec![],
        last_t: unsafe { glfwGetTime() as f64 },
    }
}

// Implement simulation
impl Simulatable for Simulation {
    fn simulate(&mut self) {
        // Step 1: Get dt (in sec)
        let t = unsafe { glfwGetTime() as f64 };
        let dt: f32 = self.last_t as f32 - t as f32;

        // Step 2: Verlet integrate particles
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

        // Next: ?
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
        };

        // Insert into particles list
        self.particles.push(particle);
    }
}