mod renderer;

use renderer::DrawLine;

mod shaders;
mod window;

extern crate glfw;

use self::glfw::{Context, Key, Action, GlfwReceiver};

extern crate gl;

// Settings
const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;
const SCR_TITLE: &'static str = "supernova";

// Store state
struct State {
    lines: Vec<Vec<(f32, f32)>>,
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

    // Create renderer
    let renderer = renderer::create_line_renderer();

    // Store state
    let mut state = State {
        lines: Vec::new(),
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

        // Render triangle
        for line in &state.lines {
            renderer.draw_polyline(line, 32.0);
        }

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
                if action == Action::Press {
                    state.mouse_held = true;
                    state.lines.push(Vec::new());
                } else {
                    state.mouse_held = false;
                }
            },

            glfw::WindowEvent::CursorPos(x, y) => {
                // Handle adding points
                if state.mouse_held {
                    let current_line = state.lines.last_mut().unwrap();

                    // Is there a previous point?
                    if current_line.len() > 0 {
                        // Get previous point
                        let prev_point = current_line.last().unwrap();

                        // Check distance
                        let distance = f32::sqrt(f32::powf(x as f32 - prev_point.0, 2.0) + f32::powf(y as f32 - prev_point.1, 2.0));

                        if distance > 4.0 {
                            state.lines.last_mut().unwrap().push((x as f32, y as f32));
                        }
                    } else {
                        state.lines.last_mut().unwrap().push((x as f32, y as f32));
                    }
                }
            },

            // Key events
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),

            // Fallthrough case
            _ => {}
        }
    }
}