use crate::shaders;

extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ffi::c_void;
use std::mem;
use std::ptr;

// Temporary
pub struct Renderer {
    // TODO: move these things
    vao: GLuint,

    // Store shader program
    shader: GLuint,
}

pub trait Draw {
    fn draw(&self);
}

// Implement draw for renderer
impl Draw for Renderer {
    fn draw(&self) {
        unsafe {
            // Use the shader program
            gl::UseProgram(self.shader);

            // Bind the vao
            gl::BindVertexArray(self.vao);

            // Draw triangle
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

// Function to create the renderer
pub fn create_renderer() -> Renderer {
    // Define vertices
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0,  0.5, 0.0  // top
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
        include_str!("shaders/triangle.vert"), 
        include_str!("shaders/triangle.frag")
    );

    // Now unbind vbo, vao
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Create renderer
    return Renderer {
        vao: vao,
        shader: program
    }
}