use gl;
use std::rc::Rc;
use super::gl_wrapper::GlWrapper;
use glam::Vec2;

pub struct Renderer {
    gl: Rc<GlWrapper>,
    basic_shader: Option<u32>,
    rect_vao: Option<u32>,
    rect_vbo: Option<u32>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.cleanup();
    }
}

impl Renderer {
    pub fn cleanup(&mut self) {
        if let Some(shader) = self.basic_shader.take() {
            let _ = self.gl.delete_program(shader);
        }
        if let Some(vao) = self.rect_vao.take() {
            let _ = self.gl.delete_vertex_array(vao);
        }
        if let Some(vbo) = self.rect_vbo.take() {
            let _ = self.gl.delete_buffer(vbo);
        }
    }
}

impl Renderer {
    pub fn new() -> Self {
        let gl_wrapper = GlWrapper::new();
        
        Self {
            gl: Rc::new(gl_wrapper),
            basic_shader: None,
            rect_vao: None,
            rect_vbo: None,
        }
    }
    
    /// Create a renderer with an existing GlWrapper (for use with WindowManager)
    pub fn new_with_gl(gl_wrapper: Rc<GlWrapper>) -> Self {
        Self {
            gl: gl_wrapper,
            basic_shader: None,
            rect_vao: None,
            rect_vbo: None,
        }
    }
    
    /// Initialize the renderer (call after OpenGL context is ready)
    pub fn initialize(&mut self) -> Result<(), String> {
        // The GlWrapper is already initialized in WindowManager
        // Just create the shaders and geometry
        
        println!("Initializing renderer...");
        let basic_shader = Self::create_basic_shader(&self.gl)?;
        println!("Created basic shader: {}", basic_shader);
        
        let (rect_vao, rect_vbo) = Self::create_rect_geometry(&self.gl)?;
        println!("Created rectangle geometry - VAO: {}, VBO: {}", rect_vao, rect_vbo);
        
        self.basic_shader = Some(basic_shader);
        self.rect_vao = Some(rect_vao);
        self.rect_vbo = Some(rect_vbo);
        
        println!("Renderer initialized successfully!");
        Ok(())
    }
    
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) -> Result<(), String> {
        self.gl.set_clear_color(r, g, b, a)?;
        self.gl.clear_color_buffer()
    }
    
    pub fn draw_rect(&self, position: Vec2, size: Vec2, color: (f32, f32, f32)) -> Result<(), String> {
        let shader = self.basic_shader.ok_or("Renderer not initialized")?;
        let vao = self.rect_vao.ok_or("Renderer not initialized")?;
        
        self.gl.use_program(shader)?;
        
        // Set uniforms
        let pos_loc = self.gl.get_uniform_location(shader, "rect_position")?;
        let size_loc = self.gl.get_uniform_location(shader, "rect_size")?;
        let color_loc = self.gl.get_uniform_location(shader, "color")?;
        
        self.gl.set_uniform_2f(pos_loc, position.x, position.y)?;
        self.gl.set_uniform_2f(size_loc, size.x, size.y)?;
        self.gl.set_uniform_3f(color_loc, color.0, color.1, color.2)?;
        
        // Draw the rectangle
        self.gl.bind_vertex_array(vao)?;
        self.gl.draw_arrays(gl::TRIANGLE_STRIP, 0, 4)?;
        
        Ok(())
    }    
    
    fn create_basic_shader(gl: &GlWrapper) -> Result<u32, String> {
        let vertex_shader_source = r#"
            #version 330 core
            layout (location = 0) in vec2 position;
            
            uniform vec2 rect_position;
            uniform vec2 rect_size;
            
            void main() {
                vec2 world_pos = rect_position + position * rect_size;
                gl_Position = vec4(world_pos, 0.0, 1.0);
            }
        "#;
        
        let fragment_shader_source = r#"
            #version 330 core
            out vec4 FragColor;
            
            uniform vec3 color;
            
            void main() {
                FragColor = vec4(color, 1.0);
            }
        "#;
        
        let vertex_shader = gl.create_shader(gl::VERTEX_SHADER)?;
        gl.set_shader_source(vertex_shader, vertex_shader_source)?;
        gl.compile_shader(vertex_shader)?;
        
        // Check vertex shader compilation
        let mut success = 0;
        gl.get_shader_iv(vertex_shader, gl::COMPILE_STATUS, &mut success)?;
        if success == 0 {
            let info_log = gl.get_shader_info_log(vertex_shader)?;
            gl.delete_shader(vertex_shader)?;
            return Err(format!("Vertex shader compilation failed: {}", info_log));
        }
        
        let fragment_shader = gl.create_shader(gl::FRAGMENT_SHADER)?;
        gl.set_shader_source(fragment_shader, fragment_shader_source)?;
        gl.compile_shader(fragment_shader)?;
        
        // Check fragment shader compilation
        let mut success = 0;
        gl.get_shader_iv(fragment_shader, gl::COMPILE_STATUS, &mut success)?;
        if success == 0 {
            let info_log = gl.get_shader_info_log(fragment_shader)?;
            gl.delete_shader(vertex_shader)?;
            gl.delete_shader(fragment_shader)?;
            return Err(format!("Fragment shader compilation failed: {}", info_log));
        }
        
        let shader_program = gl.create_program()?;
        gl.attach_shader(shader_program, vertex_shader)?;
        gl.attach_shader(shader_program, fragment_shader)?;
        gl.link_program(shader_program)?;
        
        // Check program linking
        let mut success = 0;
        gl.get_program_iv(shader_program, gl::LINK_STATUS, &mut success)?;
        if success == 0 {
            let info_log = gl.get_program_info_log(shader_program)?;
            gl.delete_shader(vertex_shader)?;
            gl.delete_shader(fragment_shader)?;
            gl.delete_program(shader_program)?;
            return Err(format!("Shader program linking failed: {}", info_log));
        }
        
        gl.delete_shader(vertex_shader)?;
        gl.delete_shader(fragment_shader)?;
        
        Ok(shader_program)
    }

    
    fn create_rect_geometry(gl: &GlWrapper) -> Result<(u32, u32), String> {
        let vertices: [f32; 8] = [
            -0.5, -0.5,  // bottom-left
             0.5, -0.5,  // bottom-right
            -0.5,  0.5,  // top-left
             0.5,  0.5,  // top-right
        ];
        
        let vao = gl.gen_vertex_array()?;
        let vbo = gl.gen_buffer()?;
        
        gl.bind_vertex_array(vao)?;
        gl.bind_buffer(gl::ARRAY_BUFFER, vbo)?;
        gl.set_buffer_data(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW)?;
        
        gl.set_vertex_attrib_pointer(0, 2, gl::FLOAT, false, 2 * std::mem::size_of::<f32>() as i32, 0)?;
        gl.enable_vertex_attrib_array(0)?;
        
        gl.bind_buffer(gl::ARRAY_BUFFER, 0)?;
        gl.bind_vertex_array(0)?;
        
        Ok((vao, vbo))
    }

}
