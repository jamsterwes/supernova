use crate::rendering::shaders::{self, get_uniform_location};

extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ffi::c_void;
use std::mem;
use std::ptr;

// Class for rendering lines
pub struct CircleRenderer {
    // TODO: move these things
    vao: GLuint,

    // Store shader program
    shader: GLuint,

    // Store position uniform
    uniform_position: GLint,

    // Store radius uniform
    uniform_radius: GLint,

    // Store resolution uniform
    uniform_resolution: GLint,

    // Store color uniform
    uniform_color: GLint,
}

pub trait DrawCircle {
    fn draw(&self, position: (f32, f32), radius: f32, color: (f32, f32, f32, f32));
    fn draw_multi(&self, positions: &Vec<(f32, f32)>, radii: &Vec<f32>, colors: &Vec<(f32, f32, f32, f32)>);
}

// Implement draw for renderer
impl DrawCircle for CircleRenderer {
    fn draw(&self, position: (f32, f32), radius: f32, color: (f32, f32, f32, f32)) {
        unsafe {
            // Use the shader program
            gl::UseProgram(self.shader);

            // Bind the vao
            gl::BindVertexArray(self.vao);

            // Set resolution uniform
            gl::Uniform2f(self.uniform_resolution, crate::SCR_WIDTH as f32, crate::SCR_HEIGHT as f32);

            // Set from/to uniforms
            gl::Uniform2f(self.uniform_position, position.0, position.1);
            gl::Uniform1f(self.uniform_radius, radius);

            // Set color uniform
            gl::Uniform4f(self.uniform_color, color.0, color.1, color.2, color.3);

            // Draw quad
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }

    fn draw_multi(&self, positions: &Vec<(f32, f32)>, radii: &Vec<f32>, colors: &Vec<(f32, f32, f32, f32)>) {
        unsafe {
            // Use the shader program
            gl::UseProgram(self.shader);

            // Bind the vao
            gl::BindVertexArray(self.vao);

            // Set resolution uniform
            gl::Uniform2f(self.uniform_resolution, crate::SCR_WIDTH as f32, crate::SCR_HEIGHT as f32);

            for i in 0..positions.len() {
                // Set position/radius uniforms
                gl::Uniform2f(self.uniform_position, positions[i].0, positions[i].1);
                gl::Uniform1f(self.uniform_radius, radii[i]);
                
                // Set color uniform
                gl::Uniform4f(self.uniform_color, colors[i].0, colors[i].1, colors[i].2, colors[i].3);

                // Draw quad
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }
        }
    }
}

// Function to create the circle renderer
pub fn create_circle_renderer() -> CircleRenderer {
    // Define vertices
    let vertices: [f32; 12] = [
        -1.0, -1.0, 0.0,  // BL
        -1.0, 1.0, 0.0,   // TL
        1.0, -1.0, 0.0,   // BR
        1.0, 1.0, 0.0,    // TR
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
        include_str!("../../../shaders/circle.vert"), 
        include_str!("../../../shaders/circle.frag")
    );

    // Now unbind vbo, vao
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Create renderer
    return CircleRenderer {
        vao: vao,
        shader: program,
        uniform_position: get_uniform_location(program, "position"),
        uniform_radius: get_uniform_location(program, "radius"),
        uniform_resolution: get_uniform_location(program, "resolution"),
        uniform_color: get_uniform_location(program, "color"),
    }
}