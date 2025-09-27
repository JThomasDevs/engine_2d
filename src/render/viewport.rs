use glam::Vec2;

/// Validation error for viewport operations
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidTextHeightFraction(String),
    InvalidBaseFontSize(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidTextHeightFraction(msg) => write!(f, "Invalid text height fraction: {}", msg),
            ValidationError::InvalidBaseFontSize(msg) => write!(f, "Invalid base font size: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Viewport defines the logical coordinate system for rendering
/// All rendering coordinates are specified in this logical space, and the viewport
/// handles conversion to OpenGL's NDC space automatically
#[derive(Debug, Clone)]
pub struct Viewport {
    /// Logical coordinate bounds (what the user specifies coordinates in)
    pub logical_bounds: (f32, f32, f32, f32), // (x_min, x_max, y_min, y_max)
    /// Target text height as fraction of logical viewport height
    pub text_height_fraction: f32,
    /// Base font size for normalization
    pub base_font_size: f32,
    /// Whether text size should be viewport-independent (true) or viewport-relative (false)
    pub viewport_independent_text: bool,
}

impl Viewport {
    /// Create a new viewport with default settings
    pub fn new() -> Self {
        Self {
            logical_bounds: (-1.0, 1.0, -1.0, 1.0), // Default OpenGL NDC
            text_height_fraction: 0.05, // 5% of viewport height
            base_font_size: 16.0,
            viewport_independent_text: true, // Default to viewport-independent text
        }
    }

    /// Create a viewport with custom logical bounds
    pub fn with_bounds(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Self {
        Self {
            logical_bounds: (x_min, x_max, y_min, y_max),
            text_height_fraction: 0.05,
            base_font_size: 16.0,
            viewport_independent_text: true,
        }
    }

    /// Set the text height as a fraction of the logical viewport height
    /// 
    /// # Arguments
    /// * `fraction` - Must be greater than 0.0 and less than or equal to 1.0
    /// 
    /// # Returns
    /// * `Ok(())` if the fraction is valid
    /// * `Err(ValidationError)` if the fraction is invalid
    pub fn set_text_height_fraction(&mut self, fraction: f32) -> Result<(), ValidationError> {
        if fraction <= 0.0 {
            return Err(ValidationError::InvalidTextHeightFraction(
                format!("Text height fraction must be greater than 0.0, got {}", fraction)
            ));
        }
        if fraction > 1.0 {
            return Err(ValidationError::InvalidTextHeightFraction(
                format!("Text height fraction must be less than or equal to 1.0, got {}", fraction)
            ));
        }
        self.text_height_fraction = fraction;
        Ok(())
    }

    /// Set the base font size for normalization
    /// 
    /// # Arguments
    /// * `size` - Must be greater than 0.0
    /// 
    /// # Returns
    /// * `Ok(())` if the size is valid
    /// * `Err(ValidationError)` if the size is invalid
    pub fn set_base_font_size(&mut self, size: f32) -> Result<(), ValidationError> {
        if size <= 0.0 {
            return Err(ValidationError::InvalidBaseFontSize(
                format!("Base font size must be greater than 0.0, got {}", size)
            ));
        }
        self.base_font_size = size;
        Ok(())
    }
    
    /// Set whether text should be viewport-independent or viewport-relative
    /// 
    /// # Arguments
    /// * `independent` - If true, text size stays constant regardless of viewport scale.
    ///                   If false, text scales with viewport (original behavior).
    pub fn set_viewport_independent_text(&mut self, independent: bool) {
        self.viewport_independent_text = independent;
    }
    
    /// Convert from top-left origin coordinates to viewport coordinates
    /// 
    /// This is useful for UI text positioning where (0,0) is top-left.
    /// The input coordinates are in the range [0,1] where:
    /// - (0,0) = top-left corner
    /// - (1,1) = bottom-right corner
    /// 
    /// # Arguments
    /// * `top_left_pos` - Position in top-left coordinate system (0,0 = top-left, 1,1 = bottom-right)
    /// 
    /// # Returns
    /// * Position in viewport coordinate system
    pub fn top_left_to_viewport(&self, top_left_pos: Vec2) -> Vec2 {
        let x_range = self.logical_bounds.1 - self.logical_bounds.0;
        let y_range = self.logical_bounds.3 - self.logical_bounds.2;
        
        // Convert from top-left (0,0) to viewport coordinates
        let viewport_x = self.logical_bounds.0 + top_left_pos.x * x_range;
        let viewport_y = self.logical_bounds.3 - top_left_pos.y * y_range; // Flip Y axis
        
        Vec2::new(viewport_x, viewport_y)
    }

    /// Calculate the scale factor for text rendering based on font size
    /// Returns a direct scale factor where 1.0 = normal size, 2.0 = double size, etc.
    pub fn calculate_scale_factor(&self, font_size: f32) -> f32 {
        // Direct pixel scaling: font_size directly corresponds to pixel size
        // This bypasses the complex viewport scaling logic
        font_size / self.base_font_size
    }

    /// Convert logical coordinates to OpenGL NDC coordinates
    pub fn logical_to_ndc(&self, logical_pos: Vec2) -> Vec2 {
        let x_range = self.logical_bounds.1 - self.logical_bounds.0;
        let y_range = self.logical_bounds.3 - self.logical_bounds.2;
        
        // Convert from logical space to [0,1] then to [-1,1] NDC
        let normalized_x = (logical_pos.x - self.logical_bounds.0) / x_range;
        let normalized_y = (logical_pos.y - self.logical_bounds.2) / y_range;
        
        Vec2::new(normalized_x * 2.0 - 1.0, normalized_y * 2.0 - 1.0)
    }

    /// Convert OpenGL NDC coordinates to logical coordinates
    pub fn ndc_to_logical(&self, ndc_pos: Vec2) -> Vec2 {
        let x_range = self.logical_bounds.1 - self.logical_bounds.0;
        let y_range = self.logical_bounds.3 - self.logical_bounds.2;
        
        // Convert from [-1,1] NDC to [0,1] then to logical space
        let normalized_x = (ndc_pos.x + 1.0) / 2.0;
        let normalized_y = (ndc_pos.y + 1.0) / 2.0;
        
        Vec2::new(
            self.logical_bounds.0 + normalized_x * x_range,
            self.logical_bounds.2 + normalized_y * y_range
        )
    }

    /// Get the logical coordinate ranges
    pub fn get_logical_ranges(&self) -> (f32, f32) {
        (self.logical_bounds.1 - self.logical_bounds.0, self.logical_bounds.3 - self.logical_bounds.2)
    }

    /// Get the logical bounds
    pub fn get_logical_bounds(&self) -> (f32, f32, f32, f32) {
        self.logical_bounds
    }

    /// Set the logical bounds
    pub fn set_logical_bounds(&mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        self.logical_bounds = (x_min, x_max, y_min, y_max);
    }

    /// Get the center of the logical viewport
    pub fn get_center(&self) -> Vec2 {
        Vec2::new(
            (self.logical_bounds.0 + self.logical_bounds.1) / 2.0,
            (self.logical_bounds.2 + self.logical_bounds.3) / 2.0
        )
    }

    /// Get the size of the logical viewport
    pub fn get_size(&self) -> Vec2 {
        Vec2::new(
            self.logical_bounds.1 - self.logical_bounds.0,
            self.logical_bounds.3 - self.logical_bounds.2
        )
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}
