mod rendering;
use rendering::{general::RenderGrid};
use rendering::shapes::circle::{DrawCircle, create_circle_renderer};

mod simulation;

mod window;

extern crate glfw;
use glfw::{Context, Key, Action, GlfwReceiver};
use simulation::particles::{create_simulation, Simulatable};

extern crate gl;

// Settings
const SCR_WIDTH: u32 = 1920;
const SCR_HEIGHT: u32 = 1080;
const SCR_TITLE: &'static str = "supernova";

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

    // Create a simulation
    let mut sim = create_simulation();

    // Create a circle renderer
    let circle_renderer = create_circle_renderer();

    // Generate circles
    let positions: Vec<(f32, f32)> = vec![
        (100.0, 100.0),
        (200.0, 800.0),
        (700.0, 500.0),
        (1100.0, 300.0),
        (256.0, 128.0),
        (208.0, 401.0),
    ];
    
    // Populate simulation
    for pos in positions {
        sim.add_particle_with_momentum(pos, 5.0, (10.0, 10.0));
    }

    // Render loop
    while !window.should_close() {
        // Process events
        process_events(&mut window, &events);

        // Clear background
        unsafe { 
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Simulate
        sim.simulate();
        
        // Draw simulation
        for particle in &sim.particles {
            circle_renderer.draw(particle.position, 16.0, (1.0, 0.0, 0.0, 1.0))
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

            // Click events
            glfw::WindowEvent::MouseButton(glfw::MouseButtonLeft, action, _) => {
                if action == glfw::Action::Release {
                    // state.mousel_held = false;
                } else {
                    // state.mousel_held = true;
                }
            },

            // Click events
            glfw::WindowEvent::MouseButton(glfw::MouseButtonRight, action, _) => {
                if action == glfw::Action::Release {
                    // state.mouser_held = false;
                } else {
                    // state.mouser_held = true;
                }
            },

            glfw::WindowEvent::CursorPos(x, y) => {
                // ...
            },

            // Key events
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),

            // Fallthrough case
            _ => {}
        }
    }
}