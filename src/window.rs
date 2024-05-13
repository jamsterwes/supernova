extern crate glfw;

use self::glfw::Context;

extern crate gl;

// Struct for storing window construction settings
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

// Used to initialize GLFW at beginning of program
pub fn init_glfw() -> glfw::Glfw {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    // Set OpenGL hints (v4.4 Compatibility)
    glfw.window_hint(glfw::WindowHint::ContextVersion(4,4));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Compat));

    // Return GLFW instance
    return glfw;
}

// Create a window
pub fn create_window(glfw: &mut glfw::Glfw, settings: WindowSettings) -> (glfw::PWindow, glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    // Create window    
    let (mut window, events) = glfw.create_window(settings.width, settings.height, settings.title.as_str(), glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    
    // Set context to current
    window.make_current();

    // Set what events to poll
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_framebuffer_size_polling(true);

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    // Load OpenGL function pointers into window
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Enable blending
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    return (window, events);
}