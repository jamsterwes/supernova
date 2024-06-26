extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ffi::c_void;

// Function to create a new grid texture (RGBA)
pub fn create_grid_texture() -> GLuint {
    // Use screen resolution
    let width = crate::SCR_WIDTH as i32;
    let height = crate::SCR_HEIGHT as i32;

    // Create texture
    let tex_id: GLuint = unsafe { 
        let mut id: GLuint = 0;
        gl::GenTextures(1, &mut id);
        id
    };

    // Set texture size/format
    unsafe {
        let mut data: Vec<f32> = vec![0.0; 4 * width as usize * height as usize];
        gl::BindTexture(gl::TEXTURE_2D, tex_id);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as i32, width, height, 0, gl::RGBA, gl::FLOAT, data.as_mut_ptr() as *mut c_void);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
    }

    // Return texture ID
    return tex_id;
}

// Function to write one pixel to grid texture (not efficient)
pub fn write_grid_texture_patch(tex_id: GLuint, x: usize, y: usize, width: usize, height: usize, px_data: (f32, f32, f32, f32)) {
    // Allocate texture data
    let mut data: Vec<f32> = vec![0.0; 4 * width as usize * height as usize];

    // Fill patch
    for y in 0..height {
        for x in 0..width {
            data[y * 4 + x * 4 * width] = px_data.0;
            data[y * 4 + x * 4 * width + 1] = px_data.1;
            data[y * 4 + x * 4 * width + 2] = px_data.2;
            data[y * 4 + x * 4 * width + 3] = px_data.3;
        }
    }

    // Now upload patch
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, tex_id);
        gl::TexSubImage2D(gl::TEXTURE_2D, 0, x as i32, y as i32, width as i32, height as i32, gl::RGBA, gl::FLOAT, data.as_mut_ptr() as *mut c_void);
    }
}

// Function to copy tex A->B
pub fn copy_grid_texture(tex_src_id: GLuint, tex_dest_id: GLuint) {
    // Use screen resolution
    let width = crate::SCR_WIDTH as i32;
    let height = crate::SCR_HEIGHT as i32;

    // Copy
    unsafe {
        gl::CopyImageSubData(tex_src_id, gl::TEXTURE_2D, 0, 0, 0, 0, tex_dest_id, gl::TEXTURE_2D, 0, 0, 0, 0, width, height, 1);
    }
}