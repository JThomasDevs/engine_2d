#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub target_fps: Option<u32>,
    pub show_fps: bool,
    pub vsync: bool,
    pub fullscreen: bool,
    /// Viewport configuration for text rendering
    pub viewport: ViewportConfig,
    /// Fallback font path for text rendering when specified fonts are not found
    pub fallback_font_path: String,
}

/// Configuration for the viewport coordinate system
#[derive(Debug, Clone)]
pub struct ViewportConfig {
    /// Logical coordinate bounds (x_min, x_max, y_min, y_max)
    pub logical_bounds: (f32, f32, f32, f32),
    /// Target text height as fraction of logical viewport height
    pub text_height_fraction: f32,
    /// Base font size for normalization
    pub base_font_size: f32,
    /// Whether text size should be viewport-independent (true) or viewport-relative (false)
    pub viewport_independent_text: bool,
}

impl ViewportConfig {
    /// Create a viewport config with custom logical bounds
    pub fn with_bounds(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Self {
        Self {
            logical_bounds: (x_min, x_max, y_min, y_max),
            text_height_fraction: 0.002, // 0.2% for college essay title size
            base_font_size: 16.0,
            viewport_independent_text: true,
        }
    }
    
    /// Create a viewport config for traditional OpenGL NDC coordinates (-1 to 1)
    pub fn ndc() -> Self {
        Self {
            logical_bounds: (-1.0, 1.0, -1.0, 1.0),
            text_height_fraction: 0.004, // 0.4% for smaller NDC range
            base_font_size: 16.0,
            viewport_independent_text: true,
        }
    }
    
    /// Create a viewport config for pixel coordinates (0 to width, 0 to height)
    pub fn pixel_based(width: f32, height: f32) -> Self {
        Self {
            logical_bounds: (0.0, width, 0.0, height),
            text_height_fraction: 0.002, // 0.2% for college essay title size
            base_font_size: 16.0,
            viewport_independent_text: false, // Pixel-based should be viewport-relative
        }
    }
    
    /// Create a viewport config for UI coordinates (0 to 1, 0 to 1)
    pub fn ui_based() -> Self {
        Self {
            logical_bounds: (0.0, 1.0, 0.0, 1.0),
            text_height_fraction: 0.004, // 0.4% for UI
            base_font_size: 16.0,
            viewport_independent_text: true,
        }
    }
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            logical_bounds: (-1.0, 1.0, -1.0, 1.0), // Default to -1 to 1 range
            text_height_fraction: 0.002, // 0.2% of viewport height - college essay title size
            base_font_size: 16.0,
            viewport_independent_text: true, // Default to viewport-independent text
        }
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "Rust 2D Engine".to_string(),
            window_width: 800,
            window_height: 600,
            target_fps: Some(60),
            show_fps: false,
            vsync: true,
            fullscreen: false,
            viewport: ViewportConfig::default(),
            fallback_font_path: "assets/fonts/default.ttf".to_string(),
        }
    }
}
