use std::rc::Rc;
use super::gl_wrapper::GlWrapper;
use super::texture::{TextureManager, TextureId};
use glam::Vec2;

/// A sprite that can be rendered with a texture
#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture_id: TextureId,
    pub position: Vec2,
    pub size: Vec2,
    pub tint_color: (f32, f32, f32),
    pub alpha: f32,
}

impl Sprite {
    /// Create a new sprite
    pub fn new(texture_id: TextureId, position: Vec2, size: Vec2) -> Self {
        Self {
            texture_id,
            position,
            size,
            tint_color: (1.0, 1.0, 1.0), // White tint (no color change)
            alpha: 1.0, // Fully opaque
        }
    }

    /// Create a new sprite with tint color
    pub fn new_with_tint(texture_id: TextureId, position: Vec2, size: Vec2, tint_color: (f32, f32, f32)) -> Self {
        Self {
            texture_id,
            position,
            size,
            tint_color,
            alpha: 1.0,
        }
    }

    /// Create a new sprite with tint color and alpha
    pub fn new_with_tint_alpha(texture_id: TextureId, position: Vec2, size: Vec2, tint_color: (f32, f32, f32), alpha: f32) -> Self {
        Self {
            texture_id,
            position,
            size,
            tint_color,
            alpha,
        }
    }

    /// Set the position of the sprite
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    /// Set the size of the sprite
    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }

    /// Set the tint color of the sprite
    pub fn set_tint_color(&mut self, tint_color: (f32, f32, f32)) {
        self.tint_color = tint_color;
    }

    /// Set the alpha (transparency) of the sprite
    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha.clamp(0.0, 1.0);
    }
}

/// Sprite renderer that handles rendering sprites with textures
pub struct SpriteRenderer {
    gl: Rc<GlWrapper>,
    texture_manager: Option<TextureManager>,
    sprite_shader: Option<u32>,
    sprite_vao: Option<u32>,
    sprite_vbo: Option<u32>,
    initialized: bool,
}

impl SpriteRenderer {
    /// Create a new sprite renderer
    pub fn new(gl: Rc<GlWrapper>) -> Self {
        Self {
            gl,
            texture_manager: None,
            sprite_shader: None,
            sprite_vao: None,
            sprite_vbo: None,
            initialized: false,
        }
    }

    /// Initialize the sprite renderer
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Ok(());
        }

        // Create texture manager
        self.texture_manager = Some(TextureManager::new(Rc::clone(&self.gl)));

        // Create sprite shader
        let sprite_shader = Self::create_sprite_shader(&self.gl)?;
        println!("Created sprite shader: {}", sprite_shader);

        // Create sprite geometry (quad with texture coordinates)
        let (sprite_vao, sprite_vbo) = Self::create_sprite_geometry(&self.gl)?;
        println!("Created sprite geometry - VAO: {}, VBO: {}", sprite_vao, sprite_vbo);

        self.sprite_shader = Some(sprite_shader);
        self.sprite_vao = Some(sprite_vao);
        self.sprite_vbo = Some(sprite_vbo);
        self.initialized = true;

        println!("Sprite renderer initialized successfully!");
        Ok(())
    }

    /// Get a reference to the texture manager
    pub fn texture_manager(&mut self) -> &mut TextureManager {
        self.texture_manager.as_mut().expect("Sprite renderer not initialized")
    }

    /// Render a sprite
    pub fn render_sprite(&self, sprite: &Sprite) -> Result<(), String> {
        if !self.initialized {
            return Err("Sprite renderer not initialized".to_string());
        }

        let shader = self.sprite_shader.ok_or("Sprite shader not available")?;
        let vao = self.sprite_vao.ok_or("Sprite VAO not available")?;
        let texture_manager = self.texture_manager.as_ref().ok_or("Texture manager not available")?;

        // Use sprite shader
        self.gl.use_program(shader)?;

        // Bind texture
        texture_manager.bind_texture(sprite.texture_id)?;

        // Set uniforms
        let pos_loc = self.gl.get_uniform_location(shader, "sprite_position")?;
        let size_loc = self.gl.get_uniform_location(shader, "sprite_size")?;
        let tint_loc = self.gl.get_uniform_location(shader, "tint_color")?;
        let alpha_loc = self.gl.get_uniform_location(shader, "alpha")?;
        let texture_loc = self.gl.get_uniform_location(shader, "texture_sampler")?;

        self.gl.set_uniform_2f(pos_loc, sprite.position.x, sprite.position.y)?;
        self.gl.set_uniform_2f(size_loc, sprite.size.x, sprite.size.y)?;
        self.gl.set_uniform_3f(tint_loc, sprite.tint_color.0, sprite.tint_color.1, sprite.tint_color.2)?;
        self.gl.set_uniform_1f(alpha_loc, sprite.alpha)?;
        self.gl.set_uniform_1i(texture_loc, 0)?; // Texture unit 0

        // Draw the sprite
        self.gl.bind_vertex_array(vao)?;
        self.gl.draw_arrays(gl::TRIANGLE_STRIP, 0, 4)?;

        Ok(())
    }

    /// Create sprite shader program
    fn create_sprite_shader(gl: &GlWrapper) -> Result<u32, String> {
        let vertex_shader_source = include_str!("shaders/sprite.vert");
        let fragment_shader_source = include_str!("shaders/sprite.frag");

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


    /// Create sprite geometry (quad with texture coordinates)
    fn create_sprite_geometry(gl: &GlWrapper) -> Result<(u32, u32), String> {
        // Vertices: position (x, y) + texture coordinates (u, v)
        let vertices: [f32; 16] = [
            // Position    // TexCoords
            -0.5, -0.5,    0.0, 1.0,  // bottom-left
             0.5, -0.5,    1.0, 1.0,  // bottom-right
            -0.5,  0.5,    0.0, 0.0,  // top-left
             0.5,  0.5,    1.0, 0.0,  // top-right
        ];

        let vao = gl.gen_vertex_array()?;
        let vbo = gl.gen_buffer()?;

        gl.bind_vertex_array(vao)?;
        gl.bind_buffer(gl::ARRAY_BUFFER, vbo)?;
        gl.set_buffer_data(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW)?;

        // Position attribute (location 0)
        gl.set_vertex_attrib_pointer(0, 2, gl::FLOAT, false, 4 * std::mem::size_of::<f32>() as i32, 0)?;
        gl.enable_vertex_attrib_array(0)?;

        // Texture coordinate attribute (location 1)
        gl.set_vertex_attrib_pointer(1, 2, gl::FLOAT, false, 4 * std::mem::size_of::<f32>() as i32, 2 * std::mem::size_of::<f32>() as usize)?;
        gl.enable_vertex_attrib_array(1)?;

        gl.bind_buffer(gl::ARRAY_BUFFER, 0)?;
        gl.bind_vertex_array(0)?;

        Ok((vao, vbo))
    }


    /// Cleanup resources
    pub fn cleanup(&mut self) {
        if let Some(shader) = self.sprite_shader.take() {
            let _ = self.gl.delete_program(shader);
        }
        if let Some(vao) = self.sprite_vao.take() {
            let _ = self.gl.delete_vertex_array(vao);
        }
        if let Some(vbo) = self.sprite_vbo.take() {
            let _ = self.gl.delete_buffer(vbo);
        }
        if let Some(ref mut texture_manager) = self.texture_manager {
            let _ = texture_manager.clear_all();
        }
    }
}

impl Drop for SpriteRenderer {
    fn drop(&mut self) {
        self.cleanup();
    }
}
