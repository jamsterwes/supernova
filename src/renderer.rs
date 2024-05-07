use crate::shaders;

extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ffi::{c_void, CString};
use std::mem;
use std::ptr;

// Class for rendering lines
pub struct LineRenderer {
    // TODO: move these things
    vao: GLuint,

    // Store shader program
    shader: GLuint,

    // Store from uniform
    uniform_from: GLint,

    // Store to uniform
    uniform_to: GLint,

    // Store resolution uniform
    uniform_resolution: GLint,

    // Store line width uniform
    uniform_width: GLint,
}

pub trait DrawLine {
    fn draw_line(&self, from: (f32, f32), to: (f32, f32), line_width: f32);
    fn draw_polyline(&self, coords: &Vec<(f32, f32)>, line_width: f32);
}

// Implement draw for renderer
impl DrawLine for LineRenderer {
    fn draw_line(&self, from: (f32, f32), to: (f32, f32), line_width: f32) {
        unsafe {
            // Use the shader program
            gl::UseProgram(self.shader);

            // Bind the vao
            gl::BindVertexArray(self.vao);

            // Set from/to uniforms
            gl::Uniform2f(self.uniform_from, from.0, from.1);
            gl::Uniform2f(self.uniform_to, to.0, to.1);

            // Set resolution uniform (TODO: get this elsewhere)
            gl::Uniform2f(self.uniform_resolution, crate::SCR_WIDTH as f32, crate::SCR_HEIGHT as f32);

            // Set line width
            gl::Uniform1f(self.uniform_width, line_width);

            // Draw triangle
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }

    fn draw_polyline(&self, coords: &Vec<(f32, f32)>, line_width: f32) {
        // If less than 2 coords, ignore
        if coords.len() < 2 {
            return;
        }

        // Draw lines
        for pair in coords.windows(2) {
            self.draw_line(pair[0], pair[1], line_width);
        }
    }
}

// Function to create the line renderer
pub fn create_line_renderer() -> LineRenderer {
    // Define vertices
    let vertices: [f32; 12] = [
        0.0, -0.5, 0.0,  // BL
        0.0, 0.5, 0.0,   // TL
        1.0, -0.5, 0.0,   // BR
        1.0, 0.5, 0.0,    // TR
    ];

    // Build VAO
    let mut vao = 0;
    unsafe { 
        // Generate vertex array object
        gl::GenVertexArrays(1, &mut vao);

        // Bind to it
        gl::BindVertexArray(vao);
    }

    // Build VBO
    let mut vbo = 0;
    unsafe {
        // Generate vertex buffer object
        gl::GenBuffers(1, &mut vbo);

        // Bind to it
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Buffer vertices
        gl::BufferData(gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        // Set vertex attribute 0: 
        // - vec3 pos;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    // Load shaders
    let program = shaders::build_vertex_fragment(
        include_str!("shaders/line.vert"), 
        include_str!("shaders/line.frag")
    );

    // Now unbind vbo, vao
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Create renderer
    return LineRenderer {
        vao: vao,
        shader: program,
        uniform_from: get_uniform_location(program, "from"),
        uniform_to: get_uniform_location(program, "to"),
        uniform_resolution: get_uniform_location(program, "resolution"),
        uniform_width: get_uniform_location(program, "width"),
    }
}

// Helper function to get uniform location
fn get_uniform_location(program: GLuint, name: &str) -> GLint {
    let c_name = CString::new(name).unwrap();
    return unsafe { gl::GetUniformLocation(program, c_name.as_ptr()) };
}