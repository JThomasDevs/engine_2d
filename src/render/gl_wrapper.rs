use gl;
use std::ffi::CString;
use glfw::{Glfw, Window as GlfwWindow};

/// Safe wrapper around OpenGL functionality
pub struct GlWrapper {
    initialized: bool,
    #[allow(dead_code)]
    glfw: Option<Glfw>,
    #[allow(dead_code)]
    window: Option<GlfwWindow>,
}

impl GlWrapper {
    pub fn new() -> Self {
        Self {
            initialized: false,
            glfw: None,
            window: None,
        }
    }
    
    /// Initialize OpenGL context with GLFW window
    pub fn initialize(&mut self, window: &mut glfw::Window) -> Result<(), String> {
        // Load OpenGL function pointers using the provided window
        gl::load_with(|s| window.get_proc_address(s).map_or(std::ptr::null(), |f| f as *const _));
        
        // Mark as initialized
        self.initialized = true;
        
        Ok(())
    }
    
    /// Check if OpenGL is initialized
    pub fn check_initialized(&self) -> Result<(), String> {
        if !self.initialized {
            return Err("OpenGL context not initialized".to_string());
        }
        Ok(())
    }
    
    /// Set the viewport dimensions
    pub fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), String> {
        debug_assert!(self.initialized, "GlWrapper must be initialized before use");
        self.check_initialized()?;
        unsafe {
            gl::Viewport(x, y, width, height);
        }
        Ok(())
    }
    
    /// Set the clear color
    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
        Ok(())
    }
    
    /// Clear the color buffer
    pub fn clear_color_buffer(&self) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        Ok(())
    }
    
    /// Enable blending
    pub fn enable_blending(&self) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Enable(gl::BLEND);
        }
        Ok(())
    }
    
    /// Set blend function
    pub fn set_blend_func(&self, src: u32, dst: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::BlendFunc(src, dst);
        }
        Ok(())
    }
    
    /// Use a shader program
    pub fn use_program(&self, program: u32) -> Result<(), String> {
        debug_assert!(self.initialized, "GlWrapper must be initialized before use");
        self.check_initialized()?;
        unsafe {
            gl::UseProgram(program);
        }
        Ok(())
    }
    
    /// Set a 3D float uniform
    pub fn set_uniform_3f(&self, location: i32, x: f32, y: f32, z: f32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Uniform3f(location, x, y, z);
        }
        Ok(())
    }
    
    /// Set a 2D float uniform
    pub fn set_uniform_2f(&self, location: i32, x: f32, y: f32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Uniform2f(location, x, y);
        }
        Ok(())
    }
    
    /// Get uniform location
    pub fn get_uniform_location(&self, program: u32, name: &str) -> Result<i32, String> {
        self.check_initialized()?;
        unsafe {
            let c_str = CString::new(name)
               .map_err(|_| "Invalid uniform name: contains null byte")?;
            Ok(gl::GetUniformLocation(program, c_str.as_ptr() as *const i8))
        }
    }
    
    /// Get shader parameter
    pub fn get_shader_iv(&self, shader: u32, pname: u32, params: &mut i32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::GetShaderiv(shader, pname, params);
        }
        Ok(())
    }
    
    /// Get shader info log
    pub fn get_shader_info_log(&self, shader: u32) -> Result<String, String> {
        self.check_initialized()?;
        unsafe {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            let error = String::from_utf8_lossy(&buffer).to_string();
            Ok(error)
        }
    }
    
    /// Get program parameter
    pub fn get_program_iv(&self, program: u32, pname: u32, params: &mut i32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::GetProgramiv(program, pname, params);
        }
        Ok(())
    }
    
    /// Get program info log
    pub fn get_program_info_log(&self, program: u32) -> Result<String, String> {
        self.check_initialized()?;
        unsafe {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            let error = String::from_utf8_lossy(&buffer).to_string();
            Ok(error)
        }
    }
    
    /// Bind vertex array object
    pub fn bind_vertex_array(&self, vao: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::BindVertexArray(vao);
        }
        Ok(())
    }
    
    /// Draw arrays
    pub fn draw_arrays(&self, mode: u32, first: i32, count: i32) -> Result<(), String> {
        debug_assert!(self.initialized, "GlWrapper must be initialized before use");
        self.check_initialized()?;
        unsafe {
            gl::DrawArrays(mode, first, count);
        }
        Ok(())
    }
    
    /// Create shader
    pub fn create_shader(&self, shader_type: u32) -> Result<u32, String> {
        self.check_initialized()?;
        unsafe {
            let shader = gl::CreateShader(shader_type);
            if shader == 0 {
                return Err("Failed to create shader".to_string());
            }
            Ok(shader)
        }
    }
    
    /// Set shader source
    pub fn set_shader_source(&self, shader: u32, source: &str) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            let c_str = CString::new(source).map_err(|_| "Invalid shader source")?;
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            Ok(())
        }
    }
    
    /// Compile shader
    pub fn compile_shader(&self, shader: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::CompileShader(shader);
            
            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0u8; len as usize];
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
                let error = String::from_utf8_lossy(&buffer).to_string();
                return Err(format!("Shader compilation failed: {}", error));
            }
            
            Ok(())
        }
    }
    
    /// Create program
    pub fn create_program(&self) -> Result<u32, String> {
        self.check_initialized()?;
        unsafe {
            let program = gl::CreateProgram();
            if program == 0 {
                return Err("Failed to create program".to_string());
            }
            Ok(program)
        }
    }
    
    /// Attach shader to program
    pub fn attach_shader(&self, program: u32, shader: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::AttachShader(program, shader);
        }
        Ok(())
    }
    
    /// Link program
    pub fn link_program(&self, program: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::LinkProgram(program);
            
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0u8; len as usize];
                gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
                let error = String::from_utf8_lossy(&buffer).to_string();
                return Err(format!("Program linking failed: {}", error));
            }
            
            Ok(())
        }
    }
    
    /// Delete shader
    pub fn delete_shader(&self, shader: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::DeleteShader(shader);
        }
        Ok(())
    }
    
    /// Generate vertex array object
    pub fn gen_vertex_array(&self) -> Result<u32, String> {
        self.check_initialized()?;
        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            
            if vao == 0 {
                let error = gl::GetError();
                return Err(format!("Failed to generate vertex array object. OpenGL error: {}", error));
            }
            
            Ok(vao)
        }
    }
    
    /// Generate buffer
    pub fn gen_buffer(&self) -> Result<u32, String> {
        self.check_initialized()?;
        unsafe {
            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);

            if buffer == 0 {
                let error = gl::GetError();
                return Err(format!("Failed to generate buffer. OpenGL error: {}", error));
            }

            Ok(buffer)
        }
    }
    
    /// Bind buffer
    pub fn bind_buffer(&self, target: u32, buffer: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::BindBuffer(target, buffer);
        }
        Ok(())
    }
    
    /// Set buffer data
    pub fn set_buffer_data(&self, target: u32, data: &[f32], usage: u32) -> Result<(), String> {
        self.check_initialized()?;
        
        let byte_count = data.len()
            .checked_mul(std::mem::size_of::<f32>())
            .and_then(|v| v.try_into().ok())
            .ok_or_else(|| "Buffer size overflow: data too large for OpenGL buffer".to_string())?;
        
        unsafe {
            gl::BufferData(
                target,
                byte_count,
                data.as_ptr() as *const _,
                usage,
            );
        }
        Ok(())
    }
    
    /// Set vertex attribute pointer
    pub fn set_vertex_attrib_pointer(&self, index: u32, size: i32, data_type: u32, normalized: bool, stride: i32, offset: usize) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::VertexAttribPointer(index, size, data_type, normalized as u8, stride, offset as *const std::ffi::c_void);
        }
        Ok(())
    }
    
    /// Enable vertex attribute array
    pub fn enable_vertex_attrib_array(&self, index: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::EnableVertexAttribArray(index);
        }
        Ok(())
    }
    
    /// Delete program
    pub fn delete_program(&self, program: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::DeleteProgram(program);
        }
        Ok(())
    }
    
    /// Delete vertex array object
    pub fn delete_vertex_array(&self, vao: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::DeleteVertexArrays(1, &vao);
        }
        Ok(())
    }
    
    /// Delete buffer
    pub fn delete_buffer(&self, buffer: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::DeleteBuffers(1, &buffer);
        }
        Ok(())
    }
    
    // ===== TEXTURE METHODS =====
    
    /// Generate texture
    pub fn gen_texture(&self) -> Result<u32, String> {
        self.check_initialized()?;
        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
        }
        Ok(texture)
    }
    
    /// Bind texture
    pub fn bind_texture(&self, target: u32, texture: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::BindTexture(target, texture);
        }
        Ok(())
    }
    
    /// Set texture parameter
    pub fn tex_parameter_i(&self, target: u32, pname: u32, param: i32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::TexParameteri(target, pname, param);
        }
        Ok(())
    }
    
    /// Upload texture image data
    pub fn tex_image_2d(&self, target: u32, level: i32, internal_format: i32, width: i32, height: i32, border: i32, format: u32, data_type: u32, data: Option<&[u8]>) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::TexImage2D(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                data_type,
                data.map(|d| d.as_ptr() as *const std::ffi::c_void).unwrap_or(std::ptr::null())
            );
        }
        Ok(())
    }
    
    /// Delete texture
    pub fn delete_texture(&self, texture: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::DeleteTextures(1, &texture);
        }
        Ok(())
    }
    
    /// Set uniform for texture sampler
    pub fn set_uniform_1i(&self, location: i32, value: i32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Uniform1i(location, value);
        }
        Ok(())
    }
    
    /// Set uniform for float value
    pub fn set_uniform_1f(&self, location: i32, value: f32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Uniform1f(location, value);
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests { }