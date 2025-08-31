#[cfg(feature = "gl")]
use gl;
use std::ffi::CString;
#[cfg(feature = "glfw")]
use glfw::{Glfw, Window as GlfwWindow};

/// Safe wrapper around OpenGL functionality
#[cfg(feature = "glfw")]
pub struct GlWrapper {
    initialized: bool,
    glfw: Option<Glfw>,
    window: Option<GlfwWindow>,
}

#[cfg(not(feature = "glfw"))]
pub struct GlWrapper {
    initialized: bool,
}

impl GlWrapper {
    #[cfg(feature = "glfw")]
    pub fn new() -> Self {
        Self {
            initialized: false,
            glfw: None,
            window: None,
        }
    }

    #[cfg(not(feature = "glfw"))]
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }
    
    /// Initialize OpenGL context with GLFW window
    #[cfg(all(feature = "glfw", feature = "gl"))]
    pub fn initialize(&mut self, window: &mut glfw::Window) -> Result<(), String> {
        // Load OpenGL function pointers using the provided window
        gl::load_with(|s| window.get_proc_address(s).map_or(std::ptr::null(), |f| f as *const _));
        
        // Mark as initialized
        self.initialized = true;
        
        Ok(())
    }

    #[cfg(not(all(feature = "glfw", feature = "gl")))]
    pub fn initialize(&mut self) -> Result<(), String> {
        // No-op for headless mode
        self.initialized = true;
        Ok(())
    }
    
    /// Check if OpenGL is initialized
    fn check_initialized(&self) -> Result<(), String> {
        if !self.initialized {
            return Err("OpenGL context not initialized".to_string());
        }
        Ok(())
    }
    
    /// Set the viewport dimensions
    #[cfg(feature = "gl")]
    pub fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Viewport(x, y, width, height);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn set_viewport(&self, _x: i32, _y: i32, _width: i32, _height: i32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Set the clear color
    #[cfg(feature = "gl")]
    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn set_clear_color(&self, _r: f32, _g: f32, _b: f32, _a: f32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Clear the color buffer
    #[cfg(feature = "gl")]
    pub fn clear_color_buffer(&self) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn clear_color_buffer(&self) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Enable blending
    #[cfg(feature = "gl")]
    pub fn enable_blending(&self) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Enable(gl::BLEND);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn enable_blending(&self) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Set blend function
    #[cfg(feature = "gl")]
    pub fn set_blend_func(&self, src: u32, dst: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::BlendFunc(src, dst);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn set_blend_func(&self, _src: u32, _dst: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Use a shader program
    #[cfg(feature = "gl")]
    pub fn use_program(&self, program: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::UseProgram(program);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn use_program(&self, _program: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Set a 3D float uniform
    #[cfg(feature = "gl")]
    pub fn set_uniform_3f(&self, location: i32, x: f32, y: f32, z: f32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Uniform3f(location, x, y, z);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn set_uniform_3f(&self, _location: i32, _x: f32, _y: f32, _z: f32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Set a 2D float uniform
    #[cfg(feature = "gl")]
    pub fn set_uniform_2f(&self, location: i32, x: f32, y: f32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::Uniform2f(location, x, y);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn set_uniform_2f(&self, _location: i32, _x: f32, _y: f32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Get uniform location
    #[cfg(feature = "gl")]
    pub fn get_uniform_location(&self, program: u32, name: &str) -> Result<i32, String> {
        self.check_initialized()?;
        unsafe {
            let c_str = CString::new(name)
               .map_err(|_| "Invalid uniform name: contains null byte")?;
            Ok(gl::GetUniformLocation(program, c_str.as_ptr() as *const i8))
        }
    }

    #[cfg(not(feature = "gl"))]
    pub fn get_uniform_location(&self, _program: u32, _name: &str) -> Result<i32, String> {
        // No-op for headless mode
        Ok(0) // Return a default value or handle as appropriate
    }
    
    /// Get shader parameter
    #[cfg(feature = "glfw")]
    pub fn get_shader_iv(&self, shader: u32, pname: u32, params: &mut i32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::GetShaderiv(shader, pname, params);
        }
        Ok(())
    }

    #[cfg(not(feature = "glfw"))]
    pub fn get_shader_iv(&self, _shader: u32, _pname: u32, _params: &mut i32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Get shader info log
    #[cfg(feature = "glfw")]
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

    #[cfg(not(feature = "glfw"))]
    pub fn get_shader_info_log(&self, _shader: u32) -> Result<String, String> {
        // No-op for headless mode
        Ok(String::new())
    }
    
    /// Get program parameter
    #[cfg(feature = "gl")]
    pub fn get_program_iv(&self, program: u32, pname: u32, params: &mut i32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::GetProgramiv(program, pname, params);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn get_program_iv(&self, _program: u32, _pname: u32, _params: &mut i32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Get program info log
    #[cfg(feature = "gl")]
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

    #[cfg(not(feature = "gl"))]
    pub fn get_program_info_log(&self, _program: u32) -> Result<String, String> {
        // No-op for headless mode
        Ok(String::new())
    }
    
    /// Bind vertex array object
    #[cfg(feature = "gl")]
    pub fn bind_vertex_array(&self, vao: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::BindVertexArray(vao);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn bind_vertex_array(&self, _vao: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Draw arrays
    #[cfg(feature = "gl")]
    pub fn draw_arrays(&self, mode: u32, first: i32, count: i32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::DrawArrays(mode, first, count);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn draw_arrays(&self, _mode: u32, _first: i32, _count: i32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Create shader
    #[cfg(feature = "gl")]
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

    #[cfg(not(feature = "gl"))]
    pub fn create_shader(&self, _shader_type: u32) -> Result<u32, String> {
        // No-op for headless mode
        Ok(0) // Return a default value or handle as appropriate
    }
    
    /// Set shader source
    #[cfg(feature = "gl")]
    pub fn set_shader_source(&self, shader: u32, source: &str) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            let c_str = CString::new(source).map_err(|_| "Invalid shader source")?;
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            Ok(())
        }
    }

    #[cfg(not(feature = "gl"))]
    pub fn set_shader_source(&self, _shader: u32, _source: &str) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Compile shader
    #[cfg(feature = "gl")]
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

    #[cfg(not(feature = "gl"))]
    pub fn compile_shader(&self, _shader: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Create program
    #[cfg(feature = "gl")]
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

    #[cfg(not(feature = "gl"))]
    pub fn create_program(&self) -> Result<u32, String> {
        // No-op for headless mode
        Ok(0) // Return a default value or handle as appropriate
    }
    
    /// Attach shader to program
    #[cfg(feature = "gl")]
    pub fn attach_shader(&self, program: u32, shader: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::AttachShader(program, shader);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn attach_shader(&self, _program: u32, _shader: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Link program
    #[cfg(feature = "gl")]
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

    #[cfg(not(feature = "gl"))]
    pub fn link_program(&self, _program: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Delete shader
    #[cfg(feature = "gl")]
    pub fn delete_shader(&self, shader: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::DeleteShader(shader);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn delete_shader(&self, _shader: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Generate vertex array object
    #[cfg(feature = "gl")]
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

    #[cfg(not(feature = "gl"))]
    pub fn gen_vertex_array(&self) -> Result<u32, String> {
        // No-op for headless mode
        Ok(0) // Return a default value or handle as appropriate
    }
    
    /// Generate buffer
    #[cfg(feature = "gl")]
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

    #[cfg(not(feature = "gl"))]
    pub fn gen_buffer(&self) -> Result<u32, String> {
        // No-op for headless mode
        Ok(0) // Return a default value or handle as appropriate
    }
    
    /// Bind buffer
    #[cfg(feature = "gl")]
    pub fn bind_buffer(&self, target: u32, buffer: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::BindBuffer(target, buffer);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn bind_buffer(&self, _target: u32, _buffer: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Set buffer data
    #[cfg(feature = "gl")]
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

    #[cfg(not(feature = "gl"))]
    pub fn set_buffer_data(&self, _target: u32, _data: &[f32], _usage: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Set vertex attribute pointer
    #[cfg(feature = "gl")]
    pub fn set_vertex_attrib_pointer(&self, index: u32, size: i32, data_type: u32, normalized: bool, stride: i32, offset: usize) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::VertexAttribPointer(index, size, data_type, normalized as u8, stride, offset as *const std::ffi::c_void);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn set_vertex_attrib_pointer(&self, _index: u32, _size: i32, _data_type: u32, _normalized: bool, _stride: i32, _offset: usize) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Enable vertex attribute array
    #[cfg(feature = "gl")]
    pub fn enable_vertex_attrib_array(&self, index: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::EnableVertexAttribArray(index);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn enable_vertex_attrib_array(&self, _index: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Delete program
    #[cfg(feature = "gl")]
    pub fn delete_program(&self, program: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::DeleteProgram(program);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn delete_program(&self, _program: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Delete vertex array object
    #[cfg(feature = "gl")]
    pub fn delete_vertex_array(&self, vao: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::DeleteVertexArrays(1, &vao);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn delete_vertex_array(&self, _vao: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
    
    /// Delete buffer
    #[cfg(feature = "gl")]
    pub fn delete_buffer(&self, buffer: u32) -> Result<(), String> {
        self.check_initialized()?;
        unsafe {
            gl::DeleteBuffers(1, &buffer);
        }
        Ok(())
    }

    #[cfg(not(feature = "gl"))]
    pub fn delete_buffer(&self, _buffer: u32) -> Result<(), String> {
        // No-op for headless mode
        Ok(())
    }
}
