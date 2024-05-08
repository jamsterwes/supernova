extern crate gl;
use std::ffi::c_void;

use gl::types::GLuint;

use rendering::shaders;
use rendering::textures;

pub struct Simulation {
    pub mass: GLuint,
    pub velocity: GLuint,
    pub temp: GLuint,
    advect_velocity_comp: GLuint,
    project_velocity_comp: GLuint,
}

pub trait Simulatable {
    fn simulate(&self);  // TODO: dt
}

// Create a simulation object
pub fn create_simulation() -> Simulation {
    // Create advection compute shader
    let advect_velocity_comp = shaders::build_compute(include_str!("../shaders/grids/advect_velocity.comp"));

    // Create projection compute shader
    let project_velocity_comp = shaders::build_compute(include_str!("../shaders/grids/project_velocity.comp"));

    return Simulation {
        mass: textures::create_grid_texture(),
        velocity: textures::create_grid_texture(),
        temp: textures::create_grid_texture(),
        advect_velocity_comp: advect_velocity_comp,
        project_velocity_comp: project_velocity_comp,
    }
}

// Implement simulation
impl Simulatable for Simulation {
    fn simulate(&self) {
        advect_velocity(self, 1.0 / 60.0);
        // project_velocity(self);
    }
}

// Helper function to advect velocity
fn advect_velocity(sim: &Simulation, dt: f32) {
    // Step 1: Get uniform
    let uniform_dt = shaders::get_uniform_location(sim.advect_velocity_comp, "dt");

    // Step 2: Advect velocity
    unsafe {
        // Set DT (todo: not 60FPS)
        gl::Uniform1f(uniform_dt, dt);

        // (0): velocity read
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, sim.velocity);
        // gl::BindImageTexture(0, sim.velocity, 0, gl::FALSE, 0, gl::READ_ONLY, gl::RGBA32F);

        // (1): velocity write
        gl::BindImageTexture(1, sim.temp, 0, gl::FALSE, 0, gl::WRITE_ONLY, gl::RGBA32F);

        // Dispatch program
        gl::UseProgram(sim.advect_velocity_comp);
        gl::DispatchCompute(crate::SCR_WIDTH, crate::SCR_HEIGHT, 1);
        gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);

        // Copy temp -> velocity
        textures::copy_grid_texture(sim.temp, sim.velocity);
    }
}

// Helper function to project velocity
fn project_velocity(sim: &Simulation) {
    // Step 1: Project velocity
    unsafe {
        // (0): velocity read
        gl::BindImageTexture(0, sim.velocity, 0, gl::FALSE, 0, gl::READ_ONLY, gl::RGBA32F);

        // (1): velocity write
        gl::BindImageTexture(1, sim.temp, 0, gl::FALSE, 0, gl::WRITE_ONLY, gl::RGBA32F);

        // Dispatch program
        gl::UseProgram(sim.project_velocity_comp);
        gl::DispatchCompute(crate::SCR_WIDTH, crate::SCR_HEIGHT, 1);
        gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);

        // Copy temp -> velocity
        textures::copy_grid_texture(sim.temp, sim.velocity);
    }
}