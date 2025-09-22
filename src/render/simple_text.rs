use crate::render::text::{Text, TextRenderer, TextAlign};
use crate::render::text_utils::TextUtils;
use crate::render::viewport::Viewport;
use glam::Vec2;
use std::rc::Rc;

/// Simple, intuitive text rendering API for end users
pub struct SimpleTextRenderer {
    text_renderer: TextRenderer,
}

impl SimpleTextRenderer {
    /// Create a new simple text renderer
    pub fn new(gl: Rc<crate::render::gl_wrapper::GlWrapper>) -> Result<Self, String> {
        let text_renderer = TextRenderer::new(gl);
        Ok(Self { text_renderer })
    }

    /// Initialize the text renderer
    pub fn initialize(&mut self) -> Result<(), String> {
        self.text_renderer.initialize()
    }

    /// Load a font with a simple name
    pub fn load_font(&mut self, name: &str, font_path: &str, size: u32) -> Result<(), String> {
        self.text_renderer.load_font(name, font_path, size)
    }

    /// Load a font with a specific size (creates a unique font name with size suffix)
    pub fn load_font_sized(&mut self, name: &str, font_path: &str, size: u32) -> Result<String, String> {
        let font_name = format!("{}_{}", name, size);
        self.text_renderer.load_font(&font_name, font_path, size)?;
        Ok(font_name)
    }

    /// Check if a font is loaded
    pub fn has_font(&self, name: &str) -> bool {
        self.text_renderer.has_font(name)
    }

    /// Check if a font with specific size is loaded
    pub fn has_font_sized(&self, name: &str, size: u32) -> bool {
        let font_name = format!("{}_{}", name, size);
        self.text_renderer.has_font(&font_name)
    }

    /// Draw text at a position with default styling
    pub fn draw_text(&self, text: &str, x: f32, y: f32, font_name: &str) -> Result<(), String> {
        let text_obj = TextUtils::simple_text(text, Vec2::new(x, y), font_name);
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw text with custom color
    pub fn draw_text_colored(&self, text: &str, x: f32, y: f32, font_name: &str, r: f32, g: f32, b: f32) -> Result<(), String> {
        let mut text_obj = TextUtils::simple_text(text, Vec2::new(x, y), font_name);
        text_obj.config.color = (r, g, b);
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw text with custom color and alpha
    pub fn draw_text_with_alpha(&self, text: &str, x: f32, y: f32, font_name: &str, r: f32, g: f32, b: f32, alpha: f32) -> Result<(), String> {
        let mut text_obj = TextUtils::simple_text(text, Vec2::new(x, y), font_name);
        text_obj.config.color = (r, g, b);
        text_obj.config.alpha = alpha;
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw centered text
    pub fn draw_text_centered(&self, text: &str, x: f32, y: f32, font_name: &str) -> Result<(), String> {
        let text_obj = TextUtils::title_text(text, Vec2::new(x, y), font_name);
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw right-aligned text
    pub fn draw_text_right(&self, text: &str, x: f32, y: f32, font_name: &str) -> Result<(), String> {
        let mut text_obj = TextUtils::simple_text(text, Vec2::new(x, y), font_name);
        text_obj.config.align = TextAlign::Right;
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw text with custom alignment
    pub fn draw_text_aligned(&self, text: &str, x: f32, y: f32, font_name: &str, align: TextAlign) -> Result<(), String> {
        let mut text_obj = TextUtils::simple_text(text, Vec2::new(x, y), font_name);
        text_obj.config.align = align;
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw a title (large, centered text)
    pub fn draw_title(&self, text: &str, x: f32, y: f32, font_name: &str) -> Result<(), String> {
        let text_obj = TextUtils::title_text(text, Vec2::new(x, y), font_name);
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw a subtitle (medium, centered text)
    pub fn draw_subtitle(&self, text: &str, x: f32, y: f32, font_name: &str) -> Result<(), String> {
        let text_obj = TextUtils::subtitle_text(text, Vec2::new(x, y), font_name);
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw info text (blue color)
    pub fn draw_info(&self, text: &str, x: f32, y: f32, font_name: &str) -> Result<(), String> {
        let text_obj = TextUtils::info_text(text, Vec2::new(x, y), font_name);
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw warning text (orange color)
    pub fn draw_warning(&self, text: &str, x: f32, y: f32, font_name: &str) -> Result<(), String> {
        let text_obj = TextUtils::warning_text(text, Vec2::new(x, y), font_name);
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw error text (red color)
    pub fn draw_error(&self, text: &str, x: f32, y: f32, font_name: &str) -> Result<(), String> {
        let text_obj = TextUtils::error_text(text, Vec2::new(x, y), font_name);
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw success text (green color)
    pub fn draw_success(&self, text: &str, x: f32, y: f32, font_name: &str) -> Result<(), String> {
        let text_obj = TextUtils::colored_text(text, Vec2::new(x, y), font_name, (0.0, 1.0, 0.0));
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw text with a specific size (automatically loads font if needed)
    /// Uses top-left coordinate system (0,0 = top-left, positive Y goes down)
    pub fn draw_text_sized(&mut self, text: &str, x: f32, y: f32, font_name: &str, font_path: &str, size: u32) -> Result<(), String> {
        let sized_font_name = format!("{}_{}", font_name, size);
        
        // Load the font with this size if not already loaded
        if !self.has_font(&sized_font_name) {
            self.load_font(&sized_font_name, font_path, size)?;
        }
        
        // Convert from top-left coordinates to viewport coordinates
        let top_left_pos = Vec2::new(x, y);
        let viewport_pos = self.text_renderer.viewport.top_left_to_viewport(top_left_pos);
        
        let text_obj = TextUtils::simple_text(text, viewport_pos, &sized_font_name);
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw colored text with a specific size (automatically loads font if needed)
    /// Uses top-left coordinate system (0,0 = top-left, positive Y goes down)
    pub fn draw_text_colored_sized(&mut self, text: &str, x: f32, y: f32, font_name: &str, font_path: &str, size: u32, r: f32, g: f32, b: f32) -> Result<(), String> {
        let sized_font_name = format!("{}_{}", font_name, size);
        
        // Load the font with this size if not already loaded
        if !self.has_font(&sized_font_name) {
            self.load_font(&sized_font_name, font_path, size)?;
        }
        
        // Convert from top-left coordinates to viewport coordinates
        let top_left_pos = Vec2::new(x, y);
        let viewport_pos = self.text_renderer.viewport.top_left_to_viewport(top_left_pos);
        
        let mut text_obj = TextUtils::simple_text(text, viewport_pos, &sized_font_name);
        text_obj.config.color = (r, g, b);
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw centered text with a specific size (automatically loads font if needed)
    /// Uses top-left coordinate system (0,0 = top-left, positive Y goes down)
    pub fn draw_text_centered_sized(&mut self, text: &str, x: f32, y: f32, font_name: &str, font_path: &str, size: u32) -> Result<(), String> {
        let sized_font_name = format!("{}_{}", font_name, size);
        
        // Load the font with this size if not already loaded
        if !self.has_font(&sized_font_name) {
            self.load_font(&sized_font_name, font_path, size)?;
        }
        
        // Convert from top-left coordinates to viewport coordinates
        let top_left_pos = Vec2::new(x, y);
        let viewport_pos = self.text_renderer.viewport.top_left_to_viewport(top_left_pos);
        
        // Create a text object with center alignment and the specified size
        let mut text_obj = Text::new(text.to_string(), viewport_pos, sized_font_name);
        text_obj.config.align = TextAlign::Center;
        text_obj.config.font_size = size;
        text_obj.config.color = (1.0, 1.0, 0.0); // Yellow color for demo titles
        
        self.text_renderer.render_text(&text_obj)
    }

    /// Draw text with custom line spacing
    pub fn draw_text_with_spacing(&self, text: &str, x: f32, y: f32, font_name: &str, line_spacing: f32) -> Result<(), String> {
        let mut text_obj = TextUtils::simple_text(text, Vec2::new(x, y), font_name);
        text_obj.config.line_spacing = line_spacing;
        self.text_renderer.render_text(&text_obj)
    }

    /// Set the coordinate range for text rendering
    pub fn set_coordinate_range(&mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        self.text_renderer.set_coordinate_range(x_min, x_max, y_min, y_max);
    }

    /// Get the current coordinate range
    pub fn get_coordinate_range(&self) -> ((f32, f32), (f32, f32)) {
        self.text_renderer.get_coordinate_range()
    }

    /// Set the target text height as a fraction of the coordinate range
    /// e.g., 0.05 means text should be 5% of the coordinate range height
    pub fn set_target_text_height(&mut self, height_fraction: f32) -> Result<(), super::viewport::ValidationError> {
        self.text_renderer.set_target_text_height(height_fraction)
    }

    /// Set the base font size for normalization
    pub fn set_base_font_size(&mut self, font_size: f32) -> Result<(), super::viewport::ValidationError> {
        self.text_renderer.set_base_font_size(font_size)
    }

    /// Get a mutable reference to the viewport for advanced configuration
    pub fn viewport_mut(&mut self) -> &mut Viewport {
        self.text_renderer.viewport_mut()
    }
    
    /// Set whether text should be viewport-independent or viewport-relative
    /// 
    /// # Arguments
    /// * `independent` - If true, text size stays constant regardless of viewport scale.
    ///                   If false, text scales with viewport (original behavior).
    pub fn set_viewport_independent_text(&mut self, independent: bool) {
        self.text_renderer.viewport_mut().set_viewport_independent_text(independent);
    }
    
    /// Draw text using top-left coordinate system
    /// 
    /// # Arguments
    /// * `text` - Text to render
    /// * `x` - X position (0.0 = left edge, 1.0 = right edge)
    /// * `y` - Y position (0.0 = top edge, 1.0 = bottom edge)
    /// * `font_name` - Name of the font to use
    /// * `font_path` - Path to the font file
    /// * `size` - Font size in pixels
    pub fn draw_text_top_left(&mut self, text: &str, x: f32, y: f32, font_name: &str, font_path: &str, size: u32) -> Result<(), String> {
        let top_left_pos = Vec2::new(x, y);
        let viewport_pos = self.text_renderer.viewport.top_left_to_viewport(top_left_pos);
        self.draw_text_sized(text, viewport_pos.x, viewport_pos.y, font_name, font_path, size)
    }
    
    /// Draw colored text using top-left coordinate system
    /// 
    /// # Arguments
    /// * `text` - Text to render
    /// * `x` - X position (0.0 = left edge, 1.0 = right edge)
    /// * `y` - Y position (0.0 = top edge, 1.0 = bottom edge)
    /// * `font_name` - Name of the font to use
    /// * `font_path` - Path to the font file
    /// * `size` - Font size in pixels
    /// * `r` - Red component (0.0 to 1.0)
    /// * `g` - Green component (0.0 to 1.0)
    /// * `b` - Blue component (0.0 to 1.0)
    pub fn draw_text_colored_top_left(&mut self, text: &str, x: f32, y: f32, font_name: &str, font_path: &str, size: u32, r: f32, g: f32, b: f32) -> Result<(), String> {
        let top_left_pos = Vec2::new(x, y);
        let viewport_pos = self.text_renderer.viewport.top_left_to_viewport(top_left_pos);
        self.draw_text_colored_sized(text, viewport_pos.x, viewport_pos.y, font_name, font_path, size, r, g, b)
    }
    
    /// Draw centered text using top-left coordinate system
    /// 
    /// # Arguments
    /// * `text` - Text to render
    /// * `x` - X position (0.0 = left edge, 1.0 = right edge)
    /// * `y` - Y position (0.0 = top edge, 1.0 = bottom edge)
    /// * `font_name` - Name of the font to use
    /// * `font_path` - Path to the font file
    /// * `size` - Font size in pixels
    pub fn draw_text_centered_top_left(&mut self, text: &str, x: f32, y: f32, font_name: &str, font_path: &str, size: u32) -> Result<(), String> {
        let top_left_pos = Vec2::new(x, y);
        let viewport_pos = self.text_renderer.viewport.top_left_to_viewport(top_left_pos);
        self.draw_text_centered_sized(text, viewport_pos.x, viewport_pos.y, font_name, font_path, size)
    }
    
    /// Draw text with word wrapping using top-left coordinate system
    /// 
    /// # Arguments
    /// * `text` - Text to render
    /// * `x` - X position (0.0 = left edge, 1.0 = right edge)
    /// * `y` - Y position (0.0 = top edge, 1.0 = bottom edge)
    /// * `font_name` - Name of the font to use
    /// * `font_path` - Path to the font file
    /// * `size` - Font size in pixels
    /// * `max_width` - Maximum width for wrapping (in viewport coordinates)
    pub fn draw_text_wrapped_top_left(&mut self, text: &str, x: f32, y: f32, font_name: &str, _font_path: &str, size: u32, max_width: f32) -> Result<(), String> {
        let top_left_pos = Vec2::new(x, y);
        let viewport_pos = self.text_renderer.viewport.top_left_to_viewport(top_left_pos);
        
        // Create a text object with wrapping configuration
        let mut text_obj = super::text::Text::new(text.to_string(), viewport_pos, font_name.to_string());
        text_obj.set_wrap(super::text::TextWrap::Word);
        text_obj.set_max_width(Some(max_width));
        text_obj.config.font_size = size;
        
        self.text_renderer.render_text(&text_obj)
    }
    
    /// Draw text with ellipsis truncation using top-left coordinate system
    /// 
    /// # Arguments
    /// * `text` - Text to render
    /// * `x` - X position (0.0 = left edge, 1.0 = right edge)
    /// * `y` - Y position (0.0 = top edge, 1.0 = bottom edge)
    /// * `font_name` - Name of the font to use
    /// * `font_path` - Path to the font file
    /// * `size` - Font size in pixels
    /// * `max_width` - Maximum width before truncation (in viewport coordinates)
    pub fn draw_text_ellipsis_top_left(&mut self, text: &str, x: f32, y: f32, font_name: &str, _font_path: &str, size: u32, max_width: f32) -> Result<(), String> {
        let top_left_pos = Vec2::new(x, y);
        let viewport_pos = self.text_renderer.viewport.top_left_to_viewport(top_left_pos);
        
        // Create a text object with ellipsis configuration
        let mut text_obj = super::text::Text::new(text.to_string(), viewport_pos, font_name.to_string());
        text_obj.set_wrap(super::text::TextWrap::Ellipsis);
        text_obj.set_max_width(Some(max_width));
        text_obj.config.font_size = size;
        
        self.text_renderer.render_text(&text_obj)
    }

    /// Get a reference to the viewport
    pub fn viewport(&self) -> &Viewport {
        &self.text_renderer.viewport
    }

    /// Get the underlying text renderer for advanced usage
    pub fn get_renderer(&self) -> &TextRenderer {
        &self.text_renderer
    }

    /// Get the underlying text renderer mutably for advanced usage
    pub fn get_renderer_mut(&mut self) -> &mut TextRenderer {
        &mut self.text_renderer
    }
}

