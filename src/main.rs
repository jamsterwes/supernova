mod rendering;
use rendering::general::RenderGrid;

mod simulation;

mod window;

extern crate glfw;
use glfw::{Context, Key, Action, GlfwReceiver};

extern crate gl;
use simulation::{Simulatable, Simulation};

// Settings
const SCR_WIDTH: u32 = 1024;
const SCR_HEIGHT: u32 = 1024;
const SCR_TITLE: &'static str = "supernova";

// Store state
struct State {
    simulation: Simulation,
    mousel_held: bool,
    mouser_held: bool,
}

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

    // Create neutron grid renderer
    let neutron_renderer = rendering::general::make_grid_renderer(include_str!("../shaders/grids/velocity.frag"));

    // Store state
    let mut state = State {
        simulation: simulation::create_simulation(),
        mousel_held: false,
        mouser_held: false,
    };

    // Render loop
    while !window.should_close() {
        // Process events
        process_events(&mut window, &mut state, &events);

        // Simulate
        state.simulation.simulate();

        // Clear background
        unsafe { 
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Render neutron grid
        neutron_renderer.render_grid(state.simulation.velocity);

        // Swap buffers (present what we just drew)
        window.swap_buffers();

        // Poll for events
        glfw.poll_events();
    }
}

// Function for handling events
fn process_events(window: &mut glfw::Window, state: &mut State, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
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
                    state.mousel_held = false;
                } else {
                    state.mousel_held = true;
                }
            },

            // Click events
            glfw::WindowEvent::MouseButton(glfw::MouseButtonRight, action, _) => {
                if action == glfw::Action::Release {
                    state.mouser_held = false;
                } else {
                    state.mouser_held = true;
                }
            },

            glfw::WindowEvent::CursorPos(x, y) => {
                // Handle adding velocity
                if state.mousel_held {
                    // Write a particle
                    rendering::textures::write_grid_texture_patch(state.simulation.velocity, x as usize, y as usize, 8, 8, (6.0, 0.0, 0.0, 0.0));
                    // rendering::textures::write_grid_texture_patch(state.simulation.mass, x as usize, y as usize, 8, 8, (0.0, 0.5, 1.0, 0.0));
                }

                // Handle adding mass
                if state.mouser_held {
                    // Write a particle
                    rendering::textures::write_grid_texture_patch(state.simulation.velocity, x as usize, y as usize, 8, 8, (0.0, 6.0, 0.0, 0.0));
                    // rendering::textures::write_grid_texture_patch(state.simulation.mass, x as usize, y as usize, 8, 8, (0.0, 1.0, 0.5, 0.0));
                }
            },

            // Key events
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),

            // Fallthrough case
            _ => {}
        }
    }
}