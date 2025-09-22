use std::rc::Rc;
use std::collections::HashMap;
use super::gl_wrapper::GlWrapper;
use super::texture::{TextureManager, TextureId};
use super::viewport::Viewport;
use glam::Vec2;
use std::fs;

#[cfg(feature = "opengl")]
use fontdue::{Font, FontSettings};

/// A single character/glyph with its rendering information
#[derive(Debug, Clone)]
pub struct Glyph {
    pub texture_id: TextureId,
    pub size: Vec2,           // Size of the glyph in pixels
    pub bearing: Vec2,        // Offset from baseline to top-left of glyph
    pub advance: f32,         // Horizontal advance to next character
}

/// Font information and glyph cache
#[derive(Debug)]
pub struct FontInfo {
    pub name: String,
    pub size: u32,
    pub glyphs: HashMap<char, Glyph>,
    pub line_height: f32,
    pub ascender: f32,
    pub descender: f32,
    #[cfg(feature = "opengl")]
    pub fontdue_font: Option<Font>,
}

impl FontInfo {
    pub fn new(name: String, size: u32) -> Self {
        Self {
            name,
            size,
            glyphs: HashMap::new(),
            line_height: size as f32 * 1.2, // Default line height
            ascender: size as f32 * 0.8,    // Default ascender
            descender: size as f32 * 0.2,   // Default descender
            #[cfg(feature = "opengl")]
            fontdue_font: None,
        }
    }
}

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// Text wrapping options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextWrap {
    None,           // No wrapping, text may overflow
    Word,           // Wrap at word boundaries
    Character,      // Wrap at any character
    Ellipsis,       // Truncate with "..." if too long
}

/// Text rendering configuration
#[derive(Debug, Clone)]
pub struct TextConfig {
    pub font_size: u32,
    pub color: (f32, f32, f32),
    pub alpha: f32,
    pub align: TextAlign,
    pub max_width: Option<f32>,
    pub line_spacing: f32,
    pub wrap: TextWrap,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            font_size: 16,
            color: (1.0, 1.0, 1.0), // White
            alpha: 1.0,
            align: TextAlign::Left,
            max_width: None,
            line_spacing: 1.2,
            wrap: TextWrap::None,
        }
    }
}

/// A text object that can be rendered
#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
    pub position: Vec2,
    pub config: TextConfig,
    pub font_name: String,
}

impl Text {
    pub fn new(content: String, position: Vec2, font_name: String) -> Self {
        Self {
            content,
            position,
            config: TextConfig::default(),
            font_name,
        }
    }

    pub fn with_config(content: String, position: Vec2, font_name: String, config: TextConfig) -> Self {
        Self {
            content,
            position,
            config,
            font_name,
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn set_color(&mut self, color: (f32, f32, f32)) {
        self.config.color = color;
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.config.alpha = alpha.clamp(0.0, 1.0);
    }

    pub fn set_align(&mut self, align: TextAlign) {
        self.config.align = align;
    }

    pub fn set_wrap(&mut self, wrap: TextWrap) {
        self.config.wrap = wrap;
    }

    pub fn set_max_width(&mut self, max_width: Option<f32>) {
        self.config.max_width = max_width;
    }
}

/// Text renderer that handles font loading and text rendering
pub struct TextRenderer {
    gl: Rc<GlWrapper>,
    texture_manager: Option<TextureManager>,
    text_shader: Option<u32>,
    text_vao: Option<u32>,
    text_vbo: Option<u32>,
    fonts: HashMap<String, FontInfo>,
    initialized: bool,
    // Viewport configuration - defines the logical coordinate system
    pub viewport: Viewport,
}

impl TextRenderer {
    /// Create a new text renderer
    pub fn new(gl: Rc<GlWrapper>) -> Self {
        Self {
            gl,
            texture_manager: None,
            text_shader: None,
            text_vao: None,
            text_vbo: None,
            fonts: HashMap::new(),
            initialized: false,
            viewport: Viewport::new(),
        }
    }

    /// Set the coordinate range for text rendering
    pub fn set_coordinate_range(&mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        self.viewport.logical_bounds = (x_min, x_max, y_min, y_max);
    }

    /// Get the current coordinate range
    pub fn get_coordinate_range(&self) -> ((f32, f32), (f32, f32)) {
        ((self.viewport.logical_bounds.0, self.viewport.logical_bounds.1), 
         (self.viewport.logical_bounds.2, self.viewport.logical_bounds.3))
    }

    /// Set the target text height as a fraction of the coordinate range
    /// e.g., 0.05 means text should be 5% of the coordinate range height
    pub fn set_target_text_height(&mut self, height_fraction: f32) -> Result<(), super::viewport::ValidationError> {
        self.viewport.set_text_height_fraction(height_fraction)
    }

    /// Set the base font size for normalization
    pub fn set_base_font_size(&mut self, font_size: f32) -> Result<(), super::viewport::ValidationError> {
        self.viewport.set_base_font_size(font_size)
    }

    /// Get a mutable reference to the viewport for advanced configuration
    pub fn viewport_mut(&mut self) -> &mut Viewport {
        &mut self.viewport
    }

    /// Initialize the text renderer
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Ok(());
        }

        // Create texture manager
        self.texture_manager = Some(TextureManager::new(Rc::clone(&self.gl)));

        // Create text shader
        let text_shader = Self::create_text_shader(&self.gl)?;
        println!("Created text shader: {}", text_shader);

        // Create text geometry (quad with texture coordinates)
        let (text_vao, text_vbo) = Self::create_text_geometry(&self.gl)?;
        println!("Created text geometry - VAO: {}, VBO: {}", text_vao, text_vbo);
        
        // Enable blending for alpha transparency
        self.gl.enable_blending()?;
        self.gl.set_blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)?;

        self.text_shader = Some(text_shader);
        self.text_vao = Some(text_vao);
        self.text_vbo = Some(text_vbo);
        self.initialized = true;

        println!("Text renderer initialized successfully!");
        Ok(())
    }

    /// Get a reference to the texture manager
    pub fn texture_manager(&mut self) -> &mut TextureManager {
        self.texture_manager.as_mut().expect("Text renderer not initialized")
    }

    /// Load a font from a TTF file using fontdue
    pub fn load_font(&mut self, name: &str, font_path: &str, size: u32) -> Result<(), String> {
        if !self.initialized {
            return Err("Text renderer not initialized".to_string());
        }

        println!("Loading font: {} from {}", name, font_path);
        
        // Load font file
        let font_data = fs::read(font_path)
            .map_err(|e| format!("Failed to read font file '{}': {}", font_path, e))?;
        
        // Parse font with fontdue using high-quality settings
        let font_settings = FontSettings {
            scale: 40.0, // Higher scale for better quality
            collection_index: 0,
        };
        let fontdue_font = Font::from_bytes(font_data, font_settings)
            .map_err(|e| format!("Failed to parse font '{}': {:?}", font_path, e))?;
        
        let mut font_info = FontInfo::new(name.to_string(), size);
        font_info.fontdue_font = Some(fontdue_font);
        
        // Get font metrics
        let metrics = font_info.fontdue_font.as_ref().unwrap().metrics('A', size as f32);
        font_info.line_height = metrics.height as f32;
        font_info.ascender = metrics.height as f32 * 0.8; // Approximate ascender
        font_info.descender = metrics.height as f32 * 0.2; // Approximate descender
        
            // Pre-generate glyphs for common ASCII characters using fontdue
            self.generate_glyphs_with_fontdue(&mut font_info, size)?;
        
        self.fonts.insert(name.to_string(), font_info);
        println!("Font '{}' loaded successfully with {} glyphs", name, self.fonts[name].glyphs.len());
        
        Ok(())
    }

    /// Generate glyphs using fontdue
    fn generate_glyphs_with_fontdue(&mut self, font_info: &mut FontInfo, size: u32) -> Result<(), String> {
        // Generate glyphs for common ASCII characters
        for ch in 32..=126 { // ASCII printable characters
            let char_str = ch as u8 as char;
            
            // Rasterize the character using fontdue with higher resolution
            let render_scale = (size as f32 * 2.0).max(32.0); // Render at 2x resolution for better quality
            let (metrics, bitmap) = font_info.fontdue_font.as_ref().unwrap().rasterize(char_str, render_scale);
            
            
            // Create texture from the bitmap
            let texture_id = self.create_texture_from_bitmap(&bitmap, metrics.width as u32, metrics.height as u32)?;
            
            // Scale down metrics to match the requested font size
            let scale_factor = size as f32 / render_scale;
            let glyph = Glyph {
                texture_id,
                size: Vec2::new(metrics.width as f32 * scale_factor, metrics.height as f32 * scale_factor),
                bearing: Vec2::new(metrics.xmin as f32 * scale_factor, metrics.ymin as f32 * scale_factor),
                advance: metrics.advance_width * scale_factor,
            };
            
            font_info.glyphs.insert(char_str, glyph);
        }
        
        Ok(())
    }

    /// Create a texture from fontdue bitmap data
    fn create_texture_from_bitmap(&mut self, bitmap: &[u8], width: u32, height: u32) -> Result<TextureId, String> {
        let texture_manager = self.texture_manager.as_mut().unwrap();
        
        // For font textures, we typically use the grayscale data directly as the alpha channel
        // and set RGB to white (255, 255, 255) so the text color can be applied in the shader
        let mut pixels = vec![0u8; (width * height * 4) as usize];
        
        for i in 0..(width * height) as usize {
            let pixel_index = i * 4;
            let alpha = bitmap[i];
            pixels[pixel_index] = 255;     // R (white)
            pixels[pixel_index + 1] = 255; // G (white)
            pixels[pixel_index + 2] = 255; // B (white)
            pixels[pixel_index + 3] = alpha; // A (glyph shape)
        }
        
        texture_manager.create_texture_from_data(width, height, &pixels)
    }

    /// Render text
    pub fn render_text(&self, text: &Text) -> Result<(), String> {
        if !self.initialized {
            return Err("Text renderer not initialized".to_string());
        }

        let font = self.fonts.get(&text.font_name)
            .ok_or_else(|| format!("Font '{}' not found", text.font_name))?;

        let shader = self.text_shader.ok_or("Text shader not initialized")?;
        let vao = self.text_vao.ok_or("Text VAO not initialized")?;

        self.gl.use_program(shader)?;

        // Set text color and alpha
        let color_loc = self.gl.get_uniform_location(shader, "text_color")?;
        self.gl.set_uniform_3f(color_loc, text.config.color.0, text.config.color.1, text.config.color.2)?;
        
        let alpha_loc = self.gl.get_uniform_location(shader, "alpha")?;
        self.gl.set_uniform_1f(alpha_loc, text.config.alpha)?;
        
        // Set texture uniform
        let texture_loc = self.gl.get_uniform_location(shader, "text_texture")?;
        self.gl.set_uniform_1i(texture_loc, 0)?; // Use texture unit 0

        // Process text with wrapping
        let wrapped_content = self.process_text_wrapping(text, font);
        
        // Calculate text width for alignment (use first line for alignment)
        let first_line = wrapped_content.lines().next().unwrap_or("");
        let text_width = self.calculate_text_width(first_line, font);
        let scale_factor = self.viewport.calculate_scale_factor(font.size as f32);
        
        let start_x = match text.config.align {
            TextAlign::Left => {
                // For left alignment, subtract the first character's bearing to position text exactly at the specified x
                let first_char_bearing = text.content.chars().next()
                    .and_then(|ch| font.glyphs.get(&ch))
                    .map(|glyph| glyph.bearing.x * scale_factor)
                    .unwrap_or(0.0);
                text.position.x - first_char_bearing
            },
            TextAlign::Center => {
                // For center alignment, also account for the first character's bearing
                let first_char_bearing = text.content.chars().next()
                    .and_then(|ch| font.glyphs.get(&ch))
                    .map(|glyph| glyph.bearing.x * scale_factor)
                    .unwrap_or(0.0);
                text.position.x - text_width / 2.0 - first_char_bearing
            },
            TextAlign::Right => {
                // For right alignment, also account for the first character's bearing
                let first_char_bearing = text.content.chars().next()
                    .and_then(|ch| font.glyphs.get(&ch))
                    .map(|glyph| glyph.bearing.x * scale_factor)
                    .unwrap_or(0.0);
                text.position.x - text_width - first_char_bearing
            },
        };

        // Render each character
        let mut current_x = start_x;
        let mut current_y = text.position.y;

        for ch in wrapped_content.chars() {
            if ch == '\n' {
                current_x = start_x;
                current_y -= font.line_height * text.config.line_spacing * scale_factor; // Scale line height
                continue;
            }

            if let Some(glyph) = font.glyphs.get(&ch) {
                // Calculate glyph position (scaled for normalized coordinates)
                let glyph_x = current_x + glyph.bearing.x * scale_factor;
                let glyph_y = current_y + glyph.bearing.y * scale_factor;

                // Render the glyph
                self.render_glyph(glyph, Vec2::new(glyph_x, glyph_y), shader, vao, font.size, scale_factor)?;

                // Advance to next character (scaled for normalized coordinates)
                current_x += glyph.advance * scale_factor;
            }
        }

        Ok(())
    }

    /// Render a single glyph
    fn render_glyph(&self, glyph: &Glyph, position: Vec2, shader: u32, vao: u32, font_size: u32, scale_factor: f32) -> Result<(), String> {
        // Use the scale factor passed from the main render loop (no duplicate calculation)
        let scaled_size = Vec2::new(glyph.size.x * scale_factor, glyph.size.y * scale_factor);
        
        // Convert logical position to NDC coordinates
        let gl_position = self.viewport.logical_to_ndc(position);
        
        // Scale the glyph size for NDC space
        let (x_range, y_range) = self.viewport.get_logical_ranges();
        let gl_size = Vec2::new(
            scaled_size.x * (2.0 / x_range),  // Scale width for NDC space
            scaled_size.y * (2.0 / y_range)   // Scale height for NDC space
        );
        
        // Set glyph position and size
        let pos_loc = self.gl.get_uniform_location(shader, "glyph_position")?;
        let size_loc = self.gl.get_uniform_location(shader, "glyph_size")?;
        
        self.gl.set_uniform_2f(pos_loc, gl_position.x, gl_position.y)?;
        self.gl.set_uniform_2f(size_loc, gl_size.x, gl_size.y)?;

        // Bind the glyph texture to texture unit 0
        let texture_manager = self.texture_manager.as_ref().unwrap();
        self.gl.active_texture(0x84C0)?; // GL_TEXTURE0
        texture_manager.bind_texture(glyph.texture_id)?;

        // Draw the quad
        self.gl.bind_vertex_array(vao)?;
        self.gl.draw_arrays(gl::TRIANGLE_STRIP, 0, 4)?;

        Ok(())
    }

    /// Calculate the width of text in logical coordinates
    fn calculate_text_width(&self, text: &str, font: &FontInfo) -> f32 {
        let mut width: f32 = 0.0;
        let mut max_width: f32 = 0.0;
        let scale_factor = self.viewport.calculate_scale_factor(font.size as f32);
        let (_x_range, _) = self.viewport.get_logical_ranges();

        for ch in text.chars() {
            if ch == '\n' {
                max_width = max_width.max(width);
                width = 0.0;
            } else if let Some(glyph) = font.glyphs.get(&ch) {
                width += glyph.advance * scale_factor;
            }
        }

        max_width.max(width)
    }

    /// Process text with wrapping based on configuration
    fn process_text_wrapping(&self, text: &Text, font: &FontInfo) -> String {
        match text.config.wrap {
            TextWrap::None => text.content.clone(),
            TextWrap::Word => self.wrap_text_by_words(&text.content, font, text.config.max_width),
            TextWrap::Character => self.wrap_text_by_characters(&text.content, font, text.config.max_width),
            TextWrap::Ellipsis => self.truncate_text_with_ellipsis(&text.content, font, text.config.max_width),
        }
    }

    /// Wrap text at word boundaries
    fn wrap_text_by_words(&self, text: &str, font: &FontInfo, max_width: Option<f32>) -> String {
        let max_width = match max_width {
            Some(width) => width,
            None => {
                // Use viewport width as default
                let (x_range, _) = self.viewport.get_logical_ranges();
                x_range * 0.9 // 90% of viewport width
            }
        };

        let mut result = String::new();
        let mut current_line = String::new();
        let mut current_width = 0.0;
        let scale_factor = self.viewport.calculate_scale_factor(font.size as f32);

        for word in text.split_whitespace() {
            let word_width = self.calculate_word_width(word, font, scale_factor);
            
            if current_width + word_width > max_width && !current_line.is_empty() {
                // Start new line
                result.push_str(&current_line);
                result.push('\n');
                current_line.clear();
                current_width = 0.0;
            }
            
            if !current_line.is_empty() {
                current_line.push(' ');
                current_width += self.calculate_char_width(' ', font, scale_factor);
            }
            
            current_line.push_str(word);
            current_width += word_width;
        }
        
        if !current_line.is_empty() {
            result.push_str(&current_line);
        }
        
        result
    }

    /// Wrap text at character boundaries
    fn wrap_text_by_characters(&self, text: &str, font: &FontInfo, max_width: Option<f32>) -> String {
        let max_width = match max_width {
            Some(width) => width,
            None => {
                let (x_range, _) = self.viewport.get_logical_ranges();
                x_range * 0.9
            }
        };

        let mut result = String::new();
        let mut current_line = String::new();
        let mut current_width = 0.0;
        let scale_factor = self.viewport.calculate_scale_factor(font.size as f32);

        for ch in text.chars() {
            if ch == '\n' {
                result.push_str(&current_line);
                result.push('\n');
                current_line.clear();
                current_width = 0.0;
                continue;
            }

            let char_width = self.calculate_char_width(ch, font, scale_factor);
            
            if current_width + char_width > max_width && !current_line.is_empty() {
                result.push_str(&current_line);
                result.push('\n');
                current_line.clear();
                current_width = 0.0;
            }
            
            current_line.push(ch);
            current_width += char_width;
        }
        
        if !current_line.is_empty() {
            result.push_str(&current_line);
        }
        
        result
    }

    /// Truncate text with ellipsis if too long
    fn truncate_text_with_ellipsis(&self, text: &str, font: &FontInfo, max_width: Option<f32>) -> String {
        let max_width = match max_width {
            Some(width) => width,
            None => {
                let (x_range, _) = self.viewport.get_logical_ranges();
                x_range * 0.9
            }
        };

        let scale_factor = self.viewport.calculate_scale_factor(font.size as f32);
        let ellipsis_width = self.calculate_word_width("...", font, scale_factor);
        
        if self.calculate_text_width(text, font) <= max_width {
            return text.to_string();
        }

        let mut result = String::new();
        let mut current_width = 0.0;
        
        for ch in text.chars() {
            let char_width = self.calculate_char_width(ch, font, scale_factor);
            
            if current_width + char_width + ellipsis_width > max_width {
                result.push_str("...");
                break;
            }
            
            result.push(ch);
            current_width += char_width;
        }
        
        result
    }

    /// Calculate the width of a single character
    fn calculate_char_width(&self, ch: char, font: &FontInfo, scale_factor: f32) -> f32 {
        font.glyphs.get(&ch)
            .map(|glyph| glyph.advance * scale_factor)
            .unwrap_or(0.0)
    }

    /// Calculate the width of a word
    fn calculate_word_width(&self, word: &str, font: &FontInfo, scale_factor: f32) -> f32 {
        word.chars()
            .map(|ch| self.calculate_char_width(ch, font, scale_factor))
            .sum()
    }

    /// Create the text shader
    fn create_text_shader(gl: &GlWrapper) -> Result<u32, String> {
        let vertex_source = include_str!("shaders/text.vert");
        let fragment_source = include_str!("shaders/text.frag");

        let vertex_shader = gl.create_shader(gl::VERTEX_SHADER)?;
        gl.set_shader_source(vertex_shader, vertex_source)?;
        gl.compile_shader(vertex_shader)?;

        let fragment_shader = gl.create_shader(gl::FRAGMENT_SHADER)?;
        gl.set_shader_source(fragment_shader, fragment_source)?;
        gl.compile_shader(fragment_shader)?;

        let program = gl.create_program()?;
        let _ = gl.attach_shader(program, vertex_shader);
        let _ = gl.attach_shader(program, fragment_shader);
        gl.link_program(program)?;

        // Clean up shaders
        let _ = gl.delete_shader(vertex_shader);
        let _ = gl.delete_shader(fragment_shader);

        Ok(program)
    }

    /// Create the text geometry (quad with texture coordinates)
    fn create_text_geometry(gl: &GlWrapper) -> Result<(u32, u32), String> {
        // Quad vertices with texture coordinates
        let vertices: [f32; 16] = [
            // Position    // Texture coords
            0.0, 0.0,      0.0, 1.0,  // Bottom-left
            1.0, 0.0,      1.0, 1.0,  // Bottom-right
            0.0, 1.0,      0.0, 0.0,  // Top-left
            1.0, 1.0,      1.0, 0.0,  // Top-right
        ];

        let vao = gl.gen_vertex_array()?;
        let vbo = gl.gen_buffer()?;

        let _ = gl.bind_vertex_array(vao);
        let _ = gl.bind_buffer(gl::ARRAY_BUFFER, vbo);
        gl.set_buffer_data(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW)?;

        // Position attribute
        gl.set_vertex_attrib_pointer(0, 2, gl::FLOAT, false, 4 * 4, 0)?;
        gl.enable_vertex_attrib_array(0)?;

        // Texture coordinate attribute
        gl.set_vertex_attrib_pointer(1, 2, gl::FLOAT, false, 4 * 4, 2 * 4)?;
        gl.enable_vertex_attrib_array(1)?;

        let _ = gl.bind_vertex_array(0);

        Ok((vao, vbo))
    }

    /// Get a font by name
    pub fn get_font(&self, name: &str) -> Option<&FontInfo> {
        self.fonts.get(name)
    }

    /// Check if a font is loaded
    pub fn has_font(&self, name: &str) -> bool {
        self.fonts.contains_key(name)
    }

    /// Get all loaded font names
    pub fn get_font_names(&self) -> Vec<String> {
        self.fonts.keys().cloned().collect()
    }
}

impl Drop for TextRenderer {
    fn drop(&mut self) {
        if let Some(shader) = self.text_shader.take() {
            let _ = self.gl.delete_program(shader);
        }
        if let Some(vao) = self.text_vao.take() {
            let _ = self.gl.delete_vertex_array(vao);
        }
        if let Some(vbo) = self.text_vbo.take() {
            let _ = self.gl.delete_buffer(vbo);
        }
    }
}
