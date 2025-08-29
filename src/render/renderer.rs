use super::gl_wrapper::GlWrapper;
use glam::Vec2;

pub struct Renderer {
    gl: GlWrapper,
    basic_shader: Option<u32>,
    rect_vao: Option<u32>,
    rect_vbo: Option<u32>,
}

impl Renderer {
    pub fn new() -> Self {
        let gl_wrapper = GlWrapper::new();
        
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
        
        let basic_shader = Self::create_basic_shader(&self.gl)?;
        let (rect_vao, rect_vbo) = Self::create_rect_geometry(&self.gl)?;
        
        self.basic_shader = Some(basic_shader);
        self.rect_vao = Some(rect_vao);
        self.rect_vbo = Some(rect_vbo);
        
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
        
        // Set color uniform
        let color_location = self.gl.get_uniform_location(shader, "color")?;
        self.gl.set_uniform_3f(color_location, color.0, color.1, color.2)?;
        
        // Set position and size uniforms
        let pos_location = self.gl.get_uniform_location(shader, "rect_position")?;
        let size_location = self.gl.get_uniform_location(shader, "rect_size")?;
        self.gl.set_uniform_2f(pos_location, position.x, position.y)?;
        self.gl.set_uniform_2f(size_location, size.x, size.y)?;
        
        // Draw rectangle
        self.gl.bind_vertex_array(vao)?;
        self.gl.draw_arrays(gl::TRIANGLE_STRIP, 0, 4)?;
        self.gl.bind_vertex_array(0)?;
        
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
        
        let fragment_shader = gl.create_shader(gl::FRAGMENT_SHADER)?;
        gl.set_shader_source(fragment_shader, fragment_shader_source)?;
        gl.compile_shader(fragment_shader)?;
        
        let shader_program = gl.create_program()?;
        gl.attach_shader(shader_program, vertex_shader)?;
        gl.attach_shader(shader_program, fragment_shader)?;
        gl.link_program(shader_program)?;
        
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
        
        gl.set_vertex_attrib_pointer(0, 2, gl::FLOAT, false, 2 * std::mem::size_of::<f32>() as i32, std::ptr::null())?;
        gl.enable_vertex_attrib_array(0)?;
        
        gl.bind_buffer(gl::ARRAY_BUFFER, 0)?;
        gl.bind_vertex_array(0)?;
        
        Ok((vao, vbo))
    }
}
