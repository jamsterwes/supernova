mod window;
mod shaders;

extern crate glfw;
use self::glfw::{Context, Key, Action, GlfwReceiver};

extern crate gl;

// Settings
const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;
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

    // Load shaders
    let program = shaders::build_vertex_fragment(
        include_str!("shaders/triangle.vert"), 
        include_str!("shaders/triangle.frag")
    );

    // Render loop
    while !window.should_close() {
        // Process events
        process_events(&mut window, &events);

        // Swap and poll
        unsafe { 
            gl::ClearColor(0.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers();
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