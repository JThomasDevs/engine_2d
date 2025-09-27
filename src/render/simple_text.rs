use glam::Vec2;
use crate::render::text::TextAlign;

/// Text anchor points for positioning text relative to screen/viewport
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAnchor {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// A simple, clean text object for basic text editor functionality
#[derive(Debug, Clone)]
pub struct SimpleText {
    pub content: String,
    pub font_size: u32,
    pub font_name: Option<String>, // None = use default font
    pub position: Vec2,
    pub color: (f32, f32, f32),
    pub align: TextAlign,
    pub anchor: TextAnchor, // Anchor point for positioning
}

impl SimpleText {
    /// Create a new SimpleText with just content and font size
    pub fn new(content: String, font_size: u32) -> Self {
        Self {
            content,
            font_size,
            font_name: None, // Use default font
            position: Vec2::new(0.0, 0.0),
            color: (1.0, 1.0, 1.0), // White
            align: TextAlign::Center, // Default to center alignment
            anchor: TextAnchor::MiddleCenter, // Default to center anchor
        }
    }

    /// Set the font name (fluent method)
    pub fn font(mut self, font_name: String) -> Self {
        self.font_name = Some(font_name);
        self
    }
    
    /// Set the text color (fluent method)
    pub fn color(mut self, color: (f32, f32, f32)) -> Self {
        self.color = color;
        self
    }
    
    /// Set the position (fluent method)
    pub fn position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }
    
    /// Set the font size (fluent method)
    pub fn size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
        self
    }
    
    /// Set the text alignment (fluent method)
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Set the anchor point (fluent method)
    pub fn anchor(mut self, anchor: TextAnchor) -> Self {
        self.anchor = anchor;
        self
    }
    
    /// Get the font name, returning default if none is set
    pub fn get_font_name(&self) -> &str {
        self.font_name.as_deref().unwrap_or("default")
    }

    /// Update the content
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    /// Update the position
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    /// Update the color
    pub fn set_color(&mut self, color: (f32, f32, f32)) {
        self.color = color;
    }

    /// Update the font size
    pub fn set_font_size(&mut self, font_size: u32) {
        self.font_size = font_size;
    }

    /// Update the font name
    pub fn set_font(&mut self, font_name: Option<String>) {
        self.font_name = font_name;
    }

    /// Calculate the final position based on anchor and viewport bounds
    pub fn calculate_anchored_position(&self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Vec2 {
        let (anchor_x, anchor_y) = match self.anchor {
            TextAnchor::TopLeft => (x_min, y_max),
            TextAnchor::TopCenter => (x_min + (x_max - x_min) * 0.5, y_max),
            TextAnchor::TopRight => (x_max, y_max),
            TextAnchor::MiddleLeft => (x_min, y_min + (y_max - y_min) * 0.5),
            TextAnchor::MiddleCenter => (x_min + (x_max - x_min) * 0.5, y_min + (y_max - y_min) * 0.5),
            TextAnchor::MiddleRight => (x_max, y_min + (y_max - y_min) * 0.5),
            TextAnchor::BottomLeft => (x_min, y_min),
            TextAnchor::BottomCenter => (x_min + (x_max - x_min) * 0.5, y_min),
            TextAnchor::BottomRight => (x_max, y_min),
        };

        // Add the offset position to the anchor point
        Vec2::new(anchor_x + self.position.x, anchor_y + self.position.y)
    }
}

/// Simple text renderer that focuses on basic text editor functionality
pub struct SimpleTextRenderer {
    // We'll integrate with the existing TextRenderer for now
    // This keeps the OpenGL rendering logic intact
    text_renderer: crate::render::text::TextRenderer,
    default_font_path: String,
}

impl SimpleTextRenderer {
    /// Create a new SimpleTextRenderer
    pub fn new(gl: std::rc::Rc<crate::render::gl_wrapper::GlWrapper>) -> Self {
        Self {
            text_renderer: crate::render::text::TextRenderer::new(gl),
            default_font_path: "assets/fonts/default.ttf".to_string(),
        }
    }

    /// Initialize the renderer
    pub fn initialize(&mut self) -> Result<(), String> {
        self.text_renderer.initialize()?;
        
        // Load default font
        self.text_renderer.load_font("default", &self.default_font_path, 16)?;
        
        Ok(())
    }

    /// Set the default font path
    pub fn set_default_font_path(&mut self, path: String) {
        self.default_font_path = path;
    }

    /// Render a SimpleText object
    pub fn render(&mut self, text: &SimpleText) -> Result<(), String> {
        // Get viewport dimensions for anchor calculations
        let viewport = &self.text_renderer.viewport;
        let (x_min, x_max, y_min, y_max) = viewport.logical_bounds;
        
        // Calculate the final position based on anchor
        let final_position = text.calculate_anchored_position(x_min, x_max, y_min, y_max);
        
        // Create a TextConfig with the SimpleText properties
        let mut text_config = crate::render::text::TextConfig::default();
        text_config.font_size = text.font_size;
        text_config.color = text.color;
        text_config.align = text.align;
        
        // Create a Text object directly with the calculated position
        let render_text = crate::render::text::Text::with_config(
            text.content.clone(),
            final_position,
            text.get_font_name().to_string(),
            text_config
        );

        self.text_renderer.render_text(&render_text)
    }

    /// Get a mutable reference to the underlying text renderer for advanced operations
    pub fn text_renderer_mut(&mut self) -> &mut crate::render::text::TextRenderer {
        &mut self.text_renderer
    }
    
    /// Set whether text should be viewport-independent or viewport-relative
    pub fn set_viewport_independent_text(&mut self, independent: bool) {
        self.text_renderer.viewport_mut().set_viewport_independent_text(independent);
    }
}
