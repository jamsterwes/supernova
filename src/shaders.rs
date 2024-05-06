extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ffi::CString;
use std::ptr;
use std::str;

// Function to construct a basic vertex+fragment program
pub fn build_vertex_fragment(vertex_src: &str, fragment_src: &str) -> GLuint {
    // Compile shaders
    let vertex = unsafe { compile_shader(gl::VERTEX_SHADER, vertex_src.to_string()) };
    let fragment = unsafe { compile_shader(gl::FRAGMENT_SHADER, fragment_src.to_string()) };

    // Link program
    let program = unsafe { link_program(vec![vertex, fragment]) };
    return program;
}

// Function to link shaders into a program
unsafe fn link_program(shaders: Vec<GLuint>) -> GLuint {
    // Create shader program
    let program = gl::CreateProgram();

    // Attach shaders
    // (clone bc we need to iterate later to delete)
    for shader in shaders.clone() {
        gl::AttachShader(program, shader);
    }

    // Link program
    gl::LinkProgram(program);

    // Allocate 512 bytes for storing possible linker errors
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::<u8>::with_capacity(512);
    info_log.set_len(512 - 1);  // Leave last byte for trailing null

    // Get linker errors (if any)
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::LINKING_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }

    // Delete dangling shaders
    // (now we can consume the Vec)
    for shader in shaders {
        gl::DeleteShader(shader);
    }

    return program;
}

// Helper function to compile a single shader
unsafe fn compile_shader(type_: u32, source: String) -> GLuint {
    // Create shader
    let shader = gl::CreateShader(type_);

    // Load shader source text
    let c_str_source = CString::new(source.as_bytes()).unwrap();
    gl::ShaderSource(shader, 1, &c_str_source.as_ptr(), ptr::null());

    // Compile shader
    gl::CompileShader(shader);

    // Allocate 512 bytes for storing possible shader compile errors
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::<u8>::with_capacity(512);
    info_log.set_len(512 - 1);  // Leave last byte for trailing null

    // Get shader compile errors (if any)
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }

    return shader;
}