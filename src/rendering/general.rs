extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ffi::c_void;
use std::mem;
use std::ptr;

use super::shaders;

// Struct for storing the grid renderer
pub struct GridRenderer {
    // Store VAO for rendering full-screen quad
    vao: GLuint,

    // Store shader program
    shader: GLuint,

    // Store tex uniform
    uniform_tex: GLint,

    // Store resolution uniform
    uniform_resolution: GLint,
}

// Trait for rendering a grid
pub trait RenderGrid {
    fn render_grid(&self, tex_id: GLuint);
}

// Make a grid renderer
pub fn make_grid_renderer(frag_src: &str) -> GridRenderer {
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
        include_str!("../../shaders/grid.vert"), 
        frag_src
    );

    // Now unbind vbo, vao
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Create renderer
    return GridRenderer {
        vao: vao,
        shader: program,
        uniform_tex: shaders::get_uniform_location(program, "tex"),
        uniform_resolution: shaders::get_uniform_location(program, "resolution"),
    }
}

// Implement RenderGrid trait on GridRenderer
impl RenderGrid for GridRenderer {
    fn render_grid(&self, tex_id: GLuint) {
        // Bind shader, VAO, texture
        unsafe {
            gl::UseProgram(self.shader);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::BindVertexArray(self.vao);
        }

        // (potentially unnecessary)
        // Set tex uniform to 0
        unsafe { gl::Uniform1ui(self.uniform_tex, 0) };

        // Set resolution uniform
        unsafe { gl::Uniform2f(self.uniform_resolution, crate::SCR_WIDTH as f32, crate::SCR_HEIGHT as f32) }

        // Draw quad
        unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4) };
    }
}