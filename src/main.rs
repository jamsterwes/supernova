mod rendering;

use gl::types::GLuint;
use rendering::general::RenderGrid;

mod window;

extern crate glfw;

use self::glfw::{Context, Key, Action, GlfwReceiver};

extern crate gl;

// Settings
const SCR_WIDTH: u32 = 1024;
const SCR_HEIGHT: u32 = 1024;
const SCR_TITLE: &'static str = "supernova";

// Store state
struct State {
    neutron_tex: GLuint,
    mouse_held: bool,
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
    let neutron_renderer = rendering::general::make_grid_renderer(include_str!("../shaders/grids/test_grid.frag"));

    // Store state
    let mut state = State {
        neutron_tex: rendering::textures::create_grid_texture(),
        mouse_held: false,
    };

    // Render loop
    while !window.should_close() {
        // Process events
        process_events(&mut window, &mut state, &events);

        // Clear background
        unsafe { 
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Render neutron grid
        neutron_renderer.render_grid(state.neutron_tex);

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
                    state.mouse_held = false;
                } else {
                    state.mouse_held = true;
                }
            },

            glfw::WindowEvent::CursorPos(x, y) => {
                // Handle adding points
                if state.mouse_held {
                    // Add point to neutron grid
                    rendering::textures::write_grid_texture_patch(state.neutron_tex, x as usize, y as usize, 8, 8, (255, 255, 255, 255));
                }
            },

            // Key events
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),

            // Fallthrough case
            _ => {}
        }
    }
}