mod rendering;
use glfw::ffi::glfwGetTime;
use rendering::shapes::circle::{DrawCircle, create_circle_renderer};

mod simulation;

mod window;

extern crate glfw;
use glfw::{Context, Key, Action, GlfwReceiver};
use simulation::particles::{create_simulation, Simulatable, Simulation, ParticleType};

extern crate gl;

// Settings
const SCR_WIDTH: u32 = 1920;
const SCR_HEIGHT: u32 = 1080;
const SCR_TITLE: &'static str = "supernova";
const N_RADIUS: f32 = 4.0;
const FISSILE_RADIUS: f32 = 16.0;
const N_MOMENTUM: f32 = 100.0;

// Entrypoint
pub fn main() {
    // Init GLFW
    let mut glfw = window::init_glfw();

    // Create a window
    let (mut window, events) = window::create_window(&mut glfw, window::WindowSettings {
        width: SCR_WIDTH,
        height: SCR_HEIGHT,
        title: String::from(SCR_TITLE),
    });

    // Create a simulation (200x200 collision cells)
    let mut sim = create_simulation(200);

    // Create a circle renderer
    let circle_renderer = create_circle_renderer();

    // Store last spawn time
    let mut last_spawn_time = -1000.0 as f64;
    let mut last_cull_time = unsafe {glfwGetTime() as f64};

    // Render loop
    while !window.should_close() {
        let t = unsafe {glfwGetTime() as f64};

        // Process events
        process_events(&mut window, &events);

        // Clear background
        unsafe { 
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Is Space down?
        let space_down = window.get_key(glfw::Key::Space) == glfw::Action::Press;

        // Simulate
        // Should we cull?
        if t - last_cull_time > 1.0 {
            last_cull_time = t;
            sim.simulate(true, space_down);
        } else {
            sim.simulate(false, space_down);
        }

        // Is CTRL down?
        let ctrl_down = window.get_key(glfw::Key::LeftControl) == glfw::Action::Press;
        
        // Spawn particle (if mouse button down)
        if t - last_spawn_time > 0.1 && window.get_mouse_button(glfw::MouseButtonLeft) == glfw::Action::Press && !ctrl_down {
            // Get cursor position
            let pos = window.get_cursor_pos();

            // Generate random momentum
            let mx = (rand::random::<f32>() - 0.5) * N_MOMENTUM;
            let my = (rand::random::<f32>() - 0.5) * N_MOMENTUM;
            sim.add_particle_with_momentum((pos.0 as f32, SCR_HEIGHT as f32 - pos.1 as f32), N_RADIUS, (mx, my));

            // Set last spawn time
            last_spawn_time = t;
        }

        // Spawn reflector
        if t - last_spawn_time > 0.01 && window.get_mouse_button(glfw::MouseButtonLeft) == glfw::Action::Press && ctrl_down {
            // Get cursor position
            let pos = window.get_cursor_pos();

            // Add fuel
            sim.add_reflector((pos.0 as f32, SCR_HEIGHT as f32 - pos.1 as f32), 16.0);

            // Set last spawn time
            last_spawn_time = t;
        }
        
        // Spawn neutron
        if t - last_spawn_time > 0.01 && window.get_mouse_button(glfw::MouseButtonRight) == glfw::Action::Press && !ctrl_down {
            // Get cursor position
            let pos = window.get_cursor_pos();

            // Add fuel
            sim.add_fissile((pos.0 as f32, SCR_HEIGHT as f32 - pos.1 as f32), FISSILE_RADIUS);

            // Set last spawn time
            last_spawn_time = t;
        }
        
        // Spawn starter cap
        // if t - last_spawn_time > 0.01 && window.get_mouse_button(glfw::MouseButtonRight) == glfw::Action::Press && ctrl_down {
        //     // Get cursor position
        //     let pos = window.get_cursor_pos();

        //     // Add fuel
        //     sim.add_starter_cap((pos.0 as f32, SCR_HEIGHT as f32 - pos.1 as f32), 4.0);

        //     // Set last spawn time
        //     last_spawn_time = t;
        // }
        
        // Draw simulation
        for particle in &sim.particles {
            if particle.particle_type == ParticleType::Neutron {
                circle_renderer.draw(particle.position, particle.mass, (1.0, 0.0, 0.0, 1.0))
            } else if particle.particle_type == ParticleType::Fissile {
                circle_renderer.draw(particle.position, particle.mass, (0.0, 1.0, 0.0, 1.0))
            } else if particle.particle_type == ParticleType::StarterCap {
                circle_renderer.draw(particle.position, particle.mass, (1.0, 1.0, 0.0, 1.0))
            } else {
                circle_renderer.draw(particle.position, particle.mass, (1.0, 1.0, 1.0, 1.0))
            }
        }

        // Swap buffers (present what we just drew)
        window.swap_buffers();

        // Poll for events
        glfw.poll_events();
    }
}

// Function for handling events
fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
    // Loop through all flushed messages
    for (_, event) in glfw::flush_messages(events) {
        // Match events by type
        match event {
            // Resize event
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) };
            }

            // Key events
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),

            // Fallthrough case
            _ => {}
        }
    }
}