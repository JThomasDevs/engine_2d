use crate::render::text::{Text, TextRenderer, TextAlign};
use crate::render::text_utils::TextUtils;
use crate::render::viewport::Viewport;
use glam::Vec2;
use std::rc::Rc;
use std::collections::HashMap;
use std::fs;

/// Text anchor point for positioning text relative to coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAnchor {
    TopLeft,     // (0,0) = top-left of text box
    TopCenter,   // (0,0) = top-center of text box
    TopRight,    // (0,0) = top-right of text box
    MiddleLeft,  // (0,0) = middle-left of text box
    MiddleCenter, // (0,0) = center of text box
    MiddleRight, // (0,0) = middle-right of text box
    BottomLeft,  // (0,0) = bottom-left of text box
    BottomCenter, // (0,0) = bottom-center of text box
    BottomRight, // (0,0) = bottom-right of text box
}

impl Default for TextAnchor {
    fn default() -> Self {
        TextAnchor::TopLeft
    }
}

/// Font configuration struct that can be reused
#[derive(Debug, Clone)]
pub struct Font {
    pub name: String,
    pub path: String,
}

impl Font {
    /// Create a new font configuration
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

/// Text configuration for fluent API
#[derive(Debug, Clone)]
pub struct TextConfig {
    pub font: Option<Font>,
    pub size: u32,
    pub color: (f32, f32, f32),
    pub alpha: f32,
    pub alignment: TextAlign,
    pub max_width: Option<f32>,
    pub line_spacing: f32,
    pub anchor: TextAnchor,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            font: None,
            size: 16,
            color: (1.0, 1.0, 1.0), // White
            alpha: 1.0,
            alignment: TextAlign::Left,
            max_width: None,
            line_spacing: 1.2,
            anchor: TextAnchor::TopLeft,
        }
    }
}

impl TextConfig {
    /// Set the font
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }
    
    /// Set the font size
    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }
    
    /// Set the text color (RGB)
    pub fn color(mut self, r: f32, g: f32, b: f32) -> Self {
        self.color = (r, g, b);
        self
    }
    
    /// Set the text alpha
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }
    
    /// Set the text alignment
    pub fn align(mut self, alignment: TextAlign) -> Self {
        self.alignment = alignment;
        self
    }
    
    /// Set the maximum width for text wrapping
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }
    
    /// Set the line spacing
    pub fn line_spacing(mut self, spacing: f32) -> Self {
        self.line_spacing = spacing;
        self
    }
    
    /// Set the text anchor point
    pub fn anchor(mut self, anchor: TextAnchor) -> Self {
        self.anchor = anchor;
        self
    }
}

/// Fluent text builder for method chaining
pub struct FluentTextBuilder<'a> {
    renderer: &'a mut SimpleTextRenderer,
    font_name: String,
    config: TextConfig,
}

impl<'a> FluentTextBuilder<'a> {
    fn new(renderer: &'a mut SimpleTextRenderer, font_name: String) -> Self {
        let font = renderer.fonts.get(&font_name)
            .cloned()
            .unwrap_or_else(|| {
                let fallback_path = &renderer.fallback_font_path;
                log::warn!(
                    "Font '{}' not found, attempting to use fallback font at '{}'",
                    font_name, fallback_path
                );
                
                // Check if the fallback font file exists
                if fs::metadata(fallback_path).is_ok() {
                    log::info!("Using fallback font from '{}'", fallback_path);
                    Font::new(&font_name, fallback_path)
                } else {
                    log::error!(
                        "Fallback font file not found at '{}'. Text rendering may fail.",
                        fallback_path
                    );
                    // Still create the font object but log the error
                    // The actual text rendering will handle the missing file
                    Font::new(&font_name, fallback_path)
                }
            });
        
        let mut config = TextConfig::default();
        config.font = Some(font);
        
        Self {
            renderer,
            font_name,
            config,
        }
    }
    
    /// Set the font size
    pub fn size(mut self, size: u32) -> Self {
        self.config.size = size;
        self
    }
    
    /// Set the text color (RGB)
    pub fn color(mut self, r: f32, g: f32, b: f32) -> Self {
        self.config.color = (r, g, b);
        self
    }
    
    /// Set the text alpha
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.config.alpha = alpha;
        self
    }
    
    /// Set the text alignment
    pub fn align(mut self, alignment: TextAlign) -> Self {
        self.config.alignment = alignment;
        self
    }
    
    /// Set the maximum width for text wrapping
    pub fn max_width(mut self, width: f32) -> Self {
        self.config.max_width = Some(width);
        self
    }
    
    /// Set the line spacing
    pub fn line_spacing(mut self, spacing: f32) -> Self {
        self.config.line_spacing = spacing;
        self
    }
    
    /// Set the text anchor point
    pub fn anchor(mut self, anchor: TextAnchor) -> Self {
        self.config.anchor = anchor;
        self
    }
    
    /// Draw the text at the specified position
    pub fn draw(self, text: &str, x: f32, y: f32) -> Result<(), String> {
        self.renderer.draw_text_fluent(text, x, y, self.config)
    }
}

/// Simple, intuitive text rendering API for end users
pub struct SimpleTextRenderer {
    text_renderer: TextRenderer,
    fonts: HashMap<String, Font>, // Registry of available fonts
    fallback_font_path: String, // Configurable fallback font path
}

impl SimpleTextRenderer {
    /// Create a new simple text renderer
    pub fn new(gl: Rc<crate::render::gl_wrapper::GlWrapper>, fallback_font_path: String) -> Result<Self, String> {
        let text_renderer = TextRenderer::new(gl);
        Ok(Self { 
            text_renderer,
            fonts: HashMap::new(),
            fallback_font_path,
        })
    }
    
    /// Register a font for use with the fluent API
    pub fn register_font(&mut self, name: &str, font: Font) {
        self.fonts.insert(name.to_string(), font);
    }
    
    /// Get a font by name for fluent API
    pub fn font(&mut self, name: &str) -> FluentTextBuilder {
        FluentTextBuilder::new(self, name.to_string())
    }
    
    /// Get anchor multipliers for a given TextAnchor
    /// Returns (x_multiplier, y_multiplier) where:
    /// - x_multiplier: -1.0 for right, -0.5 for center, 0.0 for left
    /// - y_multiplier: 1.0 for top (below baseline), 0.0 for middle (at baseline), -1.0 for bottom (above baseline)
    fn get_anchor_multipliers(anchor: TextAnchor) -> (f32, f32) {
        match anchor {
            TextAnchor::TopLeft => (0.5, -4.5),
            TextAnchor::TopCenter => (-0.2, -4.5),
            TextAnchor::TopRight => (-0.8, -4.5),

            TextAnchor::MiddleLeft => (0.5, -3.5),
            TextAnchor::MiddleCenter => (-0.2, -3.5),
            TextAnchor::MiddleRight => (-0.8, -3.5),

            TextAnchor::BottomLeft => (0.5, -2.5),
            TextAnchor::BottomCenter => (-0.2, -2.5),
            TextAnchor::BottomRight => (-0.8, -2.5),
        }
    }

    /// Calculate anchor offset for text positioning
    fn calculate_anchor_offset(&self, text: &str, config: &TextConfig) -> Result<Vec2, String> {
        let font = config.font.as_ref()
            .ok_or("No font specified in TextConfig")?;
        
        let sized_font_name = format!("{}_{}", font.name, config.size);
        
        // Get font info to calculate text dimensions
        let font_info = self.text_renderer.get_font(&sized_font_name)
            .ok_or_else(|| format!("Font '{}' not found", sized_font_name))?;
        
        // Calculate text width and height
        let scale_factor = self.text_renderer.viewport.calculate_scale_factor(config.size as f32);
        let text_width = self.calculate_text_width(text, &font_info, scale_factor);
        let text_height = font_info.line_height * scale_factor;
        
        // Get the first character's bearing - the text renderer will subtract this from our position
        let first_char_bearing = text.chars().next()
            .and_then(|ch| font_info.glyphs.get(&ch))
            .map(|glyph| glyph.bearing.x * scale_factor)
            .unwrap_or(0.0);
        
        
        // Get the font's ascender (height above baseline) and descender (depth below baseline)
        let font_ascender = font_info.ascender;
        let font_descender = font_info.descender;
        let total_height = font_ascender - font_descender; // Total text height from top to bottom
        
        // Get anchor multipliers and calculate offsets
        let (x_mul, y_mul) = Self::get_anchor_multipliers(config.anchor);
        
        // Calculate offsets using multipliers
        // X offset: multiply text width by horizontal multiplier, add bearing
        let x_offset = x_mul * text_width + first_char_bearing;
        
        // Y offset: multiply total height by vertical multiplier
        // Top anchors (y_mul = 1.0): position text below baseline (positive Y)
        // Middle anchors (y_mul = 0.0): position text at baseline (no Y offset)
        // Bottom anchors (y_mul = -1.0): position text above baseline (negative Y)
        let y_offset = y_mul * total_height;
        
        let offset = Vec2::new(x_offset, y_offset);
        
        Ok(offset)
    }
    
    /// Calculate anchor offset in viewport coordinates
    fn calculate_anchor_offset_viewport(&self, text: &str, config: &TextConfig, viewport_pos: Vec2) -> Result<Vec2, String> {
        let font = config.font.as_ref()
            .ok_or("No font specified in TextConfig")?;
        
        let sized_font_name = format!("{}_{}", font.name, config.size);
        
        // Get font info to calculate text dimensions
        let font_info = self.text_renderer.get_font(&sized_font_name)
            .ok_or_else(|| format!("Font '{}' not found", sized_font_name))?;
        
        // Calculate text width and height
        let scale_factor = self.text_renderer.viewport.calculate_scale_factor(config.size as f32);
        let text_width = self.calculate_text_width(text, &font_info, scale_factor);
        
        // Get the font's ascender and descender
        let font_ascender = font_info.ascender * scale_factor;
        let font_descender = font_info.descender * scale_factor;
        let total_height = font_ascender - font_descender;
        
        // Get anchor multipliers and calculate base offsets
        let (x_mul, y_mul) = Self::get_anchor_multipliers(config.anchor);
        
        // Calculate base offsets using multipliers
        let x_offset = x_mul * text_width; // No bearing adjustment needed in viewport coordinates
        let y_offset = y_mul * total_height;
        
        // Apply coordinate system specific transforms for viewport coordinates
        // In viewport coordinates: positive Y goes up, so we need to adjust the Y offset
        // Use the calculated y_offset from multipliers for all anchors
        let final_y_offset = y_offset;
        
        let offset = Vec2::new(x_offset, final_y_offset);
        
        Ok(offset)
    }
    
    /// Calculate text width for a given text and font
    fn calculate_text_width(&self, text: &str, font: &crate::render::text::FontInfo, scale_factor: f32) -> f32 {
        let mut width = 0.0;
        for ch in text.chars() {
            if let Some(glyph) = font.glyphs.get(&ch) {
                width += glyph.advance * scale_factor;
            }
        }
        width
    }
    
    /// Draw text using the fluent API
    pub fn draw_text_fluent(&mut self, text: &str, x: f32, y: f32, config: TextConfig) -> Result<(), String> {
        let font = config.font.as_ref()
            .ok_or("No font specified in TextConfig")?;
        
        let sized_font_name = format!("{}_{}", font.name, config.size);
        
        // Load the font with this size if not already loaded
        if !self.has_font(&sized_font_name) {
            self.load_font(&sized_font_name, &font.path, config.size)?;
        }
        
        // Convert from top-left coordinates to viewport coordinates first
        let viewport_pos = self.text_renderer.viewport.top_left_to_viewport(Vec2::new(x, y));
        
        // Calculate anchor offset in viewport coordinates
        let anchor_offset = self.calculate_anchor_offset_viewport(text, &config, viewport_pos)?;
        
        // Apply anchor offset to viewport position
        let final_viewport_pos = viewport_pos + anchor_offset;
        
        // Create a text object with the configuration
        let mut text_obj = Text::new(text.to_string(), final_viewport_pos, sized_font_name);
        text_obj.config.font_size = config.size;
        text_obj.config.color = config.color;
        text_obj.config.alpha = config.alpha;
        text_obj.config.align = config.alignment;
        text_obj.config.max_width = config.max_width;
        text_obj.config.line_spacing = config.line_spacing;
        
        self.text_renderer.render_text(&text_obj)
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

